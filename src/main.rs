#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::json::{JsonValue};
use serde::Serialize;
use rocket::http::{Status, ContentType};
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
        status: Status::Created
    }
}

#[put("/<item>")]
fn remove_item(data: State<Mutex<Groceries>>, item: String) -> ApiResponse {

    let mut grocery_list =
        match data.lock() {
            Err(_) =>
                return ApiResponse {
                    json: json!({"message": "Data is currently thread locked"}),
                    status: Status::Conflict
                },
            Ok(data) => data
        };

    let length = grocery_list.0.len();
    grocery_list.0.retain(|x| x != &item.to_string());
    let new_length = grocery_list.0.len();

    if length == new_length {
        return ApiResponse {
            json: json!({"message": format!("Item {} could not be found", &item)}),
            status: Status::BadRequest
        }
    }

    ApiResponse {
        json: json!({"message": format!("Item {} removed", &item)}),
        status: Status::Accepted
    }

}

fn rocket() -> rocket::Rocket {
    let grocery_list = Mutex::new(Groceries(Vec::new()));

    rocket::ignite()
        .manage(grocery_list)
        .mount("/", routes![health])
        .mount("/list", routes![get_list])
        .mount("/add", routes![add_item])
        .mount("/remove", routes![remove_item])
}

fn main() {
    rocket().launch();
}