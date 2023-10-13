// @generated automatically by Diesel CLI.

diesel::table! {
    applications (id) {
        id -> Varchar,
        app_name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        pass_word -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        application_id -> Varchar,
    }
}

diesel::joinable!(users -> applications (application_id));

diesel::allow_tables_to_appear_in_same_query!(
    applications,
    users,
);
