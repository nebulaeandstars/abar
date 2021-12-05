use std::sync::mpsc;
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    jobs_tx: JobsSender,
}

impl ThreadPool {
    /// Returns a new ThreadPool with the given number of threads.
    ///
    /// size -> The number of threads in the pool.
    ///
    /// # Panics
    ///
    /// Will panic if the number of threads is 0.
    pub fn new(size: usize, results_tx: ResultsSender) -> Self {
        assert!(size > 0,);

        let (jobs_tx, jobs_rx) = spmc::channel();

        let workers = (0..size)
            .map(|i| Worker::new(i, jobs_rx.clone(), results_tx.clone()))
            .collect();

        ThreadPool { workers, jobs_tx }
    }

    pub fn execute(&mut self, job: JobPacket) {
        let message = Message::Job(job);
        self.jobs_tx.send(message).unwrap();
    }
}

struct Worker {
    id:     usize,
    handle: Option<JoinHandle<()>>,
}

pub enum Message {
    Job(JobPacket),
    Terminate,
}

pub struct JobPacket {
    pub id:  String,
    pub job: fn() -> String,
}

pub struct ResultPacket {
    pub id:     String,
    pub result: String,
}

pub type ResultsSender = mpsc::Sender<ResultPacket>;
pub type ResultsReceiver = mpsc::Receiver<ResultPacket>;

pub type JobsSender = spmc::Sender<Message>;
pub type JobsReceiver = spmc::Receiver<Message>;

impl Worker {
    pub fn new(id: usize, rx: JobsReceiver, tx: ResultsSender) -> Self {
        let handle = Some(thread::spawn(move || Self::listen(id, rx, tx)));
        Worker { id, handle }
    }

    fn listen(id: usize, rx: JobsReceiver, tx: ResultsSender) {
        loop {
            let message = rx.recv().unwrap();

            match message {
                Message::Job(JobPacket { id, job }) =>
                    tx.send(ResultPacket { id, result: job() }).unwrap(),
                Message::Terminate => break,
            }
        }
    }
}

impl Drop for ThreadPool {
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
        let (mut jobs_tx, jobs_rx) = spmc::channel();
        let (results_tx, _) = mpsc::channel();
        let worker = Worker::new(0, jobs_rx, results_tx);

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }

    #[test]
    fn workers_return_correct_values() {
        let (mut jobs_tx, jobs_rx) = spmc::channel();
        let (results_tx, results_rx) = mpsc::channel();
        let worker = Worker::new(0, jobs_rx, results_tx);

        jobs_tx
            .send(Message::Job(JobPacket {
                id:  String::from("test"),
                job: || String::from("the test worked :)"),
            }))
            .unwrap();

        let result = results_rx.recv().unwrap().result;
        assert_eq!(result, "the test worked :)");

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }

    #[test]
    fn pool_returns_correct_values() {
        let (results_tx, results_rx) = mpsc::channel();
        let mut pool = ThreadPool::new(4, results_tx);

        pool.execute(JobPacket {
            id:  String::from("test"),
            job: || String::from("test1"),
        });

        let result = results_rx.recv().unwrap().result;
        assert_eq!(result, "test1");
    }
}
