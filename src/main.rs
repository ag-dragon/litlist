#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use models::*;

use rocket::{
    fs::{FileServer, relative},
    serde::Serialize,
    response::{Redirect, Flash},
    form::Form,
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

#[derive(FromForm)]
pub struct CreateStory<'r> {
    title: &'r str,
}

#[get("/create")]
fn create_story_get() -> Flash<Redirect> {
    use schema::stories;

    let new_story = self::models::NewStory { title: "test_story" };

    let connection = &mut establish_connection();
    diesel::insert_into(stories::table)
        .values(&new_story)
        .returning(Story::as_returning())
        .get_result(connection)
        .expect("Error saving new story");
    Flash::success(Redirect::to("/"), "Story successfully created.")
}

#[post("/create", data="<story>")]
fn create_story(story: Form<CreateStory<'_>>) -> Flash<Redirect> {
    use schema::stories;
    
    let new_story = self::models::NewStory { title: story.title };

    let connection = &mut establish_connection();
    diesel::insert_into(stories::table)
        .values(&new_story)
        .returning(Story::as_returning())
        .get_result(connection)
        .expect("Error saving new story");
    Flash::success(Redirect::to("/"), "Story successfully created.")
}

#[delete("/<story_id>")]
fn delete_story(story_id: i32) -> Flash<Redirect> {
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(stories.filter(id.eq(story_id)))
        .execute(connection)
        .expect("Error deleting stories");

    Flash::success(Redirect::to("/"), "Story successfully deleted.")
}

#[get("/stories/<story_id>")]
fn story(story_id: i32) -> Template {
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();

    let mut query_result = stories
        .filter(id.eq(story_id))
        .select(Story::as_select())
        .load(connection)
        .expect("Error loading stories");

    let story_o = query_result.pop();
    let story: Story;
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
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();

    let query_result = stories
        .limit(5)
        .select(Story::as_select())
        .load(connection)
        .expect("Error loading stories");

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
            create_story_get,
            delete_story,
            story,
        ])
}
