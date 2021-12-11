mod config;

use std::sync::mpsc;
use std::thread;

use abar::monitor::Monitor;
use abar::threadpool::ThreadPool;

const CHANNEL_CAPACITY: usize = 100;

/// You might want to change this to fit your environment. The default works
/// well for dwm, but other setups might require printing to stdout, etc.
fn draw_bar(bar: &str) {
    std::process::Command::new("xsetroot")
        .arg("-name")
        .arg(&bar)
        .output()
        .unwrap();
}

fn main() {
    abar::cli::process_args();

    // Create "meta" channels for communicating with the main loop.
    let (monitor_tx, monitor_rx) = mpsc::sync_channel(CHANNEL_CAPACITY);

    // Create a monitor that will listen for commands on config::PORT.
    let monitor = Monitor::new(monitor_tx.clone(), config::PORT);
    thread::spawn(move || monitor.run());

    // Create a threadpool responsible for evaluating block updates.
    let threadpool = ThreadPool::new(config::NUM_WORKERS, monitor_tx);
    let statusbar = config::bar();

    // Attach the threadpool to the bar, and enter the main loop.
    statusbar.attach_threadpool(&threadpool);
    statusbar.run(draw_bar, monitor_rx);
}
