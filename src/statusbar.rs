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
}
