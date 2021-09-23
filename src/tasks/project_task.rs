use crate::{api::types::Client, Minimal, Project, ProjectAssignment, Task};

#[derive(Clone, Debug)]
pub struct ProjectTask {
    pub client: Minimal<Client>,
    pub project: Minimal<Project>,
    pub task: Minimal<Task>,
}

impl ProjectTask {
    pub fn from(project_assignments: &[ProjectAssignment]) -> Vec<Self> {
        project_assignments
            .iter()
            .flat_map(|project_assignment| {
                project_assignment
                    .task_assignments
                    .iter()
                    .map(|task_assignment| Self::new(project_assignment, &task_assignment.task))
                    .collect::<Vec<ProjectTask>>()
            })
            .collect()
    }

    fn new(project_assignment: &ProjectAssignment, task: &Minimal<Task>) -> Self {
        Self {
            client: project_assignment.client.clone(),
            project: project_assignment.project.clone(),
            task: task.clone(),
        }
    }
}

impl std::fmt::Display for ProjectTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{}: {}/{}", self.client, self.project, self.task).fmt(f)
    }
}
