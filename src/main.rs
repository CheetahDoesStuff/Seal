use color_eyre::Result;
use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use ratatui::layout::Constraint::{self, Fill};
use ratatui::layout::Layout;
use ratatui::widgets::{Block, Borders};
use ratatui::{Terminal, backend::CrosstermBackend};
use ratatui_textarea::CursorMove;

use crate::events::Event;
use crate::{app::App, events::EventHandler, tui::{Focus, Tui}};

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
    let mut scroll = 0;

    while !app.exit {
        let _ = tui.draw(&mut app);
        scroll = 0;
        match tui.events.next()? {
            Event::Key(key) => {
                match (key.code, key.modifiers) {
                    (KeyCode::Tab, _) => {
                        tui.focus = match tui.focus {
                            Focus::Regex => Focus::Input,
                            Focus::Input => Focus::Regex,
                        };
                    }
                    
                    (KeyCode::Char('e'), KeyModifiers::CONTROL) => { app.quit(); }
                    (KeyCode::Char('r'), KeyModifiers::CONTROL) => { app.switch_output_type(); }
                    (KeyCode::Up, KeyModifiers::SHIFT) => { scroll -= 1; }
                    (KeyCode::Down, KeyModifiers::SHIFT) => { scroll += 1; }
                    (KeyCode::Char('C'), KeyModifiers::CONTROL | KeyModifiers::SHIFT) |
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        tui.focused_mut().copy();
                    }
                    (KeyCode::Char('V'), KeyModifiers::CONTROL | KeyModifiers::SHIFT) |
                    (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                        tui.focused_mut().paste();
                    }
                    (KeyCode::Up, mods) if !mods.contains(KeyModifiers::SHIFT) => tui.focused_mut().move_cursor(CursorMove::Up),
                    (KeyCode::Down, mods) if !mods.contains(KeyModifiers::SHIFT) => tui.focused_mut().move_cursor(CursorMove::Down),
                    (KeyCode::Left, _) => tui.focused_mut().move_cursor(CursorMove::Back),
                    (KeyCode::Right, _) => tui.focused_mut().move_cursor(CursorMove::Forward),
                    (KeyCode::Backspace, _) => { tui.focused_mut().delete_char(); }
                    (KeyCode::Delete, _) => { tui.focused_mut().delete_next_char(); }
                    (KeyCode::Char(c), _) => { tui.focused_mut().insert_char(c); }
                    (KeyCode::Enter, _) => { if tui.focus == Focus::Input { tui.focused_mut().insert_newline(); } }
                    _ => {}
                }

                let regex_text = tui.regex_input.lines().join("\n");
                let input_text = tui.input.lines().join("\n");
                app.update_regex(&regex_text);
                app.update_input(&input_text);
            }
            Event::Mouse(mouse_event) => {
                let size = tui.terminal.size()?;
                let parent_chunks = Layout::vertical([Constraint::Length(3), Fill(1)]).split(size.into());
                let inputs_output_chunks = Layout::horizontal([Constraint::Percentage(50), Fill(1)]).split(parent_chunks[1]);
                let input_chunks = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Fill(1)
                ]).split(inputs_output_chunks[0]);

                let regex_area = Block::default().borders(Borders::ALL).inner(input_chunks[0]);
                let input_area = Block::default().borders(Borders::ALL).inner(input_chunks[1]);

                let inside = |area: ratatui::layout::Rect, col: u16, row: u16| {
                    col >= area.left() && col < area.right() && row >= area.top() && row < area.bottom()
                };

                let mouse_col = mouse_event.column;
                let mouse_row = mouse_event.row;

                match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        if inside(regex_area, mouse_col, mouse_row) {
                            tui.focus = Focus::Regex;
                            let text_row = (mouse_row - regex_area.top()) as usize;
                            let text_col = (mouse_col - regex_area.left()) as usize;
                            tui.regex_input.move_cursor(CursorMove::Jump(text_row as u16, text_col as u16));
                            tui.regex_input.start_selection();
                            tui.mouse_dragging = true;
                        } else if inside(input_area, mouse_col, mouse_row) {
                            tui.focus = Focus::Input;
                            let text_row = (mouse_row - input_area.top()) as usize;
                            let text_col = (mouse_col - input_area.left()) as usize;
                            tui.input.move_cursor(CursorMove::Jump(text_row as u16, text_col as u16));
                            tui.input.start_selection();
                            tui.mouse_dragging = true;
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if tui.mouse_dragging {
                            let (area, textarea) = match tui.focus {
                                Focus::Regex => (regex_area, &mut tui.regex_input),
                                Focus::Input => (input_area, &mut tui.input),
                            };
                            if inside(area, mouse_col, mouse_row) {
                                let text_row = (mouse_row - area.top()) as usize;
                                let text_col = (mouse_col - area.left()) as usize;
                                textarea.move_cursor(CursorMove::Jump(text_row as u16, text_col as u16));
                            }
                        }
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        tui.mouse_dragging = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    let _ = tui.exit();
    Ok(())
}
