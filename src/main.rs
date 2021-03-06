/*
 *  nickel_with_db test repo
 *      zpallin
 *
 *  The purpose of this repository is to test using a db connection with nickel
 *  to create a web application of sufficient complexity
 */

// nickel_with_db
extern crate nickel_with_db;

// nickel crates
#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate rustc_serialize;

// argv uses
use std::env;

// use database modules
use nickel_with_db::backend::*;

// nickel uses
//use nickel_mustache::Render;
use nickel::{Nickel, HttpRouter};

fn main() { 

    let mut server = Nickel::new();
    let args: Vec<_> = env::args().collect();
    let host_string = "localhost:27017".to_owned();
    let database = mongo_generate(host_string, &args);

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
