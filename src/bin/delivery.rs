use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio::try_join;

use delivery::observability;
use delivery::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    observability::init()?;

    let connection_pools = ConnectionPools {
        postgres: PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://bergey:password@localhost:15432/delivery")
            .await?,
    };

    let app = Router::new()
        .route("/menu", get(delivery::menu::menu))
        .with_state(connection_pools);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    try_join!(
        axum::serve(listener, app),
        observability::prometheus_metrics()
    )?;

    Ok(())
}

