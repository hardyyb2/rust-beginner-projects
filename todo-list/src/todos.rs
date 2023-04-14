use std::fmt;

use uuid::Uuid;

use crate::todo::Todo;

pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(index);
            true
        } else {
            println!("todo not found");
            false
        }
    }
}

impl fmt::Display for Todos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for todo in self.todos.iter() {
            writeln!(f, "{}", todo)?;
        }

        Ok(())
    }
}
