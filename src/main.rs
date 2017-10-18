extern crate diesel_frunk_test;
extern crate diesel;
extern crate frunk_core;

use self::diesel_frunk_test::*;
use self::diesel_frunk_test::models::*;
use self::diesel::prelude::*;
use frunk_core::labelled::*;

fn main() {
    let post = Post {
        id: 10,
        title: "title".to_string(),
        body: "body".to_string(),
        published: true,
    };
    let insert_post = NoIdPost::transform_from(post);
    println!("{:?}", insert_post);
}
