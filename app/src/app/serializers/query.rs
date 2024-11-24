use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Default, Deserialize, FromRow)]
pub struct NameQuery {
    pub name: String
}

#[derive(Deserialize)]
pub struct Paginator {
    pub page: i64,
    pub size: i64
}

impl Default for Paginator {
    fn default() -> Self {
        Self { page: 1, size: 30 }
    }
}
