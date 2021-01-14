extern crate diesel_join;
extern crate diesel;


use schema::{posts, users};

use self::diesel_join::*;
use self::model::*;
use self::diesel::prelude::*;

fn main() {

    let connection = establish_connection();
    let data =delete_by(&connection, &1);
 }
