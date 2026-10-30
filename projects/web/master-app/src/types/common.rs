use askama::Template;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse};

pub struct PageMeta {
    pub title: String,
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct BasePage<'a> {
    pub meta: PageMeta,
    pub content: &'a str,
}

pub struct View<T: Template> {
    pub meta: PageMeta,
    pub template: T,
}

impl<T: Template> IntoResponse for View<T> {
    fn into_response(self) -> Response<Body> {

        let content = match self.template.render() {
            Ok(html) => html,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

        let base = BasePage {
            meta: self.meta,
            content: &content,
        };

        match base.render() {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}
