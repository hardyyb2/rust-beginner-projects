mod todo;
mod todos;

use std::{io, process};
use uuid::Uuid;

use todo::Todo;
use todos::Todos;

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

const MENU_OPTIONS: [&'static str; 4] = ["add todo", "delete todo", "edit todo", "exit"];

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
                todo_list.add(new_todo);
                // println!("created new todo - {}", new_todo);
            }
            Ok(2) => {
                println!("please enter the id for todo");

                let mut delete_id = String::new();
                io::stdin()
                    .read_line(&mut delete_id)
                    .expect("failed to get id");
            }
            Ok(3) => println!("edit a todo"),
            Ok(4) => {
                println!("closing application, thanks");
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
