use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let parent_chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
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
}