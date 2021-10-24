use std::thread::{self, JoinHandle};

struct Worker
{
    id:     usize,
    handle: Option<JoinHandle<()>>,
}

enum Message
{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type Sender = spmc::Sender<Message>;
type Receiver = spmc::Receiver<Message>;

impl Worker
{
    pub fn new(id: usize, rx: Receiver) -> Self
    {
        let handle = Some(thread::spawn(move || Self::listen(id, rx)));
        Worker { id, handle }
    }

    fn listen(id: usize, rx: Receiver)
    {
        loop {
            let message = rx.recv().unwrap();

            match message {
                Message::NewJob(job) => job(),
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
    fn new_worker_terminates()
    {
        let (mut tx, rx) = spmc::channel();
        let worker = Worker::new(0, rx);

        tx.send(Message::Terminate).unwrap();
        worker.handle.unwrap().join().unwrap();
    }
}
