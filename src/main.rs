#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize};
use rocket::http::RawStr;

/*#[derive(Serialize)]
struct JsonRes {
    data: Vec(String),
    message: String,
}*/


#[get("/<num>")]
fn index(num: &RawStr) -> JsonValue {
    let mut vec = vec![1,2,3];
    match num.as_str() {
        "one" =>  json!({
                    "message": "Success",
                }),
        "two" => json!({
                    "message": vec
               }),
        _   => json!({
                    "message": "none"
               })
    }


}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}