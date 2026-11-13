use uuid::Uuid;

use crate::{domain::entities::Album, infrastructure::{self, spotify::spotify_api::spotify_search_albums}};

pub struct SearchAlbum {
    pub id: Uuid,
    pub spotify_id: String,
    pub name: String,
    pub artist: String,
    pub rating: u8,
    pub image_url: String,
    pub is_in_library: bool,
}

pub async fn search_albums(query: &String) -> Vec<SearchAlbum> {
    let search_results = spotify_search_albums(query.clone()).await;

    let saved_albums = match infrastructure::db::album_repository::get_saved_albums().await {
        Ok(x) => x,
        Err(_) => Vec::new()
    };

    let mut result_albums = Vec::new();

    for album in search_results {
        let is_in_library = saved_albums.iter().any(|x| x.spotify_id == album.spotify_id);
        let search_album = map_album_to_search_album(album, is_in_library);
        result_albums.push(search_album);
    }

    result_albums
}

fn map_album_to_search_album(album: Album, is_in_library: bool) -> SearchAlbum {
    SearchAlbum { id: album.id, spotify_id: album.spotify_id, name: album.name, artist: album.artist, rating: album.rating, image_url: album.image_url, is_in_library: is_in_library }
}