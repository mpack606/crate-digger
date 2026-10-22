use dioxus::prelude::*;

use crate::{
    domain::{self, entities::Album},
    infrastructure::spotify::spotify_api::spotify_search_albums,
};

#[component]
pub fn AlbumSearch() -> Element {
    let mut albums: Signal<Vec<Album>> = use_signal(|| Vec::new());
    let mut query = use_signal(|| "".to_string());

    // todo change save button state while saving album
    // todo change save button state based on if album is present in library
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
                button {
                    onclick: move |_| async move {
                        spotify_search(&mut albums, &query).await;
                    },
                    "Search"
                }
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
                                button {
                                    onclick: move |_| {
                                        let spotify_id = spotify_id.clone();
                                        async move {
                                            add_to_library(&spotify_id).await;
                                        }
                                    },
                                    "Add to library"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn add_to_library(spotify_id: &String) {
    // todo handle result
    domain::library::add_to_library(spotify_id).await.unwrap();
}

async fn spotify_search(albums: &mut Signal<Vec<Album>>, query: &Signal<String>) {
    let search_results = spotify_search_albums(query.read().to_string());
    *albums.write() = search_results.await;
}

#[component]
pub fn SearchAlbumCard(album_name: String, artist: String, image_url: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            img { src: image_url, class: "h-24 w-24 shrink-0 object-contain" }
            div { class: "flex min-w-0 flex-col gap-1",
                p { {album_name} }
                p { {artist} }
                p { "2025" }
            }
        }
    }
}
