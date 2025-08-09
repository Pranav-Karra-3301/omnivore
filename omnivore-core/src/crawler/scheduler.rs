use std::future::Future;
use tokio::task::JoinHandle;

pub struct Scheduler {
    max_workers: usize,
    handles: Vec<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            handles: Vec::new(),
        }
    }

    pub async fn spawn<F>(&self, task: F) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(task)
    }

    pub async fn shutdown(&self) {
        for handle in &self.handles {
            handle.abort();
        }
    }

    pub fn active_workers(&self) -> usize {
        self.handles.iter().filter(|h| !h.is_finished()).count()
    }
}