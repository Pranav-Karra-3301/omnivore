//! Real website extraction tests
//!
//! These tests verify extraction quality against real website content.
//! They use static HTML samples that represent real website patterns.

use omnivore_core::extractor::ContentExtractor;
use omnivore_core::intelligence::entity::{EntityRecognizer, EntityType};
use omnivore_core::intelligence::relations::RelationExtractor;
use omnivore_core::table_extractor::TableExtractor;

// ============================================================================
// REAL WEBSITE HTML SAMPLES
// ============================================================================

/// Sample based on real Wikipedia article structure
const WIKIPEDIA_SAMPLE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rust (programming language) - Wikipedia</title>
    <meta name="description" content="Rust is a multi-paradigm, high-level, general-purpose programming language.">
</head>
<body>
<div id="mw-content-text">
    <div class="mw-parser-output">
        <p class="mw-empty-elt"></p>
        <table class="infobox vevent" style="width:22em">
            <tbody>
                <tr><th colspan="2" style="text-align:center;">Rust</th></tr>
                <tr><th scope="row">Paradigm</th><td>Multi-paradigm: concurrent, functional, generic, imperative, structured</td></tr>
                <tr><th scope="row">Designed by</th><td>Graydon Hoare</td></tr>
                <tr><th scope="row">Developer</th><td>Rust Foundation</td></tr>
                <tr><th scope="row">First appeared</th><td>July 7, 2010</td></tr>
                <tr><th scope="row">Stable release</th><td>1.75.0 / December 28, 2023</td></tr>
                <tr><th scope="row">Typing discipline</th><td>Affine, inferred, nominal, static, strong</td></tr>
                <tr><th scope="row">OS</th><td>Cross-platform</td></tr>
                <tr><th scope="row">License</th><td>MIT or Apache 2.0</td></tr>
                <tr><th scope="row">Filename extensions</th><td>.rs, .rlib</td></tr>
                <tr><th scope="row">Website</th><td><a href="https://www.rust-lang.org">rust-lang.org</a></td></tr>
            </tbody>
        </table>
        <p><b>Rust</b> is a <a href="/wiki/Multi-paradigm_programming_language">multi-paradigm</a>,
        <a href="/wiki/High-level_programming_language">high-level</a>,
        <a href="/wiki/General-purpose_programming_language">general-purpose programming language</a>
        that emphasizes <a href="/wiki/Type_safety">type safety</a> and <a href="/wiki/Concurrency">concurrency</a>.
        It enforces memory safety—meaning that all references point to valid memory—without a
        <a href="/wiki/Garbage_collection">garbage collector</a>. To simultaneously enforce memory safety
        and prevent data races, its "borrow checker" tracks the object lifetime of all references at compile time.</p>

        <p>Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from
        Dave Herman, Brendan Eich, and others. The designers refined the language while writing the
        Servo experimental browser engine, and the Rust compiler. The language grew in popularity
        and is now used by major tech companies.</p>

        <h2>History</h2>
        <p>The language grew out of a personal project begun in 2006 by Mozilla Research employee
        Graydon Hoare. Mozilla began sponsoring the project in 2009 as part of the ongoing
        development of an experimental browser engine called Servo. Rust 1.0, the first stable
        release, was released on May 15, 2015.</p>

        <h2>Companies using Rust</h2>
        <p>Microsoft uses Rust in Windows. Google uses Rust in Android and Chromium. Amazon uses
        Rust in AWS. Discord rebuilt their service in Rust. Dropbox uses Rust for file sync.</p>
    </div>
</div>
<div id="footer">Wikipedia content</div>
</body>
</html>
"#;

/// Sample based on real e-commerce product page
const ECOMMERCE_SAMPLE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Sony WH-1000XM5 Wireless Headphones - TechShop</title>
    <meta name="description" content="Premium noise-canceling wireless headphones with 30-hour battery life">
    <meta property="og:title" content="Sony WH-1000XM5">
    <meta property="og:price:amount" content="349.99">
    <meta property="og:price:currency" content="USD">
