use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub gender: Option<String>,
    pub species: Option<String>,
    pub status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub image: Option<String>
}

#[derive(FromRow)]
pub struct Episode {
    pub id: Uuid,
    pub name: String,
    pub production_code: String,
    pub season: i16,
    pub number: i16
}
