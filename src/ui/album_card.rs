use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AlbumCardProps {
    album_name: String,
    artist: String,
    image_url: String
}

pub fn AlbumCard(props: AlbumCardProps) -> Element {
    rsx! {
        div { class: "border-solid, items-center",
            p { "Album:" }
            p { {props.album_name} }
            p { {props.artist} }
            img { src: props.image_url, height: 128, width: 128 }
        }
    }
}