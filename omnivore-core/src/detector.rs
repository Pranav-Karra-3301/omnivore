use scraper::{Html, Selector, ElementRef};
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedElements {
    pub tables: Vec<TableElement>,
    pub forms: Vec<FormElement>,
    pub dropdowns: Vec<DropdownElement>,
    pub pagination: Option<PaginationElement>,
    pub downloads: Vec<DownloadLink>,
    pub contacts: ContactInfo,
    pub interactive: Vec<InteractiveElement>,
    pub media: MediaElements,
    pub structured_data: Vec<StructuredData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableElement {
    pub selector: String,
    pub headers: Vec<String>,
    pub row_count: usize,
    pub has_numeric_data: bool,
    pub likely_data_type: String, // "financial", "product", "statistical", "generic"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormElement {
    pub selector: String,
    pub action: Option<String>,
    pub method: String,
    pub fields: Vec<FormField>,
    pub has_file_upload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub field_type: String,
    pub label: Option<String>,
    pub required: bool,
    pub options: Vec<String>, // For select/radio/checkbox
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownElement {
    pub selector: String,
    pub name: Option<String>,
    pub label: Option<String>,
    pub options: Vec<String>,
    pub option_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationElement {
    pub next_selector: Option<String>,
    pub prev_selector: Option<String>,
    pub page_numbers: Vec<String>,
    pub total_pages: Option<usize>,
    pub current_page: Option<usize>,
    pub pagination_type: String, // "numbered", "next_prev", "infinite_scroll"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    pub url: String,
    pub text: String,
    pub file_type: String,
    pub file_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub addresses: Vec<String>,
    pub social_links: Vec<SocialLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveElement {
    pub selector: String,
    pub element_type: String, // "button", "toggle", "accordion", "tab", "modal_trigger"
    pub text: String,
    pub action: String, // "click", "hover", "toggle"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaElements {
    pub images: Vec<ImageElement>,
    pub videos: Vec<VideoElement>,
    pub audio: Vec<AudioElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
    pub src: String,
    pub alt: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement {
    pub src: String,
    pub poster: Option<String>,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioElement {
    pub src: String,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredData {
    pub data_type: String, // "json-ld", "microdata", "rdfa"
    pub schema_type: String,
    pub content: serde_json::Value,
}

pub struct UniversalDetector {
    document: Html,
    base_url: Option<Url>,
}

impl UniversalDetector {
    pub fn new(html: &str, base_url: Option<&str>) -> Self {
        let document = Html::parse_document(html);
        let base_url = base_url.and_then(|u| Url::parse(u).ok());
        
        Self {
            document,
            base_url,
        }
    }
    
    pub fn detect_all(&self) -> DetectedElements {
        DetectedElements {
            tables: self.detect_tables(),
            forms: self.detect_forms(),
            dropdowns: self.detect_dropdowns(),
            pagination: self.detect_pagination(),
            downloads: self.detect_downloads(),
            contacts: self.detect_contacts(),
            interactive: self.detect_interactive(),
            media: self.detect_media(),
            structured_data: self.detect_structured_data(),
        }
    }
    
    pub fn detect_tables(&self) -> Vec<TableElement> {
        let mut tables = Vec::new();
        
        if let Ok(selector) = Selector::parse("table") {
            for (idx, table) in self.document.select(&selector).enumerate() {
                let headers = self.extract_table_headers(&table);
                let row_count = table.select(&Selector::parse("tr").unwrap()).count();
                let has_numeric = self.table_has_numeric_data(&table);
                let data_type = self.classify_table_type(&table, &headers);
                
                tables.push(TableElement {
                    selector: format!("table:nth-of-type({})", idx + 1),
                    headers,
                    row_count,
                    has_numeric_data: has_numeric,
                    likely_data_type: data_type,
                });
            }
        }
        
        // Also detect div-based tables
        self.detect_div_tables(&mut tables);
        
        tables
    }
    
    fn extract_table_headers(&self, table: &ElementRef) -> Vec<String> {
        let mut headers = Vec::new();
        
        // Try th elements
        if let Ok(selector) = Selector::parse("thead th, th") {
            for th in table.select(&selector) {
                headers.push(th.text().collect::<String>().trim().to_string());
            }
        }
        
        // If no headers found, try first row
        if headers.is_empty() {
            if let Ok(selector) = Selector::parse("tr:first-child td") {
                for td in table.select(&selector) {
                    headers.push(td.text().collect::<String>().trim().to_string());
                }
            }
        }
        
        headers
    }
    
    fn table_has_numeric_data(&self, table: &ElementRef) -> bool {
        let numeric_regex = Regex::new(r"\d+\.?\d*").unwrap();
        let text = table.text().collect::<String>();
        numeric_regex.is_match(&text)
    }
    
    fn classify_table_type(&self, table: &ElementRef, headers: &[String]) -> String {
        let text = table.text().collect::<String>().to_lowercase();
        let headers_text = headers.join(" ").to_lowercase();
        
        if headers_text.contains("price") || headers_text.contains("cost") || 
           text.contains("$") || text.contains("€") || text.contains("£") {
            "financial".to_string()
        } else if headers_text.contains("product") || headers_text.contains("item") ||
                  headers_text.contains("sku") || headers_text.contains("inventory") {
            "product".to_string()
        } else if headers_text.contains("mean") || headers_text.contains("std") ||
                  headers_text.contains("avg") || headers_text.contains("median") {
            "statistical".to_string()
        } else {
            "generic".to_string()
        }
    }
    
    fn detect_div_tables(&self, tables: &mut Vec<TableElement>) {
        // Detect common div-based table patterns
        let patterns = vec![
            ".table-row",
            "[role='table']",
            ".data-table",
            ".grid-table",
        ];
        
        for pattern in patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                if self.document.select(&selector).next().is_some() {
                    tables.push(TableElement {
                        selector: pattern.to_string(),
                        headers: Vec::new(),
                        row_count: self.document.select(&selector).count(),
                        has_numeric_data: false,
                        likely_data_type: "div-table".to_string(),
                    });
                }
            }
        }
    }
    
    pub fn detect_forms(&self) -> Vec<FormElement> {
        let mut forms = Vec::new();
        
        if let Ok(selector) = Selector::parse("form") {
            for (idx, form) in self.document.select(&selector).enumerate() {
                let action = form.value().attr("action").map(|s| s.to_string());
                let method = form.value().attr("method")
                    .unwrap_or("get")
                    .to_string();
                
                let fields = self.extract_form_fields(&form);
                let has_file = fields.iter().any(|f| f.field_type == "file");
                
                forms.push(FormElement {
                    selector: format!("form:nth-of-type({})", idx + 1),
                    action,
                    method,
                    fields,
                    has_file_upload: has_file,
                });
            }
        }
        
        forms
    }
    
    fn extract_form_fields(&self, form: &ElementRef) -> Vec<FormField> {
        let mut fields = Vec::new();
        
        // Extract input fields
        if let Ok(selector) = Selector::parse("input, select, textarea") {
            for field in form.select(&selector) {
                let name = field.value().attr("name")
                    .unwrap_or("")
                    .to_string();
                
                let field_type = if field.value().name() == "select" {
                    "select".to_string()
                } else if field.value().name() == "textarea" {
                    "textarea".to_string()
                } else {
                    field.value().attr("type")
                        .unwrap_or("text")
                        .to_string()
                };
                
                let label = self.find_label_for_field(&field);
                let required = field.value().attr("required").is_some();
                let options = self.extract_field_options(&field);
                
                if !name.is_empty() {
                    fields.push(FormField {
                        name,
                        field_type,
                        label,
                        required,
                        options,
                    });
                }
            }
        }
        
        fields
    }
    
    fn find_label_for_field(&self, field: &ElementRef) -> Option<String> {
        // Try to find associated label
        if let Some(id) = field.value().attr("id") {
            let label_selector = format!("label[for='{}']", id);
            if let Ok(selector) = Selector::parse(&label_selector) {
                if let Some(label) = self.document.select(&selector).next() {
                    return Some(label.text().collect::<String>().trim().to_string());
                }
            };
        }
        
        // Try aria-label
        field.value().attr("aria-label")
            .or_else(|| field.value().attr("placeholder"))
            .map(|s| s.to_string())
    }
    
    fn extract_field_options(&self, field: &ElementRef) -> Vec<String> {
        let mut options = Vec::new();
        
        if field.value().name() == "select" {
            if let Ok(selector) = Selector::parse("option") {
                for option in field.select(&selector) {
                    let text = option.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        options.push(text);
                    }
                }
            }
        }
        
        options
    }
    
    pub fn detect_dropdowns(&self) -> Vec<DropdownElement> {
        let mut dropdowns = Vec::new();
        
        // Standard select elements
        if let Ok(selector) = Selector::parse("select") {
            for (idx, select) in self.document.select(&selector).enumerate() {
                let name = select.value().attr("name").map(|s| s.to_string());
                let label = self.find_label_for_field(&select);
                let options = self.extract_field_options(&select);
                let option_count = options.len();
                
                dropdowns.push(DropdownElement {
                    selector: format!("select:nth-of-type({})", idx + 1),
                    name,
                    label,
                    options: options.clone(),
                    option_count,
                });
            }
        }
        
        // Custom dropdowns
        let custom_patterns = vec![
            "[role='combobox']",
            "[role='listbox']",
            ".dropdown",
            ".select-wrapper",
            "[data-toggle='dropdown']",
        ];
        
        for pattern in custom_patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                for element in self.document.select(&selector) {
                    let text = element.text().collect::<String>().trim().to_string();
                    dropdowns.push(DropdownElement {
                        selector: pattern.to_string(),
                        name: None,
                        label: Some(text),
                        options: Vec::new(),
                        option_count: 0,
                    });
                }
            }
        }
        
        dropdowns
    }
    
    pub fn detect_pagination(&self) -> Option<PaginationElement> {
        // Check for numbered pagination
        let page_patterns = vec![
            ".pagination",
            ".pager",
            "[role='navigation']",
            ".page-numbers",
            ".paginate",
        ];
        
        for pattern in page_patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                if let Some(element) = self.document.select(&selector).next() {
                    return Some(self.analyze_pagination(&element));
                }
            }
        }
        
        // Check for infinite scroll indicators
        if self.detect_infinite_scroll() {
            return Some(PaginationElement {
                next_selector: None,
                prev_selector: None,
                page_numbers: Vec::new(),
                total_pages: None,
                current_page: None,
                pagination_type: "infinite_scroll".to_string(),
            });
        }
        
        None
    }
    
    fn analyze_pagination(&self, element: &ElementRef) -> PaginationElement {
        let mut pagination = PaginationElement {
            next_selector: None,
            prev_selector: None,
            page_numbers: Vec::new(),
            total_pages: None,
            current_page: None,
            pagination_type: "numbered".to_string(),
        };
        
        // Find next/prev links
        if let Ok(selector) = Selector::parse("a[rel='next'], .next, a:contains('Next')") {
            if element.select(&selector).next().is_some() {
                pagination.next_selector = Some("a[rel='next'], .next".to_string());
            }
        }
        
        if let Ok(selector) = Selector::parse("a[rel='prev'], .prev, a:contains('Previous')") {
            if element.select(&selector).next().is_some() {
                pagination.prev_selector = Some("a[rel='prev'], .prev".to_string());
            }
        }
        
        // Extract page numbers
        if let Ok(selector) = Selector::parse("a") {
            for link in element.select(&selector) {
                let text = link.text().collect::<String>().trim().to_string();
                if text.parse::<usize>().is_ok() {
                    pagination.page_numbers.push(text);
                }
            }
        }
        
        if !pagination.page_numbers.is_empty() {
            pagination.total_pages = pagination.page_numbers
                .iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .max();
        }
        
        pagination
    }
    
    fn detect_infinite_scroll(&self) -> bool {
        // Check for common infinite scroll indicators
        let indicators = vec![
            "[data-infinite-scroll]",
            ".infinite-scroll",
            ".load-more",
            "[data-load-more]",
        ];
        
        for indicator in indicators {
            if let Ok(selector) = Selector::parse(indicator) {
                if self.document.select(&selector).next().is_some() {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn detect_downloads(&self) -> Vec<DownloadLink> {
        let mut downloads = Vec::new();
        let download_extensions = vec![
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
            "zip", "rar", "7z", "tar", "gz",
            "csv", "txt", "rtf",
            "mp3", "mp4", "avi", "mov",
            "jpg", "jpeg", "png", "gif", "svg",
        ];
        
        if let Ok(selector) = Selector::parse("a[href]") {
            for link in self.document.select(&selector) {
                if let Some(href) = link.value().attr("href") {
                    let lower_href = href.to_lowercase();
                    
                    // Check if link points to downloadable file
                    for ext in &download_extensions {
                        if lower_href.ends_with(&format!(".{}", ext)) {
                            let text = link.text().collect::<String>().trim().to_string();
                            let file_size = link.value().attr("data-size")
                                .map(|s| s.to_string());
                            
                            downloads.push(DownloadLink {
                                url: self.resolve_url(href),
                                text,
                                file_type: ext.to_string(),
                                file_size,
                            });
                            break;
                        }
                    }
                }
            }
        }
        
        downloads
    }
    
    pub fn detect_contacts(&self) -> ContactInfo {
        let mut contacts = ContactInfo {
            emails: Vec::new(),
            phones: Vec::new(),
            addresses: Vec::new(),
            social_links: Vec::new(),
        };
        
        let text = self.document.root_element().text().collect::<String>();
        
        // Extract emails
        let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        for cap in email_regex.captures_iter(&text) {
            if let Some(email) = cap.get(0) {
                contacts.emails.push(email.as_str().to_string());
            }
        }
        
        // Extract phone numbers
        let phone_regex = Regex::new(r"[\+]?[(]?[0-9]{1,4}[)]?[-.\s]?[(]?[0-9]{1,4}[)]?[-.\s]?[0-9]{1,4}[-.\s]?[0-9]{1,9}").unwrap();
        for cap in phone_regex.captures_iter(&text) {
            if let Some(phone) = cap.get(0) {
                let phone_str = phone.as_str();
                if phone_str.len() >= 10 {
                    contacts.phones.push(phone_str.to_string());
                }
            }
        }
        
        // Extract social links
        self.extract_social_links(&mut contacts.social_links);
        
        // Remove duplicates
        contacts.emails.sort();
        contacts.emails.dedup();
        contacts.phones.sort();
        contacts.phones.dedup();
        
        contacts
    }
    
    fn extract_social_links(&self, social_links: &mut Vec<SocialLink>) {
        let social_patterns = vec![
            ("facebook", "facebook.com"),
            ("twitter", "twitter.com"),
            ("linkedin", "linkedin.com"),
            ("instagram", "instagram.com"),
            ("youtube", "youtube.com"),
            ("github", "github.com"),
        ];
        
        if let Ok(selector) = Selector::parse("a[href]") {
            for link in self.document.select(&selector) {
                if let Some(href) = link.value().attr("href") {
                    for (platform, domain) in &social_patterns {
                        if href.contains(domain) {
                            social_links.push(SocialLink {
                                platform: platform.to_string(),
                                url: href.to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
    
    pub fn detect_interactive(&self) -> Vec<InteractiveElement> {
        let mut elements = Vec::new();
        
        // Detect buttons
        if let Ok(selector) = Selector::parse("button, [role='button'], .btn, .button") {
            for button in self.document.select(&selector) {
                let text = button.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    elements.push(InteractiveElement {
                        selector: "button".to_string(),
                        element_type: "button".to_string(),
                        text,
                        action: "click".to_string(),
                    });
                }
            }
        }
        
        // Detect accordions/collapsibles
        let accordion_patterns = vec![
            "[data-toggle='collapse']",
            ".accordion-header",
            ".collapsible",
        ];
        
        for pattern in accordion_patterns {
            if let Ok(selector) = Selector::parse(pattern) {
                for element in self.document.select(&selector) {
                    let text = element.text().collect::<String>().trim().to_string();
                    elements.push(InteractiveElement {
                        selector: pattern.to_string(),
                        element_type: "accordion".to_string(),
                        text,
                        action: "toggle".to_string(),
                    });
                }
            }
        }
        
        // Detect tabs
        if let Ok(selector) = Selector::parse("[role='tab'], .tab, .nav-tab") {
            for tab in self.document.select(&selector) {
                let text = tab.text().collect::<String>().trim().to_string();
                elements.push(InteractiveElement {
                    selector: "[role='tab']".to_string(),
                    element_type: "tab".to_string(),
                    text,
                    action: "click".to_string(),
                });
            }
        }
        
        elements
    }
    
    pub fn detect_media(&self) -> MediaElements {
        let mut media = MediaElements {
            images: Vec::new(),
            videos: Vec::new(),
            audio: Vec::new(),
        };
        
        // Detect images
        if let Ok(selector) = Selector::parse("img") {
            for img in self.document.select(&selector) {
                if let Some(src) = img.value().attr("src") {
                    media.images.push(ImageElement {
                        src: self.resolve_url(src),
                        alt: img.value().attr("alt").map(|s| s.to_string()),
                        width: img.value().attr("width").map(|s| s.to_string()),
                        height: img.value().attr("height").map(|s| s.to_string()),
                    });
                }
            }
        }
        
        // Detect videos
        if let Ok(selector) = Selector::parse("video, iframe[src*='youtube'], iframe[src*='vimeo']") {
            for video in self.document.select(&selector) {
                let src = video.value().attr("src")
                    .or_else(|| video.value().attr("data-src"))
                    .unwrap_or("")
                    .to_string();
                
                media.videos.push(VideoElement {
                    src: self.resolve_url(&src),
                    poster: video.value().attr("poster").map(|s| s.to_string()),
                    duration: video.value().attr("duration").map(|s| s.to_string()),
                });
            }
        }
        
        // Detect audio
        if let Ok(selector) = Selector::parse("audio") {
            for audio in self.document.select(&selector) {
                if let Some(src) = audio.value().attr("src") {
                    media.audio.push(AudioElement {
                        src: self.resolve_url(src),
                        duration: audio.value().attr("duration").map(|s| s.to_string()),
                    });
                }
            }
        }
        
        media
    }
    
    pub fn detect_structured_data(&self) -> Vec<StructuredData> {
        let mut structured = Vec::new();
        
        // Detect JSON-LD
        if let Ok(selector) = Selector::parse("script[type='application/ld+json']") {
            for script in self.document.select(&selector) {
                let content = script.text().collect::<String>();
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let schema_type = json.get("@type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    
                    structured.push(StructuredData {
                        data_type: "json-ld".to_string(),
                        schema_type,
                        content: json,
                    });
                }
            }
        }
        
        // Detect Microdata
        if let Ok(selector) = Selector::parse("[itemscope]") {
            for element in self.document.select(&selector) {
                if let Some(item_type) = element.value().attr("itemtype") {
                    structured.push(StructuredData {
                        data_type: "microdata".to_string(),
                        schema_type: item_type.to_string(),
                        content: serde_json::json!({
                            "itemtype": item_type,
                        }),
                    });
                }
            }
        }
        
        structured
    }
    
    fn resolve_url(&self, url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else if let Some(base) = &self.base_url {
            base.join(url)
                .map(|u| u.to_string())
                .unwrap_or_else(|_| url.to_string())
        } else {
            url.to_string()
        }
    }
    
    pub fn generate_extraction_report(&self) -> String {
        let detected = self.detect_all();
        let mut report = String::new();
        
        report.push_str("=== AUTOMATIC DETECTION REPORT ===\n\n");
        
        // Tables
        if !detected.tables.is_empty() {
            report.push_str(&format!("📊 Tables Found: {}\n", detected.tables.len()));
            for table in &detected.tables {
                report.push_str(&format!("  - {} ({} rows, type: {})\n", 
                    table.selector, table.row_count, table.likely_data_type));
            }
            report.push_str("\n");
        }
        
        // Forms
        if !detected.forms.is_empty() {
            report.push_str(&format!("📝 Forms Found: {}\n", detected.forms.len()));
            for form in &detected.forms {
                report.push_str(&format!("  - {} ({} fields)\n", 
                    form.selector, form.fields.len()));
            }
            report.push_str("\n");
        }
        
        // Dropdowns
        if !detected.dropdowns.is_empty() {
            report.push_str(&format!("📋 Dropdowns Found: {}\n", detected.dropdowns.len()));
            for dropdown in &detected.dropdowns {
                report.push_str(&format!("  - {} ({} options)\n", 
                    dropdown.selector, dropdown.option_count));
            }
            report.push_str("\n");
        }
        
        // Pagination
        if let Some(pagination) = &detected.pagination {
            report.push_str(&format!("📄 Pagination: {}\n", pagination.pagination_type));
            if let Some(total) = pagination.total_pages {
                report.push_str(&format!("  Total pages: {}\n", total));
            }
            report.push_str("\n");
        }
        
        // Downloads
        if !detected.downloads.is_empty() {
            report.push_str(&format!("💾 Downloadable Files: {}\n", detected.downloads.len()));
            for download in &detected.downloads {
                report.push_str(&format!("  - {} ({})\n", download.text, download.file_type));
            }
            report.push_str("\n");
        }
        
        // Contact Info
        if !detected.contacts.emails.is_empty() || !detected.contacts.phones.is_empty() {
            report.push_str("📧 Contact Information:\n");
            if !detected.contacts.emails.is_empty() {
                report.push_str(&format!("  Emails: {}\n", detected.contacts.emails.join(", ")));
            }
            if !detected.contacts.phones.is_empty() {
                report.push_str(&format!("  Phones: {}\n", detected.contacts.phones.join(", ")));
            }
            report.push_str("\n");
        }
        
        // Interactive Elements
        if !detected.interactive.is_empty() {
            report.push_str(&format!("🎯 Interactive Elements: {}\n", detected.interactive.len()));
            let buttons = detected.interactive.iter().filter(|e| e.element_type == "button").count();
            let accordions = detected.interactive.iter().filter(|e| e.element_type == "accordion").count();
            let tabs = detected.interactive.iter().filter(|e| e.element_type == "tab").count();
            
            if buttons > 0 {
                report.push_str(&format!("  Buttons: {}\n", buttons));
            }
            if accordions > 0 {
                report.push_str(&format!("  Accordions: {}\n", accordions));
            }
            if tabs > 0 {
                report.push_str(&format!("  Tabs: {}\n", tabs));
            }
            report.push_str("\n");
        }
        
        // Media
        let total_media = detected.media.images.len() + 
                         detected.media.videos.len() + 
                         detected.media.audio.len();
        if total_media > 0 {
            report.push_str(&format!("🖼️ Media Elements: {}\n", total_media));
            if !detected.media.images.is_empty() {
                report.push_str(&format!("  Images: {}\n", detected.media.images.len()));
            }
            if !detected.media.videos.is_empty() {
                report.push_str(&format!("  Videos: {}\n", detected.media.videos.len()));
            }
            if !detected.media.audio.is_empty() {
                report.push_str(&format!("  Audio: {}\n", detected.media.audio.len()));
            }
            report.push_str("\n");
        }
        
        // Structured Data
        if !detected.structured_data.is_empty() {
            report.push_str(&format!("📋 Structured Data: {}\n", detected.structured_data.len()));
            for data in &detected.structured_data {
                report.push_str(&format!("  - {} ({})\n", data.data_type, data.schema_type));
            }
        }
        
        report
    }
}