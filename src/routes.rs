use actix_web::{ get, Responder, HttpResponse, post, web, error, Result };

use crate::{ AppState, model::insert_new_application };

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

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
