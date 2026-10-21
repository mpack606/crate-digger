use dioxus::prelude::*;

use crate::{
    domain::{self, entities::Album},
    infrastructure::spotify::spotify_api::spotify_search_albums,
    ui::album_card::AlbumCard,
};

#[component]
pub fn AlbumSearch() -> Element {
    let mut albums: Signal<Vec<Album>> = use_signal(|| Vec::new());
    let mut query = use_signal(|| "".to_string());

    // todo change save button state while saving album
    rsx! {
        div { class: "items-center p-1",
            p {
                label { "Search albums: " }
            }
            p {
                input {
                    r#type: "text",
                    placeholder: "Search query...",
                    oninput: move |e| query.set(e.value()),
                }
                button { onclick: move |_| async move { spotify_search(&mut albums, &query).await; }, "Search" }
            }
            div {
                for album in albums.iter() {
                    div {
                        SearchAlbumCard {
                            album_name: album.name.clone(),
                            artist: album.artist.clone(),
                            image_url: album.image_url.clone(),
                        }
                        {
                            let spotify_id = album.spotify_id.clone();
                            rsx! {
                                button { onclick: move |_| {
                                    let spotify_id = spotify_id.clone();
                                    async move { add_to_library(&spotify_id).await; }
                                }, "Add to library" }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn add_to_library(spotify_id: &String) {
    domain::library::add_to_library(spotify_id).await;
}

async fn spotify_search(albums: &mut Signal<Vec<Album>>, query: &Signal<String>) {
    let search_results = spotify_search_albums(query.read().to_string());
    *albums.write() = search_results.await;
}

#[component]
pub fn SearchAlbumCard(
    album_name: String,
    artist: String,
    image_url: String,
    error_msg: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "border-solid, items-center",
            p { "Album:" }
            p { {album_name} }
            p { {artist} }
            img { src: image_url, height: 128, width: 128 }
                if ()
        }
    }
}
