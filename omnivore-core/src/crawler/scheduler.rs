use std::future::Future;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

pub struct Scheduler {
    semaphore: Arc<Semaphore>,
    handles: Vec<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new(max_workers: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_workers)),
            handles: Vec::new(),
        }
    }

    pub async fn spawn<F>(&self, task: F) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        tokio::spawn(async move {
            let _permit = permit; // Hold permit until task completes
            task.await;
        })
    }

    pub async fn shutdown(&self) {
        for handle in &self.handles {
            handle.abort();
        }
    }

    pub fn active_workers(&self) -> usize {
        let max = self.semaphore.available_permits();
        let total = self.semaphore.available_permits() + 
                   (self.handles.len() - self.handles.iter().filter(|h| h.is_finished()).count());
        total - max
    }
}
