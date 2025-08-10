use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use crate::config::OmnivoreConfig;

#[derive(Debug, Clone)]
pub struct AiInterpreter {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionIntent {
    pub description: String,
    pub targets: Vec<ExtractionTarget>,
    pub actions: Vec<ExtractionAction>,
    pub filters: Vec<FilterCriteria>,
    pub output_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionTarget {
    pub name: String,
    pub target_type: String, // "text", "table", "image", "link", "form", "dropdown"
    pub selectors: Vec<String>,
    pub attributes: Vec<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionAction {
    pub action_type: String, // "click", "fill", "select", "scroll", "wait"
    pub target: String,
    pub value: Option<String>,
    pub wait_after_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCriteria {
    pub field: String,
    pub operator: String, // "contains", "equals", "regex", "greater_than", "less_than"
    pub value: String,
}

impl AiInterpreter {
    pub fn new(config: &OmnivoreConfig) -> Result<Self> {
        let api_key = config.ai.openai_api_key.clone()
            .context("OpenAI API key not configured")?;
        
        Ok(Self {
            api_key,
            model: config.ai.model.clone(),
            temperature: config.ai.temperature,
            max_tokens: config.ai.max_tokens,
            client: Client::new(),
        })
    }
    
    pub async fn interpret_request(&self, user_request: &str, url: &str) -> Result<ExtractionIntent> {
        let prompt = self.build_interpretation_prompt(user_request, url);
        let response = self.call_openai(&prompt).await?;
        let intent = self.parse_response(&response)?;
        Ok(intent)
    }
    
    fn build_interpretation_prompt(&self, user_request: &str, url: &str) -> String {
        format!(r#"
You are an expert web scraping assistant. Analyze the user's request and convert it into a structured extraction plan.

User Request: "{}"
Target URL: {}

Convert this request into a JSON extraction plan with the following structure:
{{
    "description": "Brief description of what to extract",
    "targets": [
        {{
            "name": "Name of the data to extract",
            "target_type": "text|table|image|link|form|dropdown",
            "selectors": ["CSS selectors or patterns to find this data"],
            "attributes": ["text", "href", "src", etc.],
            "required": true/false
        }}
    ],
    "actions": [
        {{
            "action_type": "click|fill|select|scroll|wait",
            "target": "CSS selector or element description",
            "value": "Value to input (for fill/select)",
            "wait_after_ms": 1000
        }}
    ],
    "filters": [
        {{
            "field": "Field name to filter",
            "operator": "contains|equals|regex|greater_than|less_than",
            "value": "Filter value"
        }}
    ],
    "output_format": "json|csv|markdown|yaml"
}}

Examples of user requests and their interpretations:

1. "Get all product prices and descriptions"
   - Extract elements with product class
   - Target price and description text
   - Output as structured JSON

2. "Extract all email addresses and phone numbers"
   - Use regex patterns for emails and phones
   - Search entire page content
   - Output as CSV

3. "Download all PDF files"
   - Find all links ending in .pdf
   - Extract href attributes
   - Save files to disk

4. "Get data from all dropdown options"
   - Identify all select elements
   - Iterate through each option
   - Capture page content for each selection

5. "Extract tables with financial data"
   - Find all table elements
   - Filter tables containing currency symbols
   - Export as CSV files

Based on the user's request, provide a comprehensive extraction plan.
Return ONLY valid JSON, no additional text or explanation.
"#, user_request, url)
    }
    
    async fn call_openai(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct OpenAIRequest {
            model: String,
            messages: Vec<Message>,
            temperature: f32,
            max_tokens: u32,
            response_format: ResponseFormat,
        }
        
        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }
        
        #[derive(Serialize)]
        struct ResponseFormat {
            #[serde(rename = "type")]
            format_type: String,
        }
        
        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<Choice>,
        }
        
        #[derive(Deserialize)]
        struct Choice {
            message: ResponseMessage,
        }
        
        #[derive(Deserialize)]
        struct ResponseMessage {
            content: String,
        }
        
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are an expert web scraping assistant that converts natural language requests into structured extraction plans.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to call OpenAI API")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenAI API error: {}", error_text);
        }
        
        let api_response: OpenAIResponse = response.json().await
            .context("Failed to parse OpenAI response")?;
        
        let content = api_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .context("No response from OpenAI")?;
        
        Ok(content)
    }
    
