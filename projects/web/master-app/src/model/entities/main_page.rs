use askama::Template;
use askama_web::WebTemplate;
use core_web_types::infrastructure::users::forms::SessionUserInfo;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct MainPage {
   pub text: String,
   pub user_info: Option<SessionUserInfo>,
}

#[derive(Template, WebTemplate)]
#[template(path = "pages/physic_ball.html")]
pub struct PhycisBallPage;