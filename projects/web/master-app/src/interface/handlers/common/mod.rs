use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::http::Uri;
use crate::infrastructure::base_view::{Flags, PageMeta, View};

#[derive(Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundPage {
    pub invalid_uri: String,
}
pub async fn not_found_handler(uri: Uri) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Что-то пошло не так".into(),
            description: None,
            flags: Flags {
                show_header: false,
                show_footer: false,
                show_sidebar: false,
            },
            session: None,
        },
        template: NotFoundPage {
            invalid_uri: uri.to_string(),
        },
    }
}