use crate::prelude::*;

use askama::Template;
use axum::extract::Path;
use axum::extract::State;
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

pub async fn menu(
    State(pools): Pools,
    Path(restaurant_id): Path<RestaurantId>,
) -> Page {
    let o_name = restaurant_name(pools.clone(), restaurant_id).await?;
    let name = unwrap_or_404(o_name)?;
    let menu_items = sqlx::query_as!(
        MenuItem,
        "select name, price from menu_items where restaurant_id = $1",
        restaurant_id as RestaurantId
    )
    .fetch_all(&pools.postgres)
    .await?;

    let menu = MenuTemplate {
        name: &name,
        menu_items,
    };
    menu.render()?.as_html()
}

async fn restaurant_name(
    pools: ConnectionPools,
    id: RestaurantId,
) -> Result<Option<String>, Error> {
    let row = sqlx::query!(
        "select name from restaurants where restaurant_id = $1",
        id as RestaurantId
    )
    .fetch_optional(&pools.postgres)
    .await?;
    Ok(row.map(|r| r.name))
}
