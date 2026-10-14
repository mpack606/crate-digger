use dioxus::prelude::*;

use crate::{domain::entities::Album, ui::album_card::AlbumCard};

#[component]
pub fn AlbumSearch() -> Element {

    let mut albums: Signal<Vec<Album>> = use_signal(|| Vec::new());
    let mut query = use_signal(|| "".to_string());
    // let search_callback = move |_evt|  spotify_search(&albums, &query);

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
                button { onclick: move |_| spotify_search(&albums, &query), "Search" }
            }
            div {
                for album in albums.iter() {
                    div {
                        {album.name.clone()}
                        {album.artist.clone()}
                    }
                }
            }
        }
    }
}

fn spotify_search(albums: &Signal<Vec<Album>>, query: &Signal<String>) {
    println!("Clicked with {}", query.read())
}