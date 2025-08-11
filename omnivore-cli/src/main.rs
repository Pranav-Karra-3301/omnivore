use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use omnivore_core::{crawler::Crawler, CrawlConfig, CrawlResult, CrawlStats, PolitenessConfig, table_extractor::TableData};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;
use std::fs::File;
use std::io::{Write, Read as IORead};

mod git;
mod setup;

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Json,
    Markdown,
    Csv,
    Yaml,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrawlOutput {
    stats: CrawlStats,
    results: Vec<CrawlResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CleanCrawlOutput {
    url: String,
    pages: usize,
    words: usize,
    duration_ms: u128,
    timestamp: String,
    content: Vec<PageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageContent {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    structured: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tables: Vec<TableData>,
    words: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    links: Vec<String>,
}

#[derive(Parser)]
#[command(name = "omnivore")]
#[command(author, version, about = "High-performance Rust Web Crawler", long_about = None)]
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
    Setup {
        #[arg(help = "Interactive setup wizard for Omnivore configuration")]
        _placeholder: Option<String>,
    },
    Config {
        #[arg(help = "Show current configuration")]
        _placeholder: Option<String>,
    },
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
        
        #[arg(long, help = "Include raw HTML content in output")]
        include_raw: bool,
        
        #[arg(long, help = "Exclude URLs/links from the content output")]
        exclude_urls: bool,
        
        #[arg(long, help = "Organize output in folder structure")]
        organize: bool,
        
        #[arg(long, value_enum, default_value = "json", help = "Output format")]
        format: OutputFormat,
        
        #[arg(long, help = "Compress output to ZIP file")]
        zip: bool,
        
        #[arg(long, help = "Extract and save tables as CSV files")]
        extract_tables: bool,
        
        #[arg(long, help = "Use browser engine for JavaScript-rendered content")]
        browser: bool,
        
        #[arg(long, help = "Interact with dropdowns and filters (requires --browser)")]
        interact: bool,
        
        #[arg(long, help = "Automatic detection and extraction of all elements")]
        auto: bool,
        
        #[arg(long, value_name = "QUERY", help = "Natural language extraction query (requires OpenAI API)")]
        ai: Option<String>,
        
        #[arg(long, help = "Use extraction template")]
        template: Option<String>,
    },

    Parse {
        #[arg(help = "File to parse")]
        file: PathBuf,

        #[arg(short, long, help = "Parsing rules file")]
        rules: Option<PathBuf>,

        #[arg(short, long, help = "Output file for parsed results")]
        output: Option<PathBuf>,
    },


    Stats {
        #[arg(help = "Show statistics for a crawl session")]
        session: Option<String>,
    },

    Git(git::GitArgs),
    
    Docs {
        #[arg(help = "Open Omnivore documentation in browser")]
        _placeholder: Option<String>,
    },

    #[command(hide = true)]
    GenerateCompletions {
        #[arg(help = "Shell to generate completions for")]
        shell: clap_complete::Shell,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check if no args or help requested
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) || args.contains(&"help".to_string()) {
        print_banner();
    }
    
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(if cli.verbose { "debug" } else { "info" })
        .init();

    match cli.command {
        Commands::Setup { .. } => {
            setup::run_setup().await?;
        }
        Commands::Config { .. } => {
            setup::show_config().await?;
        }
        Commands::Crawl {
            url,
            workers,
            depth,
            output,
            respect_robots,
            delay,
            include_raw,
            exclude_urls,
            organize,
            format,
            zip,
            extract_tables,
            browser,
            interact,
            auto,
            ai,
            template,
        } => {
            crawl_command(url, workers, depth, output, respect_robots, delay, include_raw, exclude_urls, organize, format, zip, extract_tables, browser, interact, auto, ai, template).await?;
        }
        Commands::Parse { file, rules, output } => {
            parse_command(file, rules, output).await?;
        }
        Commands::Stats { session } => {
            stats_command(session).await?;
        }
        Commands::Git(args) => {
            git::execute_git_command(args).await?;
        }
        Commands::Docs { .. } => {
            docs_command().await?;
        }
        Commands::GenerateCompletions { shell } => {
            generate_completions(shell);
        }
    }

    Ok(())
}

