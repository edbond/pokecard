use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, PartialEq, Selectable)]
#[diesel(table_name = crate::schema::cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    pub id: Option<i32>,
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
