use rocket::serde::json::Json;

use crate::{
    models::{response::GenericResponse, terrain::CreateTerrainDTO},
    services::terrain_service,
};

#[post("/terrains", data = "<terrain>")]
pub fn create_terrain(terrain: Json<CreateTerrainDTO>) -> Json<GenericResponse> {
    Json(terrain_service::create(&terrain))
}
