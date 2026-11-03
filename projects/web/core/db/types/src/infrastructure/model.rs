use axum::http::StatusCode;

pub trait ToHttpCode {
   fn http_code(&self) -> StatusCode;
}