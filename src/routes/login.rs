use actix_web::{ Scope, web };
use crate::handlers::user::login_user;

pub fn login() -> Scope {
    web::scope("/app")
        .route("/login", web::post().to(login_user))
}