use std::collections::HashMap;
use std::process;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

use crate::{config::Config, git};

pub struct Github<'a> {
    token: &'a str,
    client: reqwest::Client,
}

impl<'a> Github<'a> {
    pub fn new(token: &'a str) -> Self {
        let client = reqwest::Client::new();
        Self { token, client }
    }

    pub async fn create_pr(&self, config: Config) -> Result<String, Box<dyn std::error::Error>> {
        let repo = git::get_current_repo()?;
        let default_desc = Config::get_default_desc()?;
        let base = git::get_default_branch()?;
        let draft = String::from("true");

        let body = HashMap::from([
            ("title", &config.pr_name),
            ("head", &config.branch),
            ("base", &base),
            ("body", &default_desc),
            ("draft", &draft),
        ]);

        let response = self
            .client
            .post(format!("https://api.github.com/repos/{}/pulls", &repo))
            .body(serde_json::to_string(&body)?)
            .headers(Github::construct_headers(self.token))
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Could not create PR");
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        let text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&text)?;

        Ok(json["html_url"].to_string())
    }

    pub async fn assign_to_pr(
        &self,
        username: &str,
        number: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let repo = git::get_current_repo()?;

        let body = HashMap::from([("assignees", vec![username])]);

        let response = self
            .client
            .post(format!(
                "https://api.github.com/repos/{}/issues/{}/assignees",
                &repo, number
            ))
            .body(serde_json::to_string(&body)?)
            .headers(Github::construct_headers(self.token))
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Could not assign you to the PR");
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        Ok(())
    }

    pub async fn get_username(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .headers(Github::construct_headers(self.token))
            .send()
            .await?;

        let text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&text)?;

        Ok(json["login"].as_str().unwrap().to_string())
    }

    fn construct_headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(header_value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
            headers.insert(AUTHORIZATION, header_value);
        }
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(""));
        headers
    }
}