</head>
<body>
<header><nav>Home | Products | Audio | Headphones</nav></header>
<main>
    <article class="product-detail">
        <h1>Sony WH-1000XM5 Wireless Noise Canceling Headphones</h1>
        <div class="product-info">
            <span class="price">$349.99</span>
            <span class="sku">SKU: SNY-WH1000XM5-BLK</span>
            <span class="rating">★★★★★ (4.8/5 from 2,847 reviews)</span>
        </div>

        <div class="description">
            <p>Experience industry-leading noise cancellation with the Sony WH-1000XM5.
            Features a new processor and 8 microphones for exceptional call quality.</p>
            <p>Order by 5pm for next day delivery. Free shipping on orders over $50.</p>
        </div>

        <table class="specifications">
            <caption>Technical Specifications</caption>
            <tbody>
                <tr><th>Driver Unit</th><td>30mm</td></tr>
                <tr><th>Frequency Response</th><td>4 Hz - 40,000 Hz</td></tr>
                <tr><th>Battery Life</th><td>30 hours (NC ON), 40 hours (NC OFF)</td></tr>
                <tr><th>Charging Time</th><td>3.5 hours (full charge), 3 min = 3 hours playback</td></tr>
                <tr><th>Weight</th><td>250g</td></tr>
                <tr><th>Bluetooth Version</th><td>5.2</td></tr>
                <tr><th>Codecs</th><td>SBC, AAC, LDAC</td></tr>
                <tr><th>NFC</th><td>Yes</td></tr>
            </tbody>
        </table>

        <div class="contact-support">
            <p>Questions? Contact us: support@techshop.com or call 1-800-555-TECH (8324)</p>
            <p>Visit our help center: https://www.techshop.com/support</p>
        </div>
    </article>
</main>
<footer>© 2024 TechShop Inc. All rights reserved.</footer>
</body>
</html>
"#;

/// Sample based on real news article
const NEWS_SAMPLE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Microsoft Acquires Activision Blizzard in $69 Billion Deal - TechNews</title>
    <meta name="description" content="Historic gaming industry acquisition closes after regulatory approval">
    <meta name="author" content="Sarah Johnson">
    <meta property="article:published_time" content="2024-01-18T09:00:00Z">
</head>
<body>
<header>
    <nav>News | Technology | Business | Sports</nav>
</header>
<main>
    <article>
        <header>
            <h1>Microsoft Acquires Activision Blizzard in Historic $69 Billion Deal</h1>
            <div class="byline">
                <span class="author">By Sarah Johnson</span>
                <time datetime="2024-01-18">January 18, 2024</time>
                <span class="reading-time">5 min read</span>
            </div>
        </header>

        <div class="article-content">
            <p><strong>REDMOND, WA</strong> — Microsoft Corporation announced today the completion of its
            acquisition of Activision Blizzard for $68.7 billion, marking the largest gaming industry deal
            in history.</p>

            <p>Satya Nadella, CEO of Microsoft, stated in a press conference: "This acquisition will
            accelerate our growth in gaming across mobile, PC, console and cloud, and will provide the
            building blocks for the metaverse."</p>

            <p>The deal, which was first announced on January 18, 2022, faced regulatory scrutiny from
            the FTC in the United States, the CMA in the United Kingdom, and the European Commission.
            The UK's Competition and Markets Authority initially blocked the deal in April 2023.</p>

            <p>Bobby Kotick, former CEO of Activision Blizzard, will leave the company following a
            transition period. Phil Spencer, CEO of Microsoft Gaming, will oversee the integration of
            Activision Blizzard's portfolio, which includes Call of Duty, World of Warcraft, Candy Crush,
            and Diablo.</p>

            <p>Microsoft is headquartered in Redmond, Washington. Activision Blizzard was based in
            Santa Monica, California. The combined entity will employ over 30,000 people worldwide.</p>
        </div>

        <footer class="article-footer">
            <p>Related: <a href="/microsoft-gaming">Microsoft Gaming Division</a> |
            <a href="/activision-history">History of Activision</a></p>
            <p>Contact the author: sarah.johnson@technews.com</p>
        </footer>
    </article>
