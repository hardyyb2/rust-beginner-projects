struct Todo {
    id: i32,
    title: String,
    desc: String,
    completed: bool,
}

impl Todo {
    fn new(id: i32, title: String, desc: String, completed: bool) -> Self {
        Self {
            id,
            title,
            desc,
            completed,
        }
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "id - {}\ntitle - {}\ndesc - {}\ncompleted - {}\n",
            self.id, self.title, self.desc, self.completed
        )
    }
}

fn main() {
    let new_todo = Todo::new(
        1,
        String::from("new todo"),
        String::from("this is desc"),
        false,
    );

    println!("{}", new_todo);
}
