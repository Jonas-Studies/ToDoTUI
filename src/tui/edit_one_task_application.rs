use crate::Task;

use super::{
    content::{Button, TextField},
    field::Field,
    traits::{CanBeFocused, CanHandleUserinput, MayDisplayCursor}
};

use ratatui::{
    crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, prelude::{Buffer, Position}, style::{Modifier, Stylize}, widgets::Paragraph
};

pub enum SelectableItems {
    Name,
    Finish
}

pub struct Application <'applications_lifetime> {
    status: Field <Paragraph <'applications_lifetime>>,
    name: Field <TextField <'applications_lifetime>>,
    finish: Field <Button <'applications_lifetime>>,
    index_of_selected_item: SelectableItems
}

impl Application <'_> {
    pub fn new (task_to_edit: & Task, mut area: Rect) -> Self {
        area.x += 2;
        area.width -= 4;
        area.y += 1;
        area.height -= 2;

        let [ status_area, name_area, finish_area ] = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3), Constraint::Length(3) ]
        ).spacing(1).areas(area);

        let status = Field::new(
            status_area,
            Paragraph::new(
                if task_to_edit.is_finished() {
                    String::from("Done:")
                }
                else {
                    String::from("Todo:")
                }
            ).add_modifier(Modifier::BOLD)
        );

        let mut name = Field::new(
            name_area,
            TextField::new(task_to_edit.get_name(), String::from("Name"))
        );
        name.focus();
        let index_of_selected_item = SelectableItems::Name;

        let [ finish_area ] = Layout::horizontal([ Constraint::Length(10) ]).areas(finish_area);

        let finish = Field::new(
            finish_area,
            Button::new(String::from("Delete"))
        );

        Self { status, name, finish, index_of_selected_item }
    }

    pub fn render(& self, buffer: & mut Buffer) {
        self.status.render(buffer);
        self.name.render(buffer);
        self.finish.render(buffer);
    }

    fn select_next_item(& mut self) {
        match self.index_of_selected_item {
            SelectableItems::Name => {
                self.name.unfocus();
                self.index_of_selected_item = SelectableItems::Finish;
                self.finish.focus();
            }
            SelectableItems::Finish => {
                self.finish.unfocus();
                self.index_of_selected_item = SelectableItems::Name;
                self.name.focus();
            }
        }
    }

    fn select_previous_item(& mut self) {
        match self.index_of_selected_item {
            SelectableItems::Finish => {
                self.name.unfocus();
                self.index_of_selected_item = SelectableItems::Finish;
                self.finish.focus();
            }
            SelectableItems::Name => {
                self.finish.unfocus();
                self.index_of_selected_item = SelectableItems::Name;
                self.name.focus();
            }
        }
    }

    pub fn save_task(& self, task: & mut Task) {
        task.set_name(self.name.reference_content().get_value());
    }
}

impl MayDisplayCursor for Application <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        if let SelectableItems::Name = self.index_of_selected_item {
            self.name.get_cursor_position()
        }
        else {
            None
        }
    }
}

impl CanHandleUserinput for Application <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Tab => {
                self.select_next_item();
            }
            KeyCode::BackTab => {
                self.select_previous_item();
            }
            _ => {
                if let SelectableItems::Name = self.index_of_selected_item {
                    self.name.handle_userinput(userinput);
                }
            }
        }
    }
}
