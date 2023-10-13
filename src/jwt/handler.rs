use actix_web::{
    Responder,
    web,
    cookie::{ Cookie, time::Duration as ActixWebDuration },
    HttpResponse,
    get,
};
use chrono::{ Utc, Duration };
use jsonwebtoken::{ encode, Header, EncodingKey };
use serde_json::json;

use crate::{ model::TokenClaims, AppState };

#[get("/auth/login")]
pub async fn login_user_handler(data: web::Data<AppState>) -> impl Responder {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: (1).to_string(),
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
