use axum::http::StatusCode;

pub trait ToHttpCode {
   fn http_code(&self) -> StatusCode;
}

pub trait ToHumanMessage {
   fn message(&self) -> String;
}