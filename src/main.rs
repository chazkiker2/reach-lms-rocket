// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

use reach_lms_rocket;

// #[get("/")]
// fn index() -> &'static str {
// 	"Hello, world!"
// }

fn main() {
	reach_lms_rocket::rocket().launch();
    println!("Hello, world!");
}
