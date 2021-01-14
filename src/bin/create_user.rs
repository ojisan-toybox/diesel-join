extern crate diesel;
extern crate diesel_join;

use diesel_join::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    let mut name = String::new();
    let mut image = String::new();

    println!("What would you like your name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_right(); // Remove the trailing newline

    println!(
        "\nOk! Let's write image {} (Press {} when finished)\n",
        name, EOF
    );
    stdin().read_to_string(&mut image).unwrap();

    let user = create_user(&connection, name, &image);
    println!("\nSaved draft name {} image {} with id {}", user.name, user.image.unwrap(), user.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";