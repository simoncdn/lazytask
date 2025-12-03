mod archived;
mod home;
mod tasks;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(7),
            Constraint::Percentage(63),
            Constraint::Percentage(30),
        ])
        .split(area);

    home::render(frame, sidebar[0]);
    tasks::render(frame, sidebar[1]);
    archived::render(frame, sidebar[2]);
}
