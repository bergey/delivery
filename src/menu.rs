use crate::prelude::*;

use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use sqlx::types::BigDecimal;
use tracing::error;

#[derive(Template)]
#[template(path = "menu.html")]
struct MenuTemplate<'a> {
    name: &'a str,
    menu_items: Vec<MenuItem>,
}

struct MenuItem {
    name: String,
    price: BigDecimal,
}

pub async fn menu(State(pools): Pools) -> Result<Html<String>, StatusCode> {
    let restaurant_id = 1; // TODO url param
    let menu_items = sqlx::query_as!(
        MenuItem,
        "select name, price from menu_items where restaurant_id = $1",
        restaurant_id
    )
    .fetch_all(&pools.postgres)
    .await.unwrap(); // TODO axum handler for anyhow error

    let menu = MenuTemplate {
        name: "Spaceways",
        menu_items,
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
