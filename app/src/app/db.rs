use sqlx::{postgres::PgPoolOptions, query_as, query_scalar, Error, Pool, Postgres};

use super::serializers::models::{Character, Episode};

#[derive(Clone)]
pub struct Database {
    client: Pool<Postgres>
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let client = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;

        Ok(Self { client })
    }

    pub async fn get_characters(&self, name: &str, page: i64, size: i64) -> Result<Vec<Character>, Error> {
        query_as::<_, Character>("select * from characters where name ilike $1 limit $2 offset $3")
            .bind(format!("%{}%", name))
            .bind(size)
            .bind((page - 1) * size)
            .fetch_all(&self.client)
            .await
    }

    pub async fn get_character(&self, id: &str) -> Result<Option<Character>, Error> {
        query_as::<_, Character>("select * from characters where id::text = $1")
            .bind(id)
            .fetch_optional(&self.client)
            .await
    }

    pub async fn get_total_characters(&self, name: &str) -> Result<i64, Error> {
        query_scalar("select count(*) from characters where name ilike $1")
            .bind(format!("%{}%", name))
            .fetch_one(&self.client)
            .await
    }

    pub async fn get_episodes(&self, name: &str, page: i64, size: i64) -> Result<Vec<Episode>, Error> {
        query_as::<_, Episode>("select * from episodes where name ilike $1 limit $2 offset $3")
            .bind(format!("%{}%", name))
            .bind(size)
            .bind((page - 1) * size)
            .fetch_all(&self.client)
            .await
    }

    pub async fn get_episode(&self, id: &str) -> Result<Option<Episode>, Error> {
        query_as::<_, Episode>("select * from episodes where id::text = $1")
            .bind(id)
            .fetch_optional(&self.client)
            .await
    }

    pub async fn get_total_episodes(&self, name: &str) -> Result<i64, Error> {
        query_scalar("select count(*) from episodes where name ilike $1")
            .bind(format!("%{}%", name))
            .fetch_one(&self.client)
            .await
    }
}
