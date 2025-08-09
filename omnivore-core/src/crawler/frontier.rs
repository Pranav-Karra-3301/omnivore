use crate::{Error, Result};
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use url::Url;

#[derive(Clone)]
pub struct Frontier {
    queue: PriorityQueue<Url, i32>,
    seen: HashSet<String>,
}

impl Frontier {
    pub fn new() -> Self {
        Self {
            queue: PriorityQueue::new(),
            seen: HashSet::new(),
        }
    }

    pub fn add(&mut self, url: Url, depth: u32) -> Result<()> {
        let url_str = url.as_str();

        if !self.seen.contains(url_str) {
            self.seen.insert(url_str.to_string());
            let priority = -(depth as i32);
            self.queue.push(url, priority);
        }

        Ok(())
    }

    pub fn get_next(&mut self) -> Option<(Url, u32)> {
        self.queue.pop().map(|(url, priority)| {
            let depth = (-priority) as u32;
            (url, depth)
        })
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn contains(&self, url: &Url) -> bool {
        self.seen.contains(url.as_str())
    }
}