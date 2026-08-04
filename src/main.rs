use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::App, events::EventHandler};

pub mod app;
pub mod events;

fn main() {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend);
    let events = EventHandler::new(250);
}