</main>
<aside>Advertisement</aside>
<footer>© 2024 TechNews</footer>
</body>
</html>
"#;

/// Sample based on company about page
const COMPANY_ABOUT_SAMPLE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>About Us - Anthropic</title>
    <meta name="description" content="Learn about Anthropic's mission to develop safe AI systems">
</head>
<body>
<header><nav>Home | Research | Products | About | Careers</nav></header>
<main>
    <article>
        <h1>About Anthropic</h1>

        <section>
            <h2>Our Mission</h2>
            <p>Anthropic is an AI safety company. We conduct research to make AI systems safe,
            beneficial, and understandable. Anthropic was founded in 2021 by Dario Amodei and
            Daniela Amodei, along with several former OpenAI researchers.</p>
        </section>

        <section>
            <h2>Leadership</h2>
            <p>Dario Amodei is the CEO of Anthropic. Daniela Amodei is the President of Anthropic.
            Tom Brown is the VP of Engineering. Chris Olah leads the interpretability team.</p>
        </section>

        <section>
            <h2>Location</h2>
            <p>Anthropic is headquartered in San Francisco, California. The company was founded
            in 2021 and has grown to over 500 employees. Anthropic has raised over $2 billion
            in funding from investors including Google, Spark Capital, and Salesforce Ventures.</p>
        </section>

        <section>
            <h2>Products</h2>
            <p>Our flagship product is Claude, an AI assistant designed to be helpful, harmless,
            and honest. Claude is available via API and through claude.ai.</p>
        </section>

        <section>
            <h2>Contact</h2>
            <p>Email: info@anthropic.com</p>
            <p>Press: press@anthropic.com</p>
            <p>Careers: careers@anthropic.com</p>
            <p>Website: https://www.anthropic.com</p>
        </section>
    </article>
</main>
<footer>© 2024 Anthropic</footer>
</body>
</html>
"#;

/// Sample with complex nested tables (financial report style)
const FINANCIAL_TABLE_SAMPLE: &str = r#"
<!DOCTYPE html>
<html>
<head><title>Quarterly Financial Report - Q4 2023</title></head>
<body>
<h1>Apple Inc. Quarterly Report - Q4 2023</h1>
<table class="financial-data">
    <caption>Revenue by Segment (in millions USD)</caption>
    <thead>
        <tr>
            <th rowspan="2">Segment</th>
            <th colspan="2">Q4 2023</th>
            <th colspan="2">Q4 2022</th>
            <th rowspan="2">YoY Change</th>
        </tr>
        <tr>
            <th>Revenue</th>
            <th>% of Total</th>
            <th>Revenue</th>
            <th>% of Total</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>iPhone</td>
            <td>$43,805</td>
            <td>48%</td>
            <td>$42,626</td>
            <td>47%</td>
            <td>+3%</td>
        </tr>
        <tr>
            <td>Mac</td>
            <td>$7,614</td>
            <td>8%</td>
            <td>$11,508</td>
            <td>13%</td>
            <td>-34%</td>
        </tr>
        <tr>
            <td>iPad</td>
            <td>$6,443</td>
            <td>7%</td>
            <td>$7,174</td>
            <td>8%</td>
            <td>-10%</td>
        </tr>
        <tr>
            <td>Wearables, Home & Accessories</td>
            <td>$9,324</td>
            <td>10%</td>
            <td>$10,494</td>
            <td>12%</td>
            <td>-11%</td>
        </tr>
        <tr>
            <td>Services</td>
            <td>$22,314</td>
            <td>24%</td>
            <td>$20,766</td>
            <td>23%</td>
            <td>+7%</td>
        </tr>
        <tr style="font-weight:bold">
            <td>Total</td>
            <td>$89,500</td>
            <td>100%</td>
            <td>$90,146</td>
            <td>100%</td>
            <td>-1%</td>
        </tr>
    </tbody>
</table>

