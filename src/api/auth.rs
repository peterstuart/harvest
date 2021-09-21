use crate::Result;
use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

static HARVEST_ACCOUNT_ID: &str = "Harvest-Account-Id";

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub account_id: String,
    pub access_token: String,
}

impl Auth {
    pub fn headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.access_token).parse()?,
        );
        headers.insert(HARVEST_ACCOUNT_ID, self.account_id.parse()?);

        Ok(headers)
    }
}
