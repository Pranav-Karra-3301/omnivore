use scraper::{Html, Selector, Element};
use regex::Regex;
use std::collections::HashSet;
use crate::table_extractor::{TableExtractor, TableData};

#[derive(Debug, Clone)]
pub struct ContentExtractor {
    min_text_length: usize,
    #[allow(dead_code)]
    skip_boilerplate: bool,
}

impl Default for ContentExtractor {
    fn default() -> Self {
        Self {
            min_text_length: 30,
            skip_boilerplate: true,
        }
    }
}

impl ContentExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_clean_content(&self, html: &str) -> CleanedContent {
        let document = Html::parse_document(html);
        
        let title = self.extract_title(&document);
        let description = self.extract_meta_description(&document);
        
        // Extract tables
        let table_extractor = TableExtractor::new();
        let tables = table_extractor.extract_tables(html);
        
        // Check for structured content patterns
        let structured = self.extract_structured_content(&document);
        
        // Extract main content if no structured content found
        let (content, word_count) = if structured.is_some() {
            (None, self.count_words_in_structured(&structured))
        } else {
            let main_text = self.extract_main_content_smart(&document);
            let wc = main_text.split_whitespace().count();
            (Some(main_text), wc)
        };
        
        let links = self.extract_unique_links(&document);
        
