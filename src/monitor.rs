use std::io::Read;
use std::net::TcpListener;

pub type MonitorSender = flume::Sender<Command>;
pub type MonitorReceiver = flume::Receiver<Command>;

/// Represents a command that can be sent to the meta framework.
pub enum Command {
    Refresh,
    Shutdown,
    Update(Vec<String>),
}

/// A struct for monitoring and reacting to external input over TCP.
pub struct Monitor {
    listener: TcpListener,
    tx:       MonitorSender,
}

impl Monitor {
    /// Trt to bind to the given port and return a new Monitor.
    pub fn new(tx: MonitorSender, port: usize) -> Self {
        let listener =
            TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

        Self { listener, tx }
    }

    /// Listen for incoming connections, and pipe any received commands into
    /// self.tx.
    pub fn run(self) {
        use Command::*;

        for stream in self.listener.incoming() {
            // Unpack data from the stream.
            let mut stream = stream.unwrap();
            let mut data = String::new();
            stream.read_to_string(&mut data).unwrap();

            // Check the first statement in the command.
            let mut data = data.split(" ");
            let result = match data.next() {
                Some("refresh") => self.tx.send(Refresh),
                Some("shutdown") => self.tx.send(Shutdown),
                Some("update") => {
                    // Treat the rest of the arguments as block names and
                    // forward them back to the framework to be updated.
                    let names =
                        data.map(|name| name.to_string()).collect::<Vec<_>>();
                    self.tx.send(Update(names))
                },
                _ => Ok(()),
            };

            // TODO: What to do if the listener encounters an error?
            if let Err(_err) = result {
                unimplemented!("Monitor panicked!")
            }
        }
    }
}
