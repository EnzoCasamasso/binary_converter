use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};
use std::io;

#[derive(Debug)]
pub struct FileView {
    file: Vec<usize>,
    cursor_pos: usize,
    bytes_per_line: usize,
}

impl FileView {
    pub new(file: Vec<usize>) -> Self {
       Self {
          file: file,
          cursor_pos: 0,
          bytes_per_line: 0
       };
    }

    pub fn run(&mut self) -> io::Result<(), Box<dyn std::io::Error>> {

    }

    pub fn draw(&self, f: &mut Frame) {

    }
}
