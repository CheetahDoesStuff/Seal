use std::{io, panic};

use crossterm::{event::EnableMouseCapture, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use color_eyre::Result;
use ratatui::{style::{Color, Style}};
use ratatui_textarea::TextArea;

use crate::{app::App, events::EventHandler, ui};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Regex,
    Input
}

pub struct Tui<'a> {
    pub(crate) terminal: CrosstermTerminal,
    pub events: EventHandler,
    pub regex_input: TextArea<'a>,
    pub input: TextArea<'a>,
    pub focus: Focus,
    pub mouse_dragging: bool,
}


impl<'a> Tui<'a> {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        let selection_style = Style::default()
            .fg(Color::DarkGray)
            .bg(Color::Gray);

        let mut regex_input = TextArea::default();
        regex_input.set_selection_style(selection_style);
        regex_input.set_cursor_line_style(ratatui::style::Style::default());

        let mut input = TextArea::default();
        input.set_selection_style(selection_style);
        input.set_cursor_line_style(ratatui::style::Style::default());

        Self { terminal, events, regex_input, input, focus: Focus::Regex, mouse_dragging: false }
    }

    pub fn focused_mut(&mut self) -> &mut TextArea<'a> {
        match self.focus {
            Focus::Regex => &mut self.regex_input,
            Focus::Input => &mut self.input,
        }
    }

    pub fn enter(&mut self) -> Result<()> {
        let _ = terminal::enable_raw_mode();
        let _ = execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture);

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal :(");
            panic_hook(panic);
        }));

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
        let regex_input = &mut self.regex_input;
        let input = &mut self.input;
        let focus = self.focus;
        let _ = self.terminal.draw(|frame| ui::render(app, regex_input, input, focus, frame));
        Ok(())
    }
}