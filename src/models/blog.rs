/*
 * nickel_with_db::models::blog
 *      zpallin
 *      2016
 */

// std
use time::{Timespec};
use std::collections::HashMap;

// nwdb
use backend::{DatabaseConfig, Schema};
use nickel::Params;

// mongo
use bson::{Bson, Document};
use bson::ordered::OrderedDocument;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;

fn get_conn<'a>(dbconf: &'a DatabaseConfig) -> Client {
    Client::connect(&dbconf.host, dbconf.port)
        .ok()
        .expect("Failed to initalize Blog DB")
}

pub struct BlogData {
    pub title: Schema,
    pub author: Schema,
    pub category: Schema,
    pub content: Schema,
}

#[derive(Clone,Debug)]
pub struct Blog {
    dbconf: DatabaseConfig,
}

impl Blog {
    pub fn new(db_str: &str) -> Blog {
        let dbconf = DatabaseConfig::new(db_str);

        Blog {
            dbconf: dbconf,
        }
    }

    pub fn db(&self) -> Database {
        get_conn(&self.dbconf).db(&self.dbconf.dbname)
    }

    pub fn collection(&self, coll: &str) -> Collection {
        self.db().collection(coll)
    }

    pub fn insert(&self, data: &Params) {
       
        let mut doc = OrderedDocument::new();
        doc.insert("title", data.get("title").unwrap_or("Title?"));
        doc.insert("author", data.get("author").unwrap_or("Author?"));
        doc.insert("category", data.get("category").unwrap_or("Category?"));
        doc.insert("content", data.get("content").unwrap_or("Content?"));
        
        self.collection("blog")
            .insert_one(doc,None)
            .unwrap();
    }

    pub fn all_as_Vec(&self) -> Vec<String> {

        let mut list = Vec::new();

        let cursor = self.collection("blog")
            .find(None, None)
            .unwrap();

        for result in cursor {
            if let Ok(item) = result {
                if let Some(&Bson::String(ref value)) = item.get("value") {

                    list.push(value.clone());

                } else if let Some(&Bson::String(ref value)) = item.get("id") {
                    
                    println!(
                        "{} doesn't have this collection \
                        (i don't know what this means)",
                        value
                    );

                }
            }
        }

        // return list
        list
    }
}
