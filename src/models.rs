use crate::schema::stories;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = stories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Story {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = stories)]
pub struct NewStory<'a> {
    pub title: &'a str,
}
