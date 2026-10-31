use askama::Template;
use askama_web::WebTemplate;



#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct MainPage {
   pub text: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "pages/registration.html")]
pub struct RegistrationForm;

#[derive(Template, WebTemplate)]
#[template(path = "pages/physic_ball.html")]
pub struct PhycisBallPage;