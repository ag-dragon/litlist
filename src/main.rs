#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use models::*;

use rocket::fs::{FileServer, relative};
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::response::{Redirect, Flash};
use rocket_dyn_templates::{Template, context};
use diesel::pg::PgConnection;
use diesel::prelude::*;
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

#[get("/random")]
fn get_random_lit() -> Json<Lit> {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();
    let mut results = entries
        .limit(5)
        .select(Entry::as_select())
        .load(connection)
        .expect("Error loading entires");
    print!("{:?}", results.pop());
    Json(
        Lit {
            id: 1,
            title: "Test Lit".to_string(),
        }
    )
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

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index, get_random_lit, create_entry])
}