fn print_banner() {
    let ascii_art = r#"
 ██████╗ ███╗   ███╗███╗   ██╗██╗██╗   ██╗ ██████╗ ██████╗ ███████╗
██╔═══██╗████╗ ████║████╗  ██║██║██║   ██║██╔═══██╗██╔══██╗██╔════╝
██║   ██║██╔████╔██║██╔██╗ ██║██║██║   ██║██║   ██║██████╔╝█████╗  
██║   ██║██║╚██╔╝██║██║╚██╗██║██║╚██╗ ██╔╝██║   ██║██╔══██╗██╔══╝  
╚██████╔╝██║ ╚═╝ ██║██║ ╚████║██║ ╚████╔╝ ╚██████╔╝██║  ██║███████╗
 ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝"#;
    
    println!("{}", ascii_art.purple().bold());
    println!("{}", format!("v{}", env!("CARGO_PKG_VERSION")).purple());
    println!("{}", "The Universal Web Scraper & Code Extractor".bright_white());
    
    // Show API key status
    let (_configured, status) = setup::check_api_key_status();
    println!("{}", status);
    
    println!();
}

fn generate_default_filename(url: &Url, suffix: &str, format: &OutputFormat) -> PathBuf {
    let domain = url.domain().unwrap_or("unknown");
    let sanitized_domain = domain.replace('.', "_").replace('/', "_");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let ext = match format {
        OutputFormat::Json => "json",
        OutputFormat::Markdown => "md",
        OutputFormat::Csv => "csv",
        OutputFormat::Yaml => "yaml",
        OutputFormat::Text => "txt",
    };
    PathBuf::from(format!("{}_{}{}.{}", sanitized_domain, timestamp, suffix, ext))
}

fn format_output_content(
    clean_output: &CleanCrawlOutput, 
    format: &OutputFormat,
    exclude_urls: bool,
) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&clean_output)?),
        
        OutputFormat::Markdown => {
            let mut md = String::new();
            md.push_str(&format!("# Crawl Results: {}\n\n", clean_output.url));
            md.push_str(&format!("**Date:** {}\n", clean_output.timestamp));
            md.push_str(&format!("**Pages:** {} | **Words:** {} | **Duration:** {}ms\n\n", 
                clean_output.pages, clean_output.words, clean_output.duration_ms));
            
            for page in &clean_output.content {
                md.push_str(&format!("## {}\n", page.title.as_ref().unwrap_or(&page.url)));
                md.push_str(&format!("**URL:** {}\n", page.url));
                md.push_str(&format!("**Words:** {}\n\n", page.words));
                
                if let Some(text) = &page.text {
                    md.push_str(&text);
                    md.push_str("\n\n");
                }
                
                if !exclude_urls && !page.links.is_empty() {
                    md.push_str("### Links\n");
                    for link in &page.links {
                        md.push_str(&format!("- {}\n", link));
                    }
                    md.push_str("\n");
                }
                md.push_str("---\n\n");
            }
            Ok(md)
        },
        
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            wtr.write_record(&["url", "title", "text", "word_count", "links"])?;
            
            for page in &clean_output.content {
                let links = if exclude_urls { 
                    String::new() 
                } else { 
                    page.links.join(", ")
                };
                wtr.write_record(&[
                    &page.url,
                    page.title.as_ref().unwrap_or(&String::new()),
                    page.text.as_ref().unwrap_or(&String::new()),
                    &page.words.to_string(),
                    &links,
                ])?;
            }
            
            Ok(String::from_utf8(wtr.into_inner()?)?)
        },
        
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&clean_output)?),
        
        OutputFormat::Text => {
            let mut txt = String::new();
            txt.push_str(&format!("CRAWL RESULTS: {}\n", clean_output.url));
            txt.push_str(&format!("Date: {}\n", clean_output.timestamp));
            txt.push_str(&format!("Pages: {} | Words: {}\n\n", clean_output.pages, clean_output.words));
            
            for page in &clean_output.content {
                txt.push_str(&format!("=== {} ===\n", page.title.as_ref().unwrap_or(&page.url)));
                txt.push_str(&format!("URL: {}\n", page.url));
                txt.push_str(&format!("Words: {}\n\n", page.words));
                
                if let Some(text) = &page.text {
                    txt.push_str(&text);
                    txt.push_str("\n\n");
                }
                
                if !exclude_urls && !page.links.is_empty() {
                    txt.push_str("Links:\n");
                    for link in &page.links {
                        txt.push_str(&format!("  - {}\n", link));
                    }
                }
                txt.push_str("\n");
            }
            Ok(txt)
        },
    }
}

