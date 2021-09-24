use chrono::{Local, NaiveDate};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ListTimeEntries {
    pub from: Option<NaiveDate>,
}

impl ListTimeEntries {
    pub fn for_today() -> Self {
        Self {
            from: Some(Local::today().naive_local()),
        }
    }
}
