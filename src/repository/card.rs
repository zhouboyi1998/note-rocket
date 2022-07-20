use chrono::{DateTime, Local};
use diesel::{query_dsl::methods::{FindDsl, LimitDsl}, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection};

use crate::constant::date_time_pattern::DEFAULT_DATE_TIME;
use crate::model::card::{Card, CardInsertDTO, CardUpdateDTO};
use crate::schema::note_card;
use crate::util::snowflake::Snowflake;

pub struct CardRepository;

impl CardRepository {
    pub fn list(c: &SqliteConnection) -> QueryResult<Vec<Card>> {
        note_card::table
            .limit(100)
            .load::<Card>(c)
    }

    pub fn one(c: &SqliteConnection, id: i64) -> QueryResult<Card> {
        note_card::table
            .find(id)
            .get_result::<Card>(c)
    }

    pub fn insert(c: &SqliteConnection, card_dto: CardInsertDTO) -> QueryResult<i64> {
        // 生成雪花ID
        let id: i64 = Snowflake::next_id(&mut Snowflake::new(1, 1).unwrap()).unwrap();
        // 生成创建时间
        let now: DateTime<Local> = Local::now();
        let create_time: String = now.format(DEFAULT_DATE_TIME).to_string();
        // 组装 Card 模型
        let card: Card = Card {
            id,
            title: card_dto.title,
            content: card_dto.content,
            tip: card_dto.tip,
            extra: card_dto.extra,
            create_time,
        };

        diesel::insert_into(note_card::table)
            .values(card)
            .execute(c)?;

        Ok(id)
    }

    pub fn insert_batch(c: &SqliteConnection, card_dto_vec: Vec<CardInsertDTO>) -> QueryResult<Vec<i64>> {
        // 生成雪花ID
        let snowflake: &mut Snowflake = &mut Snowflake::new(1, 1).unwrap();
        let mut ids: Vec<i64> = vec![];
        for _ in 0..card_dto_vec.len() {
            ids.push(Snowflake::next_id(snowflake).unwrap());
        }
        let result: Vec<i64> = ids.clone();
        // 生成创建时间
        let now: DateTime<Local> = Local::now();
        let create_time = now.format(DEFAULT_DATE_TIME).to_string();
        let card_vec: Vec<Card> = card_dto_vec.into_iter().map(|card| {
            Card {
                id: ids.remove(0),
                title: card.title,
                content: card.content,
                tip: card.tip,
                extra: card.extra,
                create_time: create_time.clone(),
            }
        }).collect();
        diesel::insert_into(note_card::table)
            .values(card_vec)
            .execute(c)?;

        Ok(result)
    }

    pub fn update(c: &SqliteConnection, card_dto: CardUpdateDTO) -> QueryResult<usize> {
        diesel::update(note_card::table
            .find(card_dto.id))
            .set((
                note_card::title.eq(card_dto.title.to_owned()),
                note_card::content.eq(card_dto.content.to_owned()),
                note_card::tip.eq(card_dto.tip.to_owned()),
                note_card::extra.eq(card_dto.extra.to_owned()),
            ))
            .execute(c)
    }

    pub fn update_batch(c: &SqliteConnection, card_dto_vec: Vec<CardUpdateDTO>) -> QueryResult<usize> {
        let mut total: usize = 0;
        for card_dto in card_dto_vec {
            let count: usize = diesel::update(note_card::table
                .find(card_dto.id))
                .set((
                    note_card::title.eq(card_dto.title.to_owned()),
                    note_card::content.eq(card_dto.content.to_owned()),
                    note_card::tip.eq(card_dto.tip.to_owned()),
                    note_card::extra.eq(card_dto.extra.to_owned()),
                ))
                .execute(c)?;
            total += count;
        }

        Ok(total)
    }

    pub fn delete(c: &SqliteConnection, id: i64) -> QueryResult<usize> {
        diesel::delete(note_card::table.find(id))
            .execute(c)
    }

    pub fn delete_batch(c: &SqliteConnection, ids: Vec<String>) -> QueryResult<usize> {
        let mut total: usize = 0;
        for id in ids {
            match diesel::delete(note_card::table.find(id.parse::<i64>().unwrap())).execute(c) {
                Ok(count) => total += count,
                Err(e) => return Err(e),
            }
        }

        Ok(total)
    }
}
