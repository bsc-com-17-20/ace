use argon2::{ password_hash::{ SaltString, rand_core::OsRng }, Argon2, PasswordHasher };
use chrono::{ DateTime, Utc };
use diesel::{
    prelude::{ Queryable, Insertable, Identifiable },
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

#[derive(Debug, Queryable, Identifiable, Selectable, Insertable, PartialEq)]
#[diesel(belongs_to(Application))]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub application_id: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

pub struct NewUser<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

pub fn insert_new_user(
    conn: &mut PgConnection,
    user: NewUser,
    app_id: String
) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    let uid = format!("{}", uuid::Uuid::new_v4());
    let salt = SaltString::generate(&mut OsRng);
    let hased_password = Argon2::default()
        .hash_password(user.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let new_user = User {
        id: uid.clone(),
        username: user.username.to_string(),
        email: user.email.to_string(),
        password: hased_password,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        application_id: app_id,
    };

    diesel::insert_into(users).values(&new_user).execute(conn).expect("Error inserting user");

    let usr = users
        .filter(id.eq(&uid))
        .first::<User>(conn)
        .expect("Error loading user that was just inserted");

    Ok(usr)
}

pub fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

pub fn find_user_record(conn: &mut PgConnection, user_id: String) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(&user_id))
        .first::<User>(conn)
        .expect("Error loading user that was just inserted");

    Ok(user)
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
