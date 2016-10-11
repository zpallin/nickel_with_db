/*
 * nickel_with_db::backend
 *      zpallin
 *      2016
 */

pub mod mongo;

use std::process;
use bson::Bson;

/*======================================== 
 *  Schema Struct
 */

pub struct Schema {
    value: String,
    display_type: String,
}

/*======================================== 
 *  Database Server Config Struct
 */

#[derive(Clone,Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

impl DatabaseConfig {

    pub fn new(db_str: &str) -> DatabaseConfig {

        let db_arr = db_str
            .split("/")
            .collect::<Vec<_>>();

        let host_arr = db_arr[0]
            .split(":")
            .collect::<Vec<_>>();

        // get host and db by cloning
        let host = host_arr[0]
            .clone()
            .to_owned();

        let dbname = db_arr[1]
            .clone()
            .to_owned();

        // get port by taking as u16 -> this is mongo specific
        // but could be made generic
        let port: u16 = host_arr[1]
            .to_owned()
            .parse()
            .ok()
            .expect("DatabaseConfig: Wanted a number");

        DatabaseConfig {
            host: host,
            port: port,
            dbname: dbname,
        }
    }
}

/*========================================  
 * Soon to deprecate stuff below.
 */

use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;


fn instructions(error_code: i32) {
    if error_code != 0 {
        println!("Wrong input (sorry no instructions yet)");
    }
    process::exit(error_code);
}

pub struct DataObject {
    pub client: Client,
    pub db_name: String,
    pub coll_name: String,
}

impl DataObject {

    pub fn coll_name(&self) -> String {
        self.coll_name.clone()
    }

    pub fn db_name(&self) -> String {
        self.db_name.clone()
    }

    pub fn get_db(&self) -> Database {
        self.client.db(&self.db_name)
    }

    pub fn get_coll(&self) -> Collection {
        self.get_db().collection(&self.coll_name)
    }

    pub fn add_value(&self, value: String) {
        self.get_coll().insert_one(doc!{ "value" => value }, None).unwrap();
    }

    pub fn build(&self) {
        self.get_db()
            .create_collection(&self.coll_name(), Some(db_options()))
            .ok().expect(&format!("Failed to create {} collection", &self.coll_name));
    }
    
    pub fn list(&self) -> Vec<String> {
        let coll = self.get_coll();
        let cursor = coll.find(None, None).unwrap();
        let mut list = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                if let Some(&Bson::String(ref value)) = item.get("value") {
                    // if let statement won't allow us to "move" the value
                    // so we need to clone it when we push the value into the lsit
                    list.push(value.clone());
                } else if let Some(&Bson::String(ref value)) = item.get("id") {
                    println!("{} doesn't have this collection", value);
                }
            }
        }

        // return the list
        list
    }
}

pub fn db_options() -> CreateCollectionOptions {
    let mut options = CreateCollectionOptions::new();
    options.capped = true;
    options.size = Some(100000);
    options
}

pub fn mongo_generate(host_string: String, args: &Vec<String>) -> DataObject {
    
    let host_arr = host_string.split(":").collect::<Vec<_>>();
    let host = host_arr[0].clone();
    let port: u16 = host_arr[1].to_owned().parse().ok().expect("Wanted a number");

    let client = Client::connect(host, port)
                 .ok().expect("Failed to initialize mongo_db client");

    let mut mongo_db = DataObject {
        client: client,
        db_name: "".to_owned(),
        coll_name: "".to_owned(),
    };

    if args.len() > 1 {
        
        let cmd: &str = &args[1];

        match cmd {
            "build" => {
                if args.len() == 4 {
                    println!("Building... ");
                    println!(" - {}.{}", args[2], args[3]);
                    mongo_db.db_name = args[2].clone().to_owned();
                    mongo_db.coll_name = args[3].clone().to_owned();
                    mongo_db.build();
                    instructions(0);
                } else {
                    instructions(1);
                }
            },
            "add" => {
                if args.len() == 5 {
                    println!("Adding... ");
                    mongo_db.db_name = args[2].clone().to_owned();
                    mongo_db.coll_name = args[3].clone().to_owned();
                    println!(" - {}.{} value: {}",
                             mongo_db.db_name,
                             mongo_db.coll_name,
                             args[4].clone());
                    mongo_db.add_value(args[4].clone());
                    instructions(0);
                } else {
                    instructions(1);
                }
            },
            "use" => {
                if args.len() == 4 {
                    println!("Using... ");
                    println!(" - {}.{}", args[2], args[3]);
                    mongo_db.db_name = args[2].clone().to_owned();
                    mongo_db.coll_name = args[3].clone().to_owned();
                } else {
                    instructions(1);
                }
            },
            _ => instructions(1),
        }
    }
    mongo_db
}
