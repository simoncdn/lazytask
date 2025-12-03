use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, BorderType, Borders, List, ListItem, Padding},
};

use crate::app::{App, FocusedComponents};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let title = "Tasks";
    let is_focused = matches!(app.focus, FocusedComponents::Tasks);
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::White
    };

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| ListItem::new(format!("{} - {}", task.id(), task.title())))
        .collect();
    let widget = List::new(items).block(
        Block::new()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(border_color)
            .padding(Padding {
                left: 2,
                right: 0,
                top: 0,
                bottom: 0,
            }),
    );

    frame.render_widget(widget, area);
}
