use diesel::{PgConnection, RunQueryDsl};
use argonautica::{Hasher, Verifier};
use serde::{Serialize, Deserialize};

use crate::modules::jwt::jwt_factory;

use crate::schema::users;
use crate::models;
use crate::config::Config;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
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
    pub jwt: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub exp: usize,
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
            .load::<User>(pool);

        if user_already_exists.unwrap().is_empty() {
            let user_password = User::encrypt_password(user.password.to_string()).expect("Could not encrypt password");
            let new_user = NewUser {
                name: user.name.to_string(),
                email: user.email.to_string(),
                password: user_password.to_string()
            };
            diesel::insert_into(users).values(&new_user).execute(pool).expect("Could not create new user");
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
            .get_result::<User>(pool)
            .expect("Could not find user in PG");

        let password_is_valid = User::verify_password(existing_user.password.to_string(), user.password.to_string());
        
        match password_is_valid {
            Ok(_) => {
                let logged_in_user = UserLoggedIn {
                    email: existing_user.email,
                    jwt: jwt_factory(),
                    name: existing_user.name
                };

                Ok(logged_in_user)
            },
            Err(_) => {
                Err("Password is not valid".to_string())
            }
        }
    }
}

trait UserPassWord {
    fn encrypt_password(password: String) -> Result<String, argonautica::Error>;
    fn verify_password(hash: String, password: String) -> Result<bool, argonautica::Error>;
}

impl UserPassWord for User {

    fn encrypt_password(password: String) -> Result<String, argonautica::Error> {
        let config = Config::from_env().expect("Must set env vars in config file");

        let mut hasher = Hasher::default();

        hasher
            .with_password(password)
            .with_secret_key(config.hash_secret_key)
            .hash()
    }

    fn verify_password(hash: String, password: String) -> Result<bool, argonautica::Error> {
        let config = Config::from_env().expect("Must set env vars in config file");

        let mut verifier = Verifier::default();
        
        verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(config.hash_secret_key)
            .verify()
    }
}