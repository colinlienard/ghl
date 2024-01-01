use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

pub struct Github {
    pub token: String,
}

impl Github {
    pub fn new(token: &str) -> Self {
        let token = token.to_string();
        Self { token }
    }

    // https://docs.github.com/en/rest/pulls/pulls?apiVersion=2022-11-28#create-a-pull-request
    pub async fn create_pr(github: Github) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/repos/colinlienard/gitlight/branches")
            .headers(Github::construct_headers(github.token))
            .send()
            .await?;

        let text = response.text().await?;
        let json: serde_json::Value = serde_json::from_str(&text)?;
        println!("{:#?}", json);

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
