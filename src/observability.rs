use anyhow::Result;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use std::time::Instant;
use tracing::warn;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() -> Result<()> {
    if std::env::var("LOG_FORMAT") == Ok("pretty".to_string()) {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer().json().flatten_event(true))
            .with(EnvFilter::from_default_env())
            .init();
    }

    Ok(())
}

pub fn hist_time_since(hist: &prometheus::Histogram, start: Instant) {
    let elapsed = Instant::now() - start;
    hist.observe(elapsed.as_secs_f64());
}

pub async fn prometheus_metrics() -> std::io::Result<()> {
    let router = Router::new().route("/metrics", get(string_metrics));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

// Axum wants every handler to be async
async fn string_metrics() -> Result<String, StatusCode> {
    let metrics = prometheus::gather();
    let encoder = prometheus::TextEncoder::new();
    match encoder.encode_to_string(&metrics) {
        Ok(s) => Ok(s),
        Err(err) => {
            warn!("error encoding metrics: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
