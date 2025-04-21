use ratatui::{prelude::{Buffer, Rect}, widgets::{Block, Paragraph, Widget}};

use crate::tui::content::traits::CanBeRendered;

pub struct Textinput {
    // Using a vector of chars instead of String because there would be problems with utf-8
    value: Vec<char>,
    title: String,
    cursor_offset: usize
}

impl Textinput {
    pub fn new(value: String, title: String) -> Self {
        let value = Vec::from_iter(value.chars());

        Self { value, title, cursor_offset: 0 }
    }
    fn get_value(&self) -> String {
        String::from_iter(self.value.iter())
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
}

impl CanBeRendered for Textinput {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_value()).block(
            Block::bordered().title(self.get_title())
        ).render(area, buffer);
    }
}
