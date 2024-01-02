use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTerrainDTO {
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Terrain {
    pub id: u32,
    pub description: String,
}
