mod config;
mod utils;

use std::thread;
use std::time::Duration;

fn main() {
    let statusbar = config::bar();

    loop {
        println!("{}", statusbar);
        thread::sleep(Duration::from_millis(500));
    }
}
