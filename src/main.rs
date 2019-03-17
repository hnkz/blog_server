#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
 
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket_contrib::json::Json;
use rocket::Request;
use rocket::response::{self, Responder, Response };
use serde::{Deserialize, Serialize};
use rocket::http::Method;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
	title: String,
	tags: Vec<String>,
    content: String,
	timestamp: String
}
pub struct Blogs {
	contents: Json<Vec<Blog>>
}
// For CORS
impl<'r> Responder<'r> for Blogs {
	fn respond_to(self, req: &Request) -> response::Result<'r> {
		Response::build_from(self.contents.respond_to(req).unwrap())
			.raw_header("Access-Control-Allow-Origin", "*")
			.ok()
	}
}
#[get("/blogs")]
fn get_blogs() -> Blogs {
    Blogs {
        contents: Json(get_blog_json("/home/hnkz/blog_server/blog/blog.json")),
    }
}
fn get_blog_json(filename: &str) -> Vec<Blog> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let blogs: Vec<Blog> = serde_json::from_str(&contents).expect("cannot serialize");
    blogs
}
fn main() {
    rocket::ignite()
        .mount("/", routes![get_blogs])
        .launch();
}
