use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTerrainDTO {
    pub description: String,
}
