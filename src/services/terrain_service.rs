use crate::{
    models::{
        response::{GenericResponse, TerrainListResponse},
        terrain::CreateTerrainDTO,
    },
    repositories::terrain_repository,
};

pub fn create(terrain: &CreateTerrainDTO) -> GenericResponse {
    terrain_repository::save(terrain);
    GenericResponse {
        success: true,
        message: String::from("Terreno criado com sucesso."),
    }
}

pub fn get_all() -> TerrainListResponse {
    TerrainListResponse {
        data: terrain_repository::get_all(),
    }
}
