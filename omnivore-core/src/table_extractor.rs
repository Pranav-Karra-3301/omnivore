use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub footnotes: Vec<String>,
}

impl TableData {
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        
        // Add title as comment if present
        if let Some(ref title) = self.title {
            csv.push_str(&format!("# {}\n", title));
        }
        
        // Add headers
        if !self.headers.is_empty() {
            csv.push_str(&self.headers.join(","));
            csv.push('\n');
        }
        
        // Add rows
        for row in &self.rows {
            let escaped_row: Vec<String> = row.iter().map(|cell| {
                if cell.contains(',') || cell.contains('"') || cell.contains('\n') {
                    format!("\"{}\"", cell.replace('"', "\"\""))
                } else {
                    cell.clone()
                }
            }).collect();
            csv.push_str(&escaped_row.join(","));
            csv.push('\n');
        }
        
        // Add footnotes as comments
        if !self.footnotes.is_empty() {
            csv.push_str("\n# Footnotes:\n");
            for footnote in &self.footnotes {
                csv.push_str(&format!("# {}\n", footnote));
            }
        }
        
        csv
    }
}

pub struct TableExtractor {
    min_rows: usize,
    extract_footnotes: bool,
}

impl TableExtractor {
    pub fn new() -> Self {
        Self {
            min_rows: 1,
            extract_footnotes: true,
        }
    }
    
    pub fn extract_tables(&self, html: &str) -> Vec<TableData> {
        let document = Html::parse_document(html);
        let mut tables = Vec::new();
        
        // Find all table elements
        let table_selector = Selector::parse("table").unwrap();
        
        for (idx, table_element) in document.select(&table_selector).enumerate() {
            if let Some(table_data) = self.parse_table(table_element, idx) {
                if table_data.rows.len() >= self.min_rows {
                    tables.push(table_data);
                }
            }
        }
        
        tables
    }
    
    fn parse_table(&self, table: ElementRef, table_idx: usize) -> Option<TableData> {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut caption = None;
        let mut title = None;
        let mut footnotes = Vec::new();
        
        // Extract caption
        if let Ok(caption_selector) = Selector::parse("caption") {
            if let Some(caption_element) = table.select(&caption_selector).next() {
                caption = Some(self.clean_text(&caption_element.text().collect::<String>()));
            }
        }
        
        // Look for title in previous sibling or parent
        title = self.find_table_title(table).or(caption.clone());
        
        // Extract headers from thead or first row with th elements
        if let Ok(thead_selector) = Selector::parse("thead") {
            if let Some(thead) = table.select(&thead_selector).next() {
                headers = self.extract_headers_from_element(thead);
            }
        }
        
        // If no thead, look for th elements in first row
        if headers.is_empty() {
            if let Ok(tr_selector) = Selector::parse("tr") {
                if let Some(first_row) = table.select(&tr_selector).next() {
                    let th_selector = Selector::parse("th").unwrap();
                    let th_count = first_row.select(&th_selector).count();
                    
                    if th_count > 0 {
                        headers = self.extract_headers_from_element(first_row);
                    }
                }
            }
        }
        
        // Extract data rows
        let tbody_selector = Selector::parse("tbody").unwrap_or_else(|_| Selector::parse("*").unwrap());
        let tbody = table.select(&tbody_selector).next().unwrap_or(table);
        
        let tr_selector = Selector::parse("tr").unwrap();
        for row_element in tbody.select(&tr_selector) {
            let row = self.extract_row(row_element);
            
            // Skip if this looks like a header row we already processed
            if !headers.is_empty() && row == headers {
                continue;
            }
            
            // Skip empty rows
            if !row.is_empty() && row.iter().any(|cell| !cell.trim().is_empty()) {
                rows.push(row);
            }
        }
        
        // If no explicit headers found but we have rows, use first row as headers
        if headers.is_empty() && !rows.is_empty() {
            // Check if first row looks like headers (no numbers, common header words)
            if self.looks_like_header(&rows[0]) {
                headers = rows.remove(0);
            }
        }
        
        // Extract footnotes
        if self.extract_footnotes {
            footnotes = self.extract_table_footnotes(table);
        }
        
        // Generate default title if none found
        if title.is_none() && (!headers.is_empty() || !rows.is_empty()) {
            title = Some(format!("Table {}", table_idx + 1));
        }
        
        if !rows.is_empty() || !headers.is_empty() {
            Some(TableData {
                headers,
                rows,
                title,
                caption,
                footnotes,
            })
        } else {
            None
        }
    }
    
