use crate::auth::{perform_login, SESSION_ROLE};
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
        Ok(_) => {
            // 1. Fetch the role from the session that was just created
            let role: Option<String> = session.get(SESSION_ROLE).await.unwrap_or(None);

            // 2. Redirect based on that role
            if role.as_deref() == Some("ADMIN") {
                Ok(Redirect::to("/admin"))
            } else {
                Ok(Redirect::to("/portal"))
            }
        }
        Err(e) => {
            // If login fails, show the error message on the login page
            let tpl = LoginTemplate { error: Some(e) };
            Err(Html(tpl.render().unwrap()))
        }
    }
}

pub async fn logout(session: Session) -> Redirect {
    session.clear().await;
    Redirect::to("/login")
}
