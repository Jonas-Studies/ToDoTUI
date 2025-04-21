use ratatui::prelude::{Rect, Buffer};

pub trait CanBeRendered {
    fn render (&self, area: Rect, buffer: &mut Buffer);
}

pub trait CanBeFocused {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer);
}
