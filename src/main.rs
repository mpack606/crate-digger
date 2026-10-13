use dioxus::prelude::*;

use crate::ui::{album_card::AlbumCard, album_search::AlbumSearch};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod domain;
mod infrastructure;
mod ui;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // AlbumCard { album_name: "Album 1", artist: "Artist 1" }
    rsx! {
        AlbumSearch {}
    }
}