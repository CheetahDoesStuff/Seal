use std::io;

use ratatui::DefaultTerminal;

pub struct App {
    regex: String,
    input: String,
    output: String,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {

        }
        
        Ok(())
    }
}