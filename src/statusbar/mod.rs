mod builder;

use std::fmt;
use std::sync::mpsc;

pub use builder::StatusBarBuilder;

use super::statusblock::{StatusBlock, StatusBlockBuilder};
use super::threadpool::{ResultsReceiver, ThreadPool};

pub struct StatusBar {
    pub blocks:             Vec<StatusBlock>,
    pub delimiter:          String,
    pub left_buffer:        String,
    pub right_buffer:       String,
    pub hide_empty_modules: bool,
}

impl StatusBar {
    fn new(num_threads: usize) -> Self {
        Self {
            blocks:             Vec::new(),
            delimiter:          String::new(),
            left_buffer:        String::new(),
            right_buffer:       String::new(),
            hide_empty_modules: true,
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self { Self::new(1) }
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

    #[test]
    fn default_has_correct_fields() {
        let bar = StatusBar::default();
        assert!(bar.blocks.is_empty());
        assert_eq!(bar.delimiter, "");
        assert_eq!(bar.left_buffer, "");
        assert_eq!(bar.right_buffer, "");
    }

    #[test]
    fn display_draws_blocks() {
        let mut bar = StatusBar::default();

        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default()
            .function(|| String::from("test2"))
            .build();

        bar.blocks.push(block1);
        bar.blocks.push(block2);

        assert_eq!(bar.to_string(), "test1test2");
    }

    #[test]
    fn display_draws_delimiters() {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");

        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block2 = StatusBlockBuilder::default()
            .function(|| String::from("test2"))
            .build();

        bar.blocks.push(block1);
        bar.blocks.push(block2);

        assert_eq!(bar.to_string(), " >>> test1 | test2 <<< ");
    }

    #[test]
    fn display_draws_empty_blocks_if_needed() {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");
        bar.hide_empty_modules = false;

        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block3 = StatusBlockBuilder::default()
            .function(|| String::from("test3"))
            .build();

        let block2 = StatusBlockBuilder::default().build();

        bar.blocks.push(block1);
        bar.blocks.push(block2);
        bar.blocks.push(block3);

        assert_eq!(bar.to_string(), " >>> test1 |  | test3 <<< ");
    }

    #[test]
    fn display_ignores_empty_blocks_if_needed() {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");
        bar.hide_empty_modules = true;

        let block1 = StatusBlockBuilder::default()
            .function(|| String::from("test1"))
            .build();
        let block3 = StatusBlockBuilder::default()
            .function(|| String::from("test3"))
            .build();

        let block2 = StatusBlockBuilder::default().build();

        bar.blocks.push(block1);
        bar.blocks.push(block2);
        bar.blocks.push(block3);

        assert_eq!(bar.to_string(), " >>> test1 | test3 <<< ");
    }
}
