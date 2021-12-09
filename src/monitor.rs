use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub type MonitorSender = flume::Sender<Command>;
pub type MonitorReceiver = flume::Receiver<Command>;

pub enum Command {
    Refresh,
    Shutdown,
    Update(Vec<String>),
}

pub struct Monitor {
    listener: TcpListener,
    tx:       MonitorSender,
}

impl Monitor {
    pub fn new(tx: MonitorSender, port: u32) -> Self {
        let listener =
            TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        Self { listener, tx }
    }

    pub fn run(self) {
        use Command::*;

        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut data = String::new();
            stream.read_to_string(&mut data).unwrap();

            let mut data = data.split(" ");

            let result = match data.next() {
                Some("refresh") => self.tx.send(Refresh),
                Some("shutdown") => self.tx.send(Shutdown),
                Some("update") => {
                    let names =
                        data.map(|name| name.to_string()).collect::<Vec<_>>();
                    self.tx.send(Update(names))
                },
                _ => Ok(()),
            };

            if let Err(_err) = result {
                unimplemented!("Monitor panicked!")
            }
        }
    }
}
