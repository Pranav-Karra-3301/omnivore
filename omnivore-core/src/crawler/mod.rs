pub mod browser;
pub mod frontier;
pub mod politeness;
pub mod robots;
pub mod scheduler;
pub mod worker;

use crate::{CrawlConfig, CrawlResult, CrawlStats, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

pub struct Crawler {
    config: Arc<CrawlConfig>,
    scheduler: scheduler::Scheduler,
    frontier: Arc<RwLock<frontier::Frontier>>,
    politeness_engine: Arc<politeness::PolitenessEngine>,
    stats: Arc<RwLock<CrawlStats>>,
    results: Arc<RwLock<Vec<CrawlResult>>>,
}

impl Crawler {
    pub async fn new(config: CrawlConfig) -> Result<Self> {
        let config = Arc::new(config);
        let scheduler = scheduler::Scheduler::new(config.max_workers);
        let frontier = Arc::new(RwLock::new(frontier::Frontier::new()));
        let politeness_engine =
            Arc::new(politeness::PolitenessEngine::new(config.politeness.clone()));
        let stats = Arc::new(RwLock::new(CrawlStats {
            total_urls: 0,
            successful: 0,
            failed: 0,
            in_progress: 0,
            average_response_time_ms: 0.0,
            start_time: chrono::Utc::now(),
            elapsed_time: std::time::Duration::from_secs(0),
        }));
        let results = Arc::new(RwLock::new(Vec::new()));

        Ok(Self {
            config,
            scheduler,
            frontier,
            politeness_engine,
            stats,
            results,
        })
    }

    pub async fn add_seed(&self, url: Url) -> Result<()> {
        let mut frontier = self.frontier.write().await;
        frontier.add(url, 0)?;
        Ok(())
    }

    pub async fn add_seeds(&self, urls: Vec<Url>) -> Result<()> {
        let mut frontier = self.frontier.write().await;
        for url in urls {
            frontier.add(url, 0)?;
        }
        Ok(())
    }

    pub async fn start(self: &Arc<Self>) -> Result<()> {
        let start_time = std::time::Instant::now();

        loop {
            let url_entry = {
                let mut frontier = self.frontier.write().await;
                frontier.get_next()
            };

            if let Some((url, depth)) = url_entry {
                if depth > self.config.max_depth {
                    continue;
                }

                let can_crawl = self.politeness_engine.can_crawl(&url).await;
                if !can_crawl {
                    let mut frontier = self.frontier.write().await;
                    frontier.add(url, depth)?;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    continue;
                }

                let config = self.config.clone();
                let frontier = self.frontier.clone();
                let politeness = self.politeness_engine.clone();
                let stats = self.stats.clone();
                let results = self.results.clone();

                self.scheduler
                    .spawn(async move {
                        let worker = worker::Worker::new(config.clone());
                        match worker.crawl(url.clone()).await {
                            Ok(result) => {
                                politeness.record_crawl(&url).await;

                                // Store the crawl result
                                let mut results_guard = results.write().await;
                                results_guard.push(result.clone());

                                let mut stats = stats.write().await;
                                stats.successful += 1;
                                stats.in_progress -= 1;

                                let mut frontier = frontier.write().await;
                                for link_str in result.links.iter() {
                                    if let Ok(link_url) = url::Url::parse(link_str) {
                                        let _ = frontier.add(link_url, depth + 1);
                                    }
                                }
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to crawl {}: {}", url, e);
                                tracing::error!("{}", error_msg);
                                
                                // Write to error log file
                                let error_entry = format!(
                                    "[{}] {}\n",
                                    chrono::Utc::now().to_rfc3339(),
                                    error_msg
                                );
                                if let Ok(mut file) = tokio::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open("error.log")
                                    .await
                                {
                                    use tokio::io::AsyncWriteExt;
                                    let _ = file.write_all(error_entry.as_bytes()).await;
                                }
                                
                                let mut stats = stats.write().await;
                                stats.failed += 1;
                                stats.in_progress -= 1;
                            }
                        }
                    })
                    .await;

                let mut stats = self.stats.write().await;
                stats.in_progress += 1;
                stats.total_urls += 1;
            } else {
                let stats = self.stats.read().await;
                if stats.in_progress == 0 {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }

            let mut stats = self.stats.write().await;
            stats.elapsed_time = start_time.elapsed();
        }

        Ok(())
    }

    pub async fn get_stats(&self) -> CrawlStats {
        self.stats.read().await.clone()
    }

    pub async fn get_results(&self) -> Vec<CrawlResult> {
        self.results.read().await.clone()
    }

    pub async fn stop(&self) {
        self.scheduler.shutdown().await;
    }
}
