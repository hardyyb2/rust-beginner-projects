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

    let mut todo_list = Todos::new();

    todo_list.add(new_todo);
    todo_list.delete(2);

    println!("{}", todo_list);
}
