use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::monitor::{Command, MonitorSender};

pub type ResultsSender<T> = mpsc::SyncSender<ResultPacket<T>>;
pub type ResultsReceiver<T> = mpsc::Receiver<ResultPacket<T>>;
pub type JobsSender<T> = mpsc::SyncSender<Message<T>>;
pub type JobsReceiver<T> = mpsc::Receiver<Message<T>>;

/// A pool of threads for executing work.
pub struct ThreadPool<T> {
    pub jobs_tx: JobsSender<T>,
    workers:     Vec<Worker>,
}

/// Represents a message that Workers should respond to with a ResultPacket.
pub enum Message<T> {
    Job(JobPacket<T>),
    Terminate,
}

/// Contains information required to complete a job.
pub struct JobPacket<T> {
    pub job:       fn() -> T,
    pub return_tx: ResultsSender<T>,
}

/// Contains the result of an evaluated job.
pub struct ResultPacket<T> {
    pub result: T,
}

impl<T: Send + 'static> ThreadPool<T> {
    /// Returns a new ThreadPool with the given number of threads.
    ///
    /// size -> The number of threads in the pool.
    ///
    /// # Panics
    ///
    /// Will panic if the number of threads is 0.
    pub fn new(size: usize, monitor_tx: MonitorSender) -> Self {
        assert!(size > 0,);

        let (jobs_tx, jobs_rx) = mpsc::sync_channel(100);
        let jobs_rx = Arc::new(Mutex::new(jobs_rx));

        let workers = (0..size)
            .map(|_| Worker::new(jobs_rx.clone(), monitor_tx.clone()))
            .collect();

        ThreadPool { workers, jobs_tx }
    }
}

/// Represents a single worker thread.
struct Worker {
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    /// Creates a new Worker that will listen for jobs on jobs_rx, respond on
    /// the given return channel, and notify the framework via monitor_tx.
    pub fn new<T: Send + 'static>(
        jobs_rx: Arc<Mutex<JobsReceiver<T>>>, monitor_tx: MonitorSender,
    ) -> Self {
        let handle =
            Some(thread::spawn(move || Self::listen(jobs_rx, monitor_tx)));
        Self { handle }
    }

    fn listen<T: Send + 'static>(
        jobs_rx: Arc<Mutex<JobsReceiver<T>>>, monitor_tx: MonitorSender,
    ) {
        loop {
            let message = jobs_rx.lock().unwrap().recv().unwrap();

            match message {
                Message::Terminate => break,
                Message::Job(JobPacket { job, return_tx }) => {
                    return_tx.send(ResultPacket { result: job() }).unwrap();
                    monitor_tx.send(Command::Refresh).unwrap();
                },
            }
        }
    }
}

impl<T> Drop for ThreadPool<T> {
    // When the ThreadPool is dropped, tell each Worker to stop and collect
    // their JoinHandles before continuing.
    fn drop(&mut self) {
        for _ in &self.workers {
            self.jobs_tx.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workers_can_terminate() {
        let (jobs_tx, jobs_rx): (JobsSender<String>, JobsReceiver<String>) =
            mpsc::sync_channel(10);
        let jobs_rx = Arc::new(Mutex::new(jobs_rx));

        let (monitor_tx, _monitor_rx) = mpsc::sync_channel(10);
        let worker = Worker::new(jobs_rx, monitor_tx);

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }

    #[test]
    fn workers_return_correct_values() {
        let (jobs_tx, jobs_rx) = mpsc::sync_channel(10);
        let (results_tx, results_rx) = mpsc::sync_channel(10);
        let jobs_rx = Arc::new(Mutex::new(jobs_rx));

        let (monitor_tx, _monitor_rx) = mpsc::sync_channel(10);
        let worker = Worker::new(jobs_rx, monitor_tx);

        jobs_tx
            .send(Message::Job(JobPacket {
                job:       || String::from("the test worked :)"),
                return_tx: results_tx,
            }))
            .unwrap();

        let result = results_rx.recv().unwrap().result;
        assert_eq!(result, "the test worked :)");

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }
}
