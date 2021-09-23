use super::ProjectAssignment;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectAssignments {
    pub project_assignments: Vec<ProjectAssignment>,
}
