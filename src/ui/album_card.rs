use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AlbumCardProps {
    album_name: String,
    artist: String,
    image_url: String,
    display_rating: bool,
}

pub fn AlbumCard(props: AlbumCardProps) -> Element {
    rsx! {
        div { class: "",
            p { class: "", "Album:" }
            p { class: "", {props.album_name} }
            p { class: "", {props.artist} }
            img { class: "", src: props.image_url, height: 128, width: 128 }
        }
    }
}
