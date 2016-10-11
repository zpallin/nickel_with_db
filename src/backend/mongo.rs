/*
 * nickel_with_db::backend::mongo
 *      zpallin
 *      2016
 */

// collections
pub use bson::{Bson, Document};
pub use mongodb::{Client, ThreadedClient};
pub use mongodb::coll::Collection;
pub use mongodb::db::{ThreadedDatabase, Database};
pub use mongodb::db::options::CreateCollectionOptions;
pub use backend::{DatabaseConfig};

// database structure
/*
pub struct db {
    server: DatabaseConfig,
    options: CreateCollectionOptions,
}

impl db {
    pub fn new(db_addr: String) -> db {
        // default options
        let mut ops = CreateCollectionOptions::new();
        ops.capped = true;
        ops.size = Some(100000);

        // create new_db
        db { 
            server: DatabaseConfig::new(db_addr),
            options: ops,
        }
    }

    pub fn set_options(&mut self, ops: CreateCollectionOptions) {
        self.options = ops;    
    }

    pub fn query<'a>(&self, query: &'a Query) {
        match *query {
            Query::All => println!("Get all data!"),
            Query::Select {ref query_string} => println!("Selecting: {}",query_string),
        }
    }

    // inserts a value 

    pub fn create_collection(&self, collection_name: String) {
    }
}
*/
