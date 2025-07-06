use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use tracing::error;

#[derive(Template)]
#[template(path = "menu.html")]
struct MenuTemplate<'a> {
    name: &'a str,
    menu_items: Vec<MenuItem>,
}

struct MenuItem {
    name: String,
    price: f64, // should be decimal?
}

pub async fn menu() -> Result<Html<String>, StatusCode> {
    let menu = MenuTemplate {
        name: "Spaceways",
        menu_items: vec![
            MenuItem {
                name: "Peanut Noodles".to_owned(),
                price: 10.5,
            },
            MenuItem {
                name: "Kimchi".to_owned(),
                price: 5.00,
            },
            MenuItem {
                name: "Shawarma".to_owned(),
                price: 17.50,
            },
        ],
    };
    html_response(menu.render())
}

fn html_response(s: askama::Result<String>) -> Result<Html<String>, StatusCode> {
    match s {
        Ok(html) => Ok(Html::from(html)),
        Err(err) => {
            error!("{err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
