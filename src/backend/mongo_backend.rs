/*
 * nickel_with_db::backend::mongo_backend
 *      zpallin
 *      2016
 */

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;
use backend;

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

pub struct Mongo {
    server: backend::DatabaseServer,
}

impl Mongo {
    pub fn query<'a>(query: &'a Query) {
        match *query {
            Query::All => println!("Get all data!"),
            Query::Select {ref query_string} => println!("Selecting: {}",query_string),
        }
    }
}

