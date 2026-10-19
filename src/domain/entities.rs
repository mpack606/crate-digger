use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum DomainError {
    UnknownError
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: Uuid,
    pub spotify_id: String,
    pub name: String,
    pub artist: String,
    pub rating: u8,
    pub image_url: String,
}

impl std::fmt::Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        f.write_str(&json)
    }
}
