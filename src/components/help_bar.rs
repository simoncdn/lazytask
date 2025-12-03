use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub fn render(frame: &mut Frame, area: Rect) {
    let help_text = " j/k: navigate | a: add | d: delete | space: toggle ";
    let widget = Paragraph::new(help_text);

    frame.render_widget(widget, area);
}
