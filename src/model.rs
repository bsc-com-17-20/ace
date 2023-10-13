use diesel::{ prelude::Queryable, Selectable };
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Application {
    pub id: String,
    pub app_name: String,
}
