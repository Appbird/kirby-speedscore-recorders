use std::fs;
use rocket::{http::Status, response::{content, status}};
use serde::{Serialize, Deserialize};
use serde_json;
#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
struct DataBase {
    objective: i32
}

fn generate_internal_server_error(errmsg:String) -> status::Custom<content::RawText<String>> {
    status::Custom(
        Status::InternalServerError,
        content::RawText(errmsg)
    )
}
fn generate_result_response(data:String) -> status::Custom<content::RawText<String>> {
    status::Custom(
        Status::Ok,
        content::RawText(data)
    )
}

#[get("/")]
fn index() -> status::Custom<content::RawText<String>> {
    let data = match fs::read_to_string("./server/db/data.json") {
        Ok(d) => d,
        Err(e) => return generate_internal_server_error(format!("DB File not found. {}", e)),
    };
    let db:DataBase = match serde_json::from_str(&data) {
        Ok(d) => d,
        Err(e) => return generate_internal_server_error(format!("failed to find objective in json file. {}", e)),
    };
    return generate_result_response(db.objective.to_string());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}