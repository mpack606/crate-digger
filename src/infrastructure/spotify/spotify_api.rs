use uuid::Uuid;

use crate::{domain::entities::Album, infrastructure::spotify::models::ApiError};

pub fn spotify_search_albums(query: String) -> Vec<Album> {
    // todo
    vec![
        Album {
            id: Uuid::nil(),
            spotify_id: "ABCD".to_string(),
            name: "My beatiful dark twisted fantasy".to_string(),
            artist: "Kanye West".to_string(),
            rating: 66,
            image_url: "https://i.scdn.co/image/ab67616d0000b273d9194aa18fa4c9362b47464f".to_string()
        },
        Album {
            id: Uuid::nil(),
            spotify_id: "ABCD".to_string(),
            name: "The blueprint".to_string(),
            artist: "Jay-Z".to_string(),
            rating: 66,
            image_url: "https://i.scdn.co/image/ab67616d0000b2731b3f6f34954118500ed3e7a7".to_string()
        },
    ]
}

pub fn spotify_get_album(id: &String) -> Result<Album, ApiError> {
    let album = Album {
        id: Uuid::nil(),
        spotify_id: "ABCD".to_string(),
        name: "My beatiful dark twisted fantasy".to_string(),
        artist: "Kanye West".to_string(),
        rating: 66,
        image_url: "https://i.scdn.co/image/ab67616d0000b273d9194aa18fa4c9362b47464f".to_string()
    };
    Ok(album)
}
