use crate::components;
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.view(frame))?;
            self.handle_event();
        }

        Ok(())
    }

    fn view(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(frame.area());

        components::main_layout::render(frame, layout[0]);
        components::help_bar::render(frame, layout[1]);
    }

    fn handle_event(&mut self) {
        if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
            if key.code == crossterm::event::KeyCode::Char('q') {
                self.exit = true;
            }
        }
    }
}
