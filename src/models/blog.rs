/*
 * nickel_with_db::models::blog
 *      zpallin
 *      2016
 */

use time::{Timespec};

pub struct Blog {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_time: Timespec,
}

