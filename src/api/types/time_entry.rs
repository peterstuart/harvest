use super::{Client, Hours, Id, Minimal, Project, Task};
use chrono::{DateTime, Local, NaiveTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TimeEntry {
    pub id: Id<TimeEntry>,
    pub is_running: bool,
    pub hours: Hours,
    pub client: Minimal<Client>,
    pub project: Minimal<Project>,
    pub task: Minimal<Task>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimeEntry {
    pub fn is_from_today(&self) -> bool {
        let beginning_of_today = Local::today()
            .and_time(NaiveTime::from_hms(0, 0, 0))
            .unwrap();
        self.updated_at.with_timezone(&Local) >= beginning_of_today
    }
}

impl std::fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{}: {}/{} {}",
            self.client.name, self.project.name, self.task.name, self.hours
        )
        .fmt(f)
    }
}
