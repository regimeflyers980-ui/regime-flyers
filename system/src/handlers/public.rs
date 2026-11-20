use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> Html<String> {
    let tpl = IndexTemplate;
    Html(tpl.render().unwrap())
}
