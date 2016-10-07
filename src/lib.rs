

#[macro_use] extern crate nickel;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate time;
extern crate rustc_serialize;
extern crate mustache;

pub mod models;
pub mod views;
pub mod backend;
pub mod helpers;
