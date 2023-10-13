mod config;
mod model;
mod jwt;
use std::vec;

use actix_cors::Cors;
use actix_web::{
    Responder,
    get,
    HttpResponse,
    App,
    middleware::Logger,
    HttpServer,
    web,
    http::header,
};
use config::Config;
use dotenv::dotenv;

use crate::jwt::handler::login_user_handler;

pub struct AppState {
    env: Config,
}

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

// fn config(conf: &mut web::ServiceConfig) {
//     let scope = web::scope("/api").service(health_checker_handler).service(login_user_handler);
//     conf.service(scope);
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { env: config.clone() }))
            .service(health_checker_handler)
            .service(login_user_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run().await
}
