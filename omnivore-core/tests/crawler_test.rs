use omnivore_core::{crawler::Crawler, CrawlConfig};
use url::Url;

#[tokio::test]
async fn test_crawler_creation() {
    let config = CrawlConfig::default();
    let crawler = Crawler::new(config).await;
    assert!(crawler.is_ok());
}

#[tokio::test]
async fn test_add_seed_url() {
    let config = CrawlConfig::default();
    let crawler = Crawler::new(config).await.unwrap();
    
    let url = Url::parse("https://example.com").unwrap();
    let result = crawler.add_seed(url).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_frontier() {
    use omnivore_core::crawler::frontier::Frontier;
    
    let mut frontier = Frontier::new();
    let url1 = Url::parse("https://example.com/page1").unwrap();
    let url2 = Url::parse("https://example.com/page2").unwrap();
    
    frontier.add(url1.clone(), 0).unwrap();
    frontier.add(url2.clone(), 1).unwrap();
    
    assert_eq!(frontier.size(), 2);
    
    let (next_url, depth) = frontier.get_next().unwrap();
    assert_eq!(depth, 0);
    assert_eq!(next_url, url1);
}

#[tokio::test]
async fn test_politeness_engine() {
    use omnivore_core::{crawler::politeness::PolitenessEngine, PolitenessConfig};
    
    let config = PolitenessConfig::default();
    let engine = PolitenessEngine::new(config);
    
    let url = Url::parse("https://example.com").unwrap();
    let can_crawl = engine.can_crawl(&url).await;
    assert!(can_crawl);
}