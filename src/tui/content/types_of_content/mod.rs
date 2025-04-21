use ratatui::prelude::{Rect, Buffer};
use title::Title;

use super::traits::{CanBeFocused, CanBeRendered};

mod title;

pub enum TypesOfContent {
    Title(Title)
}

impl CanBeRendered for TypesOfContent {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            _ => { }
        }
    }
}

impl CanBeFocused for TypesOfContent {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            _ => { }
        }
    }
}
