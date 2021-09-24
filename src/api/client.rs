use super::{
    types::{CreateTimeEntry, ProjectAssignment, ProjectAssignments, TimeEntries, User},
    Auth, Id, ListTimeEntries, TimeEntry,
};
use crate::Result;
use serde::{de::DeserializeOwned, Serialize};

static BASE_URL: &str = "https://api.harvestapp.com/v2";
static USER_AGENT: &str = "Peter Stuart (peter@peterstuart.org)";

pub struct Client {
    reqwest_client: reqwest::Client,
}

impl Client {
    pub fn new(auth: &Auth) -> Result<Self> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(auth.headers()?)
            .build()?;

        Ok(Self { reqwest_client })
    }

    async fn get<Response: DeserializeOwned>(&self, endpoint: &str) -> Result<Response> {
        let url = Self::url(endpoint);
        Ok(self.reqwest_client.get(url).send().await?.json().await?)
    }

    async fn get_with_params<Params: Serialize, Response: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &Params,
    ) -> Result<Response> {
        let url = Self::url_with_params(endpoint, params)?;
        Ok(self.reqwest_client.get(url).send().await?.json().await?)
    }

    async fn patch<Response: DeserializeOwned>(&self, endpoint: &str) -> Result<Response> {
        let url = Self::url(endpoint);
        Ok(self.reqwest_client.patch(url).send().await?.json().await?)
    }

    async fn post<Body: Serialize, Response: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &Body,
    ) -> Result<Response> {
        let url = Self::url(endpoint);
        Ok(self
            .reqwest_client
            .post(url)
            .json(body)
            .send()
            .await?
            .json()
            .await?)
    }

    fn url(endpoint: &str) -> String {
        format!("{}{}", BASE_URL, endpoint)
    }

    fn url_with_params<Params: Serialize>(endpoint: &str, params: &Params) -> Result<String> {
        Ok(format!(
            "{}{}?{}",
            BASE_URL,
            endpoint,
            serde_url_params::to_string(params)?
        ))
    }
}

impl Client {
    pub async fn get_user(&self) -> Result<User> {
        self.get("/users/me.json").await
    }
}

impl Client {
    pub async fn get_time_entries(&self, params: &ListTimeEntries) -> Result<Vec<TimeEntry>> {
        let time_entries: TimeEntries = self.get_with_params("/time_entries", params).await?;
        Ok(time_entries.time_entries)
    }

    pub async fn create_time_entry(&self, params: &CreateTimeEntry) -> Result<TimeEntry> {
        self.post("/time_entries", params).await
    }

    pub async fn restart_time_entry(&self, id: Id<TimeEntry>) -> Result<TimeEntry> {
        let endpoint = format!("/time_entries/{}/restart", id);
        self.patch(&endpoint).await
    }

    pub async fn stop_time_entry(&self, id: Id<TimeEntry>) -> Result<TimeEntry> {
        let endpoint = format!("/time_entries/{}/stop", id);
        self.patch(&endpoint).await
    }
}

impl Client {
    pub async fn get_project_assignments(&self) -> Result<Vec<ProjectAssignment>> {
        let project_assignments: ProjectAssignments =
            self.get("/users/me/project_assignments").await?;
        Ok(project_assignments.project_assignments)
    }
}
