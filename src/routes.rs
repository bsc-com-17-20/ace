use actix_web::{ get, Responder, HttpResponse, post, web, error, Result };

use crate::{
    AppState,
    model::{
        insert_new_application,
        get_all_applications,
        NewUserSchema,
        insert_new_user,
        find_all_user_records,
    },
};

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("api/auth/applications")]
pub async fn get_all_applications_handler(data: web::Data<AppState>) -> Result<impl Responder> {
    let applications = web
        ::block(move || {
            let mut conn = data.pool.get().expect("Couldn't get db connection from pool");
            get_all_applications(&mut conn)
        }).await
        .unwrap()
        .map_err(error::ErrorInternalServerError)
        .unwrap();
    let json = serde_json::to_string(&applications).unwrap();
    Ok(web::Json(json))
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

#[post("api/auth/users/{app_id}")]
pub async fn insert_user_handler(
    data: web::Data<AppState>,
    body: web::Json<NewUserSchema>,
    app_id: web::Path<(String,)>
) -> Result<impl Responder> {
    let (app_id,) = app_id.into_inner();
    let new_user = body.into_inner();

    let user = web
        ::block(move || {
            let mut conn = data.pool.get().expect("Couldn't get db connection from pool");
            insert_new_user(&mut conn, new_user, app_id)
        }).await
        .unwrap()
        .map_err(error::ErrorInternalServerError)
        .unwrap();
    Ok(HttpResponse::Ok().json(user))
}

#[get("api/auth/users/{app_id}")]
pub async fn find_all_user_records_handler(
    data: web::Data<AppState>,
    app_id: web::Path<(String,)>
) -> Result<impl Responder> {
    let (app_id,) = app_id.into_inner();

    let users = web
        ::block(move || {
            let mut conn = data.pool.get().expect("Couldn't get db connection from pool");
            find_all_user_records(&mut conn, app_id)
        }).await
        .unwrap()
        .map_err(error::ErrorInternalServerError)
        .unwrap();

    let json = serde_json::to_string(&users).unwrap();

    Ok(web::Json(json))
}
