#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_sync_db_pools::database;

mod model;
mod repository;
mod schema;

use model::card::{Card, NewCard};

use repository::card::CardRepositiry;

#[database("sqlite_path")]
struct Connection(diesel::SqliteConnection);

#[get("/")]
async fn list(connention: Connection) -> Value {
    connention
        .run(|c| {
            let result = CardRepositiry::list(c).expect("Error list");
            json!(result)
        })
        .await
}

#[get("/<id>")]
async fn one(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::one(c, id).expect("Error get");
            json!(result)
        })
        .await
}

#[post("/", format = "json", data = "<new_card>")]
async fn insert(connention: Connection, new_card: Json<NewCard>) -> Value {
    connention
        .run(|c| {
            let result = CardRepositiry::insert(c, new_card.into_inner()).expect("Error insert");
            json!(result)
        })
        .await
}

#[put("/", format = "json", data = "<card>")]
async fn update(connention: Connection, card: Json<Card>) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::update(c, card.into_inner()).expect("Error update");
            json!(result)
        })
        .await
}

#[delete("/<id>")]
async fn delete(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::delete(c, id).expect("Error delete");
            json!(result)
        })
        .await
}

#[catch(404)]
async fn not_found() -> Value {
    json!("URL Not Found!")
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/card", routes![list, one, insert, update, delete])
        .register("/", catchers!(not_found))
        .attach(Connection::fairing())
}
