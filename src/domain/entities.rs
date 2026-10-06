use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    pub name: String,
    pub artist: String,
}
