use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout}};

use crate::task::Task;

use super::content::{traits::{CanBeRendered, CanHandleUserinput}, types_of_content::{textinput::Textinput, title::Title, TypesOfContent}, Content};

pub struct Application {
    layout: Layout,
    task: Task,
    content: Vec<Content>
}

impl Application {
    pub fn new(task_to_edit: Task) -> Self {
        let layout = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3) ]
        ).spacing(1).vertical_margin(1).horizontal_margin(3);

        let mut content = Vec::new();

        content.push(
            Content::new(TypesOfContent::Title(Self::get_title(task_to_edit.is_finished())))
        );
        content.push(
            Content::new(TypesOfContent::Textinput(Textinput::new(task_to_edit.get_name(), String::from("Name"))))
        );

        Self { layout, task: task_to_edit, content }
    }
    fn get_title(is_task_finished: bool) -> Title {
        Title::new(
            String::from(
                if is_task_finished {
                    "Done:"
                }
                else {
                    "ToDo:"
                }
            )
        )
    }
}

impl CanBeRendered for Application {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }
    }
}

pub enum PossibleActions {
    Exit
}

impl CanHandleUserinput<PossibleActions> for Application {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        match userinput {
            KeyCode::Esc => {
                Some(PossibleActions::Exit)
            }
            _ => {
                None
            }
        }
    }
}
