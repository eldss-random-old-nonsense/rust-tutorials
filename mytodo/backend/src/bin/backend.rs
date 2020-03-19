#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::{databases::diesel, json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use backend::db;
use mytodo::{JsonApiResponse, Task};

/// Creates a connection pool usable by handlers.
#[database("taskdb")]
struct TaskDbConn(diesel::SqliteConnection);

/// Gets all tasks currently in the database.
#[get("/tasks")]
fn tasks_get(conn: TaskDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    for db_task in db::query_task(&*conn) {
        let api_task = Task {
            id: db_task.id,
            title: db_task.title,
        };
        response.data.push(api_task);
    }

    Json(response)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .attach(TaskDbConn::fairing())
        .attach(cors)
        .mount("/", routes![tasks_get])
        .launch();

    Ok(())
}
