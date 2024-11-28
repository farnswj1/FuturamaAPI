use askama_axum::{IntoResponse, Response};
use axum::{extract::{OriginalUri, Path, Query, State}, http::StatusCode};

use crate::app::{
    db::Database,
    serializers::query::{NameQuery, Paginator},
    templates::{
        episodes::{
            EpisodeDetailTemplate,
            EpisodeListTemplate
        },
        NotFoundTemplate
    }
};

use super::errors::AppError;

pub async fn get_episodes(
    OriginalUri(uri): OriginalUri,
    State(db): State<Database>,
    episode: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> Result<EpisodeListTemplate, AppError> {
    let Query(NameQuery { name }) = episode.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();
    let path = uri.path().to_string();

    let episodes = db.get_episodes(&name, page, size).await?;
    let count = db.get_total_episodes(&name).await?;
    let total_pages = ((count - 1) / size) + 1;

    Ok(EpisodeListTemplate {
        episodes,
        path,
        total_pages,
        page,
        size,
        name
    })
}

pub async fn get_episode(
    State(db): State<Database>,
    Path(id): Path<String>
) -> Result<Response, AppError> {
    Ok(match db.get_episode(&id).await? {
        Some(episode) => EpisodeDetailTemplate { episode }.into_response(),
        None => (StatusCode::NOT_FOUND, NotFoundTemplate).into_response()
    })
}
