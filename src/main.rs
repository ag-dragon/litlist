#[macro_use] extern crate rocket;

pub mod models;
pub mod schema;

use models::*;

use rocket::{
    fs::{FileServer, relative},
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

#[derive(FromForm)]
pub struct CreateStory {
    title: String,
    author: String,
    rating: Option<i32>,
    comment: Option<String>,
    progress: Option<i32>,
    length: Option<i32>,
    link: Option<String>,
}


#[post("/create", data="<story>")]
fn create_story(story: Form<CreateStory>) -> Flash<Redirect> {
    use schema::stories;
    
    let new_story = self::models::NewStory {
        title: story.title.clone(),
        author: story.author.clone(),
        rating: story.rating.clone(),
        comment: story.comment.clone(),
        progress: story.progress.clone(),
        length: story.length.clone(),
        link: story.link.clone(),
    };

    let connection = &mut establish_connection();
    let res = diesel::insert_into(stories::table)
        .values(&new_story)
        .returning(Story::as_returning())
        .execute(connection);

    match res {
        Ok(_) => Flash::success(Redirect::to("/"), "Story successfully created."),
        Err(e) => {
            error_!("Story creation error: {}", e);
            Flash::error(Redirect::to("/"), "Story could not be created due to internal error.")
        }
    }
}

#[delete("/<story_id>")]
fn delete_story(story_id: i32) -> Flash<Redirect> {
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();
    let res = diesel::delete(stories.filter(id.eq(story_id)))
        .execute(connection);

    match res {
        Ok(_) => Flash::success(Redirect::to("/"), "Story successfully deleted."),
        Err(e) => {
            error_!("Story deletion error: {}", e);
            Flash::error(Redirect::to("/"), "Story could not be deleted due to internal error.")
        }
    }
}

use crate::schema::stories;
#[derive(FromForm, AsChangeset)]
#[diesel(table_name = stories)]
pub struct UpdateStory<'r> {
    #[field(validate = len(1..))]
    title: Option<&'r str>,
    #[field(validate = len(1..))]
    author: Option<&'r str>,
    rating: Option<i32>,
    comment: Option<&'r str>,
    progress: Option<i32>,
    length: Option<i32>,
    link: Option<&'r str>,
}

#[put("/<story_id>", data="<story>")]
fn update_story(story_id: i32, story: Form<UpdateStory<'_>>) -> Flash<Redirect> {
    use schema::stories::dsl::*;

    let connection = &mut establish_connection();
    let res = diesel::update(stories.find(story_id))
        .set(story.into_inner())
        .returning(Story::as_returning())
        .execute(connection);

    match res {
        Ok(_) => Flash::success(Redirect::to("/"), "Story successfully, updated."),
        Err(e) => {
            error_!("Story updating error: {}", e);
            Flash::error(
                Redirect::to(format!("/stories/{}", story_id)),
                "Story could not be updated due to internal error."
            )
        }
    }
}

#[get("/stories/<story_id>")]
fn story(story_id: i32) -> Result<Template, Flash<Redirect>> {
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();

    let mut query_result = stories
        .filter(id.eq(story_id))
        .select(Story::as_select())
        .load(connection)
        .expect("Error loading stories");

    let story_o = query_result.pop();
    match story_o {
        Some(story) => Ok(Template::render("story", context! { story })),
        _ => Err(Flash::error(Redirect::to("/"), format!("No story with id: {}", story_id))),
    }
}

#[get("/")]
fn index() -> Template {
    use self::schema::stories::dsl::*;

    let connection = &mut establish_connection();

    let story_list = stories
        .select(Story::as_select())
        .load(connection)
        .expect("Error loading stories");

    Template::render("index", context! { story_list })
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
