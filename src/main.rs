#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use rocket_test::catchers;
use rocket_test::fairings::db::DBConnection;
use rocket_test::routes::{self, post, user};

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DBConnection::init())
        .mount(
            "/",
            routes![
                user::get_user,
                user::get_users,
                user::new_user,
                user::create_user,
                user::edit_user,
                user::put_user,
                user::patch_user,
                user::delete_user,
                post::get_post,
                post::get_posts,
                post::create_post,
                post::delete_post,
            ],
        )
        .mount("/assets", routes![routes::assets])
        .register(
            "/",
            catchers![
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_server_error
            ],
        )
}
