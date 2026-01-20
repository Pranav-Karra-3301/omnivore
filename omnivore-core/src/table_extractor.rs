//! Table extraction from HTML documents.
//!
//! This module provides functionality to extract structured table data from HTML,
//! handling complex table features like rowspan, colspan, and layout table detection.

use crate::patterns;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extracted table data with headers, rows, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub footnotes: Vec<String>,
}

impl TableData {
    /// Convert the table to CSV format.
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        // Add title as comment if present
        if let Some(ref title) = self.title {
            csv.push_str(&format!("# {title}\n"));
        }

        // Add headers
        if !self.headers.is_empty() {
            csv.push_str(&self.headers.join(","));
            csv.push('\n');
        }

        // Add rows
        for row in &self.rows {
            let escaped_row: Vec<String> = row
                .iter()
                .map(|cell| {
                    if cell.contains(',') || cell.contains('"') || cell.contains('\n') {
                        format!("\"{}\"", cell.replace('"', "\"\""))
                    } else {
                        cell.clone()
                    }
                })
                .collect();
            csv.push_str(&escaped_row.join(","));
            csv.push('\n');
        }

        // Add footnotes as comments
        if !self.footnotes.is_empty() {
            csv.push_str("\n# Footnotes:\n");
            for footnote in &self.footnotes {
                csv.push_str(&format!("# {footnote}\n"));
            }
        }

        csv
    }
}

/// Configuration for table extraction.
pub struct TableExtractor {
    /// Minimum number of rows to include a table
    min_rows: usize,
    /// Whether to extract footnotes
    extract_footnotes: bool,
    /// Whether to skip layout tables
    skip_layout_tables: bool,
}

impl Default for TableExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl TableExtractor {
    pub fn new() -> Self {
        Self {
            min_rows: 1,
            extract_footnotes: true,
            skip_layout_tables: true,
        }
    }

    /// Create a new extractor that includes layout tables.
    pub fn with_layout_tables() -> Self {
        Self {
            min_rows: 1,
            extract_footnotes: true,
            skip_layout_tables: false,
        }
    }

    /// Set minimum rows required for extraction.
    pub fn min_rows(mut self, min: usize) -> Self {
        self.min_rows = min;
        self
    }

    /// Extract all tables from HTML.
    pub fn extract_tables(&self, html: &str) -> Vec<TableData> {
        let document = Html::parse_document(html);
        let mut tables = Vec::new();

        for (idx, table_element) in document.select(&patterns::TABLE).enumerate() {
            // Skip layout tables if configured
            if self.skip_layout_tables && self.is_layout_table(table_element) {
                continue;
            }

            if let Some(table_data) = self.parse_table(table_element, idx) {
                if table_data.rows.len() >= self.min_rows {
                    tables.push(table_data);
                }
            }
        }

        tables
    }

    /// Detect if a table is used for layout rather than data.
    fn is_layout_table(&self, table: ElementRef) -> bool {
        // Check for layout-indicating attributes
        if let Some(role) = table.value().attr("role") {
            if role == "presentation" || role == "none" {
                return true;
            }
        }

        // Check for layout-indicating class names
        if let Some(class) = table.value().attr("class") {
            let lower_class = class.to_lowercase();
            if lower_class.contains("layout")
                || lower_class.contains("container")
                || lower_class.contains("wrapper")
                || lower_class.contains("navigation")
            {
                return true;
            }
        }

        // Count rows and check structure
        let rows: Vec<_> = table.select(&patterns::TR).collect();

        // Very few rows or inconsistent structure suggests layout table
        if rows.is_empty() {
            return true;
        }

        // Check for single-cell rows (common in layout tables)
        let single_cell_rows = rows
            .iter()
            .filter(|row| {
                let cell_count = row.select(&patterns::TD).count() + row.select(&patterns::TH).count();
                cell_count == 1
            })
            .count();

        // If most rows have only one cell, likely a layout table
        if single_cell_rows > rows.len() / 2 && rows.len() > 2 {
            return true;
        }

        // Check for nested tables (common in layout tables)
        let nested_tables = table.select(&patterns::TABLE).count();
        if nested_tables > 1 {
            return true;
        }

        // Check if table contains mostly non-text content (forms, images)
        let text_content = table.text().collect::<String>();
        let text_len = text_content.trim().len();
        let form_count = table
            .select(&patterns::FORM)
            .count();
        let img_count = table.select(&patterns::IMG).count();

        // If there are many forms/images relative to text, likely layout
        if text_len < 100 && (form_count > 0 || img_count > 3) {
            return true;
        }

        false
    }

