use std::future::Future;
use tokio::task::JoinHandle;

pub struct Scheduler {
    handles: Vec<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(_max_workers: usize) -> Self {
        Self {
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