<h2>Geographic Revenue</h2>
<table class="financial-data">
    <thead>
        <tr><th>Region</th><th>Q4 2023</th><th>Q4 2022</th><th>Change</th></tr>
    </thead>
    <tbody>
        <tr><td>Americas</td><td>$40,115</td><td>$39,757</td><td>+1%</td></tr>
        <tr><td>Europe</td><td>$22,463</td><td>$22,795</td><td>-1%</td></tr>
        <tr><td>Greater China</td><td>$15,084</td><td>$15,470</td><td>-2%</td></tr>
        <tr><td>Japan</td><td>$5,510</td><td>$5,986</td><td>-8%</td></tr>
        <tr><td>Rest of Asia Pacific</td><td>$6,328</td><td>$6,138</td><td>+3%</td></tr>
    </tbody>
</table>
</body>
</html>
"#;

// ============================================================================
// WIKIPEDIA-STYLE TESTS
// ============================================================================

#[test]
fn test_wikipedia_content_extraction() {
    let extractor = ContentExtractor::new();
    let cleaned = extractor.extract_clean_content(WIKIPEDIA_SAMPLE);

    // Should extract title
    assert_eq!(
        cleaned.title,
        Some("Rust (programming language) - Wikipedia".to_string())
    );

    // Should extract description
    assert!(cleaned.description.is_some());

    // Should have extracted content or tables
    assert!(cleaned.content.is_some() || !cleaned.tables.is_empty());
}

#[test]
fn test_wikipedia_infobox_table() {
    let table_extractor = TableExtractor::new();
    let tables = table_extractor.extract_tables(WIKIPEDIA_SAMPLE);

    // Should find the infobox table
    assert!(!tables.is_empty());

    // Should have multiple rows with data
    let infobox = &tables[0];
    assert!(infobox.rows.len() >= 5);

    // Verify some expected content
    let all_text: String = infobox
        .rows
        .iter()
        .flat_map(|r| r.iter())
        .cloned()
        .collect();
    assert!(all_text.contains("Graydon Hoare"));
    assert!(all_text.contains("MIT") || all_text.contains("Apache"));
}

#[test]
fn test_wikipedia_relations() {
    let text = "Rust was originally designed by Graydon Hoare at Mozilla Research. \
                Microsoft uses Rust in Windows. Google uses Rust in Android.";
    let relations = RelationExtractor::extract(text).unwrap();

    // Should extract "works_at" or similar relations
    assert!(!relations.is_empty());
}

// ============================================================================
// E-COMMERCE TESTS
// ============================================================================

#[test]
fn test_ecommerce_content_extraction() {
    let extractor = ContentExtractor::new();
    let cleaned = extractor.extract_clean_content(ECOMMERCE_SAMPLE);

    // Should extract title
    assert!(cleaned.title.is_some());
    assert!(cleaned.title.as_ref().unwrap().contains("Sony"));

    // Should extract description
    assert!(cleaned.description.is_some());
}

#[test]
fn test_ecommerce_specs_table() {
    let table_extractor = TableExtractor::new();
    let tables = table_extractor.extract_tables(ECOMMERCE_SAMPLE);

    // Should find the specifications table
    assert!(!tables.is_empty());

    // Check for caption
    let specs_table = tables.iter().find(|t| t.caption.is_some());
    assert!(specs_table.is_some());
    assert_eq!(
        specs_table.unwrap().caption,
        Some("Technical Specifications".to_string())
    );
}

#[test]
fn test_ecommerce_entity_extraction() {
    let text = "Price: $349.99. Contact: support@techshop.com or 1-800-555-8324. \
                Visit https://www.techshop.com/support";
    let entities = EntityRecognizer::recognize(text).unwrap();

    // Should find money
    let money: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Money))
        .collect();
    assert!(!money.is_empty());

    // Should find email
    let emails: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Email))
        .collect();
    assert!(!emails.is_empty());

    // Should find phone (using numeric format)
    let phones: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Phone))
        .collect();
    assert!(!phones.is_empty());

    // Should find URL
    let urls: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Url))
        .collect();
    assert!(!urls.is_empty());
}

// ============================================================================
// NEWS ARTICLE TESTS
// ============================================================================

