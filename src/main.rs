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
extern crate time;

// std includes
//use std::env;
use std::collections::HashMap;

// nickel with db
//use nickel_with_db::models::blog;
//use nickel_with_db::backend::mongo;
use nickel_with_db::helpers::FormHelper;
use nickel_with_db::models::blog;

// argparse
use argparse::{ArgumentParser, Store}; //StoreTrue

// nickel
//use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter, FormBody};

fn main() {

    let mut server = Nickel::new();

    /*
     * Argument Parsing
     */
    let mut bind = "localhost:8000".to_owned();
    let mut database = "".to_owned();

    // parser
    {
        // parser cleans up after this scope. 
        // mutable variables above are borrowed and then modified
        let mut ap = ArgumentParser::new();

        ap.set_description("Running Nickel with a DB attached");
        ap.refer(&mut bind)
            .add_option(&["-b", "--bind"], Store,
            "Set bind address for webserver with port specified \
            ( format: \"--bind <bindhost>:<port>\" )");
        ap.refer(&mut database)
            .add_option(&["-d", "--database"], Store,
            "Set database address, port and db for the application. \
            ( format: \"--database <hostname>:<port>/<dbname>\" )");
        ap.parse_args_or_exit();
    }

    /*
     * Server states
     */
    server.post("/blog/submit", middleware! { |_req, res|
        let form_data = try_with!(res, _req.form_body());
//        let database = mongo::db::new("localhost:27017".to_owned());
//        let db2 = mongo::DatabaseServer;

        let mut data = HashMap::new();

        data.insert("title", form_data.get("title").unwrap_or("Title?"));
        data.insert("author", form_data.get("author").unwrap_or("Author?"));
        data.insert("category", form_data.get("category").unwrap_or("Category?"));
        data.insert("content", form_data.get("content").unwrap_or("Content?"));
        //data.insert("timestamp", form_data.get("timestamp").unwrap_or("Timestamp?"));

        // test print
        for (key, value) in data {
            println!("{}: {}", key, value);
        }

        #[derive(RustcEncodable)]
        struct FakeData;
        let data = FakeData{};

        return res.render("templates/home.tpl", &data)
    });

    server.get("/blog/new", middleware! { |_req, res| 
        let schema = vec![
            ("title", "input_text"),
            ("author", "input_text"),
            ("category", "input_text"),
            ("content", "textarea"),
        ];

        let data = FormHelper::generic("blog", &schema);
        return res.render("templates/form.tpl", &data)
    });

    // catch all for home page
    server.get("/*", middleware! { |_req, res|
        #[derive(RustcEncodable)]
        struct ViewData;
        let data = ViewData;
        return res.render("templates/home.tpl", &data)
    });

    server.listen(&*bind);
}
