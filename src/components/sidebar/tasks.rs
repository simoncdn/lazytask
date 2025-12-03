use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders},
};

use crate::app::FocusedComponents;

pub fn render(frame: &mut Frame, area: Rect, focus: &FocusedComponents) {
    let title = "Tasks";
    let is_focused = matches!(focus, FocusedComponents::Tasks);
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::White
    };
    let widget = Block::new()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_color);

    frame.render_widget(widget, area);
}
