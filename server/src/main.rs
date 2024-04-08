use std::fs;
use rocket::{fs::FileServer, http::Status, response::{content, status}};
use serde::{Serialize, Deserialize};
use serde_json;

#[macro_use] extern crate rocket;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct DataTuple {
    A: i32,
    B: String
}

#[derive(Serialize, Deserialize)]
struct DataBase (Vec<DataTuple>);

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

#[get("/hello")]
fn index() -> status::Custom<content::RawText<String>> {
    let data = match fs::read_to_string("./server/db/data.json") {
        Ok(d) => d,
        Err(e) => return generate_internal_server_error(format!("DB File not found. {}", e)),
    };
    let db:DataBase = match serde_json::from_str(&data) {
        Ok(d) => d,
        Err(e) => return generate_internal_server_error(format!("failed to find objective in json file. {}", e)),
    };
    let filtered:Vec<&DataTuple> = db.0.iter().filter(|e|  e.A <= 12 ).collect();
    let filtered_json = match serde_json::to_string(&filtered) {
        Ok(d) => d,
        Err(e) => return generate_internal_server_error(format!("failed to convert result into JSON. {}", e)),
    };
    return generate_result_response(filtered_json);
}

#[get("/search?<query>")]
fn search(query:Vec<&str>) -> status::Custom<content::RawText<String>> {
    for q in query {
        println!("{}", q);
    }
    return generate_result_response("responded.".to_string());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api", routes![
        index,
        search
    ])
    .mount("/", FileServer::from("./client/static/").rank(2))
}