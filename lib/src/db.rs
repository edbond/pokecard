use crate::{
    models::{Card, NewCard},
};
use anyhow::Result;
use diesel::prelude::*;
use diesel::{sqlite::SqliteConnection, Connection};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_card(conn: &mut SqliteConnection, card: NewCard) -> Result<()> {
    let _ = diesel::insert_into(crate::schema::cards::table)
        .values(&card)
        .execute(conn)?;
    Ok(())
}

pub fn get_images(conn: &mut SqliteConnection, limit: i64) -> Result<Vec<Card>> {
    let images = Card::all_cards(conn, limit);

    Ok(images)
}
