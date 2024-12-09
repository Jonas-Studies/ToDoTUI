use ratatui::{
    crossterm::event::KeyCode, prelude::Position, widgets::WidgetRef
};

pub trait IsContent: WidgetRef + CanBeFocused + MayDisplayCursor + CanHandleUserinput {}

pub trait CanBeFocused {
    fn focused(self) -> Self;
    fn unfocused(self) -> Self;
}

pub trait MayDisplayCursor {
    fn get_cursor_position (& self) -> Option<Position>;
}

pub trait CanHandleUserinput {
    fn handle_userinput (& mut self, userinput: & KeyCode);
}
