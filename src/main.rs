#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod controller;
mod model;
mod repository;
mod schema;
mod util;
mod constant;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/card", routes![
            controller::card::list,
            controller::card::one,
            controller::card::insert,
            controller::card::insert_batch,
            controller::card::update,
            controller::card::update_batch,
            controller::card::delete,
            controller::card::delete_batch,
        ])
        .register("/", catchers!(controller::card::not_found))
        .attach(controller::card::Connection::fairing())
}
