use omnivore_core::parser::{ParseConfig, ParseRule, Parser};
use serde_json::json;

#[test]
fn test_parser_creation() {
    let config = ParseConfig {
        rules: vec![],
        schema_name: None,
        clean_text: true,
        extract_metadata: true,
    };

    let parser = Parser::new(config);
    let html = "<html><head><title>Test</title></head><body>Hello</body></html>";
    let result = parser.parse(html);
    assert!(result.is_ok());
}

#[test]
fn test_parse_with_rules() {
    let config = ParseConfig {
        rules: vec![
            ParseRule {
                name: "title".to_string(),
                selector: "title".to_string(),
                attribute: None,
                multiple: false,
                required: false,
                transform: None,
            },
            ParseRule {
                name: "body".to_string(),
                selector: "body".to_string(),
                attribute: None,
                multiple: false,
                required: false,
                transform: None,
            },
        ],
        schema_name: None,
        clean_text: true,
        extract_metadata: false,
    };

    let parser = Parser::new(config);
    let html = "<html><head><title>Test Page</title></head><body>Hello World</body></html>";
    let result = parser.parse(html).unwrap();

    assert_eq!(result["title"], json!("Test Page"));
    assert_eq!(result["body"], json!("Hello World"));
}

#[test]
fn test_extract_links() {
    let config = ParseConfig {
        rules: vec![],
        schema_name: None,
        clean_text: true,
        extract_metadata: false,
    };

    let parser = Parser::new(config);
    let html = r#"
        <html>
            <body>
                <a href="https://example.com">Link 1</a>
                <a href="/page2">Link 2</a>
                <a href="page3.html">Link 3</a>
            </body>
        </html>
    "#;

    let base_url = url::Url::parse("https://example.com").unwrap();
    let links = parser.extract_links(html, &base_url).unwrap();

    assert_eq!(links.len(), 3);
    assert!(links.iter().any(|u| u.as_str() == "https://example.com/"));
    assert!(links
        .iter()
        .any(|u| u.as_str() == "https://example.com/page2"));
    assert!(links
        .iter()
        .any(|u| u.as_str() == "https://example.com/page3.html"));
}

#[test]
fn test_html_parser() {
    use omnivore_core::parser::html::HtmlParser;

    let html = r#"
        <html>
            <head>
                <meta property="og:title" content="Test Title" />
                <meta property="og:description" content="Test Description" />
            </head>
            <body>
                <h1>Heading</h1>
                <p class="content">Paragraph text</p>
            </body>
        </html>
    "#;

    let parser = HtmlParser::new(html);

    let h1_text = parser.select("h1").unwrap();
    assert_eq!(h1_text[0], "Heading");

    let og_data = parser.extract_opengraph();
    assert_eq!(og_data["title"], json!("Test Title"));
    assert_eq!(og_data["description"], json!("Test Description"));
}
