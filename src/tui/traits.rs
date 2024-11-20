use ratatui::{
    crossterm::event::KeyCode, prelude::Position, widgets::WidgetRef
};

pub trait IsContent: WidgetRef + CanBeFocused + MayDisplayCursor {}

pub trait CanBeFocused {
    fn focus(& mut self);
    fn unfocus(& mut self);
}

pub trait MayDisplayCursor {
    fn get_cursor_position (& self) -> Option<Position>;
}

pub trait CanHandleUserinput {
    fn handle_userinput (& mut self, userinput: & KeyCode);
}
