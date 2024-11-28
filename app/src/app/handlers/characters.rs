use axum::extract::{OriginalUri, Path, Query, State};

use crate::app::{
    db::Database,
    serializers::query::{NameQuery, Paginator},
    templates::characters::{CharacterDetailTemplate, CharacterListTemplate}
};

use super::errors::AppError;

pub async fn get_characters(
    OriginalUri(uri): OriginalUri,
    State(db): State<Database>,
    character: Option<Query<NameQuery>>,
    paginator: Option<Query<Paginator>>
) -> Result<CharacterListTemplate, AppError> {
    let Query(NameQuery { name }) = character.unwrap_or_default();
    let Query(Paginator { page, size }) = paginator.unwrap_or_default();
    let path = uri.path().to_string();

    let characters = db.get_characters(&name, page, size).await?;
    let count = db.get_total_characters(&name).await?;
    let total_pages = ((count - 1) / size) + 1;

    Ok(CharacterListTemplate {
        characters,
        path,
        total_pages,
        page,
        size,
        name
    })
}

pub async fn get_character(
    State(db): State<Database>,
    Path(id): Path<String>
) -> Result<CharacterDetailTemplate, AppError> {
    let character = db.get_character(&id).await?.ok_or(AppError::NotFound)?;
    Ok(CharacterDetailTemplate { character })
}
