mod config;
mod utils;

use std::thread;
use std::time::Duration;

use abar::threadpool::ThreadPool;

fn main() {
    let threadpool = ThreadPool::new(4);
    let mut statusbar = config::bar();

    statusbar.attach_threadpool(&threadpool);

    loop {
        println!("{}", statusbar);
        thread::sleep(Duration::from_millis(500));
    }
}
