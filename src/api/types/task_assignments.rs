use super::TaskAssignment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskAssignments {
    pub task_assignments: Vec<TaskAssignment>,
}
