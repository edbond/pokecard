use crate::schema::cards::{self};
use diesel::{associations::HasTable, prelude::*, sql_query};
use std::hash::{Hash, Hasher};

#[derive(Queryable, QueryableByName, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    pub id: i32,
    pub title: String,
    pub image: Option<Vec<u8>>,
    pub price: Option<f64>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cards)]
pub struct NewCard<'a> {
    pub title: &'a str,
    pub image: Option<Vec<u8>>,
    pub price: Option<f64>,
    pub url: Option<&'a str>,
    pub image_url: Option<String>,
}

impl Eq for Card {}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Card {
    pub fn all_cards(conn: &mut SqliteConnection) -> Vec<Card> {
        cards::table
            .select(Card::as_select())
            // .filter(cards::title.like("%Biba%"))
            .filter(cards::id.ne(1651).and(cards::id.ne(2357)))
            .filter(cards::title.not_like("%Water Energy%"))
            .get_results(conn)
            .unwrap()

        // sql_query("SELECT * FROM cards").load(conn).unwrap()
    }
}
