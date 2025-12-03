use crate::components;
use crate::models::Task;
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

#[derive(Debug)]
pub enum FocusedComponents {
    Home,
    Tasks,
    Archived,
}

impl Default for FocusedComponents {
    fn default() -> Self {
        Self::Tasks
    }
}

impl FocusedComponents {
    pub fn next(&self) -> Self {
        match self {
            Self::Home => Self::Tasks,
            Self::Tasks => Self::Archived,
            Self::Archived => Self::Home,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Home => Self::Archived,
            Self::Tasks => Self::Home,
            Self::Archived => Self::Tasks,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
    pub focus: FocusedComponents,
    pub tasks: Vec<Task>,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        return App {
            tasks,
            ..Default::default()
        };
    }

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

        components::main_layout::render(frame, layout[0], &self);
        components::help_bar::render(frame, layout[1]);
    }

    fn handle_event(&mut self) {
        if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
            match key.code {
                crossterm::event::KeyCode::Char('q') => {
                    self.exit = true;
                }
                crossterm::event::KeyCode::Tab => {
                    self.focus = self.focus.next();
                }
                crossterm::event::KeyCode::BackTab => self.focus = self.focus.previous(),
                _ => {}
            }
        }
    }
}
