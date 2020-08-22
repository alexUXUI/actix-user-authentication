extern crate chrono;

use chrono::{Duration, Utc};
use std::convert::TryFrom;

use diesel::{PgConnection, RunQueryDsl};
use serde::{Serialize, Deserialize};

use crate::modules::jwt::{jwt_factory, Claims};

use crate::schema::users;
use crate::models;
use crate::modules::hash::{hash_password, verify_password};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub refresh_token: Option<String>
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct UserLoggedIn {
    pub name: String,
    pub email: String,
    pub jwt: String,
    pub refresh_token: Option<String>
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserLogout {
    pub id: i32
}

impl User {
    pub fn get_all(pool: &PgConnection) -> Vec<User> {
        use crate::schema::users::dsl::*;

        users
            .load::<User>(pool)
            .expect("Could not query PG for all users")
    }

    pub fn get(pool: &PgConnection, user_id: i32) -> User {
        use crate::schema::users::dsl::*;
        use crate::diesel::QueryDsl;
        use crate::schema::users::columns::id;
        use crate::diesel::ExpressionMethods;

        users
            .filter(id.eq(user_id))
            .first(pool)
            .expect("Could not query PG for user by ID")
    }

    pub fn create(pool: &PgConnection, user: actix_web::web::Json<models::user::NewUser>) -> Result<NewUser, String> {
        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::{email};
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;

        let user_already_exists = users
            .filter(email.eq(&user.email))
            .load::<User>(pool)
            .unwrap();

        if user_already_exists.is_empty() {
            
            let user_password = hash_password(user.password
                .to_string())
                .expect("Could not hash password");

            let new_user = NewUser {
                name: user.name.to_string(),
                email: user.email.to_string(),
                password: user_password
            };
            
            diesel::insert_into(users)
                .values(&new_user)
                .execute(pool)
                .expect("Could not create new user");

            return Ok(new_user);
        }

        Err(String::from("Email already in use"))
    }

    pub fn login(pool: &PgConnection, user: actix_web::web::Json<models::user::UserLogin>) -> Result<UserLoggedIn, String> {
        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::{name};
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;

        let existing_user = users
            .filter(name.eq(&user.name))
            .get_result::<User>(pool);
        
        match existing_user {
           Ok(registered_user) => User::handle_login(registered_user, user, pool),
           Err(error) => Err(String::from("User does not exist"))
        }
    }

    pub fn logout(pool: &PgConnection, user: UserLogout) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::{id, refresh_token};
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;

        let absent_string: Option<String> = None;

        let user_without_refresh_token: Result<User, diesel::result::Error> = diesel::update(
            users.filter(id.eq(user.id))
        )
        .set(refresh_token.eq(absent_string))
        .get_result(pool);

        match user_without_refresh_token {
            Ok(user) => Ok(user),
            Err(error) => Err(error)
        }
    }
}

trait UserPassWord {
    fn handle_login(existing_user: User, user: actix_web::web::Json<models::user::UserLogin>, pool: &PgConnection) -> Result<UserLoggedIn, String>;
}

impl UserPassWord for User {

    fn handle_login(existing_user: User, user: actix_web::web::Json<models::user::UserLogin>, pool: &PgConnection) -> Result<UserLoggedIn, String> {

        use crate::schema::users::dsl::*;
        use crate::schema::users::dsl::{id, refresh_token};
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;

        let password_is_valid = verify_password(
            existing_user.password, 
            user.password.to_string()
        );

        let one_week_from_now = Utc::now() + Duration::days(7);

        let timestamp = usize::try_from(one_week_from_now.timestamp())
            .unwrap();        

        let refresh_token_claims = Claims {
            exp: timestamp,
        };

        let refresh_jwt = jwt_factory(refresh_token_claims);

        let user_with_refresh_token: Result<User, diesel::result::Error> = diesel::update(
                users.filter(id.eq(existing_user.id))
            )
            .set(refresh_token.eq(refresh_jwt.clone()))
            .get_result(pool);

        match user_with_refresh_token {
            Ok(user_with_session) => {
                match password_is_valid {
                    Ok(result) => {
                        match result {
                            true => {
                                let fifteen_min_from_now = Utc::now() + Duration::minutes(15);

                                let timestamp = usize::try_from(fifteen_min_from_now.timestamp())
                                    .unwrap();

                                let access_token_claims = Claims {
                                    exp: timestamp
                                };

                                let logged_in_user = UserLoggedIn {
                                    email: user_with_session.email,
                                    jwt: jwt_factory(access_token_claims),
                                    name: user_with_session.name,
                                    refresh_token: user_with_session.refresh_token
                                };
                                Ok(logged_in_user)
                            },
                            false => {
                                Err(String::from("Incorrect password"))
                            }
                        }
                    },
                    Err(_) => {
                        Err(String::from("Could not verify password"))
                    }
                }        
            },
            Err(error) => {
                Err(String::from("Could not create refresh token"))
            }
        }
    }
}