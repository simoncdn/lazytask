use std::io;

mod app;
mod components;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = app::App::default().run(&mut terminal);
    ratatui::restore();

    result
}
