use axum::{Router, routing::get};
use tokio::try_join;

use delivery::observability;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    observability::init()?;

    let app = Router::new().route("/menu", get(delivery::menu::menu));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    try_join!(
        axum::serve(listener, app),
        observability::prometheus_metrics()
    )?;

    Ok(())
}
