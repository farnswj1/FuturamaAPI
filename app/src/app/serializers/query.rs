use std::num::NonZeroU64;

use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Default, Deserialize, FromRow)]
pub struct NameQuery {
    pub name: String
}

#[derive(Deserialize)]
pub struct Paginator {
    pub page: NonZeroU64,
    pub size: NonZeroU64
}

impl Default for Paginator {
    fn default() -> Self {
        Self {
            page: NonZeroU64::new(1).unwrap(),
            size: NonZeroU64::new(30).unwrap()
        }
    }
}
