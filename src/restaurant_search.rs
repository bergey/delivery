use crate::prelude::*;

use askama::Template;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
// use sqlx::postgres::types::PgPoint;

#[derive(Template)]
#[template(path = "restaurants.html")]
struct RestaurantTemplate {
    restaurants: Vec<Restaurant>,
}

struct Restaurant {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Point {
    x: Option<f64>,
    y: Option<f64>,
}

impl Point {
    pub fn to_postgres(&self) -> sqlx::postgres::types::PgPoint {
        sqlx::postgres::types::PgPoint {
            x: self.x.unwrap_or(0.0),
            y: self.y.unwrap_or(0.0),
        }
    }
}

pub async fn restaurant_search(State(pools): Pools, Query(point): Query<Point>) -> Page {
    let restaurants = sqlx::query_as!(
        Restaurant,
        "select restaurant_id as id, name from restaurants order by $1 <-> location limit 10",
        point.to_postgres()
    )
    .fetch_all(&pools.postgres)
    .await?;

    let page = RestaurantTemplate { restaurants };
    page.render()?.as_html()
}
