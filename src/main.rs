#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use models::*;

use rocket::{
    fs::{FileServer, relative},
    serde::Serialize,
    response::{Redirect, Flash},
};
use rocket_dyn_templates::{Template, context};
use diesel::{
    pg::PgConnection,
    prelude::*,
};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Lit {
    id: i32,
    title: String,
}

#[get("/create")]
fn create_entry() -> Flash<Redirect> {
    use schema::entries;

    let new_entry = self::models::NewEntry { title: "test_entry" };

    let connection = &mut establish_connection();
    diesel::insert_into(entries::table)
        .values(&new_entry)
        .returning(Entry::as_returning())
        .get_result(connection)
        .expect("Error saving new entry");
    Flash::success(Redirect::to("/"), "Entry successfully created.")
}

#[get("/entry/<entry_id>")]
fn entry(entry_id: i32) -> Template {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();

    let mut query_result = entries
        .filter(id.eq(entry_id))
        .select(Entry::as_select())
        .load(connection)
        .expect("Error loading entries");

    let entry_o = query_result.pop();
    let entry: Entry;
    match entry_o {
        Some(v) => entry = v,
        _ => return Template::render("entry", context! {}),
    }
    let lit = Lit {
        id: entry.id,
        title: entry.title,
    };

    Template::render("entry", context! { lit })
}

#[get("/")]
fn index() -> Template {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();

    let query_result = entries
        .limit(5)
        .select(Entry::as_select())
        .load(connection)
        .expect("Error loading entries");

    let mut lits: Vec<Lit> = Vec::new();
    for entry in query_result {
        lits.push(Lit { id: entry.id, title: entry.title });
    }

    Template::render("index", context! { lits })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![
            index,
            create_entry,
            entry,
        ])
}
