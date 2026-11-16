use crate::auth::perform_login;
use crate::models::LoginForm;
use askama::Template;
use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use sqlx::PgPool;
use tower_sessions::Session;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

pub async fn login_form() -> Html<String> {
    let tpl = LoginTemplate { error: None };
    Html(tpl.render().unwrap())
}

pub async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, Html<String>> {
    match perform_login(&pool, &session, &form.email, &form.password).await {
        Ok(_) => Ok(Redirect::to("/admin")),
        Err(e) => {
            let tpl = LoginTemplate { error: Some(e) };
            Err(Html(tpl.render().unwrap()))
        }
    }
}

pub async fn logout(session: Session) -> Redirect {
    session.clear().await;
    Redirect::to("/login")
}
