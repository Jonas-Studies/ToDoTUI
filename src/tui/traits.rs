use ratatui::widgets::WidgetRef;

pub trait IsContent: WidgetRef + CanBeFocused {}

pub trait CanBeFocused {
    fn focus(& mut self);
    fn unfocus(& mut self);
}
