use actix_web::{ Scope, web };
use crate::handlers::user::*;

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("", web::get().to(get_users))
        .route("{id}", web::get().to(get_user))
}