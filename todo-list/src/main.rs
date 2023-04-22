mod todo_list {
    pub mod todo;
    pub mod todos;
}

use crate::todo_list::todo::Todo;
use crate::todo_list::todos::Todos;
use colored::Colorize;
use std::{io, process, vec};
use uuid::Uuid;

fn create_todo() -> Todo {
    let (mut title, mut desc): (String, String) = (String::new(), String::new());

    println!("title -",);
    io::stdin()
        .read_line(&mut title)
        .expect("failed to read title");

    println!("desc - ");
    io::stdin()
        .read_line(&mut desc)
        .expect("failed to read title");

    let new_todo = Todo::new(Uuid::new_v4(), title, desc, false);
    new_todo
}

const MENU_OPTIONS: [&'static str; 5] = [
    "add todo",
    "delete todo",
    "delete multiple todo",
    "edit todo",
    "exit",
];

fn show_menu(todo_list: &mut Todos) {
    loop {
        println!("Menu - ");
        for (index, option) in MENU_OPTIONS.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to get option");

        let selected_option = user_input.trim().parse();

        match selected_option {
            Ok(1) => {
                let new_todo = create_todo();
                todo_list.add(new_todo.clone());
                println!("created new todo - {}", new_todo);
            }
            Ok(2) => {
                println!("please enter the id for todo");
                let mut delete_id = String::new();
                io::stdin()
                    .read_line(&mut delete_id)
                    .expect("failed to get id");

                let delete_uuid = Uuid::parse_str(&delete_id.trim()).unwrap_or_else(|_| {
                    println!("invalid uuid provided");
                    Uuid::nil()
                });

                let is_deleted = todo_list.delete(delete_uuid);

                if is_deleted {
                    println!("deleted todo with id - {}", delete_uuid);
                } else {
                    println!("todo with given id not found");
                }
            }
            Ok(3) => {
                println!("enter space separated ids to delete");

                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("failed to read uuids");

                let delete_uuids_str: Vec<&str> = user_input.trim().split(" ").collect();
                let mut delete_uuids: Vec<Uuid> = vec![];
                let mut invalid_uuids: Vec<&str> = vec![];

                for uuid_str in delete_uuids_str {
                    match Uuid::try_parse(uuid_str) {
                        Ok(uuid) => delete_uuids.push(uuid),
                        Err(_) => invalid_uuids.push(uuid_str),
                    }
                }

                println!("invalid uuids (being skipped) - {:?}", invalid_uuids);
                println!("deleting uuids - {:?}", delete_uuids);

                let result = todo_list.delete_multiple(&delete_uuids);

                if !result.deleted.is_empty() {
                    println!("deleted todos (ids) - {:?}", result.deleted);
                }

                if !result.deleted.is_empty() {
                    println!("not found todos for ids - {:?}", result.not_found)
                }
            }
            Ok(4) => println!("edit a todo"),
            Ok(5) => {
                println!("{}", "closing application, thanks".yellow());
                process::exit(0);
            }
            _ => println!("not a valid option"),
        }
    }
}

fn main() {
    let mut todo_list = Todos::new();
    show_menu(&mut todo_list);

    let new_todo = create_todo();
    todo_list.add(new_todo);

    todo_list.delete(Uuid::new_v4());

    println!("{}", todo_list);
}
