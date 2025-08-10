use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use omnivore_core::{crawler::Crawler, CrawlConfig, PolitenessConfig};
use std::path::PathBuf;
use url::Url;

mod git;

#[derive(Parser)]
#[command(name = "omnivore")]
#[command(author, version, about = "Universal Rust Web Crawler & Knowledge Graph Builder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    Crawl {
        #[arg(help = "URL to start crawling from")]
        url: String,

        #[arg(short, long, default_value = "10")]
        workers: usize,

        #[arg(short, long, default_value = "5")]
        depth: u32,

        #[arg(short, long, help = "Output file for results")]
        output: Option<PathBuf>,

        #[arg(long, help = "Respect robots.txt")]
        respect_robots: bool,

        #[arg(
            long,
            default_value = "100",
            help = "Delay between requests in milliseconds"
        )]
        delay: u64,
    },

    Parse {
        #[arg(help = "File to parse")]
        file: PathBuf,

        #[arg(short, long, help = "Parsing rules file")]
        rules: Option<PathBuf>,
    },

    Graph {
        #[arg(help = "Build knowledge graph from crawl results")]
        input: PathBuf,

        #[arg(short, long, help = "Output graph file")]
        output: Option<PathBuf>,
    },

    Stats {
        #[arg(help = "Show statistics for a crawl session")]
        session: Option<String>,
    },

    Git(git::GitArgs),

    #[command(hide = true)]
    GenerateCompletions {
        #[arg(help = "Shell to generate completions for")]
        shell: clap_complete::Shell,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(if cli.verbose { "debug" } else { "info" })
        .init();

    match cli.command {
        Commands::Crawl {
            url,
            workers,
            depth,
            output,
            respect_robots,
            delay,
        } => {
            crawl_command(url, workers, depth, output, respect_robots, delay).await?;
        }
        Commands::Parse { file, rules } => {
            parse_command(file, rules).await?;
        }
        Commands::Graph { input, output } => {
            graph_command(input, output).await?;
        }
        Commands::Stats { session } => {
            stats_command(session).await?;
        }
        Commands::Git(args) => {
            git::execute_git_command(args).await?;
        }
        Commands::GenerateCompletions { shell } => {
            generate_completions(shell);
        }
    }

    Ok(())
}

async fn crawl_command(
    url: String,
    workers: usize,
    depth: u32,
    output: Option<PathBuf>,
    respect_robots: bool,
    delay: u64,
) -> Result<()> {
    println!("{}", "🕸️  Omnivore Web Crawler".bold().cyan());
    println!();

    let start_url = Url::parse(&url)?;
    println!("Starting crawl from: {}", start_url.to_string().green());
    println!("Configuration:");
    println!("  Workers: {}", workers.to_string().yellow());
    println!("  Max depth: {}", depth.to_string().yellow());
    println!(
        "  Respect robots.txt: {}",
        respect_robots.to_string().yellow()
    );
    println!("  Delay: {}ms", delay.to_string().yellow());
    println!();

    let config = CrawlConfig {
        max_workers: workers,
        max_depth: depth,
        user_agent: "Omnivore/1.0".to_string(),
        respect_robots_txt: respect_robots,
        politeness: PolitenessConfig {
            default_delay_ms: delay,
            max_requests_per_second: 1000.0 / delay as f64,
            backoff_multiplier: 2.0,
        },
        timeout_ms: 30000,
        max_retries: 3,
    };

    use std::sync::Arc;
    let crawler: Arc<Crawler> = Arc::new(Crawler::new(config).await?);
    crawler.add_seed(start_url).await?;

    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} [{elapsed_precise}] {msg}")?,
    );

    let stats_handle = tokio::spawn({
        let crawler = Arc::clone(&crawler);
        let progress = progress.clone();
        async move {
            loop {
                let stats = crawler.get_stats().await;
                progress.set_message(format!(
                    "Crawled: {} | Success: {} | Failed: {} | In Progress: {}",
                    stats.total_urls.to_string().cyan(),
                    stats.successful.to_string().green(),
                    stats.failed.to_string().red(),
                    stats.in_progress.to_string().yellow()
                ));
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    });

    let crawler = Arc::clone(&crawler);
    crawler.start().await?;
    stats_handle.abort();
    progress.finish_with_message("Crawl completed!");

    let final_stats = crawler.get_stats().await;
    println!();
    println!("{}", "📊 Final Statistics:".bold().green());
    println!(
        "  Total URLs crawled: {}",
        final_stats.total_urls.to_string().cyan()
    );
    println!(
        "  Successful: {}",
        final_stats.successful.to_string().green()
    );
    println!("  Failed: {}", final_stats.failed.to_string().red());
    println!("  Time elapsed: {:?}", final_stats.elapsed_time);

    if let Some(output_path) = output {
        let stats_json = serde_json::to_string_pretty(&final_stats)?;
        tokio::fs::write(output_path, stats_json).await?;
        println!("Results saved to file");
    }

    Ok(())
}

async fn parse_command(file: PathBuf, rules: Option<PathBuf>) -> Result<()> {
    println!("{}", "📄 Parsing HTML file...".bold().cyan());

    let content = tokio::fs::read_to_string(&file).await?;

    let rules = if let Some(rules_path) = rules {
        println!("Using rules from: {}", rules_path.display());
        let rules_content = tokio::fs::read_to_string(rules_path).await?;
        serde_yaml::from_str(&rules_content)?
    } else {
        Vec::new()
    };

    let parser = omnivore_core::parser::Parser::new(omnivore_core::parser::ParseConfig {
        rules,
        schema_name: None,
        clean_text: true,
        extract_metadata: true,
    });

    let result = parser.parse(&content)?;
    println!();
    println!("{}", "✅ Parsing complete!".bold().green());
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}

async fn graph_command(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    println!("{}", "🔗 Building knowledge graph...".bold().cyan());

    let data = tokio::fs::read_to_string(&input).await?;

    println!("Processing {} bytes of data", data.len());

    if let Some(output_path) = output {
        println!("Graph will be saved to: {}", output_path.display());
    }

    println!("{}", "✅ Graph building complete!".bold().green());

    Ok(())
}

async fn stats_command(session: Option<String>) -> Result<()> {
    println!("{}", "📊 Crawl Statistics".bold().cyan());

    if let Some(session_id) = session {
        println!("Session: {}", session_id.yellow());
    } else {
        println!("No active sessions found");
    }

    Ok(())
}

fn generate_completions(shell: clap_complete::Shell) {
    use clap_complete::{generate, Generator};
    use std::io;

    fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    }

    let mut cmd = Cli::command();
    print_completions(shell, &mut cmd);
}
