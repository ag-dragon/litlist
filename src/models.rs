use crate::schema::stories;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = stories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub progress: Option<i32>,
    pub length: Option<i32>,
    pub link: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = stories)]
pub struct NewStory {
    pub title: String,
    pub author: String,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub progress: Option<i32>,
    pub length: Option<i32>,
    pub link: Option<String>,
}
