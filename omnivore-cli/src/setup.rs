use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use omnivore_core::config::{OmnivoreConfig, ExtractionTemplate, PatternRule};
use std::fs;
use std::path::PathBuf;

pub async fn run_setup() -> Result<()> {
    println!("{}", "🔧 Omnivore Setup Wizard".bold().cyan());
    println!("{}", "This wizard will help you configure Omnivore for optimal performance.\n".dimmed());
    
    // Load existing config or create new
    let mut config = OmnivoreConfig::load().unwrap_or_default();
    
    // Main menu
    loop {
        let choices = vec![
            "Configure AI (OpenAI API)",
            "Configure Extraction Settings",
            "Configure Browser Settings",
            "Configure Output Settings",
            "Create Extraction Template",
            "Test Configuration",
            "Save and Exit",
            "Exit without Saving",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to configure?")
            .items(&choices)
            .default(0)
            .interact()?;
        
        match selection {
            0 => configure_ai(&mut config)?,
            1 => configure_extraction(&mut config)?,
            2 => configure_browser(&mut config)?,
            3 => configure_output(&mut config)?,
            4 => create_template()?,
            5 => test_configuration(&config).await?,
            6 => {
                config.save()?;
                println!("{}", "✅ Configuration saved successfully!".green());
                break;
            }
            7 => {
                println!("{}", "Configuration not saved.".yellow());
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}

fn configure_ai(config: &mut OmnivoreConfig) -> Result<()> {
    println!("\n{}", "🤖 AI Configuration".bold());
    
    // API Key
    let current_key = config.ai.openai_api_key.as_ref()
        .map(|k| {
            if k.len() > 8 {
                format!("{}...{}", &k[..4], &k[k.len()-4..])
            } else {
                "****".to_string()
            }
        })
        .unwrap_or_else(|| "Not set".to_string());
    
    println!("Current API key: {}", current_key.dimmed());
    
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to configure OpenAI API key?")
        .default(true)
        .interact()?
    {
        let api_key: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter your OpenAI API key")
            .validate_with(|input: &String| {
                if input.starts_with("sk-") && input.len() > 20 {
                    Ok(())
                } else {
                    Err("Invalid API key format. Should start with 'sk-' and be at least 20 characters")
                }
            })
            .interact()?;
        
        config.ai.openai_api_key = Some(api_key);
        println!("{}", "✓ API key configured".green());
    }
    
    // Model selection
    let models = vec![
        "gpt-4-turbo-preview",
        "gpt-4",
        "gpt-3.5-turbo",
    ];
    
    let model_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select AI model")
        .items(&models)
        .default(0)
        .interact()?;
    
    config.ai.model = models[model_idx].to_string();
    
    // Enable natural language
    config.ai.enable_natural_language = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Enable natural language extraction? (requires API key)")
        .default(true)
        .interact()?;
    
    println!("{}", "✓ AI configuration updated".green());
    Ok(())
}

fn configure_extraction(config: &mut OmnivoreConfig) -> Result<()> {
    println!("\n{}", "📊 Extraction Settings".bold());
    
    config.extraction.auto_detect_tables = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-detect and extract tables?")
        .default(true)
        .interact()?;
    
    config.extraction.auto_detect_forms = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-detect forms?")
        .default(true)
        .interact()?;
    
    config.extraction.auto_detect_dropdowns = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-detect dropdowns and interact with them?")
        .default(true)
        .interact()?;
    
    config.extraction.auto_detect_pagination = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-detect pagination?")
        .default(true)
        .interact()?;
    
    if config.extraction.auto_detect_pagination {
        config.extraction.auto_follow_pagination = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Automatically follow pagination?")
            .default(false)
            .interact()?;
        
        if config.extraction.auto_follow_pagination {
            let max_pages: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Maximum pages to follow")
                .default("10".to_string())
                .interact()?;
            
            config.extraction.max_pagination_pages = max_pages.parse().unwrap_or(10);
        }
    }
    
    config.extraction.auto_detect_downloads = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-detect downloadable files?")
        .default(true)
        .interact()?;
    
    config.extraction.auto_extract_emails = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-extract email addresses?")
        .default(true)
        .interact()?;
    
    config.extraction.auto_extract_phones = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto-extract phone numbers?")
        .default(true)
        .interact()?;
    
    println!("{}", "✓ Extraction settings updated".green());
    Ok(())
}

fn configure_browser(config: &mut OmnivoreConfig) -> Result<()> {
    println!("\n{}", "🌐 Browser Settings".bold());
    
    if let Ok(driver_path) = which::which("chromedriver") {
        println!("Found ChromeDriver at: {}", driver_path.display().to_string().green());
        config.browser.driver_path = Some(driver_path.to_string_lossy().to_string());
    } else {
        println!("{}", "⚠️  ChromeDriver not found in PATH".yellow());
        
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to specify ChromeDriver path manually?")
            .default(false)
            .interact()?
        {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter ChromeDriver path")
                .interact()?;
            
            config.browser.driver_path = Some(path);
        }
    }
    
    config.browser.headless = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Run browser in headless mode?")
        .default(true)
        .interact()?;
    
    let timeout: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Page load timeout (seconds)")
        .default("30".to_string())
        .interact()?;
    
    config.browser.timeout = timeout.parse().unwrap_or(30);
    
    config.browser.wait_for_dynamic = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wait for dynamic content to load?")
        .default(true)
        .interact()?;
    
    config.browser.screenshot_errors = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Take screenshots on errors?")
        .default(false)
        .interact()?;
    
    println!("{}", "✓ Browser settings updated".green());
    Ok(())
}

fn configure_output(config: &mut OmnivoreConfig) -> Result<()> {
    println!("\n{}", "💾 Output Settings".bold());
    
    let formats = vec!["json", "csv", "markdown", "yaml", "text"];
    let format_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Default output format")
        .items(&formats)
        .default(0)
        .interact()?;
    
    config.output.default_format = formats[format_idx].to_string();
    
    config.output.organize_output = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Organize output in folders?")
        .default(true)
        .interact()?;
    
    config.output.compress_output = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Compress output to ZIP?")
        .default(false)
        .interact()?;
    
    config.output.save_raw_html = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Save raw HTML?")
        .default(false)
        .interact()?;
    
    // Webhook configuration
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Configure webhook for results?")
        .default(false)
        .interact()?
    {
        let webhook_url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter webhook URL")
            .validate_with(|input: &String| {
                if input.starts_with("http://") || input.starts_with("https://") {
                    Ok(())
                } else {
                    Err("URL must start with http:// or https://")
                }
            })
            .interact()?;
        
        config.output.webhook_url = Some(webhook_url);
    }
    
    println!("{}", "✓ Output settings updated".green());
    Ok(())
}

fn create_template() -> Result<()> {
    println!("\n{}", "📝 Create Extraction Template".bold());
    
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Template name")
        .interact()?;
    
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Template description")
        .interact()?;
    
    let template_types = vec![
        "E-commerce (products, prices)",
        "Academic (papers, citations)",
        "News (articles, headlines)",
        "Contact (emails, phones, addresses)",
        "Custom",
    ];
    
    let template_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template type")
        .items(&template_types)
        .default(0)
        .interact()?;
    
    let mut patterns = Vec::new();
    
    match template_idx {
        0 => {
            // E-commerce template
            patterns.push(PatternRule {
                name: "products".to_string(),
                pattern_type: "css".to_string(),
                selector: ".product, [itemtype*='Product']".to_string(),
                extract: vec!["name".to_string(), "price".to_string(), "description".to_string()],
                transform: None,
                required: true,
            });
        }
        1 => {
            // Academic template
            patterns.push(PatternRule {
                name: "papers".to_string(),
                pattern_type: "css".to_string(),
                selector: ".paper, article".to_string(),
                extract: vec!["title".to_string(), "authors".to_string(), "abstract".to_string()],
                transform: None,
                required: true,
            });
        }
        2 => {
            // News template
            patterns.push(PatternRule {
                name: "articles".to_string(),
                pattern_type: "css".to_string(),
                selector: "article, .article, .news-item".to_string(),
                extract: vec!["headline".to_string(), "date".to_string(), "content".to_string()],
                transform: None,
                required: true,
            });
        }
        3 => {
            // Contact template
            patterns.push(PatternRule {
                name: "contacts".to_string(),
                pattern_type: "regex".to_string(),
                selector: r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
                extract: vec!["email".to_string()],
                transform: None,
                required: false,
            });
        }
        _ => {
            // Custom template
            println!("Define custom extraction patterns:");
            
            loop {
                if !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Add extraction pattern?")
                    .default(true)
                    .interact()?
                {
                    break;
                }
                
                let pattern_name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pattern name")
                    .interact()?;
                
                let pattern_types = vec!["css", "xpath", "regex"];
                let type_idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pattern type")
                    .items(&pattern_types)
                    .default(0)
                    .interact()?;
                
                let selector: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Selector/Pattern")
                    .interact()?;
                
                let extract_fields: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Fields to extract (comma-separated)")
                    .interact()?;
                
                let extract: Vec<String> = extract_fields
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                patterns.push(PatternRule {
                    name: pattern_name,
                    pattern_type: pattern_types[type_idx].to_string(),
                    selector,
                    extract,
                    transform: None,
                    required: false,
                });
            }
        }
    }
    
    let template = ExtractionTemplate {
        name: name.clone(),
        description,
        version: "1.0.0".to_string(),
        author: None,
        patterns,
        pipelines: Vec::new(),
        output_schema: None,
    };
    
    template.save()?;
    
    println!("{}", format!("✅ Template '{}' created successfully!", name).green());
    println!("{}", format!("Use it with: omnivore crawl <URL> --template {}", name).dimmed());
    
    Ok(())
}

async fn test_configuration(config: &OmnivoreConfig) -> Result<()> {
    println!("\n{}", "🧪 Testing Configuration".bold());
    
    // Test AI configuration
    if let Some(api_key) = &config.ai.openai_api_key {
        print!("Testing OpenAI API connection... ");
        
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                println!("{}", "✓".green());
            }
            _ => {
                println!("{}", "✗ Failed".red());
            }
        }
    } else {
        println!("{}", "⚠️  OpenAI API key not configured".yellow());
    }
    
    // Test ChromeDriver
    if let Some(driver_path) = &config.browser.driver_path {
        print!("Testing ChromeDriver... ");
        
        let path = PathBuf::from(driver_path);
        if path.exists() && path.is_file() {
            println!("{}", "✓".green());
        } else {
            println!("{}", "✗ Not found".red());
        }
    } else {
        println!("{}", "⚠️  ChromeDriver path not configured".yellow());
    }
    
    // Test template directory
    print!("Checking templates directory... ");
    if config.templates.templates_dir.exists() {
        let template_count = ExtractionTemplate::list_templates()?.len();
        println!("{}", format!("✓ ({} templates found)", template_count).green());
    } else {
        fs::create_dir_all(&config.templates.templates_dir)?;
        println!("{}", "✓ Created".green());
    }
    
    println!("\n{}", "Configuration test complete!".bold().green());
    Ok(())
}

pub fn check_api_key_status() -> (bool, String) {
    match OmnivoreConfig::load() {
        Ok(config) => {
            if config.ai.openai_api_key.is_some() {
                (true, format!("{} OpenAI API configured", "✓".green()))
            } else {
                (false, format!("{} OpenAI API not configured (run 'omnivore setup')", "✗".red()))
            }
        }
        Err(_) => {
            (false, format!("{} Configuration not found (run 'omnivore setup')", "✗".yellow()))
        }
    }
}

pub async fn show_config() -> Result<()> {
    println!("{}", "📋 Omnivore Configuration".bold().cyan());
    println!();
    
    match OmnivoreConfig::load() {
        Ok(config) => {
            // AI Configuration
            println!("{}", "🤖 AI Settings:".bold());
            println!("  API Key: {}", if config.ai.openai_api_key.is_some() {
                "✓ Configured".green().to_string()
            } else {
                "✗ Not configured".red().to_string()
            });
            println!("  Model: {}", config.ai.model.yellow());
            println!("  Natural Language: {}", if config.ai.enable_natural_language {
                "✓ Enabled".green().to_string()
            } else {
                "✗ Disabled".dimmed().to_string()
            });
            println!();
            
            // Extraction Settings
            println!("{}", "📊 Extraction Settings:".bold());
            println!("  Auto-detect tables: {}", bool_status(config.extraction.auto_detect_tables));
            println!("  Auto-detect forms: {}", bool_status(config.extraction.auto_detect_forms));
            println!("  Auto-detect dropdowns: {}", bool_status(config.extraction.auto_detect_dropdowns));
            println!("  Auto-detect pagination: {}", bool_status(config.extraction.auto_detect_pagination));
            println!("  Auto-follow pagination: {}", bool_status(config.extraction.auto_follow_pagination));
            if config.extraction.auto_follow_pagination {
                println!("    Max pages: {}", config.extraction.max_pagination_pages.to_string().yellow());
            }
            println!("  Auto-detect downloads: {}", bool_status(config.extraction.auto_detect_downloads));
            println!("  Auto-extract emails: {}", bool_status(config.extraction.auto_extract_emails));
            println!("  Auto-extract phones: {}", bool_status(config.extraction.auto_extract_phones));
            println!();
            
            // Browser Settings
            println!("{}", "🌐 Browser Settings:".bold());
            if let Some(ref driver_path) = config.browser.driver_path {
                println!("  ChromeDriver: {}", driver_path.green());
            } else {
                println!("  ChromeDriver: {}", "Not configured".yellow());
            }
            println!("  Headless mode: {}", bool_status(config.browser.headless));
            println!("  Page timeout: {}s", config.browser.timeout.to_string().yellow());
            println!();
            
            // Output Settings
            println!("{}", "💾 Output Settings:".bold());
            println!("  Default format: {}", config.output.default_format.yellow());
            println!("  Organize output: {}", bool_status(config.output.organize_output));
            println!("  Compress output: {}", bool_status(config.output.compress_output));
            if let Some(ref webhook) = config.output.webhook_url {
                println!("  Webhook: {}", webhook.cyan());
            }
            println!();
            
            // Advanced Settings
            println!("{}", "⚙️  Advanced Settings:".bold());
            println!("  Max workers: {}", config.advanced.max_workers.to_string().yellow());
            println!("  Max depth: {}", config.advanced.max_depth.to_string().yellow());
            println!("  Respect robots.txt: {}", bool_status(config.advanced.respect_robots));
            println!("  Rate limit: {}ms", config.advanced.rate_limit_ms.to_string().yellow());
            println!();
            
            // Templates
            println!("{}", "📝 Templates:".bold());
            match ExtractionTemplate::list_templates() {
                Ok(templates) if !templates.is_empty() => {
                    for template in templates {
                        println!("  - {}", template.green());
                    }
                }
                _ => {
                    println!("  No templates found");
                }
            }
            println!();
            
            println!("{}", "💡 Tip: Run 'omnivore setup' to modify configuration".dimmed());
        }
        Err(_) => {
            println!("{}", "⚠️  No configuration found!".yellow());
            println!();
            println!("Run {} to create your configuration", "'omnivore setup'".bold());
        }
    }
    
    Ok(())
}

fn bool_status(value: bool) -> String {
    if value {
        "✓".green().to_string()
    } else {
        "✗".dimmed().to_string()
    }
}