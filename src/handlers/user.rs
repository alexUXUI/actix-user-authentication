use crate::models::user::{User, NewUser, UserLogin, UserLoggedIn};
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
pub struct NewUserResponse {
    new_user: NewUser
}

pub async fn create_user(pool: web::Data<PgPool>, user: web::Json<NewUser>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool).expect("Could not connect to PG from create user handler");
    let new_user = User::create(&pg_pool, user);
    // @todo make sure response can handle potential failer 
    HttpResponse::Ok().json(NewUserResponse { new_user: new_user.unwrap() })
}

#[derive(Debug, Serialize)]
pub struct UserLoggedInResponse {
    user_logged_in: UserLoggedIn
}

pub async fn login_user(pool: web::Data<PgPool>, user: web::Json<UserLogin>) -> impl Responder {
    let pool = pg_pool_handler(pool).expect("Could not connect to PG from login handler");
    let logged_in_user = User::login(&pool, user);

    println!("Does handler have access to logged in user with JWT? {:#?}", logged_in_user);

    HttpResponse::Ok().body("logged in")
}