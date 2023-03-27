use crate::{models::item_model::Item, repository::mongodb_repos::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/item", data = "<new_item>")]
pub fn create_item(
    db: &State<MongoRepo>,
    new_item: Json<Item>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Item {
        id: None,
        // item_type: new_item.item_type.to_owned(),
    };
    let item_detail = db.create_item(data);
    match item_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/item/<path>")]
    pub fn get_item(db: &State<MongoRepo>, path: String) -> Result<Json<Item>, Status> {
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest);
        };
        let item_detail = db.get_item(&id);
        match item_detail {
            Ok(item) => Ok(Json(item)),
            Err(_) => Err(Status::InternalServerError),
        }
    }