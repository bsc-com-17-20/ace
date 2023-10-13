mod schema;
mod config;
mod model;
mod jwt;

use actix_cors::Cors;
use actix_web::{ error, post };
use actix_web::{
    Responder,
    get,
    HttpResponse,
    App,
    middleware::Logger,
    HttpServer,
    web,
    http::header,
    Result,
};
use config::Config;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use model::insert_new_application;
use std::env;
use crate::jwt::handler::login_user_handler;

pub struct AppState {
    env: Config,
    pool: DbPool,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/api/auth/applications/{app_name}")]
pub async fn register_app_handler(
    data: web::Data<AppState>,
    app_name: web::Path<(String,)>
) -> Result<impl Responder> {
    let (app_name,) = app_name.into_inner();

    let application = web
        ::block(move || {
            let mut conn = data.pool.get().expect("Couldn't get db connection from pool");
            insert_new_application(&mut conn, app_name)
        }).await
        .unwrap()
        .map_err(error::ErrorInternalServerError)
        .unwrap();
    Ok(HttpResponse::Ok().json(application))
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::new(manager).unwrap();
    let config = Config::init();

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
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run().await
}
