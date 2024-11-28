use askama_axum::{IntoResponse, Response};
use axum::extract::{OriginalUri, Path, Query, State};

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

pub async fn get_episodes(
    OriginalUri(uri): OriginalUri,
    State(db): State<Database>,
    episode: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> EpisodeListTemplate {
    let Query(NameQuery { name }) = episode.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();
    let path = uri.path().to_string();

    let episodes = db.get_episodes(&name, page, size).await.unwrap();
    let count = db.get_total_episodes(&name).await.unwrap();
    let total_pages = ((count - 1) / size) + 1;

    EpisodeListTemplate {
        episodes,
        path,
        total_pages,
        page,
        size,
        name
    }
}

pub async fn get_episode(
    State(db): State<Database>,
    Path(id): Path<String>
) -> Response {
    match db.get_episode(&id).await.unwrap() {
        Some(episode) => EpisodeDetailTemplate { episode }.into_response(),
        None => NotFoundTemplate.into_response()
    }
}
