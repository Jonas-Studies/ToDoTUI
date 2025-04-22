use core::{clone::Clone, ops::DerefMut};

use ratatui::{crossterm::event::KeyCode, layout::Layout};

use crate::tui::content::{traits::{CanBeFocused, CanBeRendered, CanHandleUserinput, MayDisplayCursor}, Content};

use super::TypesOfContent;

pub struct Container<PossibleActions> where PossibleActions: Clone {
    layout: Layout,
    content: Vec<Content<PossibleActions>>,
    nr_of_focused_content: Option<usize> 
}

impl <PossibleActions> Container<PossibleActions> where PossibleActions: Clone {
    pub fn new(layout: Layout) -> Self {
        let content = Vec::new();

        Self { layout, content, nr_of_focused_content: None }
    }
    pub fn push_content(&mut self, content: Content<PossibleActions>) {
        self.content.push(content);
    }
    pub fn reference_content(&self, nr: usize) -> &Content<PossibleActions> {
        &self.content[nr]
    }
    fn reference_content_mutable(&mut self,nr: usize) -> &mut Content<PossibleActions> {
        &mut self.content[nr]
    }
    fn get_nr_of_last_content(&self) -> usize {
        self.content.len() - 1
    }
    fn get_nr_of_first_content(&self) -> usize {
        0
    }
    pub fn focus_first(&mut self) -> Option<usize> {
        self.nr_of_focused_content = Some(self.get_nr_of_first_content());

        if self.reference_content(self.get_nr_of_first_content()).can_be_focused() {
            self.nr_of_focused_content
        }
        else {
            self.focus_next()
        }
    }
    fn focus_last(&mut self) -> Option<usize> {
        self.nr_of_focused_content = Some(self.get_nr_of_last_content());

        if self.reference_content(self.get_nr_of_last_content()).can_be_focused() {
            self.nr_of_focused_content
        }
        else {
            self.focus_previous()
        }
    }
    fn focus_next(&mut self) -> Option<usize> {
        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
            if nr_of_focused_content == self.get_nr_of_last_content() {
                None
            }
            else {
                self.nr_of_focused_content = Some(nr_of_focused_content + 1);

                if self.reference_content(nr_of_focused_content + 1).can_be_focused() {
                    self.nr_of_focused_content
                }
                else {
                    self.focus_next()
                }
            }
        }
        else {
            self.focus_first()
        }
    }
    fn focus_previous(&mut self) -> Option<usize> {
        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
            if nr_of_focused_content == self.get_nr_of_first_content() {
                None
            }
            else {
                self.nr_of_focused_content = Some(nr_of_focused_content - 1);

                if self.reference_content(nr_of_focused_content - 1).can_be_focused() {
                    self.nr_of_focused_content
                }
                else {
                    self.focus_previous()
                }
            }
        }
        else {
            self.focus_last()
        }
    }
}

impl <PossibleActions> CanBeRendered for Container<PossibleActions> where PossibleActions: Clone {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }
    }
}

impl <PossibleActions> CanBeFocused for Container<PossibleActions> where PossibleActions: Clone {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
            let areas = self.layout.split(area);

            for (nr_of_area, area) in areas.iter().enumerate() {
                if nr_of_area == nr_of_focused_content {
                    self.content[nr_of_area].render_focused(*area, buffer);
                }
                else {
                    self.content[nr_of_area].render(*area, buffer);
                }
            }
        }
        else {
            self.render(area, buffer);
        }
    }
}

impl <PossibleActions> CanHandleUserinput<PossibleActions> for Container<PossibleActions> where PossibleActions: Clone {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        let mut result = None;

        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
            // Special case when selected content is a container too. Then, in case the focus is
            // supposed to be changed, it needs to be decided if that should happen in the current
            // or the focused container
            if let TypesOfContent::Contaier(container) = self.reference_content_mutable(nr_of_focused_content).deref_mut() {
                match userinput {
                    KeyCode::Tab => {
                        if container.focus_next().is_none() {
                            if self.focus_next().is_none() {
                                self.focus_first();
                            }
                        }
                    }
                    KeyCode::BackTab => {
                        if container.focus_previous().is_none() {
                            if self.focus_previous().is_none() {
                                self.focus_last();
                            }
                        }
                    }
                    _ => {
                        result = self.reference_content_mutable(nr_of_focused_content).handle_userinpt(userinput);
                    }
                }
            }
            else {
                match userinput {
                    KeyCode::Tab => {
                        if self.focus_next().is_none() {
                            self.focus_first();
                        }
                        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
                            if let TypesOfContent::Contaier(container) = self.reference_content_mutable(nr_of_focused_content).deref_mut() {
                                container.focus_first();
                            }
                        }
                    }
                    KeyCode::BackTab => {
                        if self.focus_previous().is_none() {
                            self.focus_last();
                        }
                        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
                            if let TypesOfContent::Contaier(container) = self.reference_content_mutable(nr_of_focused_content).deref_mut() {
                                container.focus_last();
                            }
                        }
                    }
                    _ => {
                        result = self.reference_content_mutable(nr_of_focused_content).handle_userinpt(userinput);
                    }
                }

            }
        }
        
        return result;
    }
}

impl <PossibleActions: Clone> MayDisplayCursor for Container<PossibleActions> {
    fn get_cursor_position(&self, area: ratatui::prelude::Rect) -> Option<ratatui::prelude::Position> {
        let mut result = None;

        if let Some(nr_of_focused_content) = self.nr_of_focused_content {
            if self.reference_content(nr_of_focused_content).can_display_cursor() {
                let areas = self.layout.split(area);

                result = self.reference_content(nr_of_focused_content).get_cursor_position(areas[nr_of_focused_content]);
            }
        }

        return result;
    }
}
