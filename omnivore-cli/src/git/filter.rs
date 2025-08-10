use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct FilteredFile {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub size: u64,
}

pub struct FileFilter {
    root_path: PathBuf,
    include_patterns: Option<GlobSet>,
    exclude_patterns: Option<GlobSet>,
    use_gitignore: bool,
    exclude_binary: bool,
    max_file_size: Option<u64>,
    default_excludes: GlobSet,
}

impl FileFilter {
    pub fn new(root_path: PathBuf) -> Self {
        let default_excludes = build_default_excludes();
        
        Self {
            root_path,
            include_patterns: None,
            exclude_patterns: None,
            use_gitignore: true,
            exclude_binary: false,
            max_file_size: None,
            default_excludes,
        }
    }

    pub fn ignore_gitignore(&mut self) {
        self.use_gitignore = false;
    }

    pub fn set_include_patterns(&mut self, patterns: Vec<String>) -> Result<()> {
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            let glob = Glob::new(&pattern)
                .with_context(|| format!("Invalid include pattern: {}", pattern))?;
            builder.add(glob);
        }
        self.include_patterns = Some(builder.build()?);
        Ok(())
    }

    pub fn set_exclude_patterns(&mut self, patterns: Vec<String>) -> Result<()> {
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            let glob = Glob::new(&pattern)
                .with_context(|| format!("Invalid exclude pattern: {}", pattern))?;
            builder.add(glob);
        }
        self.exclude_patterns = Some(builder.build()?);
        Ok(())
    }

    pub fn exclude_binary_files(&mut self) {
        self.exclude_binary = true;
    }

    pub fn set_max_file_size(&mut self, max_size: u64) {
        self.max_file_size = Some(max_size);
    }

    pub fn filter_files(&self) -> Result<Vec<FilteredFile>> {
        let mut filtered_files = Vec::new();
        let gitignore = if self.use_gitignore {
            Some(self.build_gitignore()?)
        } else {
            None
        };

        let walker = WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| self.should_traverse_dir(e, &gitignore));

        for entry in walker {
            let entry = entry?;
            
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            let relative_path = path
                .strip_prefix(&self.root_path)
                .unwrap_or(path)
                .to_path_buf();

            if !self.should_include_file(&relative_path, &entry, &gitignore)? {
                continue;
            }

            let metadata = entry.metadata()?;
            filtered_files.push(FilteredFile {
                path: path.to_path_buf(),
                relative_path,
                size: metadata.len(),
            });
        }

        Ok(filtered_files)
    }

    fn build_gitignore(&self) -> Result<Gitignore> {
        let mut builder = GitignoreBuilder::new(&self.root_path);
        
        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_name() == ".gitignore" {
                let gitignore_path = entry.path();
                let _parent = gitignore_path.parent().unwrap_or(&self.root_path);
                builder.add(gitignore_path);
            }
        }
        
        Ok(builder.build()?)
    }

    fn should_traverse_dir(&self, entry: &DirEntry, gitignore: &Option<Gitignore>) -> bool {
        let path = entry.path();
        let relative_path = path
            .strip_prefix(&self.root_path)
            .unwrap_or(path);

        if self.default_excludes.is_match(relative_path) {
            return false;
        }

        if let Some(ref gi) = gitignore {
            if gi.matched(relative_path, entry.file_type().is_dir()).is_ignore() {
                return false;
            }
        }

        true
    }

    fn should_include_file(
        &self,
        relative_path: &Path,
        entry: &DirEntry,
        gitignore: &Option<Gitignore>,
    ) -> Result<bool> {
        if self.default_excludes.is_match(relative_path) {
            return Ok(false);
        }

        if let Some(ref gi) = gitignore {
            if gi.matched(relative_path, false).is_ignore() {
                return Ok(false);
            }
        }

        if let Some(ref exclude) = self.exclude_patterns {
            if exclude.is_match(relative_path) {
                return Ok(false);
            }
        }

        if let Some(ref include) = self.include_patterns {
            if !include.is_match(relative_path) {
                return Ok(false);
            }
        }

        let metadata = entry.metadata()?;
        if let Some(max_size) = self.max_file_size {
            if metadata.len() > max_size {
                return Ok(false);
            }
        }

        if self.exclude_binary && is_likely_binary(entry.path())? {
            return Ok(false);
        }

        Ok(true)
    }
}

fn build_default_excludes() -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    
    let patterns = vec![
        ".git/**",
        ".svn/**",
        ".hg/**",
        ".bzr/**",
        "**/.git/**",
        "**/.svn/**",
        "**/.hg/**",
        "**/.bzr/**",
        "**/node_modules/**",
        "**/target/**",
        "**/dist/**",
        "**/build/**",
        "**/.DS_Store",
        "**/Thumbs.db",
        "**/*.pyc",
        "**/__pycache__/**",
        "**/.pytest_cache/**",
        "**/.mypy_cache/**",
        "**/.tox/**",
        "**/.coverage",
        "**/.idea/**",
        "**/.vscode/**",
        "**/*.swp",
        "**/*.swo",
        "**/*~",
    ];
    
    for pattern in patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }
    
    builder.build().expect("Failed to build default excludes")
}

fn is_likely_binary(path: &Path) -> Result<bool> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let binary_extensions = HashSet::from([
        "exe", "dll", "so", "dylib", "a", "lib", "o", "obj",
        "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp",
        "mp3", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4a", "wav",
        "zip", "tar", "gz", "bz2", "xz", "7z", "rar",
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "ttf", "otf", "woff", "woff2", "eot",
        "db", "sqlite", "sqlite3",
        "jar", "war", "ear",
        "pyc", "pyo", "class",
        "min.js", "min.css",
    ]);

    if binary_extensions.contains(extension) {
        return Ok(true);
    }

    if let Ok(contents) = fs::read(path) {
        if contents.len() > 8192 {
            let sample = &contents[..8192];
            let null_count = sample.iter().filter(|&&b| b == 0).count();
            return Ok(null_count > 0);
        }
        
        let null_count = contents.iter().filter(|&&b| b == 0).count();
        Ok(null_count > 0)
    } else {
        Ok(false)
    }
}