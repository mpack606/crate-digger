use dioxus::prelude::*;

use crate::ui::{album_search::AlbumSearch, navigation_menu::NavigationMenu};

const MAIN_CSS: Asset = asset!("/assets/main.css");
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
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded" }
        div { class: "flex min-h-screen justify-center bg-sleeve-white p-6",
            div { class: "flex w-full max-w-6xl flex-col gap-8 md:flex-row border border-groove-gray rounded-xl",
                NavigationMenu {}
                AlbumSearch {}
            }
        }
    }
}
