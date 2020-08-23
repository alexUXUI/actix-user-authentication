use actix_web::{ Scope, web };
use crate::handlers::user::{logout_user, reauth_user};

pub fn session() -> Scope {
    web::scope("/session")
        .route("/logout", web::post().to(logout_user))
        .route("/refresh", web::post().to(reauth_user))
}
