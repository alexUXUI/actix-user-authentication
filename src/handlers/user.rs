use crate::models::user::{User, UserManager, NewUser, UserLogin, UserLoggedIn, UserLogout, NewTokens};
use crate::db::db_connection::{ pg_pool_handler, PgPool };
use actix_web::{ Responder, web, HttpResponse, http::{Cookie}, HttpRequest, HttpMessage };
use crate::modules::jwt::{validate_token};

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

            let cookie = Cookie::build("refresh_token", user.clone().refresh_token.unwrap())
                .domain("http://localhost:3000")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(UserLoginResponse {
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

#[derive(Serialize, Debug, Clone)]
pub struct ReauthResponse {
    new_acccess_token: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReauthRequestBody {
    id: i32
}

pub async fn reauth_user(req: HttpRequest, pool: web::Data<PgPool>, user: web::Json<ReauthRequestBody>) -> impl Responder {

    let refresh_token = req
        .cookie("refresh_token")
        .unwrap()
        .to_string();

    let token_is_still_valid = validate_token(&refresh_token);

    let pg_pool = pg_pool_handler(pool).expect("Could not connect to PG from reauth EP");

    match token_is_still_valid {
        true => {
            let refresh_token_is_valid = User::validate_refresh_token(&pg_pool, refresh_token, &user.id);

            match refresh_token_is_valid {
                Ok(token) => {
                    let reauthed_user = User::reauth(&pg_pool, &user.id);

                    match reauthed_user {
                        Ok(new_tokens) => {
                            let NewTokens { refresh_token, access_token } = new_tokens;
                            
                            let cookie = Cookie::build("refresh_token", refresh_token)
                                .domain("http://localhost:3000")
                                .secure(true)
                                .http_only(true)
                                .finish();

                            HttpResponse::Ok()
                                .cookie(cookie)
                                .json(ReauthResponse {
                                    new_acccess_token: access_token
                                }
                            )
                        },
                        Err(error) => {
                            HttpResponse::Ok().json(ReauthResponse {
                                new_acccess_token: String::from(format!("failed: {}", error))
                            })
                        }
                    }
                },
                Err(error) => {
                    HttpResponse::Ok().json(ReauthResponse {
                        new_acccess_token: String::from(format!("failed: {}", error))
                    })
                }
            }
        },
        false => {
            HttpResponse::Ok().json(ReauthResponse {
                new_acccess_token: String::from("failed")
            })
        }
    }
}