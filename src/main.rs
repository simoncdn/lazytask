use std::io;

mod app;
mod components;
mod models;
mod services;

const DATA_FILE: &str = "data/data.json";

fn main() -> io::Result<()> {
    let tasks = services::load_tasks(DATA_FILE).expect("Tasks loading error");

    let mut terminal = ratatui::init();
    let mut app = app::App::new(tasks);
    let result = app.run(&mut terminal);
    ratatui::restore();

    result
}
