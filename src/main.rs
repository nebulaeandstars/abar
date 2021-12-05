use std::thread;
use std::time::Duration;

use abar::{StatusBarBuilder, StatusBlockBuilder};

fn main() {
    let blocks = vec![
        StatusBlockBuilder::new(|| String::from("test1")).build(),
        StatusBlockBuilder::new(|| String::from("test2")).build(),
    ];

    let statusbar = StatusBarBuilder::new()
        .blocks(blocks)
        .delimiter(" | ")
        .left_buffer(" >>> ")
        .right_buffer(" <<< ")
        .build();

    loop {
        println!("{}", statusbar);
        thread::sleep(Duration::from_secs(1));
    }
}
