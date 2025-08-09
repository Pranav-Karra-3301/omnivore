use crate::PolitenessConfig;
use dashmap::DashMap;
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use url::Url;

pub struct PolitenessEngine {
    config: PolitenessConfig,
    domain_limiters: Arc<DashMap<String, Arc<RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>>>>,
    last_access: Arc<DashMap<String, Instant>>,
}

impl PolitenessEngine {
    pub fn new(config: PolitenessConfig) -> Self {
        Self {
            config,
            domain_limiters: Arc::new(DashMap::new()),
            last_access: Arc::new(DashMap::new()),
        }
    }

    pub async fn can_crawl(&self, url: &Url) -> bool {
        let domain = match url.domain() {
            Some(d) => d.to_string(),
            None => return false,
        };

        let limiter = self.get_or_create_limiter(&domain);
        
        if let Some(last) = self.last_access.get(&domain) {
            let elapsed = last.elapsed();
            let min_delay = Duration::from_millis(self.config.default_delay_ms);
            
            if elapsed < min_delay {
                return false;
            }
        }

        limiter.check().is_ok()
    }

    pub async fn record_crawl(&self, url: &Url) {
        if let Some(domain) = url.domain() {
            self.last_access.insert(domain.to_string(), Instant::now());
        }
    }

    fn get_or_create_limiter(&self, domain: &str) -> Arc<RateLimiter<governor::state::direct::NotKeyed, governor::state::InMemoryState, governor::clock::DefaultClock>> {
        self.domain_limiters
            .entry(domain.to_string())
            .or_insert_with(|| {
                let quota = Quota::per_second(
                    NonZeroU32::new(self.config.max_requests_per_second as u32)
                        .unwrap_or(NonZeroU32::new(1).unwrap())
                );
                Arc::new(RateLimiter::direct(quota))
            })
            .clone()
    }

    pub fn update_delay(&mut self, _domain: &str, delay_ms: u64) {
        self.config.default_delay_ms = delay_ms;
    }
}