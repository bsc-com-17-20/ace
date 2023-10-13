use diesel::{
    prelude::{ Queryable, Insertable },
    Selectable,
    PgConnection,
    QueryResult,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Application {
    pub id: String,
    pub app_name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::applications)]
struct NewApplication<'a> {
    id: &'a str,
    app_name: String,
}

pub fn insert_new_application(conn: &mut PgConnection, name: String) -> QueryResult<Application> {
    use crate::schema::applications::dsl::*;

    let uid = format!("{}", uuid::Uuid::new_v4());
    let new_application = NewApplication {
        id: &uid,
        app_name: name,
    };

    diesel
        ::insert_into(applications)
        .values(&new_application)
        .execute(conn)
        .expect("Error inserting application");

    let application = applications
        .filter(id.eq(&uid))
        .first::<Application>(conn)
        .expect("Error loading application that was just inserted");
    Ok(application)
}

pub fn get_all_applications(conn: &mut PgConnection) -> QueryResult<Vec<Application>> {
    use crate::schema::applications::dsl::*;

    let application = applications.load::<Application>(conn).expect("Error loading applications");
    Ok(application)
}
