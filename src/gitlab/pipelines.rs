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
    let endpoint_page1 = format!("/projects/{}/pipelines/{}/jobs?per_page=100&page=1", encoded_path, pipeline_id);
    let mut all_jobs: Vec<Job> = client.fetch_api(&endpoint_page1).await?;
    
    if all_jobs.len() == 100 {
        let mut handles = Vec::new();
        for page in 2..=10 {
            let endpoint = format!("/projects/{}/pipelines/{}/jobs?per_page=100&page={}", encoded_path, pipeline_id, page);
            let client_clone = client.clone();
            handles.push(tokio::spawn(async move {
                client_clone.fetch_api::<Vec<Job>>(&endpoint).await
            }));
        }
        
        for handle in handles {
            if let Ok(Ok(jobs)) = handle.await {
                if jobs.is_empty() {
                    break;
                }
                let jobs_len = jobs.len();
                all_jobs.extend(jobs);
                if jobs_len < 100 {
                    break;
                }
            } else {
                break;
            }
        }
    }
    all_jobs.reverse();
    Ok(all_jobs)
}

pub async fn get_job_trace(client: &GitlabClient, project_path: &str, job_id: u64) -> Result<String> {
    let encoded_path = project_path.replace("/", "%2F");
    let endpoint = format!("/projects/{}/jobs/{}/trace", encoded_path, job_id);
    client.fetch_raw_api(&endpoint).await
}
