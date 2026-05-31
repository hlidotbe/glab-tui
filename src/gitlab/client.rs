use anyhow::{Context, Result};
use reqwest::Client;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitlabClient {
    pub client: Client,
    pub host: String,
    pub token: String,
}

impl GitlabClient {
    pub async fn new() -> Result<Self> {
        let host = get_gitlab_host()?;
        let token = get_gitlab_token(&host).await?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "PRIVATE-TOKEN",
            reqwest::header::HeaderValue::from_str(&token)?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            host,
            token,
        })
    }
}

pub async fn get_project_context() -> Result<String> {
    // Execute `git remote get-url origin`
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        return Ok("unknown/unknown".to_string());
    }

    let url = String::from_utf8(output.stdout)?.trim().to_string();
    
    // Parse url to extract namespace/repo
    // git@gitlab.com:namespace/repo.git or https://gitlab.com/namespace/repo.git
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

fn get_gitlab_host() -> Result<String> {
    // Try to get from git remote first
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .context("Failed to execute git command")?;

    if output.status.success() {
        let url = String::from_utf8(output.stdout)?.trim().to_string();
        if url.starts_with("git@") {
            if let Some(host_part) = url.split(':').next() {
                if let Some(host) = host_part.split('@').nth(1) {
                    return Ok(host.to_string());
                }
            }
        } else if url.starts_with("http") {
            let parts: Vec<&str> = url.split('/').collect();
            if parts.len() > 2 {
                return Ok(parts[2].to_string());
            }
        }
    }
    
    // Default to gitlab.com
    Ok("gitlab.com".to_string())
}

async fn get_gitlab_token(host: &str) -> Result<String> {
    let output = Command::new("glab")
        .args(["config", "get", "token", "-h", host])
        .output()
        .context("Failed to execute glab command")?;

    if output.status.success() {
        let token = String::from_utf8(output.stdout)?.trim().to_string();
        if !token.is_empty() {
            return Ok(token);
        }
    }
    
    anyhow::bail!("Could not find GitLab token for host {}", host)
}
