use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnivoreConfig {
    #[serde(default)]
    pub ai: AiConfig,
    
    #[serde(default)]
    pub extraction: ExtractionConfig,
    
    #[serde(default)]
    pub browser: BrowserConfig,
    
    #[serde(default)]
    pub output: OutputConfig,
    
    #[serde(default)]
    pub templates: TemplateConfig,
    
    #[serde(default)]
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub openai_api_key: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub enable_natural_language: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionConfig {
    pub auto_detect_tables: bool,
    pub auto_detect_forms: bool,
    pub auto_detect_dropdowns: bool,
    pub auto_detect_pagination: bool,
    pub auto_detect_downloads: bool,
    pub auto_extract_emails: bool,
    pub auto_extract_phones: bool,
    pub auto_extract_addresses: bool,
    pub auto_follow_pagination: bool,
    pub max_pagination_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub driver_path: Option<String>,
    pub headless: bool,
    pub timeout: u32,
    pub wait_for_dynamic: bool,
    pub screenshot_errors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub default_format: String,
    pub organize_output: bool,
    pub compress_output: bool,
    pub save_raw_html: bool,
    pub save_screenshots: bool,
    pub database_export: Option<DatabaseConfig>,
    pub cloud_export: Option<CloudConfig>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub table_name: String,
    pub batch_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub provider: String, // "s3", "gcs", "azure"
    pub bucket: String,
    pub credentials: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub templates_dir: PathBuf,
    pub default_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    pub max_workers: usize,
    pub max_depth: u32,
    pub respect_robots: bool,
    pub user_agent: String,
    pub rate_limit_ms: u64,
    pub retry_attempts: u32,
    pub deduplication: bool,
}

impl Default for OmnivoreConfig {
    fn default() -> Self {
        Self {
            ai: AiConfig::default(),
            extraction: ExtractionConfig::default(),
            browser: BrowserConfig::default(),
            output: OutputConfig::default(),
            templates: TemplateConfig::default(),
            advanced: AdvancedConfig::default(),
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            openai_api_key: None,
            model: "gpt-4-turbo-preview".to_string(),
            temperature: 0.3,
            max_tokens: 2000,
            enable_natural_language: false,
        }
    }
}

impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            auto_detect_tables: true,
            auto_detect_forms: true,
            auto_detect_dropdowns: true,
            auto_detect_pagination: true,
            auto_detect_downloads: true,
            auto_extract_emails: true,
            auto_extract_phones: true,
            auto_extract_addresses: true,
            auto_follow_pagination: false,
            max_pagination_pages: 10,
        }
    }
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            driver_path: None,
            headless: true,
            timeout: 30,
            wait_for_dynamic: true,
            screenshot_errors: false,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_format: "json".to_string(),
            organize_output: true,
            compress_output: false,
            save_raw_html: false,
            save_screenshots: false,
            database_export: None,
            cloud_export: None,
            webhook_url: None,
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            templates_dir: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".omnivore")
                .join("templates"),
            default_template: None,
        }
    }
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            max_workers: 10,
            max_depth: 5,
            respect_robots: true,
            user_agent: "Omnivore/1.0".to_string(),
            rate_limit_ms: 100,
            retry_attempts: 3,
            deduplication: true,
        }
    }
}

impl OmnivoreConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }
        
        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        
        let config: Self = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    pub fn config_path() -> Result<PathBuf> {
        // Check for OMNIVORE_CONFIG env var first
        if let Ok(path) = env::var("OMNIVORE_CONFIG") {
            return Ok(PathBuf::from(path));
        }
        
        // Default to ~/.omnivore/config.toml
        let home = dirs::home_dir()
            .context("Failed to get home directory")?;
        
        Ok(home.join(".omnivore").join("config.toml"))
    }
    
    pub fn merge_with_env(&mut self) {
        // Override with environment variables if present
        if let Ok(key) = env::var("OMNIVORE_OPENAI_API_KEY") {
            self.ai.openai_api_key = Some(key);
        }
        
        if let Ok(model) = env::var("OMNIVORE_AI_MODEL") {
            self.ai.model = model;
        }
        
        if let Ok(webhook) = env::var("OMNIVORE_WEBHOOK_URL") {
            self.output.webhook_url = Some(webhook);
        }
        
        if let Ok(ua) = env::var("OMNIVORE_USER_AGENT") {
            self.advanced.user_agent = ua;
        }
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate configuration
        if self.ai.enable_natural_language && self.ai.openai_api_key.is_none() {
            anyhow::bail!("OpenAI API key is required when natural language mode is enabled");
        }
        
        if self.advanced.max_workers == 0 {
            anyhow::bail!("max_workers must be greater than 0");
        }
        
        if self.advanced.max_depth == 0 {
            anyhow::bail!("max_depth must be greater than 0");
        }
        
        Ok(())
    }
    
    pub fn is_configured(&self) -> bool {
        // Check if basic configuration is complete
        self.ai.openai_api_key.is_some() || !self.ai.enable_natural_language
    }
    
    pub fn get_api_key(&self) -> Option<&str> {
        self.ai.openai_api_key.as_deref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionTemplate {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub patterns: Vec<PatternRule>,
    pub pipelines: Vec<String>,
    pub output_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRule {
    pub name: String,
    pub pattern_type: String, // "css", "xpath", "regex", "json_path"
    pub selector: String,
    pub extract: Vec<String>,
    pub transform: Option<String>,
    pub required: bool,
}

impl ExtractionTemplate {
    pub fn load(name: &str) -> Result<Self> {
        let config = OmnivoreConfig::load()?;
        let template_path = config.templates.templates_dir.join(format!("{}.yaml", name));
        
        if !template_path.exists() {
            anyhow::bail!("Template '{}' not found", name);
        }
        
        let content = fs::read_to_string(&template_path)
            .context("Failed to read template file")?;
        
        let template: Self = serde_yaml::from_str(&content)
            .context("Failed to parse template file")?;
        
        Ok(template)
    }
    
    pub fn save(&self) -> Result<()> {
        let config = OmnivoreConfig::load()?;
        let template_path = config.templates.templates_dir.join(format!("{}.yaml", self.name));
        
        // Create templates directory if it doesn't exist
        fs::create_dir_all(&config.templates.templates_dir)
            .context("Failed to create templates directory")?;
        
        let content = serde_yaml::to_string(self)
            .context("Failed to serialize template")?;
        
        fs::write(&template_path, content)
            .context("Failed to write template file")?;
        
        Ok(())
    }
    
    pub fn list_templates() -> Result<Vec<String>> {
        let config = OmnivoreConfig::load()?;
        let templates_dir = &config.templates.templates_dir;
        
        if !templates_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut templates = Vec::new();
        
        for entry in fs::read_dir(templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    templates.push(stem.to_string());
                }
            }
        }
        
        Ok(templates)
    }
}