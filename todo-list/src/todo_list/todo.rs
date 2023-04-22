use colored::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct Todo {
    pub id: Uuid,
    title: String,
    desc: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: Uuid, title: String, desc: String, completed: bool) -> Self {
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
        let completed = if self.completed {
            "yes".green()
        } else {
            "no".red()
        };

        writeln!(
            f,
            "id - {}\ntitle - {}\ndesc - {}\ncompleted - {}\n",
            self.id.to_string().blue(),
            self.title.green(),
            self.desc.green(),
            completed
        )
    }
}
