//! Centralized, pre-compiled patterns for HTML parsing and content extraction.
//!
//! This module provides lazy-initialized selectors and regex patterns that are
//! compiled once at first use. This avoids runtime panics from invalid patterns
//! and improves performance by avoiding repeated compilation.

use once_cell::sync::Lazy;
use regex::Regex;
use scraper::Selector;

// ============================================================================
// CSS SELECTORS - Table Extraction
// ============================================================================

/// Selector for `<table>` elements
pub static TABLE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("table").expect("Invalid selector: table"));

/// Selector for `<tr>` (table row) elements
pub static TR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("tr").expect("Invalid selector: tr"));

/// Selector for `<td>` (table data) elements
pub static TD: Lazy<Selector> =
    Lazy::new(|| Selector::parse("td").expect("Invalid selector: td"));

/// Selector for `<th>` (table header) elements
pub static TH: Lazy<Selector> =
    Lazy::new(|| Selector::parse("th").expect("Invalid selector: th"));

/// Selector for `<thead>` elements
pub static THEAD: Lazy<Selector> =
    Lazy::new(|| Selector::parse("thead").expect("Invalid selector: thead"));

/// Selector for `<tbody>` elements
pub static TBODY: Lazy<Selector> =
    Lazy::new(|| Selector::parse("tbody").expect("Invalid selector: tbody"));

/// Selector for `<caption>` elements
pub static CAPTION: Lazy<Selector> =
    Lazy::new(|| Selector::parse("caption").expect("Invalid selector: caption"));

// ============================================================================
// CSS SELECTORS - Content Extraction
// ============================================================================

/// Selector for `<title>` elements
pub static TITLE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("title").expect("Invalid selector: title"));

/// Selector for meta description
pub static META_DESCRIPTION: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("meta[name=\"description\"]").expect("Invalid selector: meta description")
});

/// Selector for `<a>` (anchor/link) elements
pub static ANCHOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("a").expect("Invalid selector: a"));

/// Selector for links with href attribute
pub static ANCHOR_HREF: Lazy<Selector> =
    Lazy::new(|| Selector::parse("a[href]").expect("Invalid selector: a[href]"));

/// Selector for `<img>` elements
pub static IMG: Lazy<Selector> =
    Lazy::new(|| Selector::parse("img").expect("Invalid selector: img"));

/// Selector for `<video>` elements
pub static VIDEO: Lazy<Selector> =
    Lazy::new(|| Selector::parse("video").expect("Invalid selector: video"));

/// Selector for `<audio>` elements
pub static AUDIO: Lazy<Selector> =
    Lazy::new(|| Selector::parse("audio").expect("Invalid selector: audio"));

/// Selector for `<form>` elements
pub static FORM: Lazy<Selector> =
    Lazy::new(|| Selector::parse("form").expect("Invalid selector: form"));

/// Selector for `<input>` elements
pub static INPUT: Lazy<Selector> =
    Lazy::new(|| Selector::parse("input").expect("Invalid selector: input"));

/// Selector for `<select>` elements
pub static SELECT: Lazy<Selector> =
    Lazy::new(|| Selector::parse("select").expect("Invalid selector: select"));

/// Selector for `<textarea>` elements
pub static TEXTAREA: Lazy<Selector> =
    Lazy::new(|| Selector::parse("textarea").expect("Invalid selector: textarea"));

/// Selector for `<button>` elements
pub static BUTTON: Lazy<Selector> =
    Lazy::new(|| Selector::parse("button").expect("Invalid selector: button"));

// ============================================================================
// CSS SELECTORS - Structured Data
// ============================================================================

/// Selector for JSON-LD scripts
pub static JSON_LD: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("script[type=\"application/ld+json\"]")
        .expect("Invalid selector: JSON-LD script")
});

/// Selector for microdata items
pub static MICRODATA: Lazy<Selector> =
    Lazy::new(|| Selector::parse("[itemscope]").expect("Invalid selector: microdata"));

