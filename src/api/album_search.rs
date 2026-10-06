use rocket::serde::json::Json;

use crate::{domain::entities::Album, infrastructure::spotify::spotify_api::spotify_search_albums};

#[get("/?<query>")]
pub async fn album_search(query: String) -> Json<Vec<Album>> {
    Json(spotify_search_albums(query))
}
