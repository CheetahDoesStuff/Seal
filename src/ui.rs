use ratatui::{Frame, layout::{Alignment, Constraint::{self, Fill}, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};
use ratatui_textarea::TextArea;

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let parent_chunks = Layout::vertical([
        Constraint::Length(3),
        Fill(1)
    ]).split(frame.area());
    
    let top_block = Block::default()
                    .title(" Seal - Regex Editor ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
    
    let top_content = Paragraph::new(format!("This is a pretty cool test! output: {}", app.get_output()))
                    .block(top_block)
                    .style(Style::default().fg(Color::Cyan))
                    .alignment(Alignment::Center);

    frame.render_widget(top_content, parent_chunks[0]);

    let inputs_output_chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Fill(1)
    ]).split(parent_chunks[1]);

    let input_chunks = Layout::vertical([
        Constraint::Length(3),
        Fill(1)
    ]).split(inputs_output_chunks[0]);

    let regex_block = Block::default()
                    .title("Regex")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
    
    let mut regex_content = TextArea::default();
    regex_content.set_selection_style(Style::default().fg(Color::DarkGray).bg(Color::Gray));
    regex_content.set_cursor_line_style(Style::default());
    regex_content.set_block(regex_block);
    
    frame.render_widget(&regex_content, input_chunks[0]);
}