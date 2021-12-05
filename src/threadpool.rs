use std::thread::{self, JoinHandle};

pub type ResultsSender<T> = flume::Sender<ResultPacket<T>>;
pub type ResultsReceiver<T> = flume::Receiver<ResultPacket<T>>;
pub type JobsSender<T> = flume::Sender<Message<T>>;
pub type JobsReceiver<T> = flume::Receiver<Message<T>>;

pub struct ThreadPool<T> {
    pub jobs_tx: JobsSender<T>,
    workers:     Vec<Worker>,
}

pub enum Message<T> {
    Job(JobPacket<T>),
    Terminate,
}

pub struct JobPacket<T> {
    pub job:       fn() -> T,
    pub return_tx: ResultsSender<T>,
}

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
    pub fn new(size: usize) -> Self {
        assert!(size > 0,);

        let (jobs_tx, jobs_rx) = flume::bounded(100);

        let workers =
            (0..size).map(|i| Worker::new(i, jobs_rx.clone())).collect();

        ThreadPool { workers, jobs_tx }
    }

    pub fn execute(&mut self, job: JobPacket<T>) {
        let message = Message::Job(job);
        self.jobs_tx.send(message).unwrap();
    }
}

struct Worker {
    id:     usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new<T: Send + 'static>(id: usize, rx: JobsReceiver<T>) -> Self {
        let handle = Some(thread::spawn(move || Self::listen(rx)));
        Self { id, handle }
    }

    fn listen<T: Send + 'static>(rx: JobsReceiver<T>) {
        loop {
            let message = rx.recv().unwrap();

            match message {
                Message::Terminate => break,
                Message::Job(JobPacket { job, return_tx }) =>
                    return_tx.send(ResultPacket { result: job() }).unwrap(),
            }
        }
    }
}

impl<T> Drop for ThreadPool<T> {
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
        let (mut jobs_tx, jobs_rx): (JobsSender<String>, JobsReceiver<String>) =
            flume::bounded(10);
        let worker = Worker::new(0, jobs_rx);

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }

    #[test]
    fn workers_return_correct_values() {
        let (mut jobs_tx, jobs_rx): (JobsSender<String>, JobsReceiver<String>) =
            flume::bounded(10);
        let (results_tx, results_rx) = flume::bounded(10);
        let worker = Worker::new(0, jobs_rx);

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

    #[test]
    fn pool_returns_correct_values() {
        let (results_tx, results_rx) = flume::bounded(10);
        let mut pool = ThreadPool::new(4);

        pool.execute(JobPacket {
            job:       || String::from("test1"),
            return_tx: results_tx,
        });

        let result = results_rx.recv().unwrap().result;
        assert_eq!(result, "test1");
    }
}
