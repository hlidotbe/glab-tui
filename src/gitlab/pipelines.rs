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
    let mut stage_min_id = std::collections::HashMap::new();
    for j in all_jobs.iter() {
        let entry = stage_min_id.entry(j.stage.clone()).or_insert(j.id);
        if j.id < *entry {
            *entry = j.id;
        }
    }
    all_jobs.sort_by(|a, b| {
        let min_a = stage_min_id.get(&a.stage).cloned().unwrap_or(0);
        let min_b = stage_min_id.get(&b.stage).cloned().unwrap_or(0);
        if min_a != min_b {
            min_a.cmp(&min_b)
        } else if a.stage != b.stage {
            a.stage.cmp(&b.stage)
        } else {
            a.id.cmp(&b.id)
        }
    });
    Ok(all_jobs)
}

pub async fn get_job_trace(client: &GitlabClient, project_path: &str, job_id: u64) -> Result<String> {
    let encoded_path = project_path.replace("/", "%2F");
    let endpoint = format!("/projects/{}/jobs/{}/trace", encoded_path, job_id);
    client.fetch_raw_api(&endpoint).await
}
