/*
 *  nickel_with_db test repo
 *      zpallin
 *
 *  The purpose of this repository is to test using a db connection with nickel
 *  to create a web application of sufficient complexity
 */

// mongo crates
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate time;

// nickel crates
#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate rustc_serialize;

// argv uses
use std::env;

// mongo uses
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::Collection;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::db::options::CreateCollectionOptions;
//use time::Timespec;

// nickel uses
//use nickel_mustache::Render;
use nickel::{Nickel, HttpRouter};

/*
 * data object for passing around data
 */
struct DataObject {
    client: Client,
    db_name: String,
    coll_name: String,
}

impl DataObject {

    fn coll_name(&self) -> String { 
        self.coll_name.clone()
    }

    fn db_name(&self) -> String {
        self.db_name.clone()
    }

    fn get_db(&self) -> Database {
        self.client.db(&self.db_name)
    }

    fn get_coll(&self) -> Collection {
        self.get_db().collection(&self.coll_name)
    }

    fn add_value(&self, value: String) {
        self.get_coll().insert_one(doc!{ "value" => value },None).unwrap();
    }

    fn build(&self) {
        self.get_db()
            .create_collection(&self.coll_name(), Some(db_options()))
            .ok().expect(&format!("Failed to create {} collection", &self.coll_name));
    }

    fn list(&self) -> Vec<String> {

        let coll = self.get_coll();
        let cursor = coll.find(None, None).unwrap();
        let mut list = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                if let Some(&Bson::String(ref value)) = item.get("value") {

                    // if let statement won't allow us to move "question"
                    // so we need to clone it
                    println!("{}", value);
                    list.push(value.clone());
                } else if let Some(&Bson::String(ref value)) = item.get("id") {
                    println!("{} doesn't have question", value);
                } else {
                    println!("Nada");
                }
            }
        }
        list
    }
}

/*
 * functions used for interacting with db
 */
fn db_options() -> CreateCollectionOptions {
    let mut options = CreateCollectionOptions::new();
    options.capped = true;
    options.size = Some(100000);
    options
}

fn instructions(error_code: i32) {
    if error_code != 0 {
        println!("Wrong input (sorry no instructions yet)");
    }
    std::process::exit(error_code);
}

fn handle_args(c: Client, args: &Vec<String>) -> DataObject {
    let mut database = DataObject{ 
        client: c,
        db_name: "".to_owned(),
        coll_name: "".to_owned() 
    };

    if args.len() > 1 {
        let command: &str = &args[1];
        match command {
            "build" => {
                if args.len() == 4 {
                    println!("Building...");
                    println!(" - {}.{}", args[2], args[3]);
                    database.db_name = args[2].clone().to_owned();
                    database.coll_name = args[3].clone().to_owned();
                    database.build();
                    instructions(0);
                } else {
                    instructions(1);
                }
            },
            "add" => {
                if args.len() == 5 {
                    println!("Adding...");
                    database.db_name = args[2].clone().to_owned();
                    database.coll_name = args[3].clone().to_owned();
                    println!(" - {}.{} value: {}", 
                             database.db_name, 
                             database.coll_name,
                             args[4].clone());
                    database.add_value(args[4].clone());
                    instructions(0);
                } else {
                    instructions(1);
                }
            },
            "use" => {
                if args.len() == 4 {
                    println!("Using...");
                    println!(" - {}.{}", args[2], args[3]);
                    database.db_name = args[2].clone().to_owned();
                    database.coll_name = args[3].clone().to_owned();
                } else {
                    instructions(1);
                }
            },
            _ => instructions(1),
        }
    }
    database
}

fn main() { 

    // get db_host
    let mut db_host_str: String = "localhost:27017".to_owned();
    let db_host_arr = db_host_str.split(":").collect::<Vec<_>>();
    let db_host = db_host_arr[0].clone();
    let db_port: u16 = db_host_arr[1].to_owned().parse().ok().expect("Wanted a number");

    let client = Client::connect(db_host, db_port)
                 .ok().expect("Failed to initialize client.");
    let args: Vec<_> = env::args().collect();
    let mut server = Nickel::new();
    let database = handle_args(client, &args);

    server.get("/", middleware! { |_req, res|
        
        #[derive(RustcEncodable)]
        struct ViewData<'a> {
            title: &'a str,
            values: &'a Vec<String>
        }

        let db_coll_name = format!("{}.{}",
                                   database.db_name,
                                   database.coll_name);
        let data = ViewData{ title: &db_coll_name, values: &database.list() };
        return res.render("templates/list.tpl", &data)
    });

    server.listen("localhost:6767");
}
