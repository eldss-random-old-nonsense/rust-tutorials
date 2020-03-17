//! A command line tool to interact with the todo database.
//!
//! The purpose of this tool is to have a layer of abstraction away
//! from direct database queries.

use mytodo::db::*;
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new <title>: create a new task");
    println!("    show: show tasks");
    println!("    done <task_id>: mark a task as done");
    println!("    remove <task_id>: remove a task");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "done" => record_task_done(&args[2..]),
        "remove" => remove_task(&args[2..]),
        _ => help(),
    }
}

/// Adds a new task to the database
fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

/// Displays all tasks in the database
fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        let status = if task.done { "done" } else { "pending" };
        println!("{}: {}, {}", task.id, task.title, status);
    }
}

/// Sets the status of a task to 'done'.
fn record_task_done(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing <task_id>");
        help();
        return;
    }

    let conn = establish_connection();
    let id: i32 = match args[0].parse() {
        Ok(int) => int,
        Err(e) => {
            println!("Error parsing task id: {}", e);
            help();
            return;
        }
    };

    update_task_done(&conn, id);
    println!("Update successful");
}

/// Removes a task.
fn remove_task(args: &[String]) {
    if args.len() < 1 {
        println!("remove: missing <task_id>");
        help();
        return;
    }

    let conn = establish_connection();
    let id: i32 = match args[0].parse() {
        Ok(int) => int,
        Err(e) => {
            println!("Error parsing task id: {}", e);
            help();
            return;
        }
    };

    delete_task(&conn, id);
    println!("Remove successful");
}
