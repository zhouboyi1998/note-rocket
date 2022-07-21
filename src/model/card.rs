use rocket::serde::{Deserialize, Serialize};

use crate::schema::note_card;

// Standard Model
#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "note_card"]
pub struct Card {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tip: String,
    pub extra: String,
    #[serde(skip_deserializing)]
    pub create_time: String,
}

// Request Body Model
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "note_card"]
pub struct NewCard {
    pub title: String,
    pub content: String,
    pub tip: String,
    pub extra: String,
}
