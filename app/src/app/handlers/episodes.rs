use axum::extract::{OriginalUri, Path, Query, State};

use crate::app::{
    db::Database,
    serializers::query::{NameQuery, Paginator},
    templates::episodes::{EpisodeDetailTemplate, EpisodeListTemplate}
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
) -> Result<EpisodeDetailTemplate, AppError> {
    let episode = db.get_episode(&id).await?.ok_or(AppError::NotFound)?;
    Ok(EpisodeDetailTemplate { episode })
}
