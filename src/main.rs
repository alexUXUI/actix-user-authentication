#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod routes;
pub mod handlers;
pub mod models;
pub mod middleware;
pub mod modules;

mod schema;
mod config;

use actix_web::{App, HttpServer, middleware::Logger};

use db::db_connection::{establish_connection};
use routes::user::user_routes;
use routes::login::login;
use handlers::health::status;

use crate::config::Config;
use dotenv::dotenv;
use env_logger::Env;

use middleware::auth;
use actix_web::middleware::errhandlers::{ErrorHandlers, ErrorHandlerResponse};
use actix_web::{http, dev, Result};

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut()
       .headers_mut()
       .insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("Error"));
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv().ok(); 

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::from_env(Env::default().default_filter_or("info")).init(); 

    let config = Config::from_env().expect("Must set env vars"); 

    println!("Start server {:#?}", config);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %T"))
            .wrap(auth::Auth)
            .data(establish_connection())
            .service(status)
            .service(login())
            .service(user_routes())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
