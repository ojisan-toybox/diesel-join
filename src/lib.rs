#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod model;
pub mod schema;

use diesel::{connection, prelude::*, sql_types::Integer};
use dotenv::dotenv;
use std::env;
use self::model::{NewPost, Post, NewUser, User};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body, user_id: &1 };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_user(conn: &PgConnection, name: &str, image: &str) -> User {
    use schema::users;

    let new_user = NewUser { name, image };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn find_post_by_uid(conn: &PgConnection, user_id: &i32)->(){
    use schema::{posts, users};

    let data = users::table.inner_join(posts::table)
    .load::<(User, Post)>(conn)
    .expect("error");
    println!("Displaying {} posts", data.len());

    for post in data {
        println!("{:?}", post.1.id);
    }
}

pub fn find_all_posts(conn: &PgConnection) -> () {
    use schema::{posts};

    let data = posts::table
    .load::<Post>(conn)
    .expect("error");

    println!("Displaying {} posts", data.len());

    for post in data {
        println!("{:?}", post.id);
        println!("{:?}", post.user_id);
    }
}

pub fn find_all_users(conn: &PgConnection) -> () {
    use schema::{users};

    let data = users::table
    .load::<User>(conn)
    .expect("error");
    println!("Displaying {} users", data.len());

    for post in data {
        println!("{:?}", post.id);
        println!("{:?}", post.name);
    }
}


pub fn delete_by(conn: &PgConnection, pid: &i32)->(){
    use schema::posts::dsl::*;
    let num_deleted = diesel::delete(posts.filter(id.eq(pid)))
        .execute(conn)
        .expect("Error deleting posts");
}