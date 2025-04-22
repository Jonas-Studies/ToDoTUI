use ratatui::layout::{Layout, Constraint};

use crate::tasks::Tasks;

use super::content::{traits::CanBeFocused, types_of_content::{container::Container, textinput::Textinput, TypesOfContent}, Content};

#[derive(Clone)]
pub enum PossibleActions {
    Exit,
    Select
}

pub struct Application {
    content: Container<PossibleActions>
}
