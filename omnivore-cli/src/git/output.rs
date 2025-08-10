use anyhow::{Context, Result};
use encoding_rs::UTF_8;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::filter::FilteredFile;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Text,
    Directory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
}

pub struct OutputWriter {
    format: OutputFormat,
    root_path: PathBuf,
    output_path: Option<PathBuf>,
    force_stdout: bool,
}

impl OutputWriter {
    pub fn new(format: OutputFormat, root_path: PathBuf) -> Self {
        Self {
            format,
            root_path,
            output_path: None,
            force_stdout: false,
        }
    }

    pub fn set_output_path(&mut self, path: PathBuf) {
        self.output_path = Some(path);
    }
    
    pub fn set_stdout_mode(&mut self) {
        self.force_stdout = true;
        self.output_path = None;
    }

    pub async fn write_files(&self, files: Vec<FilteredFile>) -> Result<usize> {
        match self.format {
            OutputFormat::Json => self.write_json(files).await,
            OutputFormat::Text => self.write_text(files).await,
            OutputFormat::Directory => self.write_directory(files).await,
        }
    }

    async fn write_json(&self, files: Vec<FilteredFile>) -> Result<usize> {
        let mut file_contents = Vec::new();
        let mut count = 0;

        for file in files {
            if let Ok(content) = read_file_content(&file.path) {
                file_contents.push(FileContent {
                    path: file.relative_path.display().to_string(),
                    content,
                });
                count += 1;
            }
        }

        let json = serde_json::to_string_pretty(&file_contents)
            .context("Failed to serialize to JSON")?;

        if self.force_stdout || self.output_path.is_none() {
            print!("{}", json);
            io::stdout().flush()?;
        } else if let Some(ref output_path) = self.output_path {
            tokio::fs::write(output_path, json)
                .await
                .context("Failed to write JSON to file")?;
        }

        Ok(count)
    }

    async fn write_text(&self, files: Vec<FilteredFile>) -> Result<usize> {
        let mut output = String::new();
        let mut count = 0;

        for file in files {
            if let Ok(content) = read_file_content(&file.path) {
                output.push_str(&format!(
                    "---\nFile: {}\n---\n{}\n",
                    file.relative_path.display(),
                    content
                ));
                count += 1;
            }
        }

        if self.force_stdout || self.output_path.is_none() {
            print!("{}", output);
            io::stdout().flush()?;
        } else if let Some(ref output_path) = self.output_path {
            tokio::fs::write(output_path, output)
                .await
                .context("Failed to write text to file")?;
        }

        Ok(count)
    }

    async fn write_directory(&self, files: Vec<FilteredFile>) -> Result<usize> {
        let output_dir = self
            .output_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Output path required for directory format"))?;

        if output_dir.exists() {
            if !output_dir.is_dir() {
                return Err(anyhow::anyhow!(
                    "Output path exists but is not a directory"
                ));
            }
        } else {
            tokio::fs::create_dir_all(&output_dir)
                .await
                .context("Failed to create output directory")?;
        }

        let mut count = 0;
        for file in files {
            let dest_path = output_dir.join(&file.relative_path);
            
            if let Some(parent) = dest_path.parent() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .context("Failed to create parent directories")?;
            }

            match tokio::fs::copy(&file.path, &dest_path).await {
                Ok(_) => count += 1,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to copy {}: {}",
                        file.relative_path.display(),
                        e
                    );
                }
            }
        }

        Ok(count)
    }
}

fn read_file_content(path: &Path) -> Result<String> {
    let bytes = fs::read(path).context("Failed to read file")?;
    
    let (cow, _, had_errors) = UTF_8.decode(&bytes);
    
    if had_errors {
        return Err(anyhow::anyhow!(
            "File contains invalid UTF-8: {}",
            path.display()
        ));
    }
    
    Ok(cow.into_owned())
}

pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}