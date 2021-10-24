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
}
