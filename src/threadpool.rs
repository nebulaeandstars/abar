use std::sync::mpsc;
use std::thread::{self, JoinHandle};

struct Worker
{
    id:     usize,
    handle: Option<JoinHandle<()>>,
}

enum Message
{
    NewJob
    {
        name: &'static str,
        job:  fn() -> String,
    },
    Terminate,
}

struct ResultPacket
{
    name:   &'static str,
    result: String,
}

type ResultsSender = mpsc::Sender<ResultPacket>;

type JobsSender = spmc::Sender<Message>;
type JobsReceiver = spmc::Receiver<Message>;

impl Worker
{
    pub fn new(id: usize, rx: JobsReceiver, tx: ResultsSender) -> Self
    {
        let handle = Some(thread::spawn(move || Self::listen(id, rx, tx)));
        Worker { id, handle }
    }

    fn listen(id: usize, rx: JobsReceiver, tx: ResultsSender)
    {
        loop {
            let message = rx.recv().unwrap();

            match message {
                Message::NewJob { name, job } =>
                    tx.send(ResultPacket { name, result: job() }).unwrap(),
                Message::Terminate => break,
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn workers_can_terminate()
    {
        let (mut jobs_tx, jobs_rx) = spmc::channel();
        let (results_tx, _) = mpsc::channel();
        let worker = Worker::new(0, jobs_rx, results_tx);

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }

    #[test]
    fn workers_return_correct_values()
    {
        let (mut jobs_tx, jobs_rx) = spmc::channel();
        let (results_tx, results_rx) = mpsc::channel();
        let worker = Worker::new(0, jobs_rx, results_tx);

        jobs_tx
            .send(Message::NewJob {
                name: "test",
                job:  || String::from("the test worked :)"),
            })
            .unwrap();

        let result = results_rx.recv().unwrap().result;
        assert_eq!(result, "the test worked :)");

        jobs_tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }
}
