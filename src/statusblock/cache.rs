use std::time::{Duration, Instant};

use crate::threadpool::{
    JobPacket, JobsSender, Message, ResultsReceiver, ResultsSender, ThreadPool,
};

/// A self-updating cache on a timer. Will probably need to be broken down.
///
/// TimedCaches will try to evaluate in the background if a JobsSender has been
/// provided via attach_threadpool(). Otherwise, they'll evaluate locally.
pub struct TimedCache<T> {
    value:           T,
    function:        fn() -> T,
    last_update:     Option<Instant>,
    update_interval: Option<Duration>,
    jobs_tx:         Option<JobsSender<T>>,
    results_tx:      Option<ResultsSender<T>>,
    results_rx:      Option<ResultsReceiver<T>>,
    waiting:         bool,
}

impl<T: Default> TimedCache<T> {
    pub fn new(update_interval: Option<Duration>, f: fn() -> T) -> Self {
        Self {
            value: T::default(),
            function: f,
            last_update: None,
            update_interval,
            jobs_tx: None,
            results_tx: None,
            results_rx: None,
            waiting: false,
        }
    }
}

#[allow(dead_code)]
impl<T> TimedCache<T> {
    pub fn with_initial_value(
        initial: T, update_interval: Option<Duration>, f: fn() -> T,
    ) -> Self {
        Self {
            value: initial,
            function: f,
            last_update: None,
            update_interval,
            jobs_tx: None,
            results_tx: None,
            results_rx: None,
            waiting: false,
        }
    }

    pub fn next_update(&self) -> Option<Instant> {
        if !self.waiting {
            match (self.update_interval, self.last_update) {
                (Some(interval), Some(last_update)) =>
                    Some(last_update + interval),
                (None, Some(_)) => None,
                (_, None) => Some(Instant::now()),
            }
        }
        else {
            None
        }
    }

    pub fn get(&mut self) -> &T {
        self.update();
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.update();
        &mut self.value
    }

    pub fn update(&mut self) {
        if self.results_rx.is_some() && self.waiting {
            let packet = self.results_rx.as_ref().unwrap().try_recv();

            if let Ok(packet) = packet {
                self.overwrite(packet.result);
                self.waiting = false;
            }

            return;
        }

        match self.last_update {
            Some(last_update) => {
                let now = Instant::now();
                let time_in_cache = now.duration_since(last_update);

                let time_for_update = self.update_interval.is_some()
                    && time_in_cache > self.update_interval.unwrap();

                if time_for_update {
                    self.update_now();
                }
            },
            None => self.update_now(),
        }
    }

    pub fn update_now(&mut self) {
        match &self.jobs_tx {
            // If there's no threadpool, update now.
            None => {
                let value = (self.function)();
                self.overwrite(value)
            },
            // Otherwise, create a job.
            Some(tx) => {
                let job = JobPacket {
                    job:       self.function,
                    return_tx: self.results_tx.as_ref().unwrap().clone(),
                };

                // Try to send the job as a message
                let result = tx.send(Message::Job(job));

                // If the threadpool has disconnected, go back to
                // single-threaded mode.
                if result.is_err() {
                    self.jobs_tx = None;
                    self.results_tx = None;
                    self.results_rx = None;
                }
                else {
                    self.waiting = true;
                }
            },
        }
    }

    pub fn attach_threadpool(&mut self, pool: &ThreadPool<T>) {
        let jobs_tx = pool.jobs_tx.clone();
        let (results_tx, results_rx) = flume::bounded(1);

        self.jobs_tx = Some(jobs_tx);
        self.results_tx = Some(results_tx);
        self.results_rx = Some(results_rx);
    }

    pub fn overwrite(&mut self, value: T) {
        self.value = value;
        self.last_update = Some(Instant::now());
    }
}

impl<T: Clone + Default> Default for TimedCache<T> {
    fn default() -> Self {
        Self {
            value:           T::default(),
            function:        T::default,
            last_update:     None,
            update_interval: None,
            jobs_tx:         None,
            results_tx:      None,
            results_rx:      None,
            waiting:         false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_evaluates_on_get() {
        let interval = Duration::from_secs(1);
        let mut cache =
            TimedCache::with_initial_value(0, Some(interval), || 1 + 1);

        assert_eq!(*cache.get(), 2);
    }

    #[test]
    fn cache_evaluates_after_update_interval() {
        let interval = Duration::from_millis(100);
        let mut cache = TimedCache::with_initial_value(
            Instant::now(),
            Some(interval),
            Instant::now,
        );

        let first_value = *cache.get();

        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(first_value, *cache.get());

        std::thread::sleep(Duration::from_millis(50));
        assert_ne!(first_value, *cache.get());
    }

    #[test]
    fn cache_evaluates_when_forced() {
        let interval = Duration::from_millis(100);
        let mut cache = TimedCache::with_initial_value(
            Instant::now(),
            Some(interval),
            Instant::now,
        );

        let first_value = *cache.get();

        cache.update_now();
        assert_ne!(first_value, *cache.get());
    }

    #[test]
    fn cache_does_not_update_when_interval_is_none() {
        let mut cache =
            TimedCache::with_initial_value(Instant::now(), None, Instant::now);
        let first_value = *cache.get();

        assert_eq!(first_value, *cache.get());
        assert_eq!(first_value, *cache.get());
        assert_eq!(first_value, *cache.get());
    }
}
