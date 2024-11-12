use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Default, Deserialize, FromRow)]
pub struct NameQuery {
    pub name: String
}

#[derive(Deserialize)]
pub struct Paginator {
    pub page: i16,
    pub size: i16
}

impl Default for Paginator {
    fn default() -> Self {
        Self { page: 1, size: 30 }
    }
}
