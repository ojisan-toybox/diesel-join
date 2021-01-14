extern crate diesel_join;
extern crate diesel;


use schema::{posts, users};

use self::diesel_join::*;
use self::model::*;
use self::diesel::prelude::*;

fn main() {

    let connection = establish_connection();
    find_post_by_uid(&connection, &1)
 }
