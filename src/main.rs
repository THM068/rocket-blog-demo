mod hash;
mod setup;

#[macro_use] extern crate rocket;
use hash::urlshortner;
use rocket::{build, Request};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use std::collections::HashMap;
use crate::setup::set_up_db;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
#[derive(FromForm)]
struct Book {
    title: String,
    author: String,
    isbn: String
}

#[derive(FromForm,Debug)]
struct UrlShortner {
    url: String
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    format!("Book title is {}", book_form.title)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Hello Thando"
    })
}

#[get("/load")]
fn loadme() -> Template {
    Template::render("hello", context! {})
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! {
        title: "About"
    })
}

#[get("/url-shortner")]
fn url_shortner() -> Template {
    Template::render("url-shortner", context! {
        title: "URL Shortner",
    })
}

#[post("/url-shortner", data = "<url>")]
fn url_shortner_post(url: Form<UrlShortner>) -> String {
    format!("URL is http://short-url.at/{}", urlshortner(url.url.as_str()))
}


#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", context! {
        title: "Contact"
    })
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello world"
}

#[get("/hello/<name>")]
fn hello_name(name: &str) -> String {
    format!("Hello {}", name)
}
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}
#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("Error setting up db: {}", err)
    };

    rocket::build()
    .manage(db)
    .mount("/", routes![index, hello, delay, hello_name, new_book, loadme, about, contact,
        url_shortner, url_shortner_post])
    .mount("/public", rocket::fs::FileServer::from("static/"))
    .register("/", catchers![not_found])
    .attach(Template::fairing())

}
