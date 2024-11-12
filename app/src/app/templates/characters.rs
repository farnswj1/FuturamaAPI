use askama_axum::Template;

use crate::app::serializers::models::Character;

#[derive(Template)]
#[template(path = "characters/list.html")]
pub struct CharacterListTemplate {
    pub characters: Vec<Character>
}

#[derive(Template)]
#[template(path = "characters/detail.html")]
pub struct CharacterDetailTemplate {
    pub character: Character
}
