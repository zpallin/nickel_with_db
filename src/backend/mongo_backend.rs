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

/*
 * I need to think of some way to have the mongo object global
 * where it gets declared in main
 * and then all models can use it...
 * I need a generic DB config object...
 */

struct Mongo {
    host: String,
    port: u16,
}


