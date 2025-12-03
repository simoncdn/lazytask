use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    app::FocusedComponents,
    components::{context_view, sidebar},
};

pub fn render(frame: &mut Frame, area: Rect, focus: &FocusedComponents) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    sidebar::render(frame, main_layout[0], focus);
    context_view::render(frame, main_layout[1]);
}
