#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use mytodo::db::{self, models::Task};
use rocket_contrib::{databases::diesel, json::Json};

/// A wrapper for responses.
#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

/// Creates a connection pool usable by handlers.
#[database("taskdb")]
struct TaskDbConn(diesel::SqliteConnection);

/// Gets all tasks currently in the database.
#[get("/tasks")]
fn tasks_get(conn: TaskDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for task in db::query_task(&*conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .attach(TaskDbConn::fairing())
        .mount("/", routes![tasks_get])
        .launch();
}
