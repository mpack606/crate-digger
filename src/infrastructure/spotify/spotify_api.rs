use crate::domain::entities::Album;

pub fn spotify_search_albums(query: String) -> Vec<Album> {
    vec![
        Album {
            name: "My beatiful dark twisted fantasy".to_string(),
            artist: "Kanye West".to_string(),
        },
        Album {
            name: "The blueprint".to_string(),
            artist: "Jay-Z".to_string(),
        },
    ]
}
