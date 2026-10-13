use dioxus::prelude::*;

#[component]
pub fn AlbumSearch() -> Element {
    rsx! {
        div {
            p {
                label { "Search albums: " }
            }
            p {
                input { r#type: "text", size: 10 }
                button { onclick: move |_| spotify_search(), "Search" }
            }
        }
    }
}

fn spotify_search() {
    println!("Clicked")
}