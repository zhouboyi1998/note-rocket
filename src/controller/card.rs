use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_sync_db_pools::database;

use crate::model::card::{Card, NewCard};
use crate::repository::card::CardRepositiry;

#[database("sqlite_path")]
pub struct Connection(diesel::SqliteConnection);

#[get("/")]
pub async fn list(connention: Connection) -> Value {
    connention
        .run(|c| {
            let result = CardRepositiry::list(c).expect("Error list");
            json!(result)
        })
        .await
}

#[get("/<id>")]
pub async fn one(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::one(c, id).expect("Error get");
            json!(result)
        })
        .await
}

#[post("/", format = "json", data = "<new_card>")]
pub async fn insert(connention: Connection, new_card: Json<NewCard>) -> Value {
    connention
        .run(|c| {
            let result = CardRepositiry::insert(c, new_card.into_inner()).expect("Error insert");
            json!(result)
        })
        .await
}

#[put("/", format = "json", data = "<card>")]
pub async fn update(connention: Connection, card: Json<Card>) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::update(c, card.into_inner()).expect("Error update");
            json!(result)
        })
        .await
}

#[delete("/<id>")]
pub async fn delete(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = CardRepositiry::delete(c, id).expect("Error delete");
            json!(result)
        })
        .await
}

#[catch(404)]
pub async fn not_found() -> Value {
    json!("URL Not Found!")
}