/// Selector for OpenGraph meta tags
pub static OG_META: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("meta[property^=\"og:\"]").expect("Invalid selector: OpenGraph meta")
});

/// Selector for Twitter Card meta tags
pub static TWITTER_META: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("meta[name^=\"twitter:\"]").expect("Invalid selector: Twitter meta")
});

// ============================================================================
// CSS SELECTORS - Document Structure
// ============================================================================

/// Selector for `<h1>` elements
pub static H1: Lazy<Selector> =
    Lazy::new(|| Selector::parse("h1").expect("Invalid selector: h1"));

/// Selector for `<h2>` elements
pub static H2: Lazy<Selector> =
    Lazy::new(|| Selector::parse("h2").expect("Invalid selector: h2"));

/// Selector for `<h3>` elements
pub static H3: Lazy<Selector> =
    Lazy::new(|| Selector::parse("h3").expect("Invalid selector: h3"));

/// Selector for all headings
pub static HEADINGS: Lazy<Selector> =
    Lazy::new(|| Selector::parse("h1, h2, h3, h4, h5, h6").expect("Invalid selector: headings"));

/// Selector for `<p>` (paragraph) elements
pub static PARAGRAPH: Lazy<Selector> =
    Lazy::new(|| Selector::parse("p").expect("Invalid selector: p"));

/// Selector for `<ul>` (unordered list) elements
pub static UL: Lazy<Selector> =
    Lazy::new(|| Selector::parse("ul").expect("Invalid selector: ul"));

/// Selector for `<ol>` (ordered list) elements
pub static OL: Lazy<Selector> =
    Lazy::new(|| Selector::parse("ol").expect("Invalid selector: ol"));

/// Selector for `<li>` (list item) elements
pub static LI: Lazy<Selector> =
    Lazy::new(|| Selector::parse("li").expect("Invalid selector: li"));

/// Selector for `<dl>` (definition list) elements
pub static DL: Lazy<Selector> =
    Lazy::new(|| Selector::parse("dl").expect("Invalid selector: dl"));

/// Selector for `<dt>` (definition term) elements
pub static DT: Lazy<Selector> =
    Lazy::new(|| Selector::parse("dt").expect("Invalid selector: dt"));

/// Selector for `<dd>` (definition description) elements
pub static DD: Lazy<Selector> =
    Lazy::new(|| Selector::parse("dd").expect("Invalid selector: dd"));

/// Selector for `<main>` elements
pub static MAIN: Lazy<Selector> =
    Lazy::new(|| Selector::parse("main").expect("Invalid selector: main"));

/// Selector for `<article>` elements
pub static ARTICLE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("article").expect("Invalid selector: article"));

/// Selector for `<section>` elements
pub static SECTION: Lazy<Selector> =
    Lazy::new(|| Selector::parse("section").expect("Invalid selector: section"));

/// Selector for `<nav>` elements
pub static NAV: Lazy<Selector> =
    Lazy::new(|| Selector::parse("nav").expect("Invalid selector: nav"));

/// Selector for `<header>` elements
pub static HEADER: Lazy<Selector> =
    Lazy::new(|| Selector::parse("header").expect("Invalid selector: header"));

/// Selector for `<footer>` elements
pub static FOOTER: Lazy<Selector> =
    Lazy::new(|| Selector::parse("footer").expect("Invalid selector: footer"));

/// Selector for `<aside>` elements
pub static ASIDE: Lazy<Selector> =
    Lazy::new(|| Selector::parse("aside").expect("Invalid selector: aside"));

// ============================================================================
// CSS SELECTORS - Interactive Elements
// ============================================================================

/// Selector for clickable elements
pub static CLICKABLE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("a, button, [onclick], [role=\"button\"]")
        .expect("Invalid selector: clickable")
});

/// Selector for expandable/collapsible elements
pub static EXPANDABLE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("[aria-expanded], details, .accordion, .collapsible")
        .expect("Invalid selector: expandable")
});

// ============================================================================
// REGEX PATTERNS - Entity Extraction
// ============================================================================

