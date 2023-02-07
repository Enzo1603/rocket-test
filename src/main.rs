#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
}
