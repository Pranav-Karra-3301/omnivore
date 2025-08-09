use crate::{Error, Result};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

pub struct RobotsChecker {
    cache: Arc<DashMap<String, CachedRobots>>,
    client: reqwest::Client,
}

struct CachedRobots {
    robots_txt: String,
    fetched_at: Instant,
    ttl: Duration,
}

impl RobotsChecker {
    pub fn new(_user_agent: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            cache: Arc::new(DashMap::new()),
            client,
        }
    }

    pub async fn is_allowed(&self, url: &Url) -> Result<bool> {
        let robots_url = self.get_robots_url(url)?;
        let domain = url
            .domain()
            .ok_or_else(|| Error::Parse("Invalid domain".to_string()))?;

        if let Some(cached) = self.cache.get(domain) {
            if cached.fetched_at.elapsed() < cached.ttl {
                return Ok(self.check_robots_txt(&cached.robots_txt, url.as_str()));
            }
        }

        let robots_txt = self.fetch_robots_txt(&robots_url).await?;

        let allowed = self.check_robots_txt(&robots_txt, url.as_str());

        self.cache.insert(
            domain.to_string(),
            CachedRobots {
                robots_txt,
                fetched_at: Instant::now(),
                ttl: Duration::from_secs(3600),
            },
        );

        Ok(allowed)
    }

    fn check_robots_txt(&self, _robots_txt: &str, _url: &str) -> bool {
        // TODO: Implement proper robots.txt parsing
        // For now, allow all URLs
        true
    }

    fn get_robots_url(&self, url: &Url) -> Result<Url> {
        let mut robots_url = url.clone();
        robots_url.set_path("/robots.txt");
        robots_url.set_query(None);
        robots_url.set_fragment(None);
        Ok(robots_url)
    }

    async fn fetch_robots_txt(&self, url: &Url) -> Result<String> {
        match self.client.get(url.as_str()).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response.text().await?)
                } else {
                    Ok(String::new())
                }
            }
            Err(_) => Ok(String::new()),
        }
    }

    pub fn get_crawl_delay(&self, domain: &str) -> Option<Duration> {
        self.cache.get(domain).and_then(|_cached| None)
    }
}