    fn parse_table(&self, table: ElementRef, table_idx: usize) -> Option<TableData> {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut caption = None;
        let mut title = None;
        let mut footnotes = Vec::new();

        // Extract caption using shared pattern
        if let Some(caption_element) = table.select(&patterns::CAPTION).next() {
            caption = Some(self.clean_text(&caption_element.text().collect::<String>()));
        }

        // Look for title in previous sibling or parent
        let _ = self.find_table_title(table).or(caption.clone());

        // Count total columns by examining first few rows
        let num_columns = self.count_columns(table);

        // Track rowspan state: column_index -> (remaining_rows, value)
        let mut rowspan_tracker: HashMap<usize, (usize, String)> = HashMap::new();

        // Extract headers from thead or first row with th elements
        if let Some(thead) = table.select(&patterns::THEAD).next() {
            headers = self.extract_headers_from_element(thead);
        }

        // If no thead, look for th elements in first row
        if headers.is_empty() {
            if let Some(first_row) = table.select(&patterns::TR).next() {
                let th_count = first_row.select(&patterns::TH).count();

                if th_count > 0 {
                    headers = self.extract_headers_from_element(first_row);
                }
            }
        }

        // Extract data rows - try tbody first, fallback to table itself
        let tbody_element = table.select(&patterns::TBODY).next().unwrap_or(table);

        for row_element in tbody_element.select(&patterns::TR) {
            let row = self.extract_row_with_rowspan(row_element, num_columns, &mut rowspan_tracker);

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

    /// Count the expected number of columns in the table.
    fn count_columns(&self, table: ElementRef) -> usize {
        let mut max_cols = 0;

        for row in table.select(&patterns::TR) {
            let mut col_count = 0;

            // Count th cells
            for th in row.select(&patterns::TH) {
                let colspan = th
                    .value()
                    .attr("colspan")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(1);
                col_count += colspan;
            }

            // Count td cells
            for td in row.select(&patterns::TD) {
                let colspan = td
                    .value()
                    .attr("colspan")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(1);
                col_count += colspan;
            }

            max_cols = max_cols.max(col_count);
        }

        max_cols
    }

    fn extract_headers_from_element(&self, element: ElementRef) -> Vec<String> {
        let mut headers = Vec::new();

        // Try th elements first using shared pattern
        for th in element.select(&patterns::TH) {
            let text = self.clean_text(&th.text().collect::<String>());
            let colspan = th
                .value()
                .attr("colspan")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(1);
            for _ in 0..colspan {
                headers.push(text.clone());
            }
        }

        // If no th elements, try td elements using shared pattern
        if headers.is_empty() {
            for td in element.select(&patterns::TD) {
                let text = self.clean_text(&td.text().collect::<String>());
                let colspan = td
                    .value()
                    .attr("colspan")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(1);
                for _ in 0..colspan {
                    headers.push(text.clone());
                }
            }
        }

        headers
    }

    /// Extract a row with rowspan support.
    fn extract_row_with_rowspan(
        &self,
        row: ElementRef,
        num_columns: usize,
        rowspan_tracker: &mut HashMap<usize, (usize, String)>,
    ) -> Vec<String> {
        let mut cells = Vec::with_capacity(num_columns);
        let mut col_idx = 0;

        // Get all cells in this row
        let cell_selector = patterns::parse_selector("th, td");
        let row_cells: Vec<_> = if let Some(ref selector) = cell_selector {
            row.select(selector).collect()
        } else {
            // Fallback: collect th and td separately
            let mut all_cells: Vec<_> = row.select(&patterns::TH).collect();
            all_cells.extend(row.select(&patterns::TD));
            all_cells
        };

        let mut cell_iter = row_cells.iter();

        while col_idx < num_columns {
            // Check if there's a rowspan value from a previous row
            if let Some((remaining, value)) = rowspan_tracker.get_mut(&col_idx) {
                cells.push(value.clone());
                *remaining -= 1;
                if *remaining == 0 {
                    rowspan_tracker.remove(&col_idx);
                }
                col_idx += 1;
                continue;
            }

            // Get next cell from this row
            if let Some(cell) = cell_iter.next() {
                let text = self.clean_text(&cell.text().collect::<String>());

                let colspan = cell
                    .value()
                    .attr("colspan")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(1);

                let rowspan = cell
                    .value()
                    .attr("rowspan")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(1);

                // Handle colspan
                for i in 0..colspan {
                    if col_idx + i < num_columns {
                        cells.push(text.clone());

                        // Track rowspan for subsequent rows
                        if rowspan > 1 {
                            rowspan_tracker.insert(col_idx + i, (rowspan - 1, text.clone()));
                        }
                    }
                }

                col_idx += colspan;
            } else {
                // No more cells, fill with empty
                cells.push(String::new());
                col_idx += 1;
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
            if class.contains("admissions")
                || class.contains("scores")
                || class.contains("demographics")
            {
                return Some(self.humanize_identifier(class));
            }
        }

        // Look for heading above the table (simplified approach)
        // In a real implementation, we'd traverse the DOM properly
        None
    }

    fn extract_table_footnotes(&self, table: ElementRef) -> Vec<String> {
        let mut footnotes = Vec::new();

        // Look for common footnote patterns using safe selector parsing
        let footnote_selectors = [
            ".footnote",
            ".table-footnote",
            "tfoot",
            "tr.footnote",
            "td[colspan]",
        ];

        for selector_str in footnote_selectors {
            if let Some(selector) = patterns::parse_selector(selector_str) {
                for element in table.select(&selector) {
                    let text = self.clean_text(&element.text().collect::<String>());

                    // Check if it looks like a footnote (starts with *, †, ‡, §, ¶, #, or number)
                    if is_footnote_text(&text) && text.len() > 5 {
                        footnotes.push(text);
                    }
                }
            }
        }

        // Also check for cells that span all columns (often used for footnotes)
        for row in table.select(&patterns::TR) {
            let cells: Vec<_> = row.select(&patterns::TD).collect();

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
            if lower.contains("year")
                || lower.contains("total")
                || lower.contains("count")
                || lower.contains("name")
                || lower.contains("date")
                || lower.contains("score")
                || lower.contains("gpa")
                || lower.contains("average")
                || lower.contains("median")
                || lower.contains("applications")
                || lower.contains("accepts")
                || lower.contains("offers")
            {
                return true;
            }

            // If it's all numbers, probably not a header
            if cell
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == ',' || c == '%')
            {
                return false;
            }
        }

        // If most cells are short and capitalized, probably headers
        let short_caps = row
            .iter()
            .filter(|cell| {
                cell.len() < 20 && cell.chars().next().is_some_and(|c| c.is_uppercase())
            })
            .count();

        short_caps > row.len() / 2
    }

    fn clean_text(&self, text: &str) -> String {
        text.trim()
            .replace(['\n', '\t'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn humanize_identifier(&self, identifier: &str) -> String {
        identifier
            .replace(['_', '-'], " ")
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

/// Check if text looks like a footnote.
fn is_footnote_text(text: &str) -> bool {
    let first_char = text.chars().next();
    matches!(first_char, Some('*' | '†' | '‡' | '§' | '¶' | '#'))
        || first_char.is_some_and(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_table() {
        let html = r#"
        <table>
            <thead><tr><th>Name</th><th>Age</th></tr></thead>
            <tbody>
                <tr><td>Alice</td><td>30</td></tr>
                <tr><td>Bob</td><td>25</td></tr>
            </tbody>
        </table>
        "#;

        let extractor = TableExtractor::new();
        let tables = extractor.extract_tables(html);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].headers, vec!["Name", "Age"]);
        assert_eq!(tables[0].rows.len(), 2);
    }

    #[test]
    fn test_extract_table_with_caption() {
        let html = r#"
        <table>
            <caption>Student Data</caption>
            <tr><th>ID</th><th>Score</th></tr>
            <tr><td>1</td><td>95</td></tr>
        </table>
        "#;

        let extractor = TableExtractor::new();
        let tables = extractor.extract_tables(html);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].caption, Some("Student Data".to_string()));
    }

    #[test]
    fn test_extract_table_with_rowspan() {
        let html = r#"
        <table>
            <tr><th>Category</th><th>Item</th><th>Value</th></tr>
            <tr><td rowspan="2">Fruit</td><td>Apple</td><td>1</td></tr>
            <tr><td>Banana</td><td>2</td></tr>
            <tr><td>Vegetable</td><td>Carrot</td><td>3</td></tr>
        </table>
        "#;

        let extractor = TableExtractor::new();
        let tables = extractor.extract_tables(html);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].rows.len(), 3);
        // Second row should have "Fruit" from rowspan
        assert_eq!(tables[0].rows[1][0], "Fruit");
    }

    #[test]
    fn test_extract_table_with_colspan() {
        let html = r#"
        <table>
            <tr><th colspan="2">Name</th><th>Age</th></tr>
            <tr><td>First</td><td>Last</td><td>30</td></tr>
        </table>
        "#;

        let extractor = TableExtractor::new();
        let tables = extractor.extract_tables(html);

        assert_eq!(tables.len(), 1);
        // Header should have "Name" twice due to colspan
        assert_eq!(tables[0].headers, vec!["Name", "Name", "Age"]);
    }

    #[test]
    fn test_skip_layout_table() {
        let html = r#"
        <table role="presentation">
            <tr><td>Layout content</td></tr>
        </table>
        <table>
            <tr><th>Real</th><th>Table</th></tr>
            <tr><td>Data</td><td>Here</td></tr>
        </table>
        "#;

        let extractor = TableExtractor::new();
        let tables = extractor.extract_tables(html);

        // Should only extract the data table, not the layout table
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].headers, vec!["Real", "Table"]);
    }

