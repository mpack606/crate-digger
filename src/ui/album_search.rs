use dioxus::prelude::*;

use crate::{
    domain::{self, entities::Album, search::SearchAlbum}, infrastructure::spotify::spotify_api::spotify_search_albums,
};

#[component]
pub fn AlbumSearch() -> Element {
    let mut albums: Signal<Vec<SearchAlbum>> = use_signal(|| Vec::new());
    let mut query = use_signal(|| "".to_string());
    let mut search_loaded = use_signal(|| false);

    // todo change save button state while saving album
    // todo change save button state based on if album is present in library
    rsx! {
        div { class: "min-w-0 flex-1 p",
            div { class: "border-b border-groove-gray p-4",
                p {
                    label { class: "font-bold", "Search albums " }
                }
                p {
                    input {
                        r#type: "text",
                        placeholder: "Search query...",
                        oninput: move |e| query.set(e.value()),
                    }
                    button {
                        onclick: move |_| async move {
                            search_albums(&mut albums, &query).await;
                            search_loaded.set(true);
                        },
                        "Search"
                    }
                }
            }
            if search_loaded() {
                p { class: "text-groove-gray",
                    {albums.len().to_string()}
                    " results"
                }
            }
            div {
                for album in albums.iter() {
                    div { class: "flex w-full items-center gap-4 border-b border-groove-gray p-4",
                        SearchAlbumCard {
                            album_name: album.name.clone(),
                            artist: album.artist.clone(),
                            image_url: album.image_url.clone(),
                        }
                        {
                            let spotify_id = album.spotify_id.clone();
                            rsx! {
                                button {
                                    class: "ml-auto shrink-0 m-4 p-4 border-2 border-crate-orange rounded-xl text-crate-orange font-bold w-36",
                                    class: if album.is_in_library { "material-symbols-rounded text-sleeve-white bg-crate-orange" },
                                    onclick: move |_| {
                                        let spotify_id = spotify_id.clone();
                                        async move {
                                            add_to_library(&spotify_id).await;
                                        }
                                    },
                                    if album.is_in_library {
                                        "check_circle"
                                    } else {
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
}

async fn add_to_library(spotify_id: &String) {
    // todo handle result
    domain::library::add_to_library(spotify_id).await.unwrap();
}

async fn search_albums(albums: &mut Signal<Vec<SearchAlbum>>, query: &Signal<String>) {
    *albums.write() = domain::search::search_albums(&query.read().to_string()).await;
}

#[component]
pub fn SearchAlbumCard(album_name: String, artist: String, image_url: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            img {
                src: image_url,
                class: "h-32 w-32 shrink-0 object-contain rounded-md",
            }
            div { class: "flex min-w-0 flex-col gap-1",
                p { class: "font-bold", {album_name} }
                p { {artist} }
                p { class: "text-groove-gray", "2025" }
            }
        }
    }
}
