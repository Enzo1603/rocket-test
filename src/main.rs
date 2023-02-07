#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use rocket::{Build, Rocket};
use rocket_db_pools::{{sqlx::PgPool}, Database};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::Type, Debug)]
#[repr(i32)]
enum UserStatus {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug, FromRow)]
struct User {
    uuid: Uuid,
    username: String,
    email: String,
    password_hash: String,
    description: String,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Database)]
#[database("main_connection")]
struct DBCOnnection(PgPool);

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build().attach(DBCOnnection::init())
}
