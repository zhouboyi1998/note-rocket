use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_sync_db_pools::database;

use crate::model::card::{CardInsertDTO, CardUpdateDTO};
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
pub async fn one(connection: Connection, id: i64) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::one(c, id)
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/", format = "json", data = "<card_dto>")]
pub async fn insert(connection: Connection, card_dto: Json<CardInsertDTO>) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::insert(c, card_dto.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/batch", format = "json", data = "<card_dto_vec>")]
pub async fn insert_batch(connection: Connection, card_dto_vec: Json<Vec<CardInsertDTO>>) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::insert_batch(c, card_dto_vec.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/", format = "json", data = "<card_dto>")]
pub async fn update(connection: Connection, card_dto: Json<CardUpdateDTO>) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::update(c, card_dto.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/batch", format = "json", data = "<card_dto_vec>")]
pub async fn update_batch(connection: Connection, card_dto_vec: Json<Vec<CardUpdateDTO>>) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::update_batch(c, card_dto_vec.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/<id>")]
pub async fn delete(connection: Connection, id: i64) -> Result<Value, status::Custom<Value>> {
    connection.run(move |c| {
        CardRepository::delete(c, id)
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/batch", format = "json", data = "<ids>")]
pub async fn delete_batch(connection: Connection, ids: Json<Vec<String>>) -> Result<Value, status::Custom<Value>> {
    connection.run(|c| {
        CardRepository::delete_batch(c, ids.into_inner())
            .map(|result| json!(result))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[catch(404)]
pub async fn not_found() -> Value {
    json!("URL Not Found!")
}
