use axum::extract::State;
use axum::{Form, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use core_library_db_types::store::users::password::Password;
use core_library_db_types::store::users::user::CreateUserRow;
use core_web_types::modules::users::forms::RegistrationUserForm;

pub async fn insert_user(
    form: CreateUserRow,
    db: PgPool,
) -> Result<i32, sqlx::Error> {
    // let password = match Password::new(&form.password, &form.confirm_password) {
    //     Ok(password) => password,
    //     Err(err) => {
    //         return (StatusCode::UNPROCESSABLE_ENTITY, "").into_response()
    //     }
    // };

    sqlx::query_scalar!(
        r#"
        INSERT INTO users(username, display_name, email, password_hash)
        VALUES($1, $2, $3, $4)
        RETURNING id
        "#,
        form.username,
        form.display_name,
        form.email,
        form.password_hash
    ).fetch_one(&db).await


    // (StatusCode::OK, password.to_string()).into_response()
}