async fn crawl_command(
    url: String,
    workers: usize,
    depth: u32,
    output: Option<PathBuf>,
    respect_robots: bool,
    delay: u64,
    include_raw: bool,
    exclude_urls: bool,
    organize: bool,
    format: OutputFormat,
    zip: bool,
    extract_tables: bool,
    browser: bool,
    interact: bool,
    auto: bool,
    ai: Option<String>,
    template: Option<String>,
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
    
    if browser {
        println!("  Browser mode: {}", "enabled".green());
        if interact {
            println!("  Interactive mode: {}", "enabled (will interact with dropdowns/filters)".green());
        }
    }
    
    // Auto mode overrides individual settings
    let (_auto_tables, _auto_interact, _auto_browser) = if auto {
        println!("  Auto mode: {}", "enabled (automatic detection and extraction)".green());
        (true, true, true)
    } else {
        (extract_tables, interact, browser)
    };
    
    if let Some(ref ai_query) = ai {
        println!("  AI mode: {}", format!("\"{}\"", ai_query).cyan());
    }
    
    if let Some(ref template_name) = template {
        println!("  Template: {}", template_name.yellow());
    }
    
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

    // Handle browser mode separately
    if browser {
        #[cfg(all())]
        {
            use omnivore_core::crawler::browser::BrowserEngine;
            
            println!("{}", "🌐 Starting browser engine...".bold().yellow());
            println!("Note: Ensure ChromeDriver is running at localhost:9515");
            println!();
            
            let mut browser_engine = BrowserEngine::new().await?;
            browser_engine.connect().await.context("Failed to connect to browser. Make sure ChromeDriver is running (chromedriver --port=9515)")?;
            
            let crawl_results = if interact {
                println!("Crawling with interactive mode (dropdowns and filters)...");
                let dynamic_content = browser_engine.crawl_with_interactions(start_url.clone()).await?;
                
                // Convert dynamic content to regular crawl results
                vec![convert_dynamic_to_crawl_result(dynamic_content)?]
            } else {
                println!("Crawling with browser (JavaScript rendering)...");
                vec![browser_engine.crawl_dynamic(start_url.clone()).await?]
            };
            
            browser_engine.disconnect().await?;
            
            // Process results similar to regular crawl
            handle_crawl_results(crawl_results, &start_url, output, organize, format, zip, extract_tables, exclude_urls).await?;
            
            return Ok(());
        }
        
        #[cfg(not(all()))]
        {
            println!("{}", "⚠️  Browser mode requires the 'browser' feature to be enabled".yellow());
            println!("Rebuild with: cargo build --features browser");
            return Err(anyhow::anyhow!("Browser feature not enabled"));
        }
    }
    
    use std::sync::Arc;
    let crawler: Arc<Crawler> = Arc::new(Crawler::new(config).await?);
    crawler.add_seed(start_url.clone()).await?;

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
    let mut crawl_results = crawler.get_results().await;
    
    // If auto mode is enabled, perform automatic extraction
    if auto {
        println!();
        println!("{}", "🤖 Auto Mode: Performing intelligent extraction...".bold().cyan());
        
        // Load config for extraction settings
        let omnivore_config = omnivore_core::config::OmnivoreConfig::load().unwrap_or_default();
        
        for result in &mut crawl_results {
            // Use detector to find all elements
            let detector = omnivore_core::detector::UniversalDetector::new(&result.content, Some(&result.url));
            let detected = detector.detect_all();
            
            // Add detection report to extracted_data
            let mut extracted = serde_json::Map::new();
            
            if omnivore_config.extraction.auto_detect_tables && !detected.tables.is_empty() {
                extracted.insert("tables".to_string(), serde_json::to_value(&detected.tables)?);
                println!("  Found {} tables in {}", detected.tables.len().to_string().green(), result.url);
            }
            
            if omnivore_config.extraction.auto_detect_forms && !detected.forms.is_empty() {
                extracted.insert("forms".to_string(), serde_json::to_value(&detected.forms)?);
                println!("  Found {} forms in {}", detected.forms.len().to_string().green(), result.url);
            }
            
            if omnivore_config.extraction.auto_detect_dropdowns && !detected.dropdowns.is_empty() {
                extracted.insert("dropdowns".to_string(), serde_json::to_value(&detected.dropdowns)?);
                println!("  Found {} dropdowns in {}", detected.dropdowns.len().to_string().green(), result.url);
            }
            
            if omnivore_config.extraction.auto_detect_pagination && detected.pagination.is_some() {
                extracted.insert("pagination".to_string(), serde_json::to_value(&detected.pagination)?);
                println!("  Found pagination in {}", result.url);
            }
            
            if omnivore_config.extraction.auto_detect_downloads && !detected.downloads.is_empty() {
                extracted.insert("downloads".to_string(), serde_json::to_value(&detected.downloads)?);
                println!("  Found {} downloadable files in {}", detected.downloads.len().to_string().green(), result.url);
            }
            
            let contact_count = detected.contacts.emails.len() + detected.contacts.phones.len();
            if contact_count > 0 {
                extracted.insert("contacts".to_string(), serde_json::to_value(&detected.contacts)?);
                println!("  Found {} contact details in {}", contact_count.to_string().green(), result.url);
            }
            
            if !detected.interactive.is_empty() {
                extracted.insert("interactive".to_string(), serde_json::to_value(&detected.interactive)?);
            }
            
            if !detected.media.images.is_empty() || !detected.media.videos.is_empty() {
                extracted.insert("media".to_string(), serde_json::to_value(&detected.media)?);
            }
            
            if !detected.structured_data.is_empty() {
                extracted.insert("structured_data".to_string(), serde_json::to_value(&detected.structured_data)?);
            }
            
            // Update result with extracted data
            result.extracted_data = serde_json::Value::Object(extracted);
        }
        
        println!("{}", "✓ Automatic extraction complete!".green());
    }
    
    // Handle AI extraction if specified
    if let Some(ref ai_query) = ai {
        println!();
        println!("{}", format!("🤖 AI Mode: Processing query \"{}\"...", ai_query).bold().cyan());
        
        let omnivore_config = omnivore_core::config::OmnivoreConfig::load().unwrap_or_default();
        
        if omnivore_config.ai.openai_api_key.is_some() {
            let smart_extractor = omnivore_core::ai::SmartExtractor::new(&omnivore_config);
            
            for result in &mut crawl_results {
                match smart_extractor.process_natural_language(ai_query, &result.url, &result.content).await {
                    Ok(extracted) => {
                        result.extracted_data = extracted;
                        println!("  ✓ Extracted data from {}", result.url.green());
                    }
                    Err(e) => {
                        println!("  ✗ Failed to extract from {}: {}", result.url.red(), e);
                    }
                }
            }
        } else {
            println!("{}", "⚠️  OpenAI API key not configured. Run 'omnivore setup' to configure.".yellow());
        }
    }
    
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
    println!(
        "  Pages with content: {}",
        crawl_results.len().to_string().cyan()
    );

    // Handle organized output
    if organize {
        // Create organized folder structure
        let domain = start_url.domain().unwrap_or("unknown");
        let sanitized_domain = domain.replace('.', "_").replace('/', "_");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let output_dir = output.unwrap_or_else(|| {
            PathBuf::from(format!("{}_{}_crawl", sanitized_domain, timestamp))
        });
        
        // Create output directory
        tokio::fs::create_dir_all(&output_dir).await?;
        
        // Create tables subdirectory if extracting tables
        let tables_dir = if extract_tables {
            let td = output_dir.join("tables");
            tokio::fs::create_dir_all(&td).await?;
            Some(td)
        } else {
            None
        };
        
        // Save each page to a separate file
        let mut index_entries = Vec::new();
        let mut all_tables = Vec::new();
        
        for (idx, result) in crawl_results.iter().enumerate() {
            let page_file = output_dir.join(format!("page_{:04}.json", idx + 1));
            
            if let Some(ref cleaned) = result.cleaned_content {
                let page_content = PageContent {
                    url: result.url.clone(),
                    title: cleaned.title.clone(),
                    text: cleaned.content.clone(),
                    structured: cleaned.structured.as_ref().map(|s| {
                        serde_json::to_value(s).unwrap_or(serde_json::Value::Null)
                    }),
                    tables: if extract_tables { cleaned.tables.clone() } else { Vec::new() },
                    words: cleaned.word_count,
                    links: if exclude_urls { Vec::new() } else { cleaned.links.clone() },
                };
                
                let page_json = serde_json::to_string_pretty(&page_content)?;
                tokio::fs::write(&page_file, page_json).await?;
                
                // Save tables as CSV if requested
                if extract_tables && tables_dir.is_some() {
                    let tables_dir = tables_dir.as_ref().unwrap();
                    for (table_idx, table) in cleaned.tables.iter().enumerate() {
                        let table_title = table.title.as_ref()
                            .map(|t| t.replace(" ", "_").replace("/", "_"))
                            .unwrap_or_else(|| format!("table_{}", table_idx + 1));
                        
                        let csv_filename = format!("page_{:04}_{}.csv", idx + 1, table_title);
                        let csv_path = tables_dir.join(&csv_filename);
                        
                        let csv_content = table.to_csv();
                        tokio::fs::write(&csv_path, csv_content).await?;
                        
                        all_tables.push(serde_json::json!({
                            "page": idx + 1,
                            "url": result.url.clone(),
                            "table_title": table.title.clone(),
                            "csv_file": csv_filename,
                            "rows": table.rows.len(),
                            "columns": table.headers.len(),
                        }));
                    }
                }
                
                index_entries.push(serde_json::json!({
                    "url": result.url,
                    "file": page_file.file_name().unwrap().to_str().unwrap(),
                    "title": cleaned.title,
                    "words": cleaned.word_count,
                    "tables_count": cleaned.tables.len(),
                }));
            }
        }
        
        // Create index file
        let mut index_data = serde_json::json!({
            "crawler": "omnivore",
            "version": env!("CARGO_PKG_VERSION"),
            "start_url": start_url.to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "stats": final_stats,
            "pages": index_entries,
        });
        
        // Add tables section if tables were extracted
        if extract_tables && !all_tables.is_empty() {
            index_data["tables"] = serde_json::json!(all_tables);
        }
        
        let index_path = output_dir.join("index.json");
        tokio::fs::write(&index_path, serde_json::to_string_pretty(&index_data)?).await?;
        
        println!();
        // Compress to ZIP if requested
        if zip {
            let zip_path = PathBuf::from(format!("{}.zip", output_dir.display()));
            let zip_file = File::create(&zip_path)?;
            let mut zip_writer = zip::ZipWriter::new(zip_file);
            let options = zip::write::FileOptions::<()>::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);
            
            // Add all files in the output directory to the ZIP
            for entry in std::fs::read_dir(&output_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    zip_writer.start_file(file_name, options)?;
                    let mut file = File::open(&path)?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    zip_writer.write_all(&buffer)?;
                }
            }
            
            zip_writer.finish()?;
            
            // Remove the original directory
            tokio::fs::remove_dir_all(&output_dir).await?;
            
            println!(
                "{}  Compressed {} pages into: {}",
                "📦".bold().green(),
                crawl_results.len().to_string().cyan(),
                zip_path.display().to_string().yellow()
            );
        } else {
            println!(
                "{}  Organized {} pages into folder: {}",
                "✅".bold().green(),
                crawl_results.len().to_string().cyan(),
                output_dir.display().to_string().yellow()
            );
        }
        
        return Ok(());
    }
    
    // Determine output path
    let output_path = output.unwrap_or_else(|| generate_default_filename(&start_url, "_crawl", &format));
    
    // Create output based on include_raw flag
    let output_content = if include_raw {
        // Include full content with raw HTML (JSON only for raw)
        let crawl_output = CrawlOutput {
            stats: final_stats,
            results: crawl_results.clone(),
        };
        serde_json::to_string_pretty(&crawl_output)?
    } else {
        // Create ultra-clean output structure
        let mut total_words = 0;
        let content: Vec<PageContent> = crawl_results
            .iter()
            .filter_map(|result| {
                if let Some(ref cleaned) = result.cleaned_content {
                    // Include if has structured content or meaningful text
                    let has_content = cleaned.content.as_ref().map_or(false, |c| c.len() > 50) 
                        || cleaned.structured.is_some();
                    
                    if has_content {
                        total_words += cleaned.word_count;
                        
                        // Convert structured content to JSON value
                        let structured = if exclude_urls {
                            // Remove links from structured content
                            cleaned.structured.as_ref().map(|s| {
                                let mut s_clone = s.clone();
                                // Clear lists that might contain URLs
                                for list in &mut s_clone.lists {
                                    list.items.retain(|item| !item.starts_with("http"));
                                }
                                serde_json::to_value(s_clone).unwrap_or(serde_json::Value::Null)
                            })
                        } else {
                            cleaned.structured.as_ref().map(|s| {
                                serde_json::to_value(s).unwrap_or(serde_json::Value::Null)
                            })
                        };
                        
                        Some(PageContent {
                            url: result.url.clone(),
                            title: cleaned.title.clone(),
                            text: cleaned.content.clone(),
                            structured,
                            tables: Vec::new(), // Tables not included in non-organized output yet
                            words: cleaned.word_count,
                            links: if exclude_urls { Vec::new() } else { cleaned.links.clone() },
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        let clean_output = CleanCrawlOutput {
            url: start_url.to_string(),
            pages: content.len(),
            words: total_words,
            duration_ms: final_stats.elapsed_time.as_millis(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            content,
        };
        
        format_output_content(&clean_output, &format, exclude_urls)?
    };
    
    // Compress to ZIP if requested (for non-organized output)
    if zip && !organize {
        let zip_path = PathBuf::from(format!("{}.zip", output_path.display()));
        let zip_file = File::create(&zip_path)?;
        let mut zip_writer = zip::ZipWriter::new(zip_file);
        let options = zip::write::FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        
        let file_name = output_path.file_name().unwrap().to_str().unwrap();
        zip_writer.start_file(file_name, options)?;
        zip_writer.write_all(output_content.as_bytes())?;
        zip_writer.finish()?;
        
        // Remove the original file
        tokio::fs::remove_file(&output_path).await?;
        
        println!();
        println!(
            "{}  Compressed output to: {}",
            "📦".bold().green(),
            zip_path.display().to_string().yellow()
        );
        return Ok(());
    }
    
    // Write output to file (for non-ZIP case)
    tokio::fs::write(&output_path, output_content).await?;
    
    println!();
    if include_raw {
        println!(
            "{}  Saved {} pages with raw HTML content to: {}",
            "✅".bold().green(),
            crawl_results.len().to_string().cyan(),
            output_path.display().to_string().yellow()
        );
    } else {
        let total_words: usize = crawl_results
            .iter()
            .filter_map(|r| r.cleaned_content.as_ref().map(|c| c.word_count))
            .sum();
        println!(
            "{}  Saved {} pages of cleaned content ({} total words) to: {}",
            "✅".bold().green(),
            crawl_results.len().to_string().cyan(),
            total_words.to_string().cyan(),
            output_path.display().to_string().yellow()
        );
    }

    Ok(())
}

fn generate_parse_filename(input_file: &PathBuf) -> PathBuf {
    let file_stem = input_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("parsed");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    PathBuf::from(format!("parsed_{}_{}.json", file_stem, timestamp))
}

async fn parse_command(file: PathBuf, rules: Option<PathBuf>, output: Option<PathBuf>) -> Result<()> {
    println!("{}", "📄 Parsing HTML file...".bold().cyan());
    println!("Input file: {}", file.display().to_string().yellow());

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
    
    // Extract text content for summary
    let text_content = parser.extract_text(&content);
    let text_preview = if text_content.len() > 200 {
        format!("{}...", &text_content[..200])
    } else {
        text_content.clone()
    };
    
    println!();
    println!("{}", "✅ Parsing complete!".bold().green());
    println!("Extracted {} characters of text", text_content.len().to_string().cyan());
    
    // Determine output path
    let output_path = output.unwrap_or_else(|| generate_parse_filename(&file));
    
    // Create output with both parsed result and text content
    let output_data = serde_json::json!({
        "parsed": result,
        "text_content": text_content,
        "source_file": file.display().to_string(),
        "parsed_at": chrono::Utc::now().to_rfc3339()
    });
    
    let output_json = serde_json::to_string_pretty(&output_data)?;
    tokio::fs::write(&output_path, output_json).await?;
    
    println!();
    println!(
        "{}  Parsed content saved to: {}",
        "✅".bold().green(),
        output_path.display().to_string().yellow()
    );
    
    if text_preview.len() > 0 {
        println!();
        println!("Text preview:");
        println!("{}", text_preview.dimmed());
    }

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

async fn docs_command() -> Result<()> {
    let url = "https://ov.pranavkarra.me/docs";
    println!("{}", "📚 Opening Omnivore documentation...".bold().cyan());
    println!("URL: {}", url.bright_blue());
    
    // Try to open the URL in the default browser
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .context("Failed to open browser")?;
    }
    
    Ok(())
}

#[cfg(all())]
fn convert_dynamic_to_crawl_result(dynamic: omnivore_core::crawler::browser::DynamicContent) -> Result<CrawlResult> {
    use omnivore_core::extractor::ContentExtractor;
    
    // Combine all content variations
    let mut combined_content = dynamic.main_content.clone();
    
    for dropdown in &dynamic.dropdown_contents {
        combined_content.push_str("\n\n--- Dropdown Variation ---\n");
        combined_content.push_str(&dropdown.content);
    }
    
    for filter in &dynamic.filter_contents {
        combined_content.push_str("\n\n--- Filter Variation ---\n");
        combined_content.push_str(&filter.content);
    }
    
    let extractor = ContentExtractor::new();
    let cleaned_content = Some(extractor.extract_clean_content(&combined_content));
    
    Ok(CrawlResult {
        url: dynamic.url,
        status_code: 200,
        content: combined_content,
        cleaned_content,
        headers: std::collections::HashMap::new(),
        extracted_data: serde_json::json!({
            "has_infinite_scroll": dynamic.has_infinite_scroll,
            "dropdown_variations": dynamic.dropdown_contents.len(),
            "filter_variations": dynamic.filter_contents.len(),
        }),
        links: Vec::new(),
        crawled_at: chrono::Utc::now(),
    })
}

#[allow(dead_code)]
async fn handle_crawl_results(
    crawl_results: Vec<CrawlResult>,
    start_url: &Url,
    output: Option<PathBuf>,
    organize: bool,
    format: OutputFormat,
    _zip: bool,
    _extract_tables: bool,
    _exclude_urls: bool,
) -> Result<()> {
    let _final_stats = CrawlStats {
        total_urls: crawl_results.len(),
        successful: crawl_results.len(),
        failed: 0,
        in_progress: 0,
        average_response_time_ms: 0.0,
        start_time: chrono::Utc::now(),
        elapsed_time: std::time::Duration::from_secs(0),
    };
    
    println!();
    println!("{}", "📊 Final Statistics:".bold().green());
    println!(
        "  Total pages processed: {}",
        crawl_results.len().to_string().cyan()
    );
    
    // Reuse existing output logic
    if organize {
        // Use existing organize logic
        let domain = start_url.domain().unwrap_or("unknown");
        let sanitized_domain = domain.replace('.', "_").replace('/', "_");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let output_dir = output.unwrap_or_else(|| {
            PathBuf::from(format!("{}_{}_crawl", sanitized_domain, timestamp))
        });
        
        tokio::fs::create_dir_all(&output_dir).await?;
        
        // Save the content
        for (idx, result) in crawl_results.iter().enumerate() {
            let page_file = output_dir.join(format!("page_{:04}.json", idx + 1));
            let page_json = serde_json::to_string_pretty(&result)?;
            tokio::fs::write(&page_file, page_json).await?;
        }
        
        println!(
            "{}  Organized {} pages into folder: {}",
            "✅".bold().green(),
            crawl_results.len().to_string().cyan(),
            output_dir.display().to_string().yellow()
        );
    } else {
        // Use existing single file output logic
        let output_path = output.unwrap_or_else(|| generate_default_filename(start_url, "_browser_crawl", &format));
        let output_json = serde_json::to_string_pretty(&crawl_results)?;
        tokio::fs::write(&output_path, output_json).await?;
        
        println!(
            "{}  Saved {} pages to: {}",
            "✅".bold().green(),
            crawl_results.len().to_string().cyan(),
            output_path.display().to_string().yellow()
        );
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
