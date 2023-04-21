use std::fmt;

use uuid::Uuid;

use crate::todo::Todo;

pub struct DeleteMultipleResult {
    pub deleted: Vec<Uuid>,
    pub not_found: Vec<Uuid>,
}

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
        let initial_len = self.todos.len();
        self.todos.retain(|todo| todo.id != id);

        if self.todos.len() < initial_len {
            true
        } else {
            false
        }
    }

    pub fn delete_multiple(&mut self, ids: &[Uuid]) -> DeleteMultipleResult {
        let mut not_found: Vec<Uuid> = vec![];
        let mut deleted: Vec<Uuid> = vec![];

        self.todos.retain(|todo| {
            if ids.contains(&todo.id) {
                deleted.push(todo.id);
                return true;
            } else {
                not_found.push(todo.id);
                false
            }
        });

        DeleteMultipleResult { deleted, not_found }
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
