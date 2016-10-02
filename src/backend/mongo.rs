/*
 * nickel_with_db::backend::mongo
 *      zpallin
 *      2016
 */

use std::collections::HashMap;
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;
use backend::{DatabaseServer};

/*
 * I need to think of some way to have the mongo object global
 * where it gets declared in main
 * and then all models can use it...
 * I need a generic DB config object...
 */

pub enum Query {
    All,
    Select {query_string: String},
}

pub struct db {
    server: DatabaseServer,
}

impl db {
    pub fn new(db_addr: String) -> db {
        db { 
            server: DatabaseServer::new(db_addr)
        }
    }

    pub fn query<'a>(&self, query: &'a Query) {
        match *query {
            Query::All => println!("Get all data!"),
            Query::Select {ref query_string} => println!("Selecting: {}",query_string),
        }
    }
    pub fn insert(&self, data: &HashMap<&str, &str>) {
        for (key, value) in data {
            println!("{}: {}", key, value);
        }
    }
}

