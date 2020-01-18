#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;
use rocket::http::{RawStr, Status, ContentType};
use rocket::request::Request;
use rocket::response;
use rocket::State;
use rocket::response::{Responder, Response};
use std::sync::Mutex;

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,

}

#[derive(Serialize)]
struct Groceries(Vec<String>);

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

#[get("/")]
fn get_list(data: State<Mutex<Groceries>>) -> ApiResponse {
    let grocery_list =
        match data.lock() {
            Err(_) =>
                return ApiResponse {
                    json: json!({"message": "Data is currently thread locked"}),
                    status: Status::Conflict
                },
            Ok(data) => data
        };

    ApiResponse {
        json: json!({"groceries": grocery_list.0 }),
        status: Status::Ok
    }
}

#[post("/<item>")]
fn add_item(data: State<Mutex<Groceries>>, item: String) -> ApiResponse {
    let mut grocery_list =
        match data.lock() {
            Err(_) =>
                return ApiResponse {
                    json: json!({"message": "Data is currently thread locked"}),
                    status: Status::Conflict
                },
            Ok(data) => data
        };

    grocery_list.0.push(item.to_string());

    ApiResponse {
        json: json!({"message": format!("Item {} added", &item)}),
        status: Status::Ok
    }
}

fn rocket() -> rocket::Rocket {
    let grocery_list = Mutex::new(Groceries(Vec::new()));

    rocket::ignite()
        .manage(grocery_list)
        .mount("/", routes![health])
        .mount("/list", routes![get_list])
        .mount("/add", routes![add_item])
}

fn main() {
    rocket().launch();
}