    fn extract_headers_from_element(&self, element: ElementRef) -> Vec<String> {
        let mut headers = Vec::new();
        
        // Try th elements first
        let th_selector = Selector::parse("th").unwrap();
        for th in element.select(&th_selector) {
            headers.push(self.clean_text(&th.text().collect::<String>()));
        }
        
        // If no th elements, try td elements
        if headers.is_empty() {
            let td_selector = Selector::parse("td").unwrap();
            for td in element.select(&td_selector) {
                headers.push(self.clean_text(&td.text().collect::<String>()));
            }
        }
        
        headers
    }
    
    fn extract_row(&self, row: ElementRef) -> Vec<String> {
        let mut cells = Vec::new();
        
        // Extract both th and td elements (some tables use th in data rows)
        let cell_selector = Selector::parse("th, td").unwrap();
        
        for cell in row.select(&cell_selector) {
            let text = self.clean_text(&cell.text().collect::<String>());
            
            // Handle colspan by duplicating the value
            let colspan = cell.value().attr("colspan")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(1);
            
            for _ in 0..colspan {
                cells.push(text.clone());
            }
        }
        
        cells
    }
    
    fn find_table_title(&self, table: ElementRef) -> Option<String> {
        // Look for common title patterns near the table
        
        // Check for id or class attributes that might indicate the table's purpose
        if let Some(id) = table.value().attr("id") {
            if !id.is_empty() {
                return Some(self.humanize_identifier(id));
            }
        }
        
        if let Some(class) = table.value().attr("class") {
            if class.contains("admissions") || class.contains("scores") || class.contains("demographics") {
                return Some(self.humanize_identifier(class));
            }
        }
        
        // Look for heading above the table (simplified approach)
        // In a real implementation, we'd traverse the DOM properly
        None
    }
    
    fn extract_table_footnotes(&self, table: ElementRef) -> Vec<String> {
        let mut footnotes = Vec::new();
        
        // Look for common footnote patterns
        let footnote_selectors = vec![
            ".footnote",
            ".table-footnote",
            "tfoot",
            "tr.footnote",
            "td[colspan]",
        ];
        
        for selector_str in footnote_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in table.select(&selector) {
                    let text = self.clean_text(&element.text().collect::<String>());
                    
                    // Check if it looks like a footnote (starts with *, †, ‡, §, ¶, #, or number)
                    if text.starts_with('*') || text.starts_with('†') || text.starts_with('‡') 
                        || text.starts_with('§') || text.starts_with('¶') || text.starts_with('#')
                        || text.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        
                        if !text.is_empty() && text.len() > 5 { // Minimum footnote length
                            footnotes.push(text);
                        }
                    }
                }
            }
        }
        
        // Also check for cells that span all columns (often used for footnotes)
        let tr_selector = Selector::parse("tr").unwrap();
        for row in table.select(&tr_selector) {
            let td_selector = Selector::parse("td").unwrap();
            let cells: Vec<_> = row.select(&td_selector).collect();
            
            if cells.len() == 1 {
                if let Some(cell) = cells.first() {
                    if let Some(colspan) = cell.value().attr("colspan") {
                        if colspan.parse::<usize>().unwrap_or(1) > 3 {
                            let text = self.clean_text(&cell.text().collect::<String>());
                            if text.len() > 10 && !self.looks_like_header(&[text.clone()]) {
                                footnotes.push(text);
                            }
                        }
                    }
                }
            }
        }
        
        footnotes
    }
    
    fn looks_like_header(&self, row: &[String]) -> bool {
        // Check if row looks like headers
        for cell in row {
            let lower = cell.to_lowercase();
            
            // Common header keywords
            if lower.contains("year") || lower.contains("total") || lower.contains("count")
                || lower.contains("name") || lower.contains("date") || lower.contains("score")
                || lower.contains("gpa") || lower.contains("average") || lower.contains("median")
                || lower.contains("applications") || lower.contains("accepts") || lower.contains("offers") {
                return true;
            }
            
            // If it's all numbers, probably not a header
            if cell.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ',' || c == '%') {
                return false;
            }
        }
        
        // If most cells are short and capitalized, probably headers
        let short_caps = row.iter().filter(|cell| {
            cell.len() < 20 && cell.chars().next().map_or(false, |c| c.is_uppercase())
        }).count();
        
        short_caps > row.len() / 2
    }
    
    fn clean_text(&self, text: &str) -> String {
        text.trim()
            .replace('\n', " ")
            .replace('\t', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn humanize_identifier(&self, identifier: &str) -> String {
        identifier
            .replace('_', " ")
            .replace('-', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}