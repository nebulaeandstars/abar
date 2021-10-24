use std::sync::mpsc;
use std::{fmt, thread};

use super::statusblock::StatusBlock;
use super::threadpool::{JobPacket, ResultsReceiver, ThreadPool};

pub struct StatusBar
{
    pub blocks:             Vec<StatusBlock>,
    pub delimiter:          String,
    pub left_buffer:        String,
    pub right_buffer:       String,
    pub hide_empty_modules: bool,
    threadpool:             ThreadPool,
    results_rx:             ResultsReceiver,
}

impl StatusBar
{
    fn new(num_threads: usize) -> Self
    {
        let (results_tx, results_rx) = mpsc::channel();

        StatusBar {
            blocks: Vec::new(),
            delimiter: String::new(),
            left_buffer: String::new(),
            right_buffer: String::new(),
            hide_empty_modules: true,
            threadpool: ThreadPool::new(num_threads, results_tx),
            results_rx,
        }
    }

    pub fn create_jobs(&mut self)
    {
        // create jobs for all pending StatusBlocks
        for block in &self.blocks {
            if block.needs_update() {
                self.threadpool.execute(JobPacket {
                    id:  block.name.clone(),
                    job: block.command,
                });
            }
        }
    }

    pub fn receive_results(&mut self)
    {
        // receive any finished jobs
        // TODO: Make this better (probably a HashMap)
        while let Ok(result) = self.results_rx.try_recv() {
            for block in &mut self.blocks {
                if block.name == result.id {
                    block.manual_update(result.result.clone());
                }
            }
        }
    }

    pub fn listen(&mut self)
    {
        // TODO: Make this better (probably a HashMap)
        loop {
            if let Ok(result) = self.results_rx.recv() {
                for block in &mut self.blocks {
                    if block.name == result.id {
                        block.manual_update(result.result.clone());
                    }
                }
            }
        }
    }
}

impl Default for StatusBar
{
    fn default() -> Self { Self::new(1) }
}

impl fmt::Display for StatusBar
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
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
mod tests
{
    use super::*;

    #[test]
    fn default_has_correct_fields()
    {
        let bar = StatusBar::default();
        assert!(bar.blocks.is_empty());
        assert_eq!(bar.delimiter, "");
        assert_eq!(bar.left_buffer, "");
        assert_eq!(bar.right_buffer, "");
    }

    #[test]
    fn display_draws_blocks()
    {
        let mut bar = StatusBar::default();

        let mut block1 = StatusBlock::default();
        let mut block2 = StatusBlock::default();
        block1.cache = String::from("test1");
        block2.cache = String::from("test2");

        bar.blocks.push(block1);
        bar.blocks.push(block2);

        assert_eq!(bar.to_string(), "test1test2");
    }

    #[test]
    fn display_draws_delimiters()
    {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");

        let mut block1 = StatusBlock::default();
        let mut block2 = StatusBlock::default();
        block1.cache = String::from("test1");
        block2.cache = String::from("test2");

        bar.blocks.push(block1);
        bar.blocks.push(block2);

        assert_eq!(bar.to_string(), " >>> test1 | test2 <<< ");
    }

    #[test]
    fn display_draws_empty_blocks_if_needed()
    {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");
        bar.hide_empty_modules = false;

        let mut block1 = StatusBlock::default();
        let block2 = StatusBlock::default();
        let mut block3 = StatusBlock::default();
        block1.cache = String::from("test1");
        block3.cache = String::from("test3");

        bar.blocks.push(block1);
        bar.blocks.push(block2);
        bar.blocks.push(block3);

        assert_eq!(bar.to_string(), " >>> test1 |  | test3 <<< ");
    }

    #[test]
    fn display_ignores_empty_blocks_if_needed()
    {
        let mut bar = StatusBar::default();
        bar.delimiter = String::from(" | ");
        bar.left_buffer = String::from(" >>> ");
        bar.right_buffer = String::from(" <<< ");
        bar.hide_empty_modules = true;

        let mut block1 = StatusBlock::default();
        let block2 = StatusBlock::default();
        let mut block3 = StatusBlock::default();
        block1.cache = String::from("test1");
        block3.cache = String::from("test3");

        bar.blocks.push(block1);
        bar.blocks.push(block2);
        bar.blocks.push(block3);

        assert_eq!(bar.to_string(), " >>> test1 | test3 <<< ");
    }

    #[test]
    fn bar_updates()
    {
        let mut bar = StatusBar::default();

        let mut block1 = StatusBlock::default();
        let mut block2 = StatusBlock::default();
        let mut block3 = StatusBlock::default();
        block1.command = || String::from("test1");
        block2.command = || String::from("test2");
        block3.command = || String::from("test3");

        bar.blocks.push(block1);
        bar.blocks.push(block2);
        bar.blocks.push(block3);

        bar.create_jobs();
        thread::sleep(std::time::Duration::from_millis(100));
        bar.receive_results();
        assert_eq!(bar.to_string(), "test1test2test3");
    }
}
