use std::sync::Mutex;
use std::time::Duration;

use super::{StatusBlock, TimedCache};

#[derive(Default)]
pub struct StatusBlockBuilder {
    pub name:            Option<String>,
    pub min_size:        Option<usize>,
    pub max_size:        Option<usize>,
    pub function:        Option<fn() -> String>,
    pub update_interval: Option<Duration>,
}

#[allow(dead_code)]
impl StatusBlockBuilder {
    pub fn new(f: fn() -> String) -> Self {
        Self { function: Some(f), ..Default::default() }
    }

    pub fn build(self) -> StatusBlock { self.into() }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn min_size(mut self, min_size: usize) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn function(mut self, f: fn() -> String) -> Self {
        self.function = Some(f);
        self
    }

    pub fn update_interval(mut self, interval: Duration) -> Self {
        self.update_interval = Some(interval);
        self
    }
}

impl From<StatusBlockBuilder> for StatusBlock {
    fn from(builder: StatusBlockBuilder) -> Self {
        let function = builder.function.unwrap_or(String::new);

        Self {
            name:     builder.name,
            min_size: builder.min_size,
            max_size: builder.max_size,
            cache:    Mutex::new(TimedCache::new(
                builder.update_interval,
                function,
            )),
        }
    }
}
