use crate::{
    domain::entities::DomainError,
    infrastructure::{
        self,
        spotify::{self},
    },
};

pub async fn add_to_library(spotify_id: &String) -> Result<(), DomainError> {
    let album = spotify::spotify_api::spotify_get_album(spotify_id).await;

    let result = match album {
        Ok(album) => infrastructure::db::album_repository::save_album(album).await,
        Err(_) => return Err(DomainError::UnknownError),
    };

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(DomainError::UnknownError),
    }
}
