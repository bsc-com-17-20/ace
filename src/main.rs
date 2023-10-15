mod schema;
mod config;
mod model;
mod jwt;
mod routes;

use actix_cors::Cors;
// use actix_web::rt::Runtime;
use actix_web::{ App, middleware::Logger, HttpServer, web, http::header };
use config::Config;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
// use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::Pool;
// use deadpool_diesel::postgres::{ Runtime, Manager, Pool };
use std::env;

use crate::jwt::handler::login_user_handler;
use crate::routes::{
    health_checker_handler,
    register_app_handler,
    get_all_applications_handler,
    insert_user_handler,
    find_all_user_records_handler,
};

pub struct AppState {
    env: Config,
    pool: DbPool,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    dotenv().ok();

    let config = Config::init();
    let pool = get_connection_pool();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { env: config.clone(), pool: pool.clone() }))
            // .app_data(web::Data::new(pool.clone()))
            .service(health_checker_handler)
            .service(login_user_handler)
            .service(register_app_handler)
            .service(get_all_applications_handler)
            .service(insert_user_handler)
            .service(find_all_user_records_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run().await
}

fn get_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool")
}
