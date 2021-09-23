use super::{Id, Minimal, Task};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TaskAssignment {
    pub id: Id<TaskAssignment>,
    pub task: Minimal<Task>,
    pub is_active: bool,
}
