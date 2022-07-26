use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_sync_db_pools::database;

use crate::model::card::{Card, NewCard};
use crate::repository::card::CardRepositiry;

#[database("sqlite_path")]
pub struct Connection(diesel::SqliteConnection);

#[get("/")]
pub async fn list(connention: Connection) -> Result<Value, status::Custom<Value>> {
    connention
        .run(|c| {
            CardRepositiry::list(c)
                .map(|result| json!(result))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        })
        .await
}

#[get("/<id>")]
pub async fn one(connention: Connection, id: i32) -> Result<Value, status::Custom<Value>> {
    connention
        .run(move |c| {
            CardRepositiry::one(c, id)
                .map(|result| json!(result))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        })
        .await
}

#[post("/", format = "json", data = "<new_card>")]
pub async fn insert(
    connention: Connection,
    new_card: Json<NewCard>,
) -> Result<Value, status::Custom<Value>> {
    connention
        .run(|c| {
            CardRepositiry::insert(c, new_card.into_inner())
                .map(|result| json!(result))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        })
        .await
}

#[put("/", format = "json", data = "<card>")]
pub async fn update(
    connention: Connection,
    card: Json<Card>,
) -> Result<Value, status::Custom<Value>> {
    connention
        .run(move |c| {
            CardRepositiry::update(c, card.into_inner())
                .map(|result| json!(result))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        })
        .await
}

#[delete("/<id>")]
pub async fn delete(connention: Connection, id: i32) -> Result<Value, status::Custom<Value>> {
    connention
        .run(move |c| {
            CardRepositiry::delete(c, id)
                .map(|result| json!(result))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        })
        .await
}

#[catch(404)]
pub async fn not_found() -> Value {
    json!("URL Not Found!")
}
