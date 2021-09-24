use crate::{Client, CreateTimeEntry, Id, ListTimeEntries, Project, Result, Task, TimeEntry};
use anyhow::anyhow;
use std::time::Duration;
use tokio::time;

pub async fn show_current(client: &Client) -> Result<()> {
    match get_current(client).await? {
        Some(time_entry) => println!("{}", time_entry),
        None => println!("No timer"),
    }

    Ok(())
}

pub async fn show_current_continuous(client: &Client) {
    loop {
        if show_current(client).await.is_err() {
            println!("Error");
        }

        time::sleep(Duration::from_secs(5)).await;
    }
}

pub async fn show_toggle(client: &Client) -> Result<()> {
    let time_entry = toggle(client).await?;
    println!("{}", time_entry);

    Ok(())
}

pub async fn show_start(client: &Client, project_id: Id<Project>, task_id: Id<Task>) -> Result<()> {
    let time_entry = start(client, project_id, task_id).await?;
    println!("{}", time_entry);

    Ok(())
}

async fn get_current(client: &Client) -> Result<Option<TimeEntry>> {
    let time_entries = client
        .get_time_entries(&ListTimeEntries::for_today())
        .await?;

    Ok(running_time_entry(&time_entries)
        .or_else(|| most_recent_time_entry_from_today(&time_entries)))
}

async fn toggle(client: &Client) -> Result<TimeEntry> {
    let current_time_entry = get_current(client)
        .await?
        .ok_or_else(|| anyhow!("There is no current timer"))?;
    if current_time_entry.is_running {
        client.stop_time_entry(current_time_entry.id).await
    } else {
        client.restart_time_entry(current_time_entry.id).await
    }
}

fn running_time_entry(time_entries: &[TimeEntry]) -> Option<TimeEntry> {
    time_entries
        .iter()
        .cloned()
        .find(|time_entry| time_entry.is_running)
}

fn most_recent_time_entry_from_today(time_entries: &[TimeEntry]) -> Option<TimeEntry> {
    let mut time_entries = time_entries.to_vec();
    time_entries.sort_by_key(|time_entry| time_entry.updated_at);

    time_entries
        .last()
        .and_then(|time_entry| {
            if time_entry.is_from_today() {
                Some(time_entry)
            } else {
                None
            }
        })
        .cloned()
}

async fn start(client: &Client, project_id: Id<Project>, task_id: Id<Task>) -> Result<TimeEntry> {
    let time_entry = client
        .get_time_entries(&ListTimeEntries::for_today())
        .await?
        .into_iter()
        .find(|time_entry| time_entry.project.id == project_id && time_entry.task.id == task_id);

    Ok(match time_entry {
        Some(time_entry) => {
            if time_entry.is_running {
                time_entry
            } else {
                client.restart_time_entry(time_entry.id).await?
            }
        }
        None => {
            let params = CreateTimeEntry::for_today(project_id, task_id);
            client.create_time_entry(&params).await?
        }
    })
}
