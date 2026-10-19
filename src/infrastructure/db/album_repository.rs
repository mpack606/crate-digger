use uuid::Uuid;

use crate::domain::entities::Album;
use crate::infrastructure::db::common::RepositoryError;

pub async fn get_saved_albums() -> Result<Vec<Album>, RepositoryError> {
    // todo
    Ok(vec![
        Album {
            id: Uuid::new_v4(),
            spotify_id: "ABCD".to_string(),
            name: "My beatiful dark twisted fantasy".to_string(),
            artist: "Kanye West".to_string(),
            rating: 2,
            image_url: "https://i.scdn.co/image/ab67616d0000b273d9194aa18fa4c9362b47464f".to_string()
        },
        Album {
            id: Uuid::new_v4(),
            spotify_id: "ABCD".to_string(),
            name: "The blueprint".to_string(),
            artist: "Jay-Z".to_string(),
            rating: 3,
            image_url: "https://i.scdn.co/image/ab67616d0000b2731b3f6f34954118500ed3e7a7".to_string()
        },
    ])
}

pub async fn save_album(album: Album) -> Result<(), RepositoryError> {
    // todo
    println!("Saved album: {}", album);
    Ok(())
}

pub async fn update_album(
    id: Uuid,
    rating: Option<u8>,
    note: Option<String>,
) -> Result<(), RepositoryError> {
    // todo
    println!(
        "Updated album with {} with values:\n{},{}",
        id,
        rating.unwrap_or_default(),
        note.unwrap_or_default()
    );
    Ok(())
}
