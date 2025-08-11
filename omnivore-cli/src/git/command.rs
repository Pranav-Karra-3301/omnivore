use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use super::{
    detector::{CodebaseDetector, get_default_include_patterns, get_smart_exclude_patterns},
    filter::FileFilter,
    organizer::CodeOrganizer,
    output::{OutputFormat, OutputWriter},
    source::{SourceAcquisition, SourceType},
};

#[derive(Args, Debug)]
pub struct GitArgs {
    #[arg(help = "Repository source (URL or local path)")]
    pub source: String,

    #[arg(
        long,
        value_delimiter = ',',
        help = "Include only files matching these patterns (comma-separated, alias for --include)"
    )]
    pub only: Option<Vec<String>>,

    #[arg(
        long,
        value_delimiter = ',',
        help = "Include files matching these patterns (comma-separated)"
    )]
    pub include: Option<Vec<String>>,

    #[arg(
        long,
        value_delimiter = ',',
        help = "Exclude files matching these patterns (comma-separated)"
    )]
    pub exclude: Option<Vec<String>>,

    #[arg(long, help = "Ignore .gitignore files")]
    pub no_gitignore: bool,

    #[arg(
        long,
        value_name = "PATH",
        help = "Output filtered files to directory"
    )]
    pub output: Option<PathBuf>,

    #[arg(long, help = "Output as JSON")]
    pub json: bool,

    #[arg(long, help = "Output to stdout instead of file")]
    pub stdout: bool,

    #[arg(long, help = "Keep temporary clone after completion (for remote repos)")]
    pub keep: bool,

    #[arg(long, default_value = "1", help = "Clone depth for remote repositories")]
    pub depth: u32,

    #[arg(long, help = "Include binary files in output")]
    pub allow_binary: bool,

    #[arg(
        long,
        value_name = "SIZE",
        help = "Maximum file size in bytes (e.g., 10485760 for 10MB)"
    )]
    pub max_file_size: Option<u64>,

    #[arg(long, help = "Verbose output")]
    pub verbose: bool,
}

