mod builder;

use std::fmt;
use std::time::{Duration, Instant};

pub use builder::StatusBarBuilder;

use super::statusblock::StatusBlock;
use crate::monitor::{Command, MonitorReceiver};
use crate::threadpool::ThreadPool;

/// Encapsulates a number of StatusBlocks.
///
/// Contains information re. how StatusBlocks should be formatted, delimited,
/// rendered, etc. as well as methods that operate across all blocks at once.
pub struct StatusBar {
    pub blocks:             Vec<StatusBlock>,
    pub delimiter:          String,
    pub left_buffer:        String,
    pub right_buffer:       String,
    pub hide_empty_modules: bool,
}

impl StatusBar {
    pub fn update(&self, names: &[String]) {
        self.blocks
            .iter()
            .filter(|block| block.name.is_some())
            .filter(|block| names.contains(block.name.as_ref().unwrap()))
            .for_each(|block| {
                block.update_now();
            })
    }

    pub fn attach_threadpool(&self, pool: &ThreadPool<String>) {
        for block in &self.blocks {
            block.attach_threadpool(pool);
        }
    }

    pub fn time_until_next_update(&self) -> Option<Duration> {
        let now = Instant::now();

        let next_update =
            self.blocks.iter().filter_map(|block| block.next_update()).min();

        next_update.map(|instant| {
            if instant > now {
                instant - now
            }
            else {
                Duration::from_secs(0)
            }
        })
    }

    pub fn run(self, draw_function: fn(&str), monitor_rx: MonitorReceiver) {
        let mut bar = String::new();

        loop {
            // Update the bar, and draw it if necessary.
            let new_bar = self.to_string();
            if bar != new_bar {
                bar = new_bar;
                draw_function(&bar);
            }

            let command: Option<Command> = {
                // If there is an incoming command, return early.
                if let Ok(command) = monitor_rx.try_recv() {
                    Some(command)
                }
                // Otherwise, wait until the next update.
                else if let Some(time) = self.time_until_next_update() {
                    monitor_rx.recv_timeout(time).ok()
                }
                // If there are no future updates, block until the next command.
                else {
                    monitor_rx.recv().ok()
                }
            };

            // Finally, respond to any external commands that came in.
            match command {
                Some(Command::Update(names)) => self.update(&names),
                Some(Command::Shutdown) => break,
                Some(Command::Refresh) | None => (),
            }
        }
    }
}

impl fmt::Display for StatusBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = self.left_buffer.clone();

        self.blocks.iter().enumerate().for_each(|(i, block)| {
            let cache = block.to_string();

            if cache.is_empty() && self.hide_empty_modules {
                return;
            }

            if i > 0 && i < self.blocks.len() {
                out.push_str(&self.delimiter);
            }

            out.push_str(&cache);
        });
        out.push_str(&self.right_buffer);

        write!(f, "{}", out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::statusblock::StatusBlockBuilder;

    #[test]
    fn default_has_correct_fields() {
        let bar = StatusBarBuilder::default().build();
        assert!(bar.blocks.is_empty());
        assert_eq!(bar.delimiter, "");
        assert_eq!(bar.left_buffer, "");
        assert_eq!(bar.right_buffer, "");
    }

    #[test]
    fn display_draws_blocks() {
        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default()
            .function(|| String::from("test2"))
            .build();

        let bar = StatusBarBuilder::new(vec![block1, block2]).build();

        assert_eq!(bar.to_string(), "test1test2");
    }

    #[test]
    fn display_draws_delimiters() {
        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default()
            .function(|| String::from("test2"))
            .build();

        let bar = StatusBarBuilder::new(vec![block1, block2])
            .delimiter(" | ")
            .left_buffer(" >>> ")
            .right_buffer(" <<< ")
            .build();

        assert_eq!(bar.to_string(), " >>> test1 | test2 <<< ");
    }

    #[test]
    fn display_draws_empty_blocks_if_needed() {
        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default().build();
        let block3 = StatusBlockBuilder::default()
            .function(|| String::from("test3"))
            .build();

        let bar = StatusBarBuilder::new(vec![block1, block2, block3])
            .delimiter(" | ")
            .left_buffer(" >>> ")
            .right_buffer(" <<< ")
            .hide_empty_modules(false)
            .build();

        assert_eq!(bar.to_string(), " >>> test1 |  | test3 <<< ");
    }

    #[test]
    fn display_ignores_empty_blocks_if_needed() {
        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default().build();
        let block3 = StatusBlockBuilder::default()
            .function(|| String::from("test3"))
            .build();

        let bar = StatusBarBuilder::new(vec![block1, block2, block3])
            .delimiter(" | ")
            .left_buffer(" >>> ")
            .right_buffer(" <<< ")
            .hide_empty_modules(true)
            .build();

        assert_eq!(bar.to_string(), " >>> test1 | test3 <<< ");
    }
}
