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

#[post("/create", data="<story>")]
fn create_story(story: Form<CreateStory<'_>>) -> Flash<Redirect> {
    use schema::stories;
    
    let new_story = self::models::NewStory {
        title: story.title.to_string(),
        author: "no_one".to_string(),
        rating: None,
        comment: None,
        progress: None,
        length: None,
        link: None,
    };

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

#[put("/<story_id>", data="<story>")]
fn update_story(story_id: i32, story: Form<CreateStory<'_>>) -> Flash<Redirect> {
    use schema::stories::dsl::*;

    let new_story = self::models::NewStory {
        title: story.title.to_string(),
        author: "no_one".to_string(),
        rating: None,
        comment: None,
        progress: None,
        length: None,
        link: None,
    };

    let connection = &mut establish_connection();
    diesel::update(stories.find(story_id))
        .set(new_story)
        .returning(Story::as_returning())
        .execute(connection)
        .unwrap();

    Flash::success(Redirect::to(format!("/stories/{}", story_id)), "Story successfully, updated.")
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
            update_story,
            delete_story,
            story,
        ])
}
