use diesel::{
    query_dsl::methods::{FindDsl, LimitDsl, OrderDsl},
    ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection,
};

use crate::model::card::{Card, NewCard};
use crate::schema::note_card;

pub struct CardRepositiry;

impl CardRepositiry {
    pub fn list(c: &SqliteConnection) -> QueryResult<Vec<Card>> {
        note_card::table.limit(100).load::<Card>(c)
    }

    pub fn one(c: &SqliteConnection, id: i32) -> QueryResult<Card> {
        note_card::table.find(id).get_result::<Card>(c)
    }

    pub fn insert(c: &SqliteConnection, new_card: NewCard) -> QueryResult<Card> {
        diesel::insert_into(note_card::table)
            .values(new_card)
            .execute(c)?;
        note_card::table.order(note_card::id.desc()).first(c)
    }

    pub fn update(c: &SqliteConnection, card: Card) -> QueryResult<Card> {
        diesel::update(note_card::table.find(card.id))
            .set((
                note_card::title.eq(card.title.to_owned()),
                note_card::content.eq(card.content.to_owned()),
                note_card::tip.eq(card.tip.to_owned()),
                note_card::extra.eq(card.extra.to_owned()),
            ))
            .execute(c)?;
        note_card::table.find(card.id).get_result::<Card>(c)
    }

    pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(note_card::table.find(id)).execute(c)
    }
}
