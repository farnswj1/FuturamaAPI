use askama_axum::{IntoResponse, Response, Template};
use axum::extract::{Path, Query, State};
use sqlx::{query_as, PgPool};

use crate::app::serializers::{
    models::Episode,
    query::{NameQuery, Paginator}
};

use super::NotFoundTemplate;

#[derive(Template)]
#[template(path = "episodes/list.html")]
pub struct EpisodeListTemplate {
    episodes: Vec<Episode>
}

pub async fn get_episodes(
    State(db): State<PgPool>,
    episode: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> EpisodeListTemplate {
    let Query(NameQuery { name }) = episode.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();

    let episodes =
        query_as::<_, Episode>("select * from episodes where name ilike $1 limit $2 offset $3")
        .bind(format!("%{}%", name))
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(&db)
        .await
        .unwrap();

    EpisodeListTemplate { episodes }
}

#[derive(Template)]
#[template(path = "episodes/detail.html")]
pub struct EpisodeDetailTemplate {
    episode: Episode
}

pub async fn get_episode(
    State(db): State<PgPool>,
    Path(id): Path<String>
) -> Response {
    let query =
        query_as::<_, Episode>("select * from episodes where id::text = $1")
        .bind(id)
        .fetch_one(&db)
        .await;

    match query {
        Ok(episode) => EpisodeDetailTemplate { episode }.into_response(),
        Err(_) => NotFoundTemplate.into_response()
    }
}
