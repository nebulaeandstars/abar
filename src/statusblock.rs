use std::fmt;
use std::time::{Duration, Instant};

pub struct StatusBlock
{
    pub name:          String,
    pub cache:         String,
    pub command:       fn() -> String,
    pub poll_interval: Option<Duration>,
    pub min_size:      Option<usize>,
    pub max_size:      Option<usize>,
    pub last_update:   Option<Instant>,
}

impl Default for StatusBlock
{
    fn default() -> Self
    {
        StatusBlock {
            name:          String::new(),
            cache:         String::new(),
            command:       String::new,
            poll_interval: None,
            min_size:      None,
            max_size:      None,
            last_update:   None,
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
}
