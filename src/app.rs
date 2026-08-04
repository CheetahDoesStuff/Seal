use std::io;

use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    regex: String,
    input: String,
    output: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn update_regex(&mut self, regex: &str) {
        self.regex = regex.to_string()
    }

    pub fn update_input(&mut self, input: &str) {
        self.input = input.to_string()
    }

    pub fn get_output(&mut self) -> &str {
        &self.output
    }
}