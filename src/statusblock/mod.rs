mod builder;
mod cache;

use std::fmt;
use std::sync::Mutex;
use std::time::Duration;

pub use builder::StatusBlockBuilder;
use cache::TimedCache;

use crate::threadpool::ThreadPool;

#[derive(Default)]
pub struct StatusBlock {
    pub name:     Option<String>,
    pub min_size: Option<usize>,
    pub max_size: Option<usize>,
    cache:        Mutex<TimedCache<String>>,
}

impl StatusBlock {
    pub fn new(f: fn() -> String, interval: Option<Duration>) -> Self {
        Self {
            cache: Mutex::new(TimedCache::new(interval, f)),
            ..Default::default()
        }
    }

    pub fn attach_threadpool(&self, pool: &ThreadPool<String>) {
        let mut cache = self.cache.lock().unwrap();
        cache.attach_threadpool(pool);
    }

    /// Updates the StatusBlock iff it's scheduled to be updated.
    pub fn update(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.update();

        drop(cache);
        self.truncate_cache()
    }

    /// Updates the StatusBlock immediately, ignoring the timer.
    pub fn update_now(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.update_now();

        drop(cache);
        self.truncate_cache()
    }

    fn truncate_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        let inner = cache.get_mut();

        if let Some(max) = self.max_size {
            inner.truncate(max);
        }
        if let Some(min) = self.min_size {
            if inner.len() < min {
                inner.push_str(&" ".repeat(min - inner.len()))
            }
        }
    }
}

impl fmt::Display for StatusBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cache = self.cache.lock().unwrap();
        let out = cache.get_mut();

        // TODO: Find out why this needs to be *here* and not in truncate_cache
        if let Some(max) = self.max_size {
            out.truncate(max);
        }
        if let Some(min) = self.min_size {
            if out.len() < min {
                out.push_str(&" ".repeat(min - out.len()))
            }
        }

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_correct_fields() {
        let block = StatusBlock::default();
        assert_eq!(block.name, None);
        assert_eq!(block.min_size, None);
        assert_eq!(block.max_size, None);
        assert_eq!(block.to_string(), "");
    }

    #[test]
    fn display_draws_the_cache() {
        let block = StatusBlock::new(|| String::from("test"), None);
        assert_eq!(block.to_string(), "test");
    }

    #[test]
    fn max_size_is_respected() {
        let mut block =
            StatusBlock::new(|| String::from("a very long string"), None);

        block.max_size = Some(10);

        block.update_now();
        assert_eq!(block.to_string(), "a very lon");
    }

    #[test]
    fn min_size_is_respected() {
        let mut block =
            StatusBlock::new(|| String::from("a short string"), None);

        block.min_size = Some(20);

        block.update_now();
        assert_eq!(block.to_string(), "a short string      ");
    }
}
