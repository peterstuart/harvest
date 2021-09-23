mod project_task;

use crate::{Client, Result};
use project_task::ProjectTask;

pub async fn list(client: &Client) -> Result<()> {
    let project_assignments = client.get_project_assignments().await?;
    let project_tasks = ProjectTask::from(&project_assignments);

    for project_task in project_tasks {
        println!("{}", project_task);
    }

    Ok(())
}
