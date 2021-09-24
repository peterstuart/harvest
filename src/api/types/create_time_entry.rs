use super::{Id, Project, Task};
use chrono::{Local, NaiveDate};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CreateTimeEntry {
    project_id: Id<Project>,
    task_id: Id<Task>,
    spent_date: NaiveDate,
}

impl CreateTimeEntry {
    pub fn new(project_id: Id<Project>, task_id: Id<Task>, spent_date: NaiveDate) -> Self {
        Self {
            project_id,
            task_id,
            spent_date,
        }
    }

    pub fn for_today(project_id: Id<Project>, task_id: Id<Task>) -> Self {
        Self {
            project_id,
            task_id,
            spent_date: Local::today().naive_local(),
        }
    }
}