/// Email address pattern (RFC 5322 simplified)
pub static EMAIL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
        .expect("Invalid regex: email")
});

/// URL pattern (http/https)
pub static URL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"https?://[^\s<>"']+"#).expect("Invalid regex: URL")
});

/// Phone number pattern (various formats)
pub static PHONE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?:\+?1[-.\s]?)?(?:\(?[0-9]{3}\)?[-.\s]?)?[0-9]{3}[-.\s]?[0-9]{4}|(?:\+[0-9]{1,3}[-.\s]?)?(?:\([0-9]{1,4}\)[-.\s]?)?[0-9]{2,4}[-.\s]?[0-9]{2,4}[-.\s]?[0-9]{2,4}",
    )
    .expect("Invalid regex: phone")
});

/// Price/currency pattern
pub static PRICE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\$[\d,]+(?:\.\d{2})?|\d+(?:,\d{3})*(?:\.\d{2})?\s*(?:USD|EUR|GBP|dollars?)")
        .expect("Invalid regex: price")
});

/// Date pattern (various formats)
pub static DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?:\d{1,2}[-/]\d{1,2}[-/]\d{2,4})|(?:\d{4}[-/]\d{1,2}[-/]\d{1,2})|(?:(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+\d{1,2},?\s+\d{4})|(?:\d{1,2}\s+(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\.?\s+\d{4})",
    )
    .expect("Invalid regex: date")
});

/// Time pattern
pub static TIME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\d{1,2}:\d{2}(?::\d{2})?(?:\s*[AaPp][Mm])?").expect("Invalid regex: time")
});

/// Percentage pattern
pub static PERCENTAGE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\d+(?:\.\d+)?%").expect("Invalid regex: percentage"));

/// Numeric pattern (integers and decimals)
pub static NUMERIC: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"-?\d+(?:,\d{3})*(?:\.\d+)?").expect("Invalid regex: numeric")
});

// ============================================================================
// REGEX PATTERNS - Content Patterns
// ============================================================================

/// Course code pattern (e.g., "CS 101", "MATH-200")
pub static COURSE_CODE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[A-Z]{2,4}\s*[-]?\s*\d{3,4}[A-Z]?").expect("Invalid regex: course code")
});

/// ZIP code pattern (US)
pub static ZIP_CODE_US: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\d{5}(?:-\d{4})?").expect("Invalid regex: ZIP code"));

/// Social Security Number pattern (masked)
pub static SSN_MASKED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"XXX-XX-\d{4}|\*{3}-\*{2}-\d{4}").expect("Invalid regex: SSN masked"));

/// Credit card pattern (masked, last 4 digits visible)
pub static CREDIT_CARD_MASKED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[*X]{4}[-\s]?[*X]{4}[-\s]?[*X]{4}[-\s]?\d{4}")
        .expect("Invalid regex: credit card masked")
});

// ============================================================================
// REGEX PATTERNS - Relation Extraction
// ============================================================================

/// "X is a/an Y" pattern - extracts "is_a" relations
pub static IS_A: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:is|was|are|were)\s+(?:a|an|the)\s+([a-zA-Z\s]{2,50})")
        .expect("Invalid regex: is_a")
});

/// "X has Y" pattern - extracts "has" relations
pub static HAS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:has|have|had)\s+(?:a|an|the)?\s*([a-zA-Z\s]{2,50})")
        .expect("Invalid regex: has")
});

/// "X works at/for Y" pattern - extracts employment relations
pub static WORKS_AT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:works?|worked)\s+(?:at|for)\s+([A-Z][a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: works_at")
});

/// "X founded/created Y" pattern - extracts founder relations
pub static FOUNDED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:founded|created|established|started)\s+([A-Z][a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: founded")
});

/// "X located in/at Y" pattern - extracts location relations
pub static LOCATED_IN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s&,.']{1,60})\s+(?:is\s+)?(?:located|based|headquartered)\s+(?:in|at)\s+([A-Z][a-zA-Z\s,]{2,60})")
        .expect("Invalid regex: located_in")
});

