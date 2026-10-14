use dioxus::prelude::*;

use crate::{domain::entities::Album, infrastructure::spotify::spotify_api::spotify_search_albums, ui::album_card::AlbumCard};

#[component]
pub fn AlbumSearch() -> Element {

    let mut albums: Signal<Vec<Album>> = use_signal(|| Vec::new());
    let mut query = use_signal(|| "".to_string());

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
                button { onclick: move |_| spotify_search(&mut albums, &query), "Search" }
            }
            div {
                for album in albums.iter() {
                    div {
                        AlbumCard {
                            album_name: album.name.clone(),
                            artist: album.artist.clone(),
                            image_url: album.image_url.clone(),
                        }
                    }
                }
            }
        }
    }
}

fn spotify_search(albums: &mut Signal<Vec<Album>>, query: &Signal<String>) {
    let search_results = spotify_search_albums(query.read().to_string());
    *albums.write() = search_results;
}