use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

/// Creates the connection to our database.
pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3"; // Would take this from env vars in a real app
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connection to {}", db))
}

/// Inserts a new task into the database.
pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        // Better error handling needed here in real app
        // Also ignoring return value: usize, num rows created
        // Other databases may allow other return values as well (like id)
        .expect("Error inserting new task");
}

/// Gets a vector of all Tasks back from the database.
pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

/// Updates a task so that it is marked as done
pub fn update_task_done(connection: &SqliteConnection, task_id: i32) {
    use schema::task::dsl::*;
    diesel::update(task.find(task_id))
        .set(done.eq(true))
        .execute(connection)
        .expect(&format!("Error updating task {}", task_id));
}

/// Deletes a task
pub fn delete_task(connection: &SqliteConnection, task_id: i32) {
    use schema::task::dsl::*;
    diesel::delete(task.find(task_id))
        .execute(connection)
        .expect(&format!("Error deleting task {}", task_id));
}
