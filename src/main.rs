/*
 *  nickel_with_db test repo
 *      zpallin
 *
 *  The purpose of this repository is to test using a db connection with nickel
 *  to create a web application of sufficient complexity
 */

#[macro_use] extern crate nickel;
extern crate nickel_mustache;
extern crate rustc_serialize;
extern crate nickel_with_db;
extern crate argparse;

// use the env
use std::env;

// nickel with db
use nickel_with_db::models::blog;
use nickel_with_db::backend::mongo_generate;
use nickel_with_db::backend::mongo_backend;
use argparse::{ArgumentParser, StoreTrue, Store};

// nickel
use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter};

fn main() { 

    let mut server = Nickel::new();
    //let args: Vec<_> = env::args().collect();
    let host_string = "localhost:27017".to_owned();

    /*
     * Argument Parsing
     */
    let mut bind = "localhost:8000".to_owned();
    {
        // parser cleans up after this scope. 
        // mutable variables above are borrowed and then modified
        let mut ap = ArgumentParser::new();
        ap.set_description("Running Nickel with a DB attached");
        ap.refer(&mut bind)
            .add_option(&["-b", "--bind"], Store,
            "Bind address for running web application");
        ap.parse_args_or_exit();
    }

//    let database = mongo_generate(host_string, &args);

/*
    server.get("/testrender", middleware! { |_req, res|        
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
*/

    server.get("**", middleware! { |_req, res|
        #[derive(RustcEncodable)]
        struct ViewData;
        let data = ViewData;
        return res.render("templates/home.tpl", &data)
    });

    server.listen(&*bind);
}
