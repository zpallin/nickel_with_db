/*
 * nickel_with_db::models::blog
 *      zpallin
 *      2016
 */

// std
use time::{Timespec};
use std::collections::HashMap;

// nwdb
use backend::{DatabaseServer, Backend, Schema};
use nickel::Params;

// mongo
use bson::{Bson, Document};
use bson::ordered::OrderedDocument;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;

pub struct BlogData {
    pub title: Schema,
    pub author: Schema,
    pub category: Schema,
    pub content: Schema,
}

pub struct Blog {
    server: DatabaseServer,
}

impl Blog {
    pub fn new(server: DatabaseServer) -> Blog {
        Blog {
            server: server,
        }
    }

    pub fn conn(&self) -> Client {
        Client::connect(&self.server.host, self.server.port)
            .ok()
            .expect("Failed to initalize Blog DB")
    }

    pub fn db(&self) -> Database {
        self.conn().db(&self.server.dbname)
    }

    pub fn collection(&self, coll: &str) -> Collection {
        self.db().collection(coll)
    }

    pub fn insert(&self, data: Params) {
       
        let mut doc = OrderedDocument::new();
        doc.insert("title", data.get("title").unwrap_or("Title?"));
        doc.insert("author", data.get("title").unwrap_or("Title?"));
        doc.insert("category", data.get("category").unwrap_or("Title?"));
        doc.insert("content", data.get("content").unwrap_or("Title?"));
        
        self.collection("blog")
            .insert_one(doc,None)
            .unwrap();
    }
}
