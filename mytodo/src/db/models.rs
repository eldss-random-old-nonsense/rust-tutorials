//! Contains data structures that model the tables within
//! the database. Note that structs with the Insertable trait
//! will likely not contain all fields because we want the
//! database to create some of the fields automatically (such
//! as the id).

use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Identifiable)]
#[table_name = "task"]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}
