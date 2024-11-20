use super::traits::{IsContent, CanBeFocused, MayDisplayCursor, CanHandleUserinput};

use ratatui::{
    crossterm::event::KeyCode, prelude::{Buffer, Position, Rect}, style::{Color, Style}, widgets::{Block, Paragraph, WidgetRef}
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

impl CanHandleUserinput for Paragraph <'_> {
    fn handle_userinput(& mut self, _userinput: & KeyCode) {}
}

pub struct TextField <'textfields_lifetime> {
    text: String,
    borders: Block <'textfields_lifetime>,
    cursor_offset: usize
}

impl TextField <'_> {
    pub fn new(text: String, title: String) -> Self {
        let borders = Block::bordered().title(title);

        Self { text, borders, cursor_offset: 0 }
    }
    fn can_cursor_move_right(& self) -> bool {
        self.cursor_offset < self.text.len()
    }
    fn can_cursor_move_left(& self) -> bool {
        self.cursor_offset > 0
    }
    fn move_cursor_right(& mut self) {
        self.cursor_offset += 1;
    }
    fn move_cursor_left(& mut self) {
        self.cursor_offset -= 1;
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
        // Minimum position of 1, 1 because the Textfield is getting rendered with borders
        Some(Position::new(1 + self.cursor_offset as u16, 1))
    }
}

impl CanHandleUserinput for TextField <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Right => {
                if self.can_cursor_move_right() {
                    self.move_cursor_right();
                }
            }
            KeyCode::Left => {
                if self.can_cursor_move_left() {
                    self.move_cursor_left()
                }
            }
            _ => {}
        }
    }
}
