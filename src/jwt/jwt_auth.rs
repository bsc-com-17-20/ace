use core::fmt;
use std::future::{ Ready, ready };

use actix_web::{
    FromRequest,
    Error as ActixWebError,
    web,
    http,
    error::ErrorUnauthorized,
    HttpMessage,
};
use jsonwebtoken::{ decode, DecodingKey, Validation };
use serde::Serialize;

use crate::{ model::TokenClaims, AppState };

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload
    ) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match
            decode::<TokenClaims>(
                &token.unwrap(),
                &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
                &Validation::default()
            )
        {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "You are not logged in, please provide token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id = uuid::Uuid::parse_str(&claims.sub.as_str()).unwrap();
        req.extensions_mut().insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
