use color_eyre::owo_colors::style;
use ratatui::{Frame, layout::{Alignment, Constraint::{self, Fill}, Layout}, style::{Color, Style, Styled}, widgets::{Block, BorderType, Borders, Paragraph}};
use ratatui_textarea::TextArea;

use crate::{app::App, tui::Focus};

pub fn render(app: &mut App, regex_input: &mut TextArea, input: &mut TextArea, focus: Focus, frame: &mut Frame) {
    let parent_chunks = Layout::vertical([
        Constraint::Length(3),
        Fill(1)
    ]).split(frame.area());
    
    let top_block = Block::default()
                    .title(" Seal - Regex Editor ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .border_type(BorderType::Rounded);
    
    let top_content = Paragraph::new(format!("<CONTROL-e> Exit | <TAB> Change input focus | <CONTROL-r> Switch output type"))
                    .block(top_block)
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

    let focused_style = Style::default().fg(Color::Cyan);
    let unfocused_style = Style::default();

    let regex_block = Block::default()
                    .title(" Regex ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(if focus == Focus::Regex { focused_style } else { unfocused_style });
    
    regex_input.set_block(regex_block);

    let input_block = Block::default()
                    .title(" Input text ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(if focus == Focus::Input { focused_style } else { unfocused_style });
    
    input.set_block(input_block);

    frame.render_widget(&*regex_input, input_chunks[0]);
    frame.render_widget(&*input, input_chunks[1]);
}