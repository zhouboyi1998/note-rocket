#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use diesel::{
    query_dsl::methods::{FindDsl, LimitDsl},
    ExpressionMethods, RunQueryDsl,
};
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_sync_db_pools::database;

mod card;
mod schema;

use card::{Card, NewCard};
use schema::note_card;

#[database("sqlite_path")]
struct Connection(diesel::SqliteConnection);

#[get("/")]
async fn list(connention: Connection) -> Value {
    connention
        .run(|c| {
            let result = note_card::table
                .limit(100)
                .load::<Card>(c)
                .expect("Error list");
            json!(result)
        })
        .await
}

#[get("/<id>")]
async fn one(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = note_card::table
                .find(id)
                .get_result::<Card>(c)
                .expect("Error get");
            json!(result)
        })
        .await
}

#[post("/", format = "json", data = "<new_card>")]
async fn insert(connention: Connection, new_card: Json<NewCard>) -> Value {
    connention
        .run(|c| {
            let result = diesel::insert_into(note_card::table)
                .values(new_card.into_inner())
                .execute(c)
                .expect("Error insert");
            json!(result)
        })
        .await
}

#[put("/", format = "json", data = "<card>")]
async fn update(connention: Connection, card: Json<Card>) -> Value {
    connention
        .run(move |c| {
            let result = diesel::update(note_card::table.find(card.id))
                .set((
                    note_card::title.eq(card.title.to_owned()),
                    note_card::content.eq(card.content.to_owned()),
                    note_card::tip.eq(card.tip.to_owned()),
                    note_card::extra.eq(card.extra.to_owned()),
                ))
                .execute(c)
                .expect("Error update");
            json!(result)
        })
        .await
}

#[delete("/<id>")]
async fn delete(connention: Connection, id: i32) -> Value {
    connention
        .run(move |c| {
            let result = diesel::delete(note_card::table.find(id))
                .execute(c)
                .expect("Error delete");
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
