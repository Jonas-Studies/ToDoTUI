use ratatui::prelude::{Rect, Buffer};
use textinput::Textinput;
use title::Title;

use super::traits::{CanBeFocused, CanBeRendered};

pub mod title;
pub mod textinput;

pub enum TypesOfContent {
    Title(Title),
    Textinput(Textinput)
}

impl CanBeRendered for TypesOfContent {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            TypesOfContent::Title(content) => { content.render(area, buffer); }
            TypesOfContent::Textinput(content) => { content.render(area, buffer); }
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
