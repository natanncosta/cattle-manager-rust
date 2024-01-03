use std::sync::Mutex;

use rocket::{serde::json::Json, State};
use rusqlite::Connection;

use crate::{
    models::{
        response::{SingleTerrainResponse, TerrainListResponse},
        terrain::CreateTerrainDTO,
    },
    services::terrain_service,
};

#[post("/terrains", data = "<terrain>")]
pub fn create_terrain(
    terrain: Json<CreateTerrainDTO>,
    conn: &State<Mutex<Connection>>,
) -> Json<SingleTerrainResponse> {
    Json(terrain_service::create(&terrain, &conn.lock().unwrap()))
}

#[get("/terrains")]
pub fn get_all_terrains(conn: &State<Mutex<Connection>>) -> Json<TerrainListResponse> {
    Json(terrain_service::get_all(&conn.lock().unwrap()))
}