    #[test]
    fn test_include_layout_table() {
        let html = r#"
        <table role="presentation">
            <tr><td>Header</td></tr>
            <tr><td>Layout content</td></tr>
        </table>
        "#;

        let extractor = TableExtractor::with_layout_tables();
        let tables = extractor.extract_tables(html);

        // Should extract layout tables when configured (min_rows = 1)
        assert_eq!(tables.len(), 1);
    }

    #[test]
    fn test_to_csv() {
        let table = TableData {
            headers: vec!["Name".to_string(), "Value".to_string()],
            rows: vec![vec!["Test".to_string(), "123".to_string()]],
            title: Some("Test Table".to_string()),
            caption: None,
            footnotes: vec![],
        };

        let csv = table.to_csv();
        assert!(csv.contains("# Test Table"));
        assert!(csv.contains("Name,Value"));
        assert!(csv.contains("Test,123"));
    }

    #[test]
    fn test_csv_escaping() {
        let table = TableData {
            headers: vec!["Field".to_string()],
            rows: vec![vec!["Value, with comma".to_string()]],
            title: None,
            caption: None,
            footnotes: vec![],
        };

        let csv = table.to_csv();
        assert!(csv.contains("\"Value, with comma\""));
    }

    #[test]
    fn test_is_footnote_text() {
        assert!(is_footnote_text("* This is a footnote"));
        assert!(is_footnote_text("1. First item"));
        assert!(is_footnote_text("† Note"));
        assert!(!is_footnote_text("Regular text"));
    }
}
