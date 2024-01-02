#[macro_use]
extern crate rocket;

use api::terrain_controller::create_terrain;
use rocket::launch;

mod api;
mod models;
mod repositories;
mod services;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api", routes![create_terrain])
}
