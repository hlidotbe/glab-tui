use super::client::GitlabClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Pipeline {
    pub id: u64,
    pub status: String,
    pub r#ref: String,
    pub updated_at: String,
}

pub async fn list_pipelines(client: &GitlabClient, project_path: &str) -> Result<Vec<Pipeline>> {
    let encoded_path = project_path.replace("/", "%2F");
    let endpoint = format!("/projects/{}/pipelines?per_page=20", encoded_path);
    client.fetch_api(&endpoint).await
}

#[derive(Debug, Deserialize, Clone)]
pub struct Job {
    pub id: u64,
    pub status: String,
    pub stage: String,
    pub name: String,
}

pub async fn list_pipeline_jobs(client: &GitlabClient, project_path: &str, pipeline_id: u64) -> Result<Vec<Job>> {
    let encoded_path = project_path.replace("/", "%2F");
    let endpoint = format!("/projects/{}/pipelines/{}/jobs?per_page=100", encoded_path, pipeline_id);
    client.fetch_api(&endpoint).await
}

pub async fn get_job_trace(client: &GitlabClient, project_path: &str, job_id: u64) -> Result<String> {
    let encoded_path = project_path.replace("/", "%2F");
    let endpoint = format!("/projects/{}/jobs/{}/trace", encoded_path, job_id);
    client.fetch_raw_api(&endpoint).await
}
