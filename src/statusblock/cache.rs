use std::time::{Duration, Instant};

use crate::threadpool::{JobsSender, ResultsReceiver};

pub struct TimedCache<T> {
    value:           Option<T>,
    function:        fn() -> T,
    last_update:     Instant,
    update_interval: Option<Duration>,
    jobs_tx:         Option<JobsSender>,
    results_rx:      Option<ResultsReceiver>,
}

impl<T> TimedCache<T> {
    pub fn new(update_interval: Option<Duration>, f: fn() -> T) -> Self {
        Self {
            value: None,
            function: f,
            last_update: Instant::now(),
            update_interval,
            jobs_tx: None,
            results_rx: None,
        }
    }

    pub fn get(&mut self) -> &T {
        self.update();
        self.value.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.update();
        self.value.as_mut().unwrap()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let time_in_cache = now.duration_since(self.last_update);

        let time_for_update = self.update_interval.is_some()
            && time_in_cache > self.update_interval.unwrap();

        if self.value.is_none() || time_for_update {
            self.update_now();
        }
    }

    pub fn update_now(&mut self) {
        let value = (self.function)();
        self.value = Some(value);
        self.last_update = Instant::now();
    }
}

impl<T: Clone + Default> Default for TimedCache<T> {
    fn default() -> Self {
        Self {
            value:           None,
            function:        T::default,
            last_update:     Instant::now(),
            update_interval: None,
            jobs_tx:         None,
            results_rx:      None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_evaluates_on_get() {
        let interval = Duration::from_secs(1);
        let mut cache = TimedCache::new(Some(interval), || 1 + 1);

        assert_eq!(*cache.get(), 2);
    }

    #[test]
    fn cache_evaluates_after_update_interval() {
        let interval = Duration::from_millis(100);
        let mut cache = TimedCache::new(Some(interval), Instant::now);

        let first_value = *cache.get();

        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(first_value, *cache.get());

        std::thread::sleep(Duration::from_millis(50));
        assert_ne!(first_value, *cache.get());
    }

    #[test]
    fn cache_evaluates_when_forced() {
        let interval = Duration::from_millis(100);
        let mut cache = TimedCache::new(Some(interval), Instant::now);

        let first_value = *cache.get();

        cache.update_now();
        assert_ne!(first_value, *cache.get());
    }

    #[test]
    fn cache_does_not_update_when_update_interval_is_none() {
        let mut cache = TimedCache::new(None, Instant::now);
        let first_value = *cache.get();

        assert_eq!(first_value, *cache.get());
        assert_eq!(first_value, *cache.get());
        assert_eq!(first_value, *cache.get());
    }
}
