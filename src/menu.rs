use crate::prelude::*;

use askama::Template;
use axum::extract::State;
use axum::response::Html;
use sqlx::types::BigDecimal;

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

pub async fn menu(State(pools): Pools) -> Result<Html<String>, Error> {
    let restaurant_id = 1; // TODO url param
    let menu_items = sqlx::query_as!(
        MenuItem,
        "select name, price from menu_items where restaurant_id = $1",
        restaurant_id
    )
    .fetch_all(&pools.postgres)
    .await.unwrap();

    let menu = MenuTemplate {
        name: "Spaceways",
        menu_items,
    };
    menu.render()?.as_html()
}
