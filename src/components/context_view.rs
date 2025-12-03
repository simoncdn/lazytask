use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let title = "Context View";
    let widget = Block::new()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(widget, area);
}
