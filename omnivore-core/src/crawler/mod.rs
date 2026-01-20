pub mod browser;
pub mod frontier;
pub mod politeness;
pub mod robots;
pub mod scheduler;
pub mod worker;

use crate::{CrawlConfig, CrawlResult, CrawlStats, Result};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Notify, RwLock};
use url::Url;

/// A bounded buffer for crawl results that can flush to disk when full.
struct ResultsBuffer {
    /// Results held in memory
    results: Vec<CrawlResult>,
    /// Maximum results to keep in memory
    max_in_memory: usize,
    /// Path to flush results to (JSON Lines format)
    flush_path: Option<String>,
    /// Count of results that have been flushed to disk
    flushed_count: usize,
}

impl ResultsBuffer {
    fn new(max_in_memory: usize, flush_path: Option<String>) -> Self {
        Self {
            results: Vec::with_capacity(max_in_memory.min(1000)),
            max_in_memory,
            flush_path,
            flushed_count: 0,
        }
    }

    /// Add a result to the buffer, flushing to disk if necessary.
    async fn push(&mut self, result: CrawlResult) -> Result<()> {
        self.results.push(result);

        // Check if we need to flush
        if self.max_in_memory > 0 && self.results.len() >= self.max_in_memory {
            self.flush().await?;
        }

        Ok(())
    }

    /// Flush all results to disk (if a path is configured).
    async fn flush(&mut self) -> Result<()> {
        if self.results.is_empty() {
            return Ok(());
        }

        if let Some(ref path) = self.flush_path {
            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await
                .map_err(std::io::Error::other)?;

            for result in &self.results {
                let line = serde_json::to_string(result)?;
                file.write_all(line.as_bytes())
                    .await
                    .map_err(std::io::Error::other)?;
                file.write_all(b"\n").await.map_err(std::io::Error::other)?;
            }

            file.flush().await.map_err(std::io::Error::other)?;

            tracing::info!("Flushed {} results to {}", self.results.len(), path);
            self.flushed_count += self.results.len();
            self.results.clear();
        } else if self.max_in_memory > 0 {
            // No flush path but memory limit reached - log warning
            tracing::warn!(
                "Results buffer full ({} items) but no flush path configured",
                self.results.len()
            );
        }

        Ok(())
    }

    /// Get all results currently in memory.
    fn get_results(&self) -> Vec<CrawlResult> {
        self.results.clone()
    }

    /// Get total count (in memory + flushed).
    fn total_count(&self) -> usize {
        self.results.len() + self.flushed_count
    }
}

pub struct Crawler {
    config: Arc<CrawlConfig>,
    scheduler: scheduler::Scheduler,
    frontier: Arc<RwLock<frontier::Frontier>>,
    politeness_engine: Arc<politeness::PolitenessEngine>,
    stats: Arc<RwLock<CrawlStats>>,
    results: Arc<RwLock<ResultsBuffer>>,
    /// Notifier for event-driven work completion
    work_completed: Arc<Notify>,
}

impl Crawler {
    pub async fn new(config: CrawlConfig) -> Result<Self> {
        let results_buffer = ResultsBuffer::new(
            config.max_results_in_memory,
            config.results_flush_path.clone(),
        );
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
        let results = Arc::new(RwLock::new(results_buffer));
        let work_completed = Arc::new(Notify::new());

        Ok(Self {
            config,
            scheduler,
            frontier,
            politeness_engine,
            stats,
            results,
            work_completed,
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
                let work_completed = self.work_completed.clone();

                self.scheduler
                    .spawn(async move {
                        let worker = match worker::Worker::new(config.clone()) {
                            Ok(w) => w,
                            Err(e) => {
                                tracing::error!("Failed to create worker: {}", e);
                                let mut stats = stats.write().await;
                                stats.failed += 1;
                                stats.in_progress -= 1;
                                work_completed.notify_one();
                                return;
                            }
                        };
                        match worker.crawl(url.clone()).await {
                            Ok(result) => {
                                politeness.record_crawl(&url).await;

                                // Store the crawl result with bounded memory
                                {
                                    let mut results_guard = results.write().await;
                                    if let Err(e) = results_guard.push(result.clone()).await {
                                        tracing::warn!("Failed to store result: {}", e);
                                    }
                                }

                                // Update stats
                                {
                                    let mut stats = stats.write().await;
                                    stats.successful += 1;
                                    stats.in_progress -= 1;
                                }

                                // Batch add discovered links
                                let links: Vec<Url> = result
                                    .links
                                    .iter()
                                    .filter_map(|link_str| Url::parse(link_str).ok())
                                    .collect();

                                if !links.is_empty() {
                                    let mut frontier = frontier.write().await;
                                    for link_url in links {
                                        if let Err(e) = frontier.add(link_url.clone(), depth + 1) {
                                            tracing::warn!(
                                                "Failed to add {} to frontier: {}",
                                                link_url,
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to crawl {url}: {e}");
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
                                    if let Err(write_err) =
                                        file.write_all(error_entry.as_bytes()).await
                                    {
                                        tracing::warn!("Failed to write error log: {}", write_err);
                                    }
                                }

                                let mut stats = stats.write().await;
                                stats.failed += 1;
                                stats.in_progress -= 1;
                            }
                        }
                        // Notify that work is completed
                        work_completed.notify_one();
                    })
                    .await;

                let mut stats = self.stats.write().await;
                stats.in_progress += 1;
                stats.total_urls += 1;
            } else {
                // No URLs in frontier - check if we should wait or exit
                let in_progress = {
                    let stats = self.stats.read().await;
                    stats.in_progress
                };

                if in_progress == 0 {
                    // No work in progress and frontier is empty - we're done
                    break;
                }

                // Wait for a worker to complete instead of busy-waiting
                // Use a timeout to periodically check the frontier for new URLs
                tokio::select! {
                    _ = self.work_completed.notified() => {
                        // A worker completed, check for new URLs
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(500)) => {
                        // Timeout - check again
                    }
                }
            }

            let mut stats = self.stats.write().await;
            stats.elapsed_time = start_time.elapsed();
        }

        // Flush any remaining results to disk
        let mut results = self.results.write().await;
        if let Err(e) = results.flush().await {
            tracing::warn!("Failed to flush final results: {}", e);
        }

        Ok(())
    }

    pub async fn get_stats(&self) -> CrawlStats {
        self.stats.read().await.clone()
    }

    /// Get results currently in memory.
    /// Note: If streaming to disk, some results may have been flushed.
    pub async fn get_results(&self) -> Vec<CrawlResult> {
        self.results.read().await.get_results()
    }

    /// Get total count of results (in memory + flushed to disk).
    pub async fn get_total_results_count(&self) -> usize {
        self.results.read().await.total_count()
    }

    pub async fn stop(&self) {
        self.scheduler.shutdown().await;
    }
}
