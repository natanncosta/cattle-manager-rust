use rusqlite::Connection;

use crate::{
    models::{
        response::{SingleTerrainResponse, TerrainListResponse},
        terrain::CreateTerrainDTO,
    },
    repositories::terrain_repository,
};

pub fn create(terrain: &CreateTerrainDTO, conn: &Connection) -> SingleTerrainResponse {
    let created_terrain = terrain_repository::save(&conn, terrain).unwrap();
    SingleTerrainResponse {
        data: created_terrain,
        success: true,
        message: String::from("Terreno criado com sucesso."),
    }
}

pub fn get_all(conn: &Connection) -> TerrainListResponse {
    TerrainListResponse {
        data: terrain_repository::get_all(&conn).unwrap(),
    }
}
