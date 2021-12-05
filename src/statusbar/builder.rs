use std::collections::BinaryHeap;

use crate::{StatusBar, StatusBlock};

#[derive(Default)]
pub struct StatusBarBuilder {
    pub blocks:             Vec<StatusBlock>,
    pub delimiter:          String,
    pub left_buffer:        String,
    pub right_buffer:       String,
    pub hide_empty_modules: bool,
}

#[allow(dead_code)]
impl StatusBarBuilder {
    pub fn new(blocks: Vec<StatusBlock>) -> Self {
        Self { blocks, ..Default::default() }
    }

    pub fn build(self) -> StatusBar { self.into() }

    pub fn blocks(mut self, blocks: Vec<StatusBlock>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn delimiter(mut self, delimiter: &str) -> Self {
        self.delimiter = String::from(delimiter);
        self
    }

    pub fn left_buffer(mut self, left_buffer: &str) -> Self {
        self.left_buffer = String::from(left_buffer);
        self
    }

    pub fn right_buffer(mut self, right_buffer: &str) -> Self {
        self.right_buffer = String::from(right_buffer);
        self
    }

    pub fn hide_empty_modules(mut self, hide_empty_modules: bool) -> Self {
        self.hide_empty_modules = hide_empty_modules;
        self
    }
}

impl From<StatusBarBuilder> for StatusBar {
    fn from(builder: StatusBarBuilder) -> Self {
        let update_queue = builder
            .blocks
            .iter()
            .enumerate()
            .map(|(i, block)| (block.next_update(), i))
            .filter(|(instant, _)| instant.is_some())
            .map(|(instant, i)| (instant.unwrap(), i))
            .collect();

        Self {
            blocks: builder.blocks,
            delimiter: builder.delimiter,
            left_buffer: builder.left_buffer,
            right_buffer: builder.right_buffer,
            hide_empty_modules: builder.hide_empty_modules,
            update_queue,
        }
    }
}
