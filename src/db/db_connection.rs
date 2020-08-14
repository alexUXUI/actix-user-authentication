  
use diesel::pg::PgConnection;
use dotenv::dotenv;
use diesel::r2d2::{ 
    Pool, PooledConnection, ConnectionManager, PoolError 
};
use actix_web::{ web, HttpResponse };
use crate::config::Config;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let db_url = Config::from_env().expect("Please set ENV vars").database_url;
    init_pool(&db_url).expect("Failed to create connection pool")
}

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
        .get()
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}