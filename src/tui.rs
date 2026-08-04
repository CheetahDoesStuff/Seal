use std::{io, panic};

use crossterm::{event::EnableMouseCapture, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use color_eyre::Result;

use crate::{app::App, events::EventHandler, ui};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventHandler
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<()> {
        let _ = terminal::enable_raw_mode();
        let _ = execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture);

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal :(");
            panic_hook(panic);
        }));

        let _ = self.terminal.hide_cursor();
        let _ = self.terminal.clear();
        Ok(())
    }

    fn reset() -> Result<()> {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen, EnableMouseCapture);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        let _ = Self::reset();
        let _ = self.terminal.show_cursor();
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        let _ = self.terminal.draw(|frame| ui::render(app, frame));
        Ok(())
    }
}