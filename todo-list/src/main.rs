mod todo;
mod todos;

use std::io;
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

fn main() {
    let mut todo_list = Todos::new();

    let new_todo = create_todo();
    todo_list.add(new_todo);

    todo_list.delete(Uuid::new_v4());

    println!("{}", todo_list);
}
