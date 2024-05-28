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
pub struct NewCard {
    pub title: String,
    pub image: Option<Vec<u8>>,
    pub price: Option<f64>,
    pub url: Option<String>,
    pub image_url: Option<String>,
}

impl Eq for Card {}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Card {
    pub fn all_cards(conn: &mut SqliteConnection, limit: i64) -> Vec<Card> {
        cards::table
            .select(Card::as_select())
            .filter(cards::image.is_not_null())
            .limit(limit)
            .order(cards::id)
            .get_results(conn)
            .expect("fetch cards")

        // sql_query("SELECT * FROM cards").load(conn).unwrap()
    }
}
