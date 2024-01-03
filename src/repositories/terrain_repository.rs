use rusqlite::{params, Connection, Error};

use crate::models::terrain::{CreateTerrainDTO, Terrain};

pub fn save(conn: &Connection, terrain: &CreateTerrainDTO) -> Result<Terrain, Error> {
    conn.execute(
        "INSERT INTO terrains (description) VALUES (?1);",
        params![&terrain.description],
    )?;
    let new_terrain: Terrain = Terrain {
        id: conn.last_insert_rowid() as u32,
        description: terrain.description.clone(),
    };
    Ok(new_terrain)
}

pub fn get_all(conn: &Connection) -> Result<Vec<Terrain>, Error> {
    let mut stmt = conn.prepare("SELECT id, description FROM terrains;")?;
    let terrain_iter = stmt.query_map([], |row| {
        Ok(Terrain {
            id: row.get(0)?,
            description: row.get(1)?,
        })
    })?;
    Ok(terrain_iter.collect::<Result<Vec<Terrain>, Error>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::terrain::CreateTerrainDTO;
    use rusqlite::Connection;

    #[test]
    fn save_terrain_test() {
        let conn = Connection::open_in_memory().expect("DB not created");
        conn.execute(
            "CREATE TABLE terrains (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL
            );",
            (),
        )
        .unwrap();
        let terrain = CreateTerrainDTO {
            description: "any_description".to_string(),
        };
        let sut = save(&conn, &terrain).expect("Not saved");
        assert_eq!(sut.id, 1);
        assert_eq!(sut.description, "any_description");
    }
}
