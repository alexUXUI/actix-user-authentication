use crate::models::user::{User, NewUser, UserLogin, UserLoggedIn, UserLogout};
use crate::db::db_connection::{ pg_pool_handler, PgPool };
use actix_web::{ Responder, web, HttpResponse };

#[derive(Serialize)]
pub struct UsersResponse {
    users: Vec<User>
}

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool).expect("Could not connect to DB from get users handler");
    let all_users = User::get_all(&pg_pool);
    // @todo make sure response can handle potential failer 
    HttpResponse::Ok().json(UsersResponse { users: all_users })
}

#[derive(Serialize)]
pub struct UserResponse {
    user: User
}

pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool).expect("Could not connect to DB from get user handler");
    let user = User::get(&pg_pool, *id);
    // @todo make sure response can handle potential failer 
    HttpResponse::Ok().json(UserResponse { user })
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    name: String,
    email: String
}

#[derive(Serialize)] 
pub struct CreateUserError {
    message: String,
    error: String
}

pub async fn create_user(pool: web::Data<PgPool>, user: web::Json<NewUser>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool).expect("Could not connect to PG from create user handler");
    let new_user = User::create(&pg_pool, user);
    
    match new_user {
        Ok(new_user) => {
            HttpResponse::Ok().json(CreateUserResponse { 
                name: new_user.name, 
                email: new_user.email 
            })
        }
        Err(error) => {
            HttpResponse::Ok().json(CreateUserError { 
                message: String::from("Failed to create user"),
                error
            })
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    user_logged_in: UserLoggedIn
}

#[derive(Debug, Serialize)]
pub struct UserLoginError {
    message: String,
    error: String
}

pub async fn login_user(pool: web::Data<PgPool>, user: web::Json<UserLogin>) -> impl Responder {
    
    let pool = pg_pool_handler(pool).expect("Could not connect to PG from login handler");
    let logged_in_user = User::login(&pool, user);

    match logged_in_user {
        Ok(user) => {
            HttpResponse::Ok().json(UserLoginResponse {
                user_logged_in: user
            })
        },
        Err(error) => {
            HttpResponse::Ok().json(UserLoginError {
                message: String::from("Could not log user in"),
                error
            })
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserLogoutResponse {
    user_logged_out: bool
}

#[derive(Debug, Serialize)]
pub struct UserLogoutError {
    user_logged_out: bool,
    message: String,
    error: String
}

pub async fn logout_user(pool: web::Data<PgPool>, user: web::Json<UserLogout>) -> impl Responder {
    let pool = pg_pool_handler(pool).expect("Could not connect to PG from logout handler");
    let logout_response = User::logout(&pool, user.clone());

    match logout_response {
        Ok(user) => {
            HttpResponse::Ok().json(UserLogoutResponse {
                user_logged_out: true
            })
        },
        Err(error) => {
            HttpResponse::Ok().json(UserLogoutError {
                user_logged_out: false,
                message: String::from(format!("Could not log user {} out", user.id)),
                error: error.to_string()
            })
        }
    }
}