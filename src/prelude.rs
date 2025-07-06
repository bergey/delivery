use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::{IntoResponse, Response};
use sqlx::PgPool;
use tracing::error;

pub type Pools = State<ConnectionPools>;

#[derive(Clone)]
pub struct ConnectionPools {
    pub postgres: PgPool,
}

// TODO trait to make State wrapper less intrusive?

// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong"),
        )
            .into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub trait AsHtml {
    fn as_html(self) -> Result<Html<String>, Error>;
}

impl AsHtml for String {
    fn as_html(self) -> Result<Html<String>, Error> {
        Ok(Html::from(self))
    }
}
