mod todo;

use colored::*;
use todo::Todo;

fn main() {
    let new_todo = Todo::new(
        1,
        String::from("new todo"),
        String::from("this is desc"),
        false,
    );

    println!("{} {}", new_todo, "hello".green());
}
