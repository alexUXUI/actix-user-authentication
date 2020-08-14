pub mod db;
pub mod routes;
pub mod handlers;
pub mod models;

mod schema;
mod config;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use actix_web::{App, HttpServer, middleware::Logger};
use db::db_connection::{establish_connection};
use routes::user::user_routes;
use handlers::health::status;

use crate::config::Config;
use dotenv::dotenv;
use env_logger::Env;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {

    dotenv().ok(); // load env vars
    env_logger::from_env(Env::default().default_filter_or("info")).init(); // load logger

    let config = Config::from_env().expect("Must set env vars"); // create a config struct out of the env vars

    println!("Start server {:#?}", config);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(establish_connection())
            .service(status)
            .service(user_routes())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
