mod config;
mod utils;

use abar::threadpool::ThreadPool;

fn main() {
    let (monitor_tx, monitor_rx) = flume::bounded(100);

    let threadpool = ThreadPool::new(4, monitor_tx);
    let statusbar = config::bar();

    statusbar.attach_threadpool(&threadpool);

    loop {
        println!("{}", statusbar);

        if let Ok(()) = monitor_rx.try_recv() {
            continue;
        }
        else if let Some(time) = statusbar.time_until_next_update() {
            let _ = monitor_rx.recv_timeout(time);
        }
        else {
            monitor_rx.recv().unwrap();
        }
    }
}
