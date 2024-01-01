use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

use crate::{config::Config, git::Git};

pub struct Github {
    pub token: String,
}

impl Github {
    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        Self { token }
    }

    pub async fn create_pr(
        github: Github,
        config: Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Git::get_current_repo()?;
        let default_desc = Config::get_default_desc()?;
        let base = Git::get_default_branch()?;
        // let draft = String::from("true");

        let mut body = HashMap::new();
        body.insert("title", &config.pr_name);
        body.insert("head", &config.branch);
        body.insert("base", &base);
        body.insert("body", &default_desc);
        // body.insert("draft", &draft);

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.github.com/repos/".to_owned() + &repo + "/pulls")
            .body(serde_json::to_string(&body)?)
            .headers(Github::construct_headers(github.token))
            .send()
            .await?;

        let text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&text)?;
        println!("{:#?}", json["html_url"]);

        Ok(())
    }

    fn construct_headers(token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(header_value) = HeaderValue::from_str(&format!("Bearer {}", token.as_str())) {
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
