#[macro_use]
extern crate rocket;

mod api;
mod domain;
mod infrastructure;

use rocket::{launch, routes};

use crate::api::album_search::album_search;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/search", routes![album_search])
        .mount("/library", routes![])
}
