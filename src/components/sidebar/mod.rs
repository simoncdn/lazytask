mod archived;
mod home;
mod tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::FocusedComponents;

pub fn render(frame: &mut Frame, area: Rect, focus: &FocusedComponents) {
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(7),
            Constraint::Percentage(63),
            Constraint::Percentage(30),
        ])
        .split(area);

    home::render(frame, sidebar[0], focus);
    tasks::render(frame, sidebar[1], focus);
    archived::render(frame, sidebar[2], focus);
}
