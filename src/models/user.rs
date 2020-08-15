use diesel::{PgConnection, RunQueryDsl};
use crate::schema::users;
use serde::{Serialize, Deserialize};
use argonautica::{Hasher, Verifier};
use crate::models;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
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
        use crate::schema::users::dsl::{name};
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;

        let user_already_exists = users
            .filter(name.eq(&user.name))
            .load::<User>(pool);

        if user_already_exists.unwrap().is_empty() {
            let user_password = User::encrypt_password(user.password.to_string());
            let new_user = NewUser {
                name: user.name.to_string(),
                email: user.email.to_string(),
                password: user_password
            };
            diesel::insert_into(users).values(&new_user).execute(pool).expect("Could not create new user");
            return Ok(new_user);
        }

        Err(String::from("Couldn't create new user"))
    }
}

trait UserPassWord {
    fn encrypt_password(password: String) -> String;
    fn verify_password(password: String) -> Result<bool, bool>;
}

impl UserPassWord for User {
    fn encrypt_password(password: String) -> String {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key("\
                secret key that you should really store in a .env file \
                instead of in code, but this is just an example\
            ")
            .hash()
            .unwrap();
    
        println!("{}", &hash);
        hash
    }

    fn verify_password(password: String) -> Result<bool, bool> {
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash("
                $argon2id$v=19$m=4096,t=192,p=4$\
                o2y5PU86Vt+sr93N7YUGgC7AMpTKpTQCk4tNGUPZMY4$\
                yzP/ukZRPIbZg6PvgnUUobUMbApfF9RH6NagL9L4Xr4\
            ")
            .with_password(password)
            .with_secret_key("\
                secret key that you should really store in a .env file \
                instead of in code, but this is just an example\
            ")
            .verify()
            .unwrap_or(false);
    
        match is_valid {
            true => Ok(true),
            false => Err(false)
        }
    }
}