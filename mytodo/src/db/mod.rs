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
