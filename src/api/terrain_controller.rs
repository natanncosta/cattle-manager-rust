use rocket::serde::json::Json;

use crate::{
    models::{
        response::{GenericResponse, TerrainListResponse},
        terrain::CreateTerrainDTO,
    },
    services::terrain_service,
};

#[post("/terrains", data = "<terrain>")]
pub fn create_terrain(terrain: Json<CreateTerrainDTO>) -> Json<GenericResponse> {
    Json(terrain_service::create(&terrain))
}

#[get("/terrains")]
pub fn get_all_terrains() -> Json<TerrainListResponse> {
    Json(terrain_service::get_all())
}
