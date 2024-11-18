use super::traits::{IsContent, CanBeFocused, MayDisplayCursor};

use ratatui::{
    prelude::{Buffer, Rect, Position},
    widgets::{WidgetRef, Block, Paragraph},
    style::{Style, Color}
};

impl IsContent for Paragraph <'_> {}

impl CanBeFocused for Paragraph <'_> {
    fn focus(& mut self) {}
    fn unfocus(& mut self) {}
}

impl MayDisplayCursor for Paragraph <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        None
    }
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

impl MayDisplayCursor for TextField <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        // Position of 1, 1 because the Textfield is getting rendered with borders
        Some(Position::new(1, 1))
    }
}