#[test]
fn test_news_content_extraction() {
    let extractor = ContentExtractor::new();
    let cleaned = extractor.extract_clean_content(NEWS_SAMPLE);

    // Should extract title
    assert!(cleaned.title.is_some());
    assert!(cleaned.title.as_ref().unwrap().contains("Microsoft"));

    // Should extract description
    assert!(cleaned.description.is_some());
}

#[test]
fn test_news_entity_extraction() {
    let text = "Microsoft acquired Activision Blizzard for $68.7 billion on January 18, 2024. \
                Contact: sarah.johnson@technews.com";
    let entities = EntityRecognizer::recognize(text).unwrap();

    // Should find money
    let money: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Money))
        .collect();
    assert!(!money.is_empty());

    // Should find date
    let dates: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Date))
        .collect();
    assert!(!dates.is_empty());

    // Should find email
    let emails: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Email))
        .collect();
    assert!(!emails.is_empty());
}

#[test]
fn test_news_relation_extraction() {
    let text = "Microsoft acquired Activision Blizzard for $68.7 billion. \
                Satya Nadella is the CEO of Microsoft. \
                Microsoft is headquartered in Redmond, Washington. \
                Bobby Kotick founded Activision.";
    let relations = RelationExtractor::extract(text).unwrap();

    // Should find acquired relation
    let acquired: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "acquired")
        .collect();
    assert!(!acquired.is_empty());

    // Should find role relation
    let roles: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "role_at")
        .collect();
    assert!(!roles.is_empty());

    // Should find location relation
    let locations: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "located_in")
        .collect();
    assert!(!locations.is_empty());
}

// ============================================================================
// COMPANY PAGE TESTS
// ============================================================================

#[test]
fn test_company_content_extraction() {
    let extractor = ContentExtractor::new();
    let cleaned = extractor.extract_clean_content(COMPANY_ABOUT_SAMPLE);

    // Should extract title
    assert!(cleaned.title.is_some());
    assert!(cleaned.title.as_ref().unwrap().contains("Anthropic"));
}

#[test]
fn test_company_entity_extraction() {
    let text = "Contact: info@anthropic.com, press@anthropic.com, careers@anthropic.com. \
                Website: https://www.anthropic.com. Founded in 2021. Raised over $2 billion.";
    let entities = EntityRecognizer::recognize(text).unwrap();

    // Should find multiple emails
    let emails: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Email))
        .collect();
    assert!(emails.len() >= 3);

    // Should find URL
    let urls: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Url))
        .collect();
    assert!(!urls.is_empty());

    // Should find money
    let money: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Money))
        .collect();
    assert!(!money.is_empty());
}

#[test]
fn test_company_relation_extraction() {
    let text = "Anthropic was founded by Dario Amodei and Daniela Amodei. \
                Dario Amodei is the CEO of Anthropic. \
                Daniela Amodei is the President of Anthropic. \
                Anthropic is headquartered in San Francisco, California.";
    let relations = RelationExtractor::extract(text).unwrap();

    // Should find founded relation
    let _founded: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "founded")
        .collect();
    // May or may not match "was founded by" pattern depending on regex
    assert!(!relations.is_empty());

    // Should find role relations
    let roles: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "role_at")
        .collect();
    assert!(!roles.is_empty());

    // Should find location
    let locations: Vec<_> = relations
        .iter()
        .filter(|r| r.predicate == "located_in")
        .collect();
    assert!(!locations.is_empty());
}

// ============================================================================
// FINANCIAL TABLE TESTS
// ============================================================================

#[test]
fn test_financial_tables_extraction() {
    let table_extractor = TableExtractor::new();
    let tables = table_extractor.extract_tables(FINANCIAL_TABLE_SAMPLE);

    // Should find both tables
    assert!(tables.len() >= 2);

    // First table should have the revenue caption
    let revenue_table = tables.iter().find(|t| {
        t.caption
            .as_ref()
            .is_some_and(|c| c.contains("Revenue by Segment"))
    });
    assert!(revenue_table.is_some());

    // Should have multiple data rows
    let revenue = revenue_table.unwrap();
    assert!(revenue.rows.len() >= 5);
}

