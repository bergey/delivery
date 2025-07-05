use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use tracing::error;

#[derive(Template)]
#[template(path = "menu.html")]
struct MenuTemplate<'a> {
    name: &'a str,
}

pub async fn menu() -> Result<Html<String>, StatusCode> {
    let menu = MenuTemplate { name: "Spaceways" };
    html_response(menu.render())
}

fn html_response(s: askama::Result<String>) -> Result<Html<String>, StatusCode> {
    match s {
        Ok(html) => Ok(Html::from(html)),
        Err(err) => {
            error!("{err}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}
