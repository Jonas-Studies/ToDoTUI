use super::traits::{IsContent, CanBeFocused};

use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{WidgetRef, Block, Paragraph},
    style::{Style, Color}
};

impl IsContent for Paragraph <'_> {}

impl CanBeFocused for Paragraph <'_> {
    fn focus(& mut self) {}
    fn unfocus(& mut self) {}
}

pub struct TextField <'textfields_lifetime> {
    text: String,
    borders: Block <'textfields_lifetime>
}

impl TextField <'_> {
    pub fn new (text: String, title: String) -> Self {
        let borders = Block::bordered().title(title);

        Self { text, borders }
    }
}

impl IsContent for TextField <'_> {}

impl WidgetRef for TextField <'_> {
    fn render_ref(& self, area: Rect, buffer: & mut Buffer) {
        Paragraph::new(self.text.clone()).block(self.borders.clone()).render_ref(area, buffer);
    }
}

impl CanBeFocused for TextField <'_> {
    fn focus(& mut self) {
        self.borders = self.borders.clone().border_style(Style::new().fg(Color::Yellow));
    }
    fn unfocus(& mut self) {
        self.borders = self.borders.clone().style(Style::default());
    }
}
