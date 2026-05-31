use super::client::GitlabClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Author {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub iid: u64,
    pub title: String,
    pub state: String,
    pub labels: Vec<String>,
    pub updated_at: String,
    pub author: Author,
}

pub async fn list_issues(client: &GitlabClient, project_path: &str) -> Result<Vec<Issue>> {
    let encoded_path = project_path.replace("/", "%2F");
    let url = format!(
        "https://{}/api/v4/projects/{}/issues?state=opened",
        client.host, encoded_path
    );

    let res = client.client.get(&url).send().await?;
    let issues: Vec<Issue> = res.json().await?;
    
    Ok(issues)
}
