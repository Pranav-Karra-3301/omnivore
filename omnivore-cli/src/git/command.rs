use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use super::{
    filter::FileFilter,
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
        help = "Include only files matching these patterns (comma-separated)"
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
        help = "Output filtered files to directory",
        conflicts_with_all = &["json", "txt"]
    )]
    pub output: Option<PathBuf>,

    #[arg(long, help = "Output as JSON", conflicts_with_all = &["output", "txt"])]
    pub json: bool,

    #[arg(long, help = "Output as plain text", conflicts_with_all = &["output", "json"])]
    pub txt: bool,

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
    println!("{}", "🔍 Omnivore Git Code Extractor".bold().cyan());
    println!();

    let progress = create_progress_bar("Initializing...");

    let source_type = SourceType::from_string(&args.source)?;
    if args.verbose {
        println!("Source type: {:?}", source_type);
    }

    progress.set_message("Acquiring source...");
    let mut acquisition = SourceAcquisition::new(source_type, args.depth, args.keep);
    let repo_path = acquisition
        .acquire()
        .await
        .context("Failed to acquire repository source")?;

    progress.set_message("Setting up filters...");
    let mut filter = FileFilter::new(repo_path.clone());
    
    if args.no_gitignore {
        filter.ignore_gitignore();
    }
    
    if let Some(include_patterns) = &args.include {
        filter.set_include_patterns(include_patterns.clone())?;
    }
    
    if let Some(exclude_patterns) = &args.exclude {
        filter.set_exclude_patterns(exclude_patterns.clone())?;
    }
    
    if !args.allow_binary {
        filter.exclude_binary_files();
    }
    
    if let Some(max_size) = args.max_file_size {
        filter.set_max_file_size(max_size);
    }

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
    let mut writer = OutputWriter::new(output_format, repo_path.clone());
    
    if let Some(output_path) = &args.output {
        writer.set_output_path(output_path.clone());
    }

    let files_written = writer
        .write_files(filtered_files)
        .await
        .context("Failed to write output")?;

    progress.finish_and_clear();
    
    println!(
        "{}",
        format!("✅ Successfully processed {} files", files_written)
            .bold()
            .green()
    );

    if args.output.is_some() {
        println!(
            "Output written to: {}",
            args.output.unwrap().display().to_string().cyan()
        );
    }

    acquisition.cleanup().await?;

    Ok(())
}

fn determine_output_format(args: &GitArgs) -> OutputFormat {
    if args.json {
        OutputFormat::Json
    } else if args.txt {
        OutputFormat::Text
    } else {
        OutputFormat::Directory
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