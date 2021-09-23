use super::{Client, Id, Minimal, Project, TaskAssignment};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectAssignment {
    pub id: Id<ProjectAssignment>,
    pub is_active: bool,
    pub project: Minimal<Project>,
    pub client: Minimal<Client>,
    pub task_assignments: Vec<TaskAssignment>,
}
