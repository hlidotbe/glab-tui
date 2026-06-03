use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProjectCache {
    pub issues: Vec<crate::gitlab::issues::Issue>,
    pub mrs: Vec<crate::gitlab::mr::MergeRequest>,
    pub pipelines: Vec<crate::gitlab::pipelines::Pipeline>,
}

fn get_cache_file_path(project_context: &str) -> PathBuf {
    let safe_name = project_context.replace('/', "_").replace('\\', "_");
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    
    let mut path = PathBuf::from(home);
    path.push(".glab-tui-cache");
    let _ = fs::create_dir_all(&path);
    path.push(format!("{}.json", safe_name));
    path
}

pub fn load_cache(project_context: &str) -> ProjectCache {
    let path = get_cache_file_path(project_context);
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(cache) = serde_json::from_str(&content) {
            return cache;
        }
    }
    ProjectCache::default()
}

pub fn save_cache(project_context: &str, cache: &ProjectCache) {
    let path = get_cache_file_path(project_context);
    if let Ok(content) = serde_json::to_string(cache) {
        let _ = fs::write(path, content);
    }
}

fn get_recent_repos_file_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    
    let mut path = PathBuf::from(home);
    path.push(".glab-tui-cache");
    let _ = fs::create_dir_all(&path);
    path.push("recent_repos.json");
    path
}

pub fn get_recent_repos() -> Vec<String> {
    let path = get_recent_repos_file_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(repos) = serde_json::from_str::<Vec<String>>(&content) {
            return repos;
        }
    }
    Vec::new()
}

pub fn add_recent_repo(repo_path: &str) {
    let mut repos = get_recent_repos();
    let repo_path = repo_path.to_string();
    if let Some(pos) = repos.iter().position(|r| r == &repo_path) {
        repos.remove(pos);
    }
    repos.insert(0, repo_path);
    repos.truncate(20);
    
    let path = get_recent_repos_file_path();
    if let Ok(content) = serde_json::to_string(&repos) {
        let _ = fs::write(path, content);
    }
}

pub fn is_git_repo(path: &str) -> bool {
    let mut p = PathBuf::from(path);
    p.push(".git");
    p.exists()
}

pub fn get_sibling_repos(current_dir: &str) -> Vec<String> {
    let mut sibling_repos = Vec::new();
    if let Ok(path) = PathBuf::from(current_dir).canonicalize() {
        if let Some(parent) = path.parent() {
            if let Ok(entries) = fs::read_dir(parent) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        let mut git_path = entry_path.clone();
                        git_path.push(".git");
                        if git_path.exists() {
                            if let Some(p_str) = entry_path.to_str() {
                                sibling_repos.push(p_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    sibling_repos
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_is_git_repo() {
        let dir = tempdir().unwrap();
        let path_str = dir.path().to_str().unwrap();
        assert!(!is_git_repo(path_str));

        let git_dir = dir.path().join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        assert!(is_git_repo(path_str));
    }

    #[test]
    fn test_get_sibling_repos() {
        let parent = tempdir().unwrap();
        let repo1 = parent.path().join("repo1");
        let repo2 = parent.path().join("repo2");
        let non_repo = parent.path().join("non_repo");

        fs::create_dir_all(&repo1.join(".git")).unwrap();
        fs::create_dir_all(&repo2.join(".git")).unwrap();
        fs::create_dir_all(&non_repo).unwrap();

        let repo1_str = repo1.to_str().unwrap();
        let siblings = get_sibling_repos(repo1_str);
        
        let has_repo2 = siblings.iter().any(|s| s.contains("repo2"));
        let has_non_repo = siblings.iter().any(|s| s.contains("non_repo"));

        assert!(has_repo2, "siblings should find repo2");
        assert!(!has_non_repo, "siblings should not find non_repo");
    }
}


