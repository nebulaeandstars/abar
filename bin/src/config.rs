use std::process::Command;
use std::time::Duration;

use abar::{StatusBar, StatusBarBuilder, StatusBlock, StatusBlockBuilder};

/// abar responds to remote commands over TCP. This defines the port that will
/// be used for both sending and receiving instructions.
pub const PORT: usize = 2227;

/// The number of concurrent worker threads that will be used to evaluate
/// blocks. If your blocks are all very fast, you can leave this at 1. However,
/// if you have some slower blocks (eg. that fetch data from the internet) you
/// might want to bump this up to 2 or 3.
pub const NUM_WORKERS: usize = 2;

/// Define your StatusBar here. You probably just want to set the delimiters, or
/// set hide_empty_modules.
pub fn bar() -> StatusBar {
    // All fields are optional; default refresh rate is 1hz
    StatusBarBuilder::new(blocks())
        .delimiter(" | ")
        .left_buffer(" >>> ")
        .right_buffer(" <<< ")
        .build()
}

/// This is the thing that you probably want to edit. A StatusBar is made up of
/// a number of blocks, each with a unique name, a closure that returns a
/// String, and an optional update interval. If you haven't used Rust much
/// before, I'd recommend copying the example syntax.
fn blocks() -> Vec<StatusBlock> {
    use abar::utils::run;

    // You can use this wrapper to invoke shell commands.
    let run_example = StatusBlockBuilder::default()
        .name("run_example")
        .function(|| run("echo hello"))
        .min_size(8)
        .build();

    // Alternatively, you can use the built-in methods,
    let shell_example = StatusBlockBuilder::default()
        .name("shell_example")
        .function(shell_example)
        .update_interval(Duration::from_secs(1))
        .build();

    // or use vanilla Rust exclusively.
    let vanilla_example = StatusBlockBuilder::default()
        .name("vanilla_example")
        .function(rand_example)
        .update_interval(Duration::from_millis(50))
        .size(6)
        .build();

    // Closures are allowed, of course.
    let closure_example = StatusBlockBuilder::default()
        .name("closure_example")
        .function(|| {
            let output = "hello from a closure";
            output.to_string()
        })
        .max_size(18)
        .build();

    // Slow blocks are unobtrusive if using multiple worker threads.
    let slow_example = StatusBlockBuilder::default()
        .name("slow_example")
        .function(slow_example)
        .update_interval(Duration::from_secs(3))
        .size(12)
        .build();

    vec![
        run_example,
        shell_example,
        closure_example,
        slow_example,
        vanilla_example,
    ]
}

/// Example showing how you can combine vanilla Rust with the shell. This
/// example displays the number of running processes.
fn shell_example() -> String {
    // this is essentially what the `run()` function looks like.
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps -A --no-headers | wc -l")
        .output()
        .expect("Error in shell function!")
        .stdout;

    // Convert the output into a String and remove trailing whitespace.
    let output = String::from_utf8(output).unwrap().trim().to_string();

    // The output can now be used as a regular string.
    format!("processes: {}", output)
}

/// One of the biggest perks of using Rust is the `cargo` dependency manager.
/// This example uses the external `rand` crate to display random numbers.
/// Additional dependencies can be defined as-needed in Cargo.toml
fn rand_example() -> String {
    use rand::random;

    format!("{}", random::<u16>())
}

/// This is similar to shell_example, but is intentionally very slow.
fn slow_example() -> String {
    use std::thread;

    use rand::random;

    thread::sleep(Duration::from_secs(1));
    format!("slow: {}", random::<u16>())
}
