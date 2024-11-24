use askama_axum::Template;

use crate::app::serializers::models::Episode;

#[derive(Template)]
#[template(path = "episodes/list.html")]
pub struct EpisodeListTemplate {
    pub episodes: Vec<Episode>,
    pub path: String,
    pub total_pages: i64,
    pub page: i64,
    pub size: i64,
    pub name: String
}

#[derive(Template)]
#[template(path = "episodes/detail.html")]
pub struct EpisodeDetailTemplate {
    pub episode: Episode
}
