use serde::Serialize;

use super::terrain::Terrain;

#[derive(Serialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct TerrainListResponse {
    pub data: Vec<Terrain>,
}

#[derive(Serialize)]
pub struct SingleTerrainResponse {
    pub data: Terrain,
    pub success: bool,
    pub message: String,
}
