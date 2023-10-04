use crate::schema::entries;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entry {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub title: &'a str,
}
