#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;
use rocket::http::{RawStr, Status, ContentType};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,

}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r>{
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/<num>")]
fn index(num: &RawStr) -> ApiResponse {
    let mut vec = vec![1,2,3];
    match num.as_str() {
        "one" =>  ApiResponse {
            json: json!({"data": vec}),
            status: Status::Ok
            },
        "two" => ApiResponse {
            json: json!({"message": "Try using two"}),
            status: Status::SeeOther
            },
        _   => ApiResponse {
            json: json!({"message": "This is so wrong"}),
            status: Status::Unauthorized
            }
    }
}

#[get("/")]
fn health() -> ApiResponse {
    ApiResponse {
        json: json!({"message": "Server is running at localhost::8000"}),
        status: Status::Ok
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/health", routes![health])
}
fn main() {
    rocket().launch();
}