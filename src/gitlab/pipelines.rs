use super::client::GitlabClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    pub id: u64,
    pub status: String,
    pub r#ref: String,
    pub updated_at: String,
}

pub async fn list_pipelines(client: &GitlabClient, project_path: &str) -> Result<Vec<Pipeline>> {
    let encoded_path = project_path.replace("/", "%2F");
    let url = format!(
        "https://{}/api/v4/projects/{}/pipelines?per_page=20",
        client.host, encoded_path
    );

    let res = client.client.get(&url).send().await?;
    let pipelines: Vec<Pipeline> = res.json().await?;
    
    Ok(pipelines)
}
