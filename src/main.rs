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

use std::sync::Arc;
use argparse::{ArgumentParser, Store};

// nickel with db
use nickel_with_db::helpers::FormHelper;
use nickel_with_db::models::blog::Blog;


// nickel
//use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter, FormBody};

fn main() {

    let mut server = Nickel::new();

    /*
     * Argument Parsing
     */

    // default values
    let mut bind = "localhost:8000".to_owned();
    let mut database = "localhost:27017".to_owned();

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

    // blog submit
    server.post("/blog/submit", middleware! { |_req, res|
        let blog = Blog::new("localhost:27017/blog");

        println!("/blog/submit");

        // get data from request
        let form_data = try_with!(res, _req.form_body());
        
        // insert data into db
        blog.insert(form_data);
        #[derive(RustcEncodable)]
        struct FakeData;
        let data = FakeData{};

        return res.render("templates/home.tpl", &data)
    });

    server.post("/blog/submit", middleware! { |_req, res|
        let blog = Blog::new("localhost:27017/blog");

        println!("/blog/submit");

        // get data from request
        let form_data = try_with!(res, _req.form_body());
        
        // insert data into db
        blog.insert(form_data);
        #[derive(RustcEncodable)]
        struct FakeData;
        let data = FakeData{};

        return res.render("templates/home.tpl", &data)
    });


    // blog new
    server.get("/otherblog/new", middleware! { |_req, res| 
        
        println!("/otherblog/new");

        let schema = vec![
            ("title", "input_text"),
            ("author", "input_text"),
            ("category", "input_text"),
            ("content", "textarea"),
        ];

        let data = FormHelper::generic("blog", &schema);
        return res.render("templates/form.tpl", &data)
    });

    // display blog
    server.get("/blog*", middleware! { |_req, res| 

        println!("/blog*");
        
        #[derive(RustcEncodable)]
        struct ViewData;
        let data = ViewData;
        return res.render("templates/home.tpl", &data)
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

