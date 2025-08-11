use anyhow::{anyhow, Context, Result};
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use url::Url;
use colored::*;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum SourceType {
    Remote(String),
    Local(PathBuf),
    LocalNonGit(PathBuf),  // New variant for non-git directories
}

impl SourceType {
    pub fn from_string(source: &str) -> Result<Self> {
        if source.starts_with("http://")
            || source.starts_with("https://")
            || source.starts_with("git@")
            || source.starts_with("ssh://")
        {
            Ok(SourceType::Remote(source.to_string()))
        } else {
            let path = PathBuf::from(source);
            // Try to resolve the path - could be relative or absolute
            let resolved_path = if path.is_absolute() {
                path
            } else {
                std::env::current_dir()?.join(path)
            };
            
            if resolved_path.is_dir() {
                let git_dir = resolved_path.join(".git");
                if !git_dir.exists() {
                    // Ask for confirmation to proceed with non-git directory
                    if Self::confirm_non_git_directory(&resolved_path)? {
                        Ok(SourceType::LocalNonGit(resolved_path.canonicalize()?))
                    } else {
                        Err(anyhow!("Operation cancelled by user"))
                    }
                } else {
                    Ok(SourceType::Local(resolved_path.canonicalize()?))
                }
            } else {
                Err(anyhow!("'{}' is not a valid directory", source))
            }
        }
    }

    fn confirm_non_git_directory(path: &Path) -> Result<bool> {
        println!();
        println!(
            "{}",
            format!(
                "⚠️  '{}' is not a Git repository (no .git directory found)",
                path.display()
            )
            .yellow()
            .bold()
        );
        print!(
            "{}",
            "Do you want to continue and analyze this directory anyway? [y/N]: "
                .bright_white()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        Ok(input == "y" || input == "yes")
    }
}

pub struct SourceAcquisition {
    source_type: SourceType,
    depth: u32,
    keep_temp: bool,
    temp_dir: Option<TempDir>,
}

impl SourceAcquisition {
    pub fn new(source_type: SourceType, depth: u32, keep_temp: bool) -> Self {
        Self {
            source_type,
            depth,
            keep_temp,
            temp_dir: None,
        }
    }

    pub async fn acquire(&mut self) -> Result<PathBuf> {
        match self.source_type.clone() {
            SourceType::Remote(url) => self.clone_remote(&url).await,
            SourceType::Local(path) => Ok(path),
            SourceType::LocalNonGit(path) => Ok(path),
        }
    }

    async fn clone_remote(&mut self, url: &str) -> Result<PathBuf> {
        let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
        let repo_path = temp_dir.path().to_path_buf();

        let url_str = url.to_string();
        let repo_path_clone = repo_path.clone();
        let depth = self.depth;
        
        let clone_result = tokio::task::spawn_blocking(move || {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                if let Ok(home) = std::env::var("HOME") {
                    let ssh_key = PathBuf::from(&home).join(".ssh/id_rsa");
                    if ssh_key.exists() {
                        return Cred::ssh_key(
                            username_from_url.unwrap_or("git"),
                            None,
                            &ssh_key,
                            None,
                        );
                    }
                    
                    let ssh_key = PathBuf::from(&home).join(".ssh/id_ed25519");
                    if ssh_key.exists() {
                        return Cred::ssh_key(
                            username_from_url.unwrap_or("git"),
                            None,
                            &ssh_key,
                            None,
                        );
                    }
                }
                
                Cred::default()
            });

            callbacks.certificate_check(|_cert, _host| {
                Ok(git2::CertificateCheckStatus::CertificateOk)
            });

            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(callbacks);
            fetch_options.depth(depth as i32);

            let mut builder = RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder.clone(&url_str, &repo_path_clone)
        })
        .await
        .context("Failed to spawn blocking task")?;

        match clone_result {
            Ok(_) => {
                self.temp_dir = Some(temp_dir);
                Ok(self.temp_dir.as_ref().unwrap().path().to_path_buf())
            }
            Err(e) => {
                if e.message().contains("authentication") || e.message().contains("401") {
                    Err(anyhow!(
                        "Authentication failed. Please ensure your Git credentials (SSH key, etc.) are configured correctly.\nError: {}",
                        e
                    ))
                } else if e.message().contains("not found") || e.message().contains("404") {
                    Err(anyhow!("Repository not found: {}", url))
                } else {
                    Err(anyhow!("Failed to clone repository: {}", e))
                }
            }
        }
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        if self.keep_temp {
            if let Some(temp_dir) = &self.temp_dir {
                println!(
                    "Temporary clone kept at: {}",
                    temp_dir.path().display()
                );
                std::mem::forget(temp_dir.path().to_path_buf());
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn is_git_repository(path: &Path) -> bool {
    path.join(".git").exists()
}

#[allow(dead_code)]
pub fn validate_url(url_str: &str) -> Result<Url> {
    if url_str.starts_with("git@") {
        let ssh_url = url_str.replace(':', "/").replace("git@", "ssh://git@");
        Url::parse(&ssh_url).context("Invalid SSH URL format")
    } else {
        Url::parse(url_str).context("Invalid URL format")
    }
}