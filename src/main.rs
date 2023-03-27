#[macro_use]
extern crate rocket;

use ble_packet_collect_backend::database;
use ble_packet_collect_backend::routes::user_route;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket.attach(database::DB::init()).mount(
        "/user",
        routes![user_route::get_user, user_route::create_user],
    )
}
