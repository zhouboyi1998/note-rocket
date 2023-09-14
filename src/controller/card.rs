use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, serde_json::json, Value};
use rocket_sync_db_pools::database;

use crate::model::card::{Card, NewCard};
use crate::repository::card::CardRepository;

#[database("sqlite_path")]
pub struct Connection(diesel::SqliteConnection);

#[get("/")]
pub async fn list(connection: Connection) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::list(c)
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/<id>")]
pub async fn one(connection: Connection, id: i32) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::one(c, id)
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/", format = "json", data = "<new_card>")]
pub async fn insert(connection: Connection, new_card: Json<NewCard>) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::insert(c, new_card.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/", format = "json", data = "<card>")]
pub async fn update(connection: Connection, card: Json<Card>) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::update(c, card.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/<id>")]
pub async fn delete(connection: Connection, id: i32) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::delete(c, id)
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[catch(404)]
pub async fn not_found() -> Value {
    json!("URL Not Found!")
}
