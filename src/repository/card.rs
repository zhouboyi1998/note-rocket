use chrono::{DateTime, Local};
use diesel::{ExpressionMethods, query_dsl::methods::{FindDsl, LimitDsl}, QueryResult, RunQueryDsl, SqliteConnection};

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
        let id = Snowflake::next_id(&mut Snowflake::new(1, 1).unwrap()).unwrap();
        // 生成创建时间
        let now: DateTime<Local> = Local::now();
        let create_time = now.format(DEFAULT_DATE_TIME).to_string();
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

    pub fn delete(c: &SqliteConnection, id: i64) -> QueryResult<usize> {
        diesel::delete(note_card::table.find(id))
            .execute(c)
    }
}
