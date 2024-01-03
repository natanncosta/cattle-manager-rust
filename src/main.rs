#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use api::terrain_controller::{create_terrain, get_all_terrains};
use rocket::launch;
use rusqlite::Connection;

mod api;
mod models;
mod repositories;
mod services;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_terrain, get_all_terrains])
        .manage({
            let conn = Connection::open("cattle_manager.db").unwrap();
            conn.execute(
                "CREATE TABLE IF NOT EXISTS terrains (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
            );",
                (),
            )
            .expect("Creation tables failed");

            Mutex::new(conn)
        })
}
