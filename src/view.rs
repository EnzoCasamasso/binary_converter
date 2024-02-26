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
    hex: Vec<(usize, u8)>,
    cursor_pos: usize,
    bytes_per_line: usize,
}
impl FileView {
    pub fn draw(&self, f: &mut Frame) {}
}
