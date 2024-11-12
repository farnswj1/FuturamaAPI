use askama_axum::Template;

use crate::app::serializers::models::Episode;

#[derive(Template)]
#[template(path = "episodes/list.html")]
pub struct EpisodeListTemplate {
    pub episodes: Vec<Episode>
}

#[derive(Template)]
#[template(path = "episodes/detail.html")]
pub struct EpisodeDetailTemplate {
    pub episode: Episode
}
