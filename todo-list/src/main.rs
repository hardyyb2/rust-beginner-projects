mod todo;
mod todos;

use todo::Todo;
use todos::Todos;

fn main() {
    let new_todo = Todo::new(
        1,
        String::from("new todo"),
        String::from("this is desc"),
        false,
    );

    let new_todo_2 = Todo::new(
        2,
        String::from("new todo 2"),
        String::from("this is desc 2"),
        true,
    );

    let mut todo_list = Todos::new();

    todo_list.add(new_todo);
    todo_list.add(new_todo_2);

    todo_list.delete(4);

    println!("{}", todo_list);
}