pub async fn execute_git_command(args: GitArgs) -> Result<()> {
    println!("{}", "🔍 Omnivore Code Analyzer".bold().cyan());
    println!();

    // Detect source type first, before creating progress bar
    // This allows the confirmation prompt to display properly for non-git directories
    let source_type = SourceType::from_string(&args.source)?;
    if args.verbose {
        println!("Source type: {:?}", source_type);
    }

    // Now create the progress bar after any user interaction
    let progress = create_progress_bar("Initializing...");

    // Show appropriate message based on source type
    match &source_type {
        SourceType::Remote(_) => {
            println!("Analyzing remote Git repository...");
        }
        SourceType::Local(_) => {
            println!("Analyzing local Git repository...");
        }
        SourceType::LocalNonGit(_) => {
            println!("{}", "Analyzing local directory (non-Git)...".yellow());
        }
    }

    progress.set_message("Acquiring source...");
    let mut acquisition = SourceAcquisition::new(source_type.clone(), args.depth, args.keep);
    let repo_path = acquisition
        .acquire()
        .await
        .context("Failed to acquire source")?;

    progress.set_message("Detecting codebase type...");
    let detector = CodebaseDetector::new(repo_path.clone());
    let codebase_info = detector.detect()?;
    
    if args.verbose {
        println!("Detected: {}", codebase_info.description);
    }

    progress.set_message("Setting up filters...");
    let mut filter = FileFilter::new(repo_path.clone());
    
    if args.no_gitignore {
        filter.ignore_gitignore();
    }
    
    let include_patterns = if let Some(only_patterns) = &args.only {
        only_patterns.iter().map(|p| normalize_pattern(p)).collect()
    } else if let Some(include_patterns) = &args.include {
        include_patterns.iter().map(|p| normalize_pattern(p)).collect()
    } else if should_use_smart_defaults(&args) {
        get_default_include_patterns(&codebase_info)
    } else {
        Vec::new()
    };
    
    if !include_patterns.is_empty() {
        filter.set_include_patterns(include_patterns)?;
    }
    
    let exclude_patterns = if let Some(exclude) = &args.exclude {
        exclude.iter().map(|p| normalize_pattern(p)).collect()
    } else if should_use_smart_defaults(&args) {
        get_smart_exclude_patterns(&codebase_info)
    } else {
        Vec::new()
    };
    
    if !exclude_patterns.is_empty() {
        filter.set_exclude_patterns(exclude_patterns)?;
    }
    
    if !args.allow_binary {
        filter.exclude_binary_files();
    }
    
    // Set a default max file size of 10MB if not specified
    let max_size = args.max_file_size.unwrap_or(10 * 1024 * 1024); // 10MB default
    filter.set_max_file_size(max_size);

    progress.set_message("Filtering files...");
    let filtered_files = filter
        .filter_files()
        .context("Failed to filter files")?;

    if filtered_files.is_empty() {
        progress.finish_with_message("No files matched the filter criteria");
        println!("{}", "⚠️  No files found matching the criteria".yellow());
        return Ok(());
    }

    progress.set_message(format!("Processing {} files...", filtered_files.len()));

    let output_format = determine_output_format(&args);
    
    let output_path = if !args.stdout && args.output.is_none() {
        let repo_name = extract_repo_name(&args.source);
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let extension = if args.json { "json" } else { "txt" };
        Some(PathBuf::from(format!("{}_{}.{}", repo_name, timestamp, extension)))
    } else {
        args.output.clone()
    };
    
    let files_written = if should_use_organized_output(&args, &output_path) {
        let organizer = CodeOrganizer::new(codebase_info, filtered_files);
        let organized = organizer.organize();
        
        let output_content = if args.json {
            organized.to_json()?
        } else {
            organized.to_formatted_text(true, &repo_path)?
        };
        
        if args.stdout {
            print!("{}", output_content);
            std::io::Write::flush(&mut std::io::stdout())?;
        } else if let Some(ref path) = output_path {
            tokio::fs::write(path, output_content).await?;
        }
        
        organized.metadata.total_files
    } else {
        let mut writer = OutputWriter::new(output_format, repo_path.clone());
        
        if let Some(ref path) = output_path {
            writer.set_output_path(path.clone());
        }
        
        if args.stdout {
            writer.set_stdout_mode();
        }
        
        writer
            .write_files(filtered_files)
            .await
            .context("Failed to write output")?
    };

    progress.finish_and_clear();
    
    println!(
        "{}",
        format!("✅ Successfully processed {} files", files_written)
            .bold()
            .green()
    );

    if let Some(path) = output_path {
        if !args.stdout {
            println!(
                "Output written to: {}",
                path.display().to_string().cyan()
            );
        }
    }

    acquisition.cleanup().await?;

    Ok(())
}

fn determine_output_format(args: &GitArgs) -> OutputFormat {
    if args.json {
        OutputFormat::Json
    } else if let Some(ref output) = args.output {
        if output.extension().and_then(|e| e.to_str()) == Some("txt") || args.stdout {
            OutputFormat::Text
        } else {
            OutputFormat::Directory
        }
    } else {
        OutputFormat::Text
    }
}

fn should_use_smart_defaults(args: &GitArgs) -> bool {
    args.only.is_none() && args.include.is_none()
}

fn should_use_organized_output(args: &GitArgs, output_path: &Option<PathBuf>) -> bool {
    if let Some(ref path) = output_path {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return ext == "txt" || args.json;
        }
    }
    args.stdout || args.json
}

fn normalize_pattern(pattern: &str) -> String {
    // If pattern looks like a file extension without wildcards, convert it to a glob pattern
    if !pattern.contains('*') && !pattern.contains('/') && !pattern.contains('?') && !pattern.contains('[') {
        // Handle patterns like "md", ".md", "rs", ".rs" etc.
        let cleaned = pattern.trim_start_matches('.');
        if !cleaned.is_empty() && cleaned.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return format!("**/*.{}", cleaned);
        }
    }
    pattern.to_string()
}

fn extract_repo_name(source: &str) -> String {
    // Extract repository name from URL or path
    let cleaned = source
        .trim_end_matches('/')
        .trim_end_matches(".git");
    
    if let Some(pos) = cleaned.rfind('/') {
        cleaned[pos + 1..].to_string()
    } else {
        cleaned.to_string()
    }
}

fn create_progress_bar(initial_message: &str) -> ProgressBar {
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    progress.set_message(initial_message.to_string());
    progress.enable_steady_tick(std::time::Duration::from_millis(100));
    progress
}