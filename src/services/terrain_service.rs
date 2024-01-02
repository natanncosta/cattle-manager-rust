use crate::{
    models::{response::GenericResponse, terrain::CreateTerrainDTO},
    repositories::terrain_repository,
};

pub fn create(terrain: &CreateTerrainDTO) -> GenericResponse {
    terrain_repository::save(terrain);
    GenericResponse {
        success: true,
        message: String::from("Terreno criado com sucesso."),
    }
}
