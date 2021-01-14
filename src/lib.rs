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

pub fn find_post_by_uid(conn: &PgConnection, user_id: &i32) -> () {
    use schema::{posts, users};

    let data = posts::table.inner_join(users::table)
    .select((users::name, posts::title))
    .load::<(Post, User)>(conn)
    .expect("error");
}