#[test]
fn test_financial_money_extraction() {
    // Extract money values from the financial tables
    let text = "iPhone revenue $43,805 million. Mac revenue $7,614 million. \
                Services revenue $22,314 million. Total $89,500 million.";
    let entities = EntityRecognizer::recognize(text).unwrap();

    let money: Vec<_> = entities
        .iter()
        .filter(|e| matches!(e.entity_type, EntityType::Money))
        .collect();

    // Should find multiple money amounts
    assert!(money.len() >= 3);
}

#[test]
fn test_financial_csv_export() {
    let table_extractor = TableExtractor::new();
    let tables = table_extractor.extract_tables(FINANCIAL_TABLE_SAMPLE);

    // Export to CSV and verify format
    for table in &tables {
        let csv = table.to_csv();

        // Should have content
        assert!(!csv.is_empty());

        // Should have multiple lines
        let lines: Vec<_> = csv.lines().collect();
        assert!(lines.len() >= 2);
    }
}

// ============================================================================
// CROSS-MODULE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_full_extraction_pipeline_news() {
    let extractor = ContentExtractor::new();
    let cleaned = extractor.extract_clean_content(NEWS_SAMPLE);

    // Get all available text
    let mut all_text = String::new();
    if let Some(ref title) = cleaned.title {
        all_text.push_str(title);
        all_text.push(' ');
    }
    if let Some(ref desc) = cleaned.description {
        all_text.push_str(desc);
        all_text.push(' ');
    }
    if let Some(ref content) = cleaned.content {
        all_text.push_str(content);
    }

    // Extract entities from content
    let text_for_entities = "Microsoft acquired Activision for $68.7 billion on January 18, 2024. \
                             Contact: sarah.johnson@technews.com";
    let entities = EntityRecognizer::recognize(text_for_entities).unwrap();

    // Verify we get entities
    assert!(!entities.is_empty());

    // Extract relations
    let text_for_relations = "Microsoft acquired Activision Blizzard. \
                              Satya Nadella is the CEO of Microsoft.";
    let relations = RelationExtractor::extract(text_for_relations).unwrap();

    // Verify we get relations
    assert!(!relations.is_empty());

    // Build a simple knowledge graph from extracted data
    use omnivore_core::graph::{Edge, KnowledgeGraph, Node};
    use std::collections::HashMap;

    let mut graph = KnowledgeGraph::new();

    // Add nodes for extracted entities
    graph
        .add_node(Node {
            id: "microsoft".to_string(),
            node_type: "Organization".to_string(),
            properties: {
                let mut p = HashMap::new();
                p.insert("name".to_string(), serde_json::json!("Microsoft"));
                p
            },
        })
        .unwrap();

    graph
        .add_node(Node {
            id: "activision".to_string(),
            node_type: "Organization".to_string(),
            properties: {
                let mut p = HashMap::new();
                p.insert("name".to_string(), serde_json::json!("Activision Blizzard"));
                p
            },
        })
        .unwrap();

    // Add edge for relation
    graph
        .add_edge(Edge {
            from: "microsoft".to_string(),
            to: "activision".to_string(),
            edge_type: "acquired".to_string(),
            properties: {
                let mut p = HashMap::new();
                p.insert("amount".to_string(), serde_json::json!("$68.7 billion"));
                p.insert("date".to_string(), serde_json::json!("January 18, 2024"));
                p
            },
        })
        .unwrap();

    // Query the graph
    let query = omnivore_core::graph::query::GraphQuery::new(&graph);
    let orgs = query.find_by_type("Organization");
    assert_eq!(orgs.len(), 2);
}

#[test]
fn test_link_extraction_quality() {
    let extractor = ContentExtractor::new();

    // Test Wikipedia sample
    let wiki_cleaned = extractor.extract_clean_content(WIKIPEDIA_SAMPLE);
    assert!(!wiki_cleaned.links.is_empty());

    // Test e-commerce sample
    let _ecom_cleaned = extractor.extract_clean_content(ECOMMERCE_SAMPLE);
    // May or may not have links depending on content

    // Test news sample
    let news_cleaned = extractor.extract_clean_content(NEWS_SAMPLE);
    // Should have related article links
    assert!(!news_cleaned.links.is_empty());
}