/// "X is part of Y" pattern - extracts part_of relations
pub static PART_OF: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:is|are|was|were)\s+(?:a\s+)?part\s+of\s+([A-Z][a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: part_of")
});

/// "X acquired/bought Y" pattern - extracts acquisition relations
pub static ACQUIRED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s&,.']{1,60})\s+(?:acquired|bought|purchased|merged\s+with)\s+([A-Z][a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: acquired")
});

/// "X CEO/founder/president of Y" pattern - extracts role relations
pub static ROLE_OF: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})(?:,|\s+is|\s+was)?\s+(?:the\s+)?(?:CEO|CTO|CFO|COO|founder|co-founder|president|chairman|director|manager)\s+(?:of|at)\s+([A-Z][a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: role_of")
});

/// "X parent/child/sibling of Y" pattern - extracts family relations
pub static FAMILY_RELATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})(?:'s|\s+is|\s+was)?\s+(?:the\s+)?(?:mother|father|parent|child|son|daughter|brother|sister|sibling)\s+(?:of\s+)?([A-Z][a-zA-Z\s]{2,50})")
        .expect("Invalid regex: family_relation")
});

/// "X invented/developed/designed Y" pattern - extracts creator relations
pub static CREATED: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)([A-Z][a-zA-Z\s]{1,50})\s+(?:invented|developed|designed|built|wrote|authored)\s+(?:the\s+)?([A-Z]?[a-zA-Z\s&,.']{2,60})")
        .expect("Invalid regex: created")
});

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Parse a selector string, returning None on failure instead of panicking.
pub fn parse_selector(selector: &str) -> Option<Selector> {
    Selector::parse(selector).ok()
}

/// Parse a regex pattern, returning None on failure instead of panicking.
pub fn parse_regex(pattern: &str) -> Option<Regex> {
    Regex::new(pattern).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectors_compile() {
        // Force initialization of all selectors to verify they compile
        let _ = &*TABLE;
        let _ = &*TR;
        let _ = &*TD;
        let _ = &*TH;
        let _ = &*ANCHOR_HREF;
        let _ = &*JSON_LD;
        let _ = &*HEADINGS;
    }

    #[test]
    fn test_email_regex() {
        assert!(EMAIL.is_match("test@example.com"));
        assert!(EMAIL.is_match("user.name+tag@domain.co.uk"));
        assert!(!EMAIL.is_match("not an email"));
    }

    #[test]
    fn test_phone_regex() {
        assert!(PHONE.is_match("555-123-4567"));
        assert!(PHONE.is_match("(555) 123-4567"));
        assert!(PHONE.is_match("+1 555 123 4567"));
    }

    #[test]
    fn test_url_regex() {
        assert!(URL.is_match("https://example.com"));
        assert!(URL.is_match("http://example.com/path?query=1"));
    }

    #[test]
    fn test_price_regex() {
        assert!(PRICE.is_match("$19.99"));
        assert!(PRICE.is_match("$1,234.56"));
        assert!(PRICE.is_match("100 USD"));
    }

    #[test]
    fn test_date_regex() {
        assert!(DATE.is_match("2024-01-15"));
        assert!(DATE.is_match("01/15/2024"));
        assert!(DATE.is_match("January 15, 2024"));
        assert!(DATE.is_match("15 Jan 2024"));
    }

    #[test]
    fn test_course_code_regex() {
        assert!(COURSE_CODE.is_match("CS 101"));
        assert!(COURSE_CODE.is_match("MATH-200"));
        assert!(COURSE_CODE.is_match("PHYS101A"));
    }

    #[test]
    fn test_parse_selector_safe() {
        assert!(parse_selector("div").is_some());
        assert!(parse_selector("div > p").is_some());
        assert!(parse_selector("[[[invalid").is_none());
    }

    #[test]
    fn test_parse_regex_safe() {
        assert!(parse_regex(r"\d+").is_some());
        assert!(parse_regex(r"[invalid").is_none());
    }
}
