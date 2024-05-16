use crate::schema::cards;
use diesel::{associations::HasTable, prelude::*, sql_query};

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

impl Card {
    pub fn all_cards(conn: &mut SqliteConnection) -> Vec<Card> {
        cards::table
            .select(Card::as_select())
            .get_results(conn)
            .unwrap()

        // sql_query("SELECT * FROM cards").load(conn).unwrap()
    }
}
