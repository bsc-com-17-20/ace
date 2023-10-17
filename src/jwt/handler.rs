use actix_web::{
    Responder,
    web,
    cookie::{ Cookie, time::Duration as ActixWebDuration },
    HttpResponse,
    get,
    error,
};
use argon2::{ PasswordHash, Argon2, PasswordVerifier };
use chrono::{ Utc, Duration };
use jsonwebtoken::{ encode, Header, EncodingKey };
use serde_json::json;

use crate::{ model::{ TokenClaims, LoginUserSchema, find_user_record }, AppState };

#[get("/api/auth/login")]
pub async fn login_user_handler(
    data: web::Data<AppState>,
    body: web::Json<LoginUserSchema>
) -> impl Responder {
    let data_clone = data.clone();
    let body_clone = body.clone();
    let user = web
        ::block(move || {
            let mut conn = data_clone.pool.get().expect("Couldn't get db connection from pool");
            find_user_record(&mut conn, body_clone.username)
        }).await
        .unwrap()
        .map_err(error::ErrorInternalServerError)
        .unwrap();

    let is_valid = {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    };

    if !is_valid {
        return HttpResponse::BadRequest().json(
            json!({"status": "fail", "message": "Invalid username or password"})
        );
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user.id,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref())
    ).unwrap();
    println!("{}", token);

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}
