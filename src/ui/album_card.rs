use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AlbumCardProps {
    album_name: String,
    artist: String
}

pub fn AlbumCard(props: AlbumCardProps) -> Element {
    rsx! {
        div {
            p { "Album:" }
            p { {props.album_name} }
            p { {props.artist} }
        }
    }
}