use crate::{Error, Result};
use dashmap::DashMap;
use robotstxt::DefaultMatcher;
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

/// Cached robots.txt data with parsed rules and metadata
struct CachedRobots {
    /// Raw robots.txt content for rule checking
    robots_txt: String,
    /// Extracted crawl delay (if specified)
    crawl_delay: Option<Duration>,
    /// When this entry was fetched
    fetched_at: Instant,
    /// How long to keep this cached
    ttl: Duration,
}

/// Robots.txt compliance checker with caching
///
/// This checker fetches and caches robots.txt files, parsing them according
/// to the Robots Exclusion Protocol. It respects User-agent, Allow, Disallow,
/// and Crawl-delay directives.
pub struct RobotsChecker {
    cache: Arc<DashMap<String, CachedRobots>>,
    client: reqwest::Client,
    user_agent: String,
}

impl RobotsChecker {
    /// Create a new RobotsChecker with the given user agent string.
    ///
    /// The user agent is used to match against User-agent directives in robots.txt.
    ///
    /// # Example
    /// ```ignore
    /// let checker = RobotsChecker::new("OmnivoreBot/1.0".to_string());
    /// ```
    pub fn new(user_agent: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(&user_agent)
            .build()
            .map_err(Error::Network)?;

        Ok(Self {
            cache: Arc::new(DashMap::new()),
            client,
            user_agent,
        })
    }

    /// Check if crawling the given URL is allowed according to robots.txt.
    ///
    /// This method will:
    /// 1. Check the cache for a valid robots.txt entry
    /// 2. If not cached or expired, fetch the robots.txt from the server
    /// 3. Parse the robots.txt and check if the URL is allowed
    ///
    /// # Returns
    /// - `Ok(true)` if crawling is allowed
    /// - `Ok(false)` if crawling is disallowed
    /// - `Err` if there was an error fetching or parsing
    ///
    /// # Note
    /// If robots.txt cannot be fetched (404, network error), crawling is assumed allowed.
    pub async fn is_allowed(&self, url: &Url) -> Result<bool> {
        let robots_url = self.get_robots_url(url)?;
        let domain = url
            .domain()
            .ok_or_else(|| Error::Parse("Invalid domain".to_string()))?;

        // Check cache first
        if let Some(cached) = self.cache.get(domain) {
            if cached.fetched_at.elapsed() < cached.ttl {
                return Ok(self.check_robots_txt(&cached.robots_txt, url));
            }
        }

        // Fetch fresh robots.txt
        let robots_txt = self.fetch_robots_txt(&robots_url).await?;

        // Parse and extract crawl delay
        let crawl_delay = self.extract_crawl_delay(&robots_txt);

        // Check if allowed
        let allowed = self.check_robots_txt(&robots_txt, url);

        // Cache the result
        self.cache.insert(
            domain.to_string(),
            CachedRobots {
                robots_txt,
                crawl_delay,
                fetched_at: Instant::now(),
                ttl: Duration::from_secs(3600), // Cache for 1 hour
            },
        );

        Ok(allowed)
    }

    /// Check if a URL is allowed according to the given robots.txt content.
    fn check_robots_txt(&self, robots_txt: &str, url: &Url) -> bool {
        // Empty robots.txt means everything is allowed
        if robots_txt.is_empty() {
            return true;
        }

        // Get the path to check (including query string)
        let path = match url.query() {
            Some(query) => format!("{}?{}", url.path(), query),
            None => url.path().to_string(),
        };

        // Use robotstxt crate to check if allowed
        // The API expects a slice of user agents to check against
        let mut matcher = DefaultMatcher::default();
        let user_agents = vec![self.user_agent.as_str()];
        matcher.allowed_by_robots(robots_txt, user_agents, &path)
    }

    /// Extract Crawl-delay directive from robots.txt content.
    ///
    /// Looks for lines like:
    /// - `Crawl-delay: 10`
    /// - `Crawl-delay: 0.5`
    fn extract_crawl_delay(&self, robots_txt: &str) -> Option<Duration> {
        let mut in_matching_section = false;
        let user_agent_lower = self.user_agent.to_lowercase();

        for line in robots_txt.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line_lower = line.to_lowercase();

            // Check for User-agent directive
            if line_lower.starts_with("user-agent:") {
                let agent = line_lower.trim_start_matches("user-agent:").trim();
                // Match our user agent or wildcard
                in_matching_section = agent == "*"
                    || user_agent_lower.contains(agent)
                    || agent.contains(&user_agent_lower);
            }
            // Check for Crawl-delay directive in matching section
            else if in_matching_section && line_lower.starts_with("crawl-delay:") {
                let delay_str = line_lower.trim_start_matches("crawl-delay:").trim();
                if let Ok(delay_secs) = delay_str.parse::<f64>() {
                    if delay_secs > 0.0 && delay_secs < 3600.0 {
                        // Cap at 1 hour to prevent abuse
                        return Some(Duration::from_secs_f64(delay_secs));
                    }
                }
            }
        }

