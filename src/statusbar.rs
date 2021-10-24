use std::fmt;

use super::statusblock::StatusBlock;

pub struct StatusBar
{
    pub blocks:             Vec<StatusBlock>,
    pub delimiter:          String,
    pub left_buffer:        String,
    pub right_buffer:       String,
    pub hide_empty_modules: bool,
}

impl Default for StatusBar
{
    fn default() -> Self
    {
        StatusBar {
            blocks:             Vec::new(),
            delimiter:          String::new(),
            left_buffer:        String::new(),
            right_buffer:       String::new(),
            hide_empty_modules: true,
        }
    }
}

impl fmt::Display for StatusBar
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut out = String::new();

        self.blocks.iter().for_each(|block| out.push_str(&block.to_string()));

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
        assert_eq!(bar.blocks, Vec::new());
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
}
