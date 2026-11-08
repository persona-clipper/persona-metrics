use askama::Template;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse};
use smart_default::SmartDefault;
use sqlx::Decode;
use crate::infrastructure::sessions::context::SessionContext;

#[derive(SmartDefault, Debug)]
pub struct Flags {
    #[default = true]
    pub show_header: bool,
    #[default = true]
    pub show_footer: bool,
    #[default = true]
    pub show_sidebar: bool,
}

#[derive(Default, Debug)]
pub struct PageMeta<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub flags: Flags,
    pub session: Option<SessionContext>,
}

#[derive(Template, Debug)]
#[template(path = "base.html")]
pub struct BasePage<'a> {
    pub meta: PageMeta<'a>,
    pub content: &'a str,
}

pub struct View<'a, T: Template> {
    pub meta: PageMeta<'a>,
    pub template: T,
}

impl<T: Template> IntoResponse for View<'_, T> {
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
