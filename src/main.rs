/*
 *  nickel_with_db test repo
 *      zpallin
 *
 *  The purpose of this repository is to test using a db connection with nickel
 *  to create a web application of sufficient complexity
 */

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate time;

use time::Timespec;
use std::env;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::db::options::CreateCollectionOptions;

/*
 * objects defined for database experiment
 */
#[derive(Debug)]
struct Poll {
    id: i32,
    question: String,
    time_created: Timespec,
}

#[derive(Debug)]
struct Vote {
    id: i32,
    name: String,
    vote: bool,
    time_created: Timespec,
}

fn db_options() -> CreateCollectionOptions {
    let mut options = CreateCollectionOptions::new();
    options.capped = true;
    options.size = Some(100000);
    options
}

fn build(c: Client) {
    println!("Lol build");
    let db = c.db("pollsapp");

    db.create_collection("polls", Some(db_options()))
        .ok().expect("Failed to create polls collection");
    
    db.create_collection("votes", Some(db_options()))
        .ok().expect("Failed to create votes collection");
}

fn list(c: Client) {
    let db = c.db("pollsapp");
 
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let client = Client::connect("192.168.33.10", 27017)
        .ok().expect("Failed to initialize client.");

    if args.len() > 1 && args[1] == "build" {
        build(client);
    }
}