        CleanedContent {
            title,
            description,
            content,
            structured,
            tables,
            links,
            word_count,
        }
    }

    fn extract_title(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        document.select(&selector)
            .next()
            .map(|el| self.clean_text(&el.text().collect::<String>()))
    }

    fn extract_meta_description(&self, document: &Html) -> Option<String> {
        let selector = Selector::parse("meta[name=\"description\"]").ok()?;
        document.select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(|s| s.to_string())
    }

    fn extract_main_content_smart(&self, document: &Html) -> String {
        // List of selectors for main content areas
        let content_selectors = vec![
            "main", "article", "[role=\"main\"]", 
            ".main-content", "#main-content", ".content",
            "#content", ".post", ".entry-content",
            ".article-body", ".story-body"
        ];
        
        // List of selectors to skip (boilerplate)
        let skip_selectors = vec![
            "nav", "header", "footer", ".nav", ".menu",
            ".sidebar", ".advertisement", ".ads", ".cookie",
            ".popup", ".modal", ".banner", ".breadcrumb",
            "#comments", ".comments", ".related", ".social",
            ".share", ".newsletter", ".subscription"
        ];
        
        let mut content_parts = Vec::new();
        let mut seen_text = HashSet::new();
        
        // Try to find main content area
        for selector_str in content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = self.extract_text_smart(element, &skip_selectors, &mut seen_text);
                    if !text.is_empty() && text.len() > 100 {
                        content_parts.push(text);
                        break;
                    }
                }
            }
        }
        
        // Fallback to body if no main content found
        if content_parts.is_empty() {
            if let Ok(selector) = Selector::parse("body") {
                if let Some(element) = document.select(&selector).next() {
                    let text = self.extract_text_smart(element, &skip_selectors, &mut seen_text);
                    if !text.is_empty() {
                        content_parts.push(text);
                    }
                }
            }
        }
        
        content_parts.join("\n\n").trim().to_string()
    }
    
    fn extract_text_smart(&self, element: scraper::ElementRef, skip_selectors: &[&str], seen: &mut HashSet<String>) -> String {
        let mut text_parts = Vec::new();
        
        // Check if this element should be skipped
        for skip_sel in skip_selectors {
            if let Ok(selector) = Selector::parse(skip_sel) {
                if element.select(&selector).next().is_some() {
                    return String::new();
                }
            }
        }
        
        // Extract text from paragraphs and headings
        let text_selectors = vec!["p", "h1", "h2", "h3", "h4", "h5", "h6", "li", "td", "blockquote"];
        
        for sel_str in text_selectors {
            if let Ok(selector) = Selector::parse(sel_str) {
                for el in element.select(&selector) {
                    let text = el.text().collect::<String>();
                    let cleaned = self.clean_text(&text);
                    
                    // Skip if too short or already seen
                    if cleaned.len() >= self.min_text_length && !seen.contains(&cleaned) {
                        seen.insert(cleaned.clone());
                        text_parts.push(cleaned);
                    }
                }
            }
        }
        
        text_parts.join(" ")
    }
    
    fn extract_structured_content(&self, document: &Html) -> Option<StructuredContent> {
        let courses = self.extract_courses(document);
        let sections = self.extract_sections(document);
        let lists = self.extract_lists(document);
        let faqs = self.extract_faqs(document);
        
        // Only return structured content if we found something
        if !courses.is_empty() || !sections.is_empty() || !lists.is_empty() || !faqs.is_empty() {
            Some(StructuredContent {
                courses,
                sections,
                lists,
                faqs,
            })
        } else {
            None
        }
    }
    
    fn extract_courses(&self, document: &Html) -> Vec<CourseInfo> {
        let mut courses = Vec::new();
        
        // Pattern 1: Course blocks (like Penn State bulletins)
        if let Ok(selector) = Selector::parse(".courseblock, .course-block, .course") {
            for element in document.select(&selector) {
                if let Some(course) = self.parse_course_block(element) {
                    courses.push(course);
                }
            }
        }
        
        // Pattern 2: Look for course code patterns in headings if no blocks found
        if courses.is_empty() {
            courses = self.extract_courses_from_headings(document);
        }
        
        courses
    }
    
    fn parse_course_block(&self, element: scraper::ElementRef) -> Option<CourseInfo> {
        // Extract course code and title from courseblocktitle
        let title_selector = Selector::parse(".course_codetitle, .courseblocktitle, .course-title").ok()?;
        let title_element = element.select(&title_selector).next()?;
        let title_text = self.clean_text(&title_element.text().collect::<String>());
        
        // Parse course code and title
        let code_pattern = regex::Regex::new(r"^([A-Z]+\s*\d{3}[A-Z]?):?\s*(.*)").ok()?;
        let captures = code_pattern.captures(&title_text)?;
        let code = captures.get(1)?.as_str().trim().to_string();
        let title = captures.get(2)?.as_str().trim().to_string();
        
        // Extract credits
        let mut credits = None;
        if let Ok(credit_selector) = Selector::parse(".course_credits, .credits") {
            if let Some(credit_el) = element.select(&credit_selector).next() {
                credits = Some(self.clean_text(&credit_el.text().collect::<String>()));
            }
        }
        
        // Extract description from courseblockmeta
        let mut description = String::new();
        if let Ok(desc_selector) = Selector::parse(".courseblockdesc, .course-description, .description") {
            if let Some(desc_el) = element.select(&desc_selector).next() {
                description = self.clean_text(&desc_el.text().collect::<String>());
            }
        }
        
        // Extract prerequisites
        let mut prerequisites = Vec::new();
        if let Ok(prereq_selector) = Selector::parse(".courseblockextra, .prerequisites") {
            for prereq_el in element.select(&prereq_selector) {
                let text = prereq_el.text().collect::<String>();
                if text.to_lowercase().contains("prerequisite") {
                    prerequisites.extend(self.extract_prerequisites(&text));
                }
            }
        }
        
        if !description.is_empty() || !title.is_empty() {
            Some(CourseInfo {
                code,
                title,
                credits,
                description,
                prerequisites,
            })
        } else {
            None
        }
    }
    
    #[allow(dead_code)]
    fn parse_course_element(&self, element: scraper::ElementRef) -> Option<CourseInfo> {
        let text = element.text().collect::<String>();
        let code_pattern = regex::Regex::new(r"([A-Z]+\s*\d{3}[A-Z]?)").ok()?;
        
        if let Some(capture) = code_pattern.find(&text) {
            let code = capture.as_str().to_string();
            
            // Try to extract title after the code
            let title_text = text.split(&code).nth(1)?;
            let title = title_text.split('\n').next()?.trim().to_string();
            
            // Extract description
            let description = text.split('\n')
                .skip(1)
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            
            Some(CourseInfo {
                code,
                title,
                credits: self.extract_credits(&text),
                description,
                prerequisites: self.extract_prerequisites(&text),
            })
        } else {
            None
        }
    }
    
    fn extract_courses_from_headings(&self, document: &Html) -> Vec<CourseInfo> {
        let mut courses = Vec::new();
        let code_pattern = regex::Regex::new(r"^([A-Z]+\s*\d{3}[A-Z]?)\s*(.*)").unwrap();
        
        for level in 2..=5 {
            let selector_str = format!("h{}", level);
            if let Ok(selector) = Selector::parse(&selector_str) {
                for heading in document.select(&selector) {
                    let heading_text = heading.text().collect::<String>();
                    
                    if let Some(captures) = code_pattern.captures(&heading_text) {
                        let code = captures.get(1).map_or("", |m| m.as_str()).trim().to_string();
                        let title = captures.get(2).map_or("", |m| m.as_str()).trim().to_string();
                        
                        // Get the next sibling content as description
                        let mut description = String::new();
                        let mut current = heading;
                        
                        // Look for next few siblings for description
                        for _ in 0..5 {
                            if let Some(sibling) = current.next_sibling_element() {
                                let tag = sibling.value().name();
                                if tag == "p" || tag == "div" {
                                    description.push_str(&sibling.text().collect::<String>());
                                    description.push(' ');
                                } else if tag.starts_with('h') {
                                    break; // Stop at next heading
                                }
                                current = sibling;
                            } else {
                                break;
                            }
                        }
                        
                        if !description.is_empty() {
                            courses.push(CourseInfo {
                                code,
                                title,
                                credits: self.extract_credits(&heading_text),
                                description: self.clean_text(&description),
                                prerequisites: self.extract_prerequisites(&description),
                            });
                        }
                    }
                }
            };
        }
        
        courses
    }
    
    #[allow(dead_code)]
    fn parse_course_dl(&self, _dl: scraper::ElementRef) -> Vec<CourseInfo> {
        let courses = Vec::new();
        // TODO: Implementation for definition list parsing
        courses
    }
    
    fn extract_credits(&self, text: &str) -> Option<String> {
        let credit_pattern = regex::Regex::new(r"\((\d+(?:-\d+)?)\s*(?:credits?|cr\.?|units?)\)").ok()?;
        credit_pattern.find(text).map(|m| m.as_str().to_string())
    }
    
    fn extract_prerequisites(&self, text: &str) -> Vec<String> {
        let mut prereqs = Vec::new();
        let prereq_pattern = regex::Regex::new(r"(?i)prerequisite[s]?:\s*([^.]+)").unwrap();
        
        if let Some(captures) = prereq_pattern.captures(text) {
            let prereq_text = captures.get(1).map_or("", |m| m.as_str());
            // Split by common delimiters
            for part in prereq_text.split(&[',', ';', '|'][..]) {
                let cleaned = self.clean_text(part);
                if !cleaned.is_empty() {
                    prereqs.push(cleaned);
                }
            }
        }
        
        prereqs
    }
    
    fn extract_sections(&self, document: &Html) -> Vec<ContentSection> {
        let mut sections = Vec::new();
        
        // Look for article, section tags with headings
        if let Ok(selector) = Selector::parse("article, section, .section, .content-section") {
            for element in document.select(&selector) {
                if let Some(section) = self.parse_section_element(element) {
                    sections.push(section);
                }
            }
        }
        
        sections
    }
    
    fn parse_section_element(&self, element: scraper::ElementRef) -> Option<ContentSection> {
        // Find heading in section
        let heading_selector = Selector::parse("h1, h2, h3, h4, h5, h6").ok()?;
        let heading = element.select(&heading_selector).next()?;
        let heading_text = self.clean_text(&heading.text().collect::<String>());
        
        // Get content excluding the heading
        let mut content_parts = Vec::new();
        if let Ok(p_selector) = Selector::parse("p") {
            for p in element.select(&p_selector) {
                let text = self.clean_text(&p.text().collect::<String>());
                if !text.is_empty() && text.len() > self.min_text_length {
                    content_parts.push(text);
                }
            }
        }
        
        if !content_parts.is_empty() {
            Some(ContentSection {
                heading: heading_text,
                content: content_parts.join(" "),
                subsections: Vec::new(),
            })
        } else {
            None
        }
    }
    
    fn extract_lists(&self, document: &Html) -> Vec<ListContent> {
        let mut lists = Vec::new();
        
        // Extract ul and ol lists, but filter out navigation
        if let Ok(selector) = Selector::parse("ul, ol") {
            for list in document.select(&selector) {
                // Skip navigation lists
                let parent_html = list.html();
                if parent_html.contains("nav") || parent_html.contains("menu") || 
                   parent_html.contains("sidebar") || parent_html.contains("breadcrumb") {
                    continue;
                }
                
                // Check if list is inside main content area
                let is_in_content = self.is_in_content_area(list);
                if !is_in_content {
                    continue;
                }
                
                let mut items = Vec::new();
                
                if let Ok(li_selector) = Selector::parse("li") {
                    for li in list.select(&li_selector) {
                        let text = self.clean_text(&li.text().collect::<String>());
                        // Filter out short navigation-like items
                        if !text.is_empty() && text.len() > 10 && !text.contains("©") {
                            items.push(text);
                        }
                    }
                }
                
                if items.len() > 2 && items.len() < 50 {  // Filter out huge navigation lists
                    let title = None;
                    lists.push(ListContent { title, items });
                }
            }
        }
        
        lists
    }
    
    fn is_in_content_area(&self, element: scraper::ElementRef) -> bool {
        // Check if element is within a content area
        let content_selectors = vec![
            "main", "article", "[role='main']", 
            ".content", "#content", ".main-content"
        ];
        
        // Walk up the DOM tree to check for content containers
        let current = element;
        for _ in 0..10 {  // Check up to 10 levels
            for selector_str in &content_selectors {
                if let Ok(selector) = Selector::parse(selector_str) {
                    if current.select(&selector).next().is_some() {
                        return true;
                    }
                }
            }
            
            // Try to get parent - this is a simplified check
            // In real implementation, we'd need proper parent traversal
            break;
        }
        
        false
    }
    
    fn extract_faqs(&self, document: &Html) -> Vec<FAQItem> {
        let mut faqs = Vec::new();
        
        // Look for FAQ patterns
        // Pattern 1: FAQ in dl/dt/dd format
        if let Ok(selector) = Selector::parse("dl.faq, dl.faqs, .faq-list dl") {
            for dl in document.select(&selector) {
                faqs.extend(self.parse_faq_dl(dl));
            }
        }
        
        // Pattern 2: Accordion/details elements
        if let Ok(selector) = Selector::parse("details, .accordion-item, .faq-item") {
            for item in document.select(&selector) {
                if let Some(faq) = self.parse_faq_item(item) {
                    faqs.push(faq);
                }
            }
        }
        
        faqs
    }
    
    fn parse_faq_dl(&self, dl: scraper::ElementRef) -> Vec<FAQItem> {
        let mut faqs = Vec::new();
        
        if let (Ok(dt_sel), Ok(dd_sel)) = (Selector::parse("dt"), Selector::parse("dd")) {
            let questions: Vec<_> = dl.select(&dt_sel).collect();
            let answers: Vec<_> = dl.select(&dd_sel).collect();
            
            for (q, a) in questions.iter().zip(answers.iter()) {
                let question = self.clean_text(&q.text().collect::<String>());
                let answer = self.clean_text(&a.text().collect::<String>());
                
                if !question.is_empty() && !answer.is_empty() {
                    faqs.push(FAQItem { question, answer });
                }
            }
        }
        
        faqs
    }
    
    fn parse_faq_item(&self, element: scraper::ElementRef) -> Option<FAQItem> {
        // For details/summary pattern
        if element.value().name() == "details" {
            if let Ok(summary_sel) = Selector::parse("summary") {
                if let Some(summary) = element.select(&summary_sel).next() {
                    let question = self.clean_text(&summary.text().collect::<String>());
                    
                    // Get answer from remaining content
                    let mut answer_parts = Vec::new();
                    for child in element.children() {
                        if let Some(el) = child.value().as_element() {
                            if el.name() != "summary" {
                                if let Some(text_el) = child.value().as_text() {
                                    answer_parts.push(text_el.to_string());
                                }
                            }
                        }
                    }
                    
                    let answer = self.clean_text(&answer_parts.join(" "));
                    if !question.is_empty() && !answer.is_empty() {
                        return Some(FAQItem { question, answer });
                    }
                }
            }
        }
        
        None
    }
    
    fn count_words_in_structured(&self, structured: &Option<StructuredContent>) -> usize {
        if let Some(s) = structured {
            let mut count = 0;
            
            for course in &s.courses {
                count += course.title.split_whitespace().count();
                count += course.description.split_whitespace().count();
            }
            
            for section in &s.sections {
                count += section.heading.split_whitespace().count();
                count += section.content.split_whitespace().count();
            }
            
            for list in &s.lists {
                for item in &list.items {
                    count += item.split_whitespace().count();
                }
            }
            
            for faq in &s.faqs {
                count += faq.question.split_whitespace().count();
                count += faq.answer.split_whitespace().count();
            }
            
            count
        } else {
            0
        }
    }
    
    fn extract_unique_links(&self, document: &Html) -> Vec<String> {
        let mut unique_links = HashSet::new();
        
        if let Ok(selector) = Selector::parse("a[href]") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    // Skip internal anchors and javascript
                    if !href.starts_with('#') && !href.starts_with("javascript:") {
                        // Only include http/https links
                        if href.starts_with("http://") || href.starts_with("https://") || href.starts_with("/") {
                            unique_links.insert(href.to_string());
                        }
                    }
                }
            }
        }
        
        let mut links: Vec<String> = unique_links.into_iter().collect();
        links.sort();
        links.truncate(50); // Limit to 50 most relevant links
        links
    }


    fn clean_text(&self, text: &str) -> String {
        // Remove excessive whitespace and normalize
        let re = Regex::new(r"\s+").unwrap();
        let cleaned = re.replace_all(text.trim(), " ");
        cleaned.to_string()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CleanedContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured: Option<StructuredContent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<TableData>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    pub word_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructuredContent {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub courses: Vec<CourseInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<ContentSection>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lists: Vec<ListContent>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub faqs: Vec<FAQItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CourseInfo {
    pub code: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentSection {
    pub heading: String,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subsections: Vec<ContentSection>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListContent {
    pub title: Option<String>,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FAQItem {
    pub question: String,
    pub answer: String,
}

