use actix_web::{ web, HttpResponse };
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;
use crate::state::read_file;

use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    println!("deleting");
    let state: Map<String, Value> = read_file("./state.json");
    let title: String = to_do_item.title.clone();
    let status: String = to_do_item.status.clone();

    match to_do_factory(&status, &title) {
        Err(_item) => return HttpResponse::BadRequest().json(format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state)
    }

    return HttpResponse::Ok().json(return_state())
}
