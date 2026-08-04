use color_eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, events::EventHandler, tui::Tui};

pub mod app;
pub mod events;
pub mod tui;
pub mod ui;
pub mod update;

fn main() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend).unwrap();
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    let _ = tui.enter();
    while !app.exit {
        let _ = tui.draw(&mut app);
        match tui.events.next().unwrap() {
            events::Event::Tick => {},
            events::Event::Key(key_event) => update::update(&mut app, key_event),
            events::Event::Mouse(mouse_event) => {},
            events::Event::Resize(_, _) => {},
        }
    }
    let _ = tui.exit();
    Ok(())
}
