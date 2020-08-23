use actix_web::{ Scope, web };
use crate::handlers::user::*;
use crate::middleware::auth;

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("/all", web::get().to(get_users))
        .route("/create", web::post().to(create_user))
        .route("/{id}", web::get().to(get_user)
    )
}