        None
    }

    /// Get the robots.txt URL for a given page URL.
    fn get_robots_url(&self, url: &Url) -> Result<Url> {
        let mut robots_url = url.clone();
        robots_url.set_path("/robots.txt");
        robots_url.set_query(None);
        robots_url.set_fragment(None);
        Ok(robots_url)
    }

    /// Fetch robots.txt content from a URL.
    ///
    /// Returns empty string if:
    /// - The file doesn't exist (404)
    /// - There's a network error
    /// - The server returns an error status
    ///
    /// This follows the standard robots.txt convention where missing or
    /// inaccessible robots.txt means no restrictions.
    async fn fetch_robots_txt(&self, url: &Url) -> Result<String> {
        match self.client.get(url.as_str()).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    response.text().await.map_err(Error::Network)
                } else {
                    // 404 or other errors mean no robots.txt restrictions
                    tracing::debug!(
                        "robots.txt not found or error for {}: status {}",
                        url,
                        response.status()
                    );
                    Ok(String::new())
                }
            }
            Err(e) => {
                // Network errors mean we can't check, so allow (but log)
                tracing::warn!("Failed to fetch robots.txt for {}: {}", url, e);
                Ok(String::new())
            }
        }
    }

    /// Get the crawl delay for a domain, if specified in its robots.txt.
    ///
    /// Returns `None` if:
    /// - No robots.txt is cached for the domain
    /// - No Crawl-delay directive was specified
    /// - The cached entry has expired
    pub fn get_crawl_delay(&self, domain: &str) -> Option<Duration> {
        self.cache.get(domain).and_then(|cached| {
            if cached.fetched_at.elapsed() < cached.ttl {
                cached.crawl_delay
            } else {
                None
            }
        })
    }

    /// Clear the robots.txt cache.
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Get the number of cached robots.txt entries.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_checker() -> RobotsChecker {
        RobotsChecker::new("TestBot/1.0".to_string()).unwrap()
    }

    #[test]
    fn test_empty_robots_allows_all() {
        let checker = create_checker();
        let url = Url::parse("https://example.com/page").unwrap();
        assert!(checker.check_robots_txt("", &url));
    }

    #[test]
    fn test_disallow_all() {
        let checker = create_checker();
        let robots = "User-agent: *\nDisallow: /";
        let url = Url::parse("https://example.com/page").unwrap();
        assert!(!checker.check_robots_txt(robots, &url));
    }

    #[test]
    fn test_allow_specific_path() {
        let checker = create_checker();
        let robots = "User-agent: *\nDisallow: /private/\nAllow: /";
        let url_allowed = Url::parse("https://example.com/public/page").unwrap();
        let url_disallowed = Url::parse("https://example.com/private/secret").unwrap();

        assert!(checker.check_robots_txt(robots, &url_allowed));
        assert!(!checker.check_robots_txt(robots, &url_disallowed));
    }

    #[test]
    fn test_user_agent_specific_rules() {
        // Use exact user agent match for specific rules
        let checker = RobotsChecker::new("TestBot".to_string()).unwrap();
        let robots = "User-agent: TestBot\nDisallow: /blocked/\n\nUser-agent: *\nAllow: /";
        let url = Url::parse("https://example.com/blocked/page").unwrap();

        // TestBot should be blocked by the specific rule
        assert!(!checker.check_robots_txt(robots, &url));
    }

    #[test]
    fn test_wildcard_user_agent() {
        // Any user agent not specifically mentioned should match wildcard
        let checker = RobotsChecker::new("OtherBot/1.0".to_string()).unwrap();
        let robots = "User-agent: TestBot\nDisallow: /blocked/\n\nUser-agent: *\nAllow: /";
        let url = Url::parse("https://example.com/blocked/page").unwrap();

        // OtherBot should be allowed by the wildcard rule
        assert!(checker.check_robots_txt(robots, &url));
    }

    #[test]
    fn test_extract_crawl_delay() {
        let checker = create_checker();

        let robots_with_delay = "User-agent: *\nCrawl-delay: 5\nDisallow: /private/";
        let delay = checker.extract_crawl_delay(robots_with_delay);
        assert_eq!(delay, Some(Duration::from_secs(5)));

        let robots_no_delay = "User-agent: *\nDisallow: /private/";
        let no_delay = checker.extract_crawl_delay(robots_no_delay);
        assert!(no_delay.is_none());
    }

    #[test]
    fn test_extract_fractional_crawl_delay() {
        let checker = create_checker();
        let robots = "User-agent: *\nCrawl-delay: 0.5";
        let delay = checker.extract_crawl_delay(robots);
        assert_eq!(delay, Some(Duration::from_millis(500)));
    }

    #[test]
    fn test_query_string_handling() {
        let checker = create_checker();
        let robots = "User-agent: *\nDisallow: /search";

        let url_with_query = Url::parse("https://example.com/search?q=test").unwrap();
        assert!(!checker.check_robots_txt(robots, &url_with_query));
    }
}
