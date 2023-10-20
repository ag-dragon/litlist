use crate::schema::stories;
use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = stories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Story {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