    fn parse_response(&self, response: &str) -> Result<ExtractionIntent> {
        serde_json::from_str(response)
            .context("Failed to parse extraction intent from AI response")
    }
    
    pub async fn suggest_selectors(&self, html_sample: &str, target_type: &str) -> Result<Vec<String>> {
        let prompt = format!(r#"
Analyze this HTML sample and suggest CSS selectors for extracting {} data:

HTML:
{}

Provide a JSON array of the most likely CSS selectors that would capture this type of data.
Consider common patterns and be specific enough to avoid false matches.

Return ONLY a JSON array of strings, like: ["selector1", "selector2", "selector3"]
"#, target_type, html_sample);
        
        let response = self.call_openai(&prompt).await?;
        let selectors: Vec<String> = serde_json::from_str(&response)
            .context("Failed to parse selector suggestions")?;
        
        Ok(selectors)
    }
    
    pub async fn classify_content(&self, text: &str) -> Result<ContentClassification> {
        let prompt = format!(r#"
Classify this web content and identify what type of data it contains:

Text:
{}

Return a JSON object with:
{{
    "content_type": "product|article|academic|contact|navigation|form|table|list",
    "data_types": ["prices", "emails", "phones", "addresses", "dates", "names"],
    "suggested_extraction": "Brief suggestion on what to extract",
    "confidence": 0.0-1.0
}}
"#, text);
        
        let response = self.call_openai(&prompt).await?;
        let classification: ContentClassification = serde_json::from_str(&response)
            .context("Failed to parse content classification")?;
        
        Ok(classification)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentClassification {
    pub content_type: String,
    pub data_types: Vec<String>,
    pub suggested_extraction: String,
    pub confidence: f32,
}

pub struct SmartExtractor {
    interpreter: Option<AiInterpreter>,
}

impl SmartExtractor {
    pub fn new(config: &OmnivoreConfig) -> Self {
        let interpreter = if config.ai.enable_natural_language {
            AiInterpreter::new(config).ok()
        } else {
            None
        };
        
        Self { interpreter }
    }
    
    pub async fn extract_with_intent(&self, intent: &ExtractionIntent, html: &str) -> Result<serde_json::Value> {
        let mut results = serde_json::Map::new();
        
        // Parse HTML
        let document = scraper::Html::parse_document(html);
        
        // Extract each target
        for target in &intent.targets {
            let mut values = Vec::new();
            
            for selector_str in &target.selectors {
                if let Ok(selector) = scraper::Selector::parse(selector_str) {
                    for element in document.select(&selector) {
                        let value = match target.target_type.as_str() {
                            "text" => element.text().collect::<String>(),
                            "link" => element.value().attr("href").unwrap_or("").to_string(),
                            "image" => element.value().attr("src").unwrap_or("").to_string(),
                            _ => element.html(),
                        };
                        
                        if !value.is_empty() {
                            values.push(value);
                        }
                    }
                }
            }
            
            // Apply filters
            for filter in &intent.filters {
                if filter.field == target.name {
                    values = self.apply_filter(values, filter);
                }
            }
            
            results.insert(target.name.clone(), serde_json::json!(values));
        }
        
        Ok(serde_json::Value::Object(results))
    }
    
    fn apply_filter(&self, mut values: Vec<String>, filter: &FilterCriteria) -> Vec<String> {
        values.retain(|v| {
            match filter.operator.as_str() {
                "contains" => v.contains(&filter.value),
                "equals" => v == &filter.value,
                "regex" => {
                    if let Ok(re) = regex::Regex::new(&filter.value) {
                        re.is_match(v)
                    } else {
                        false
                    }
                }
                _ => true,
            }
        });
        values
    }
    
    pub async fn process_natural_language(&self, request: &str, url: &str, html: &str) -> Result<serde_json::Value> {
        if let Some(interpreter) = &self.interpreter {
            let intent = interpreter.interpret_request(request, url).await?;
            self.extract_with_intent(&intent, html).await
        } else {
            anyhow::bail!("AI interpreter not configured. Please run 'omnivore setup' and configure your OpenAI API key.")
        }
    }
}