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
fn create_story() -> Flash<Redirect> {
    use schema::entries;

    let new_story = self::models::NewEntry { title: "test_story" };

    let connection = &mut establish_connection();
    diesel::insert_into(entries::table)
        .values(&new_story)
        .returning(Entry::as_returning())
        .get_result(connection)
        .expect("Error saving new story");
    Flash::success(Redirect::to("/"), "Entry successfully created.")
}

#[delete("/<story_id>")]
fn delete_story(story_id: i32) -> Flash<Redirect> {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(entries.filter(id.eq(story_id)))
        .execute(connection)
        .expect("Error deleting entries");

    Flash::success(Redirect::to("/"), "Entry successfully deleted.")
}

#[get("/stories/<story_id>")]
fn story(story_id: i32) -> Template {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();

    let mut query_result = entries
        .filter(id.eq(story_id))
        .select(Entry::as_select())
        .load(connection)
        .expect("Error loading entries");

    let story_o = query_result.pop();
    let story: Entry;
    match story_o {
        Some(v) => story = v,
        _ => return Template::render("story", context! {}),
    }
    let lit = Lit {
        id: story.id,
        title: story.title,
    };

    Template::render("story", context! { lit })
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
    for story in query_result {
        lits.push(Lit { id: story.id, title: story.title });
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
            create_story,
            delete_story,
            story,
        ])
}
