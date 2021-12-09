mod cli;
mod config;
mod utils;

use std::thread;

use abar::monitor::{Command, Monitor};
use abar::threadpool::ThreadPool;

fn main() {
    cli::process_args();

    let (monitor_tx, monitor_rx) = flume::bounded(100);

    let monitor = Monitor::new(monitor_tx.clone(), 2227);
    thread::spawn(move || monitor.run());

    let threadpool = ThreadPool::new(2, monitor_tx);
    let statusbar = config::bar();

    statusbar.attach_threadpool(&threadpool);

    let mut bar = String::new();
    loop {
        let new_bar = statusbar.to_string();

        if bar != new_bar {
            bar = new_bar;

            std::process::Command::new("xsetroot")
                .arg("-name")
                .arg(&bar)
                .output()
                .unwrap();
        }

        let command: Option<Command> = {
            // If there is an incoming command, return early.
            if let Ok(command) = monitor_rx.try_recv() {
                Some(command)
            }
            // Otherwise, wait until the next update.
            else if let Some(time) = statusbar.time_until_next_update() {
                monitor_rx.recv_timeout(time).ok()
            }
            // If there are no future updates, block until the next command.
            else {
                monitor_rx.recv().ok()
            }
        };

        match command {
            Some(Command::Update(names)) => statusbar.update(&names),
            Some(Command::Shutdown) => break,
            _ => (),
        }
    }
}
