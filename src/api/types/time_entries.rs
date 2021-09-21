use super::TimeEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeEntries {
    pub time_entries: Vec<TimeEntry>,
}
