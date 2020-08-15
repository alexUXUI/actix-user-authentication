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

mod schema;
mod config;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;

use db::db_connection::{establish_connection};
use routes::user::user_routes;
use handlers::health::status;
use middleware::authentication::validator;

use crate::config::Config;
use dotenv::dotenv;
use env_logger::Env;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv().ok(); // load env vars

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::from_env(Env::default().default_filter_or("info")).init(); // load logger

    let config = Config::from_env().expect("Must set env vars"); // create a config struct out of the env vars

    println!("Start server {:#?}", config);

    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %T"))
            .wrap(auth)
            .data(establish_connection())
            .service(status)
            .service(user_routes())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
