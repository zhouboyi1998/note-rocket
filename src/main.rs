#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod controller;
mod model;
mod repository;
mod schema;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/card", routes![
            controller::card::list,
            controller::card::one,
            controller::card::insert,
            controller::card::update,
            controller::card::delete,
        ])
        .register("/", catchers!(controller::card::not_found))
        .attach(controller::card::Connection::fairing())
}
