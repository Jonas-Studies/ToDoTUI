use super::traits::{CanBeFocused, IsContent};

use ratatui::prelude::{Rect, Buffer};

pub struct Field <Content> where Content: IsContent {
    area: Rect,
    content: Content
}

impl <Content> Field <Content> where Content: IsContent {
    pub fn new (area: Rect, content: Content) -> Self {
        Self { area, content }
    }

    pub fn render (& self, buffer: & mut Buffer) {
        self.content.render_ref(self.area, buffer);
    }
}

impl <Content> CanBeFocused for Field <Content> where Content: IsContent {
    fn focus(& mut self) {
        self.content.focus();
    }
    fn unfocus(& mut self) {
        self.content.unfocus();
    }
}
