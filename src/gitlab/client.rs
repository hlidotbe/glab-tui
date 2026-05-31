use anyhow::{Context, Result};
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct GitlabClient;

impl GitlabClient {
    pub async fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn fetch_api<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let output = Command::new("glab")
            .args(["api", endpoint])
            .output()
            .await
            .context("Failed to execute glab api command")?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("glab api failed: {}", err_msg);
        }

        let data: T = serde_json::from_slice(&output.stdout)?;
        Ok(data)
    }

    pub async fn fetch_raw_api(&self, endpoint: &str) -> Result<String> {
        let output = Command::new("glab")
            .args(["api", endpoint])
            .output()
            .await
            .context("Failed to execute glab api command")?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("glab api failed: {}", err_msg);
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    pub async fn fetch_labels(&self, project_path: &str) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct GitlabLabel {
            name: String,
        }
        let encoded_path = project_path.replace("/", "%2F");
        let endpoint = format!("/projects/{}/labels?per_page=100", encoded_path);
        let labels: Vec<GitlabLabel> = self.fetch_api(&endpoint).await?;
        Ok(labels.into_iter().map(|l| l.name).collect())
    }

    pub async fn fetch_members(&self, project_path: &str) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct GitlabMember {
            username: String,
        }
        let encoded_path = project_path.replace("/", "%2F");
        let endpoint = format!("/projects/{}/members/all?per_page=100", encoded_path);
        let members: Vec<GitlabMember> = self.fetch_api(&endpoint).await?;
        Ok(members.into_iter().map(|m| format!("@{}", m.username)).collect())
    }

    pub async fn fetch_milestones(&self, project_path: &str) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        struct GitlabMilestone {
            title: String,
        }
        let encoded_path = project_path.replace("/", "%2F");
        let endpoint = format!("/projects/{}/milestones?state=active&per_page=100", encoded_path);
        let milestones: Vec<GitlabMilestone> = self.fetch_api(&endpoint).await?;
        Ok(milestones.into_iter().map(|m| m.title).collect())
    }
}

pub async fn get_project_context() -> Result<String> {
    // Execute `git remote get-url origin`
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Ok("unknown/unknown".to_string());
    }

    let url = String::from_utf8(output.stdout)?.trim().to_string();
    
    // Parse url to extract namespace/repo
    let path = if url.starts_with("git@") {
        url.split(':').nth(1).unwrap_or("unknown/unknown")
    } else if url.starts_with("http") {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() >= 2 {
            let p = format!("{}/{}", parts[parts.len()-2], parts[parts.len()-1]);
            return Ok(p.trim_end_matches(".git").to_string());
        }
        "unknown/unknown"
    } else {
        "unknown/unknown"
    };

    Ok(path.trim_end_matches(".git").to_string())
}


