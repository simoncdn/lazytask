use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders},
};

use crate::app::{App, FocusedComponents};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let title = "Home";
    let is_focused = matches!(app.focus, FocusedComponents::Home);
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
