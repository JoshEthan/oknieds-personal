use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::item_model::Item;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;

pub struct MongoRepo {
    col: Collection<Item>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Item> = db.collection("Item");
        MongoRepo { col }
    }

    pub fn create_item(&self, new_item: Item) -> Result<InsertOneResult, Error> {
        let new_doc = Item {
            id: None,
            // item_type: new_item.item_type,
        };
        let item = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating item");
        Ok(item)
    }

    pub fn get_item(&self, id: &String) -> Result<Item, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let item_detail = self
                .col
                .find_one(filter, None)
                .ok()
                .expect("Error getting user's detail");
        Ok(item_detail.unwrap())
    }
}