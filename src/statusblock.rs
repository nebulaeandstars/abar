use std::fmt;
use std::time::{Duration, Instant};

pub struct StatusBlock
{
    pub name:            String,
    pub cache:           String,
    pub command:         fn() -> String,
    pub update_interval: Option<Duration>,
    pub min_size:        Option<usize>,
    pub max_size:        Option<usize>,
    pub last_update:     Option<Instant>,
}

impl StatusBlock
{
    /// Returns whether the StatusBlock is due to be updated.
    fn needs_update(&self) -> bool
    {
        if self.last_update.is_none() || self.update_interval.is_none() {
            self.update_interval.is_some()
        }
        else {
            let now = Instant::now();
            let last_update = self.last_update.unwrap();
            let update_interval = self.update_interval.unwrap();

            now.duration_since(last_update) > update_interval
        }
    }

    /// Updates the StatusBlock iff it's scheduled to be updated.
    fn update(&mut self)
    {
        if self.needs_update() {
            self.cache = (self.command)();
        }
    }
}

impl Default for StatusBlock
{
    fn default() -> Self
    {
        StatusBlock {
            name:            String::new(),
            cache:           String::new(),
            command:         String::new,
            update_interval: None,
            min_size:        None,
            max_size:        None,
            last_update:     None,
        }
    }
}

impl fmt::Display for StatusBlock
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.cache)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_statusblock_default()
    {
        let block = StatusBlock::default();
        assert_eq!(block.name, "");
        assert_eq!(block.cache, "");
        assert_eq!((block.command)(), "");
    }

    #[test]
    fn test_statusblock_display()
    {
        let mut block = StatusBlock::default();
        block.cache = String::from("test");
        assert_eq!(block.to_string(), "test");
    }

    #[test]
    fn test_statusblock_update()
    {
        let mut block = StatusBlock::default();
        let interval = Duration::from_nanos(1);

        block.command = || String::from("test");
        block.update_interval = Some(interval);
        std::thread::sleep(interval * 4);

        block.update();
        assert_eq!(block.to_string(), "test");
    }
}
