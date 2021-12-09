mod cli;
mod config;
mod utils;

use std::thread;

use abar::monitor::{Command, Monitor, MonitorReceiver};
use abar::threadpool::ThreadPool;
use abar::StatusBar;

const CHANNEL_CAPACITY: usize = 100;

fn main() {
    cli::process_args();

    // Create "meta" channels for communicating with the main loop.
    let (monitor_tx, monitor_rx) = flume::bounded(CHANNEL_CAPACITY);

    // Create a monitor that will listen for commands on config::PORT.
    let monitor = Monitor::new(monitor_tx.clone(), config::PORT);
    thread::spawn(move || monitor.run());

    // Create a threadpool responsible for evaluating block updates.
    let threadpool = ThreadPool::new(config::NUM_WORKERS, monitor_tx);
    let statusbar = config::bar();

    // Attach the threadpool to the bar, and enter the main loop.
    statusbar.attach_threadpool(&threadpool);
    run(statusbar, monitor_rx);
}

fn run(statusbar: StatusBar, monitor_rx: MonitorReceiver) {
    let mut bar = String::new();

    loop {
        // Update the bar, and draw it if necessary.
        let new_bar = statusbar.to_string();
        if bar != new_bar {
            bar = new_bar;
            draw_bar(&bar);
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

        // Finally, respond to any external commands that came in.
        match command {
            Some(Command::Update(names)) => statusbar.update(&names),
            Some(Command::Shutdown) => break,
            Some(Command::Refresh) | None => (),
        }
    }
}

/// You might want to change this to fit your environment. The default works
/// well for dwm, but other setups might require printing to stdout, etc.
fn draw_bar(bar: &str) {
    std::process::Command::new("xsetroot")
        .arg("-name")
        .arg(&bar)
        .output()
        .unwrap();
}
