mod api; 
mod models;
mod repository;

#[macro_use] extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use api::item_api::{create_item, get_item};
use repository::mongodb_repos::MongoRepo;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world!")))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![hello])
}