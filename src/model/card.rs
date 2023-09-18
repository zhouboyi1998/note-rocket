use rocket::serde::{Deserialize, Serialize};

use crate::schema::note_card;

/**
 * Card Model
 */
#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "note_card"]
pub struct Card {
    /**
     * ID
     */
    pub id: i64,
    /**
     * 标题
     */
    pub title: String,
    /**
     * 内容
     */
    pub content: String,
    /**
     * 提示
     */
    pub tip: String,
    /**
     * 扩展信息
     */
    pub extra: String,
    /**
     * 创建时间
     */
    pub create_time: String,
}

/**
 * Card Insert DTO
 */
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CardInsertDTO {
    pub title: String,
    pub content: String,
    pub tip: String,
    pub extra: String,
}

/**
 * Card Update DTO
 */
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CardUpdateDTO {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub tip: String,
    pub extra: String,
}
