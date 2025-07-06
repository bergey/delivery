use axum::extract::State;
use sqlx::PgPool;

pub type Pools = State<ConnectionPools>;

#[derive(Clone)]
pub struct ConnectionPools {
    pub postgres: PgPool,
}

// TODO trait to make State wrapper less intrusive?
