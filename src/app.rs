use fancy_regex::Regex;
use ratatui::{style::{Color, Style, Styled}, text::{Line, Span, Text}};

#[derive(Default, Debug, PartialEq)]
pub enum OutputType {
    #[default]
    Highlight,
    Extract,
    ExtractRaw,

}

impl OutputType {
    pub fn next(&self) -> Self {
        match self {
            OutputType::Highlight => OutputType::Extract,
            OutputType::Extract => OutputType::ExtractRaw,
            OutputType::ExtractRaw => OutputType::Highlight,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    regex: String,
    input: String,
    output: Text<'static>,
    pub output_type: OutputType,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn update_regex(&mut self, regex: &str) {
        self.regex = regex.to_string()
    }

    pub fn update_input(&mut self, input: &str) {
        self.input = input.to_string()
    }

    pub fn get_output(&mut self) -> Text<'_> {
        let output = match self.output_type {
            OutputType::Highlight => self.regex_highlight(),
            OutputType::Extract => self.regex_extract(),
            OutputType::ExtractRaw => self.regex_extract_raw(),
        };
        self.output = output.clone();
        return output
    }

    pub fn switch_output_type(&mut self) {
        self.output_type = self.output_type.next();
    }

    fn regex_highlight(&self) -> Text<'static> {
        let re = Regex::new(&self.regex).expect("Invalid regex pattern");
        let input = &self.input;

        let mut lines: Vec<Line<'static>> = Vec::new();
        let mut current_line_spans: Vec<Span<'static>> = Vec::new();
        
        let mut current_idx = 0;
        let mut match_count = 0;

        let mut push_chunk = |text: &str, style: Style| {
            let mut lines_iter = text.split('\n').peekable();
            while let Some(line_str) = lines_iter.next() {
                if !line_str.is_empty() {
                    current_line_spans.push(Span::styled(line_str.to_string(), style));
                }
                if lines_iter.peek().is_some() {
                    lines.push(Line::from(std::mem::take(&mut current_line_spans)));
                }
            }
        };

        for mat_res in re.find_iter(input) {
            if let Ok(mat) = mat_res {
                let start = mat.start();
                let end = mat.end();
                if start > current_idx {
                    push_chunk(&input[current_idx..start], Style::default());
                }
                let style = if match_count % 2 == 0 {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default().bg(Color::LightBlue).fg(Color::Black)
                };

                push_chunk(&input[start..end], style);
                current_idx = end;
                match_count += 1;
            }
        }

        if current_idx < input.len() {
            push_chunk(&input[current_idx..], Style::default());
        }

        lines.push(Line::from(current_line_spans));
        Text::from(lines)
    }


    fn regex_extract(&self) -> Text<'static> {
        let re = Regex::new(&self.regex).expect("Invalid regex pattern");
        let mut lines: Vec<Line<'static>> = Vec::new();
        let mut match_count = 0;

        for mat_res in re.find_iter(&self.input) {
            if let Ok(mat) = mat_res {
                let style = if match_count % 2 == 0 {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default().bg(Color::LightBlue).fg(Color::Black)
                };

                let styled_num = format!("{}.", match_count + 1).set_style(style);
                let styled_match = format!(" {}", mat.as_str());
                let line = Line::from(vec![styled_num, styled_match.into()]);
                lines.push(line);
                match_count += 1;
            }
        }

        return Text::from(lines)
    }

    fn regex_extract_raw(&self) -> Text<'static> {
        let re = Regex::new(&self.regex).expect("Invalid regex pattern");
        let mut spans: Vec<Span<'static>> = Vec::new();
        let mut match_count = 0;

        for mat_res in re.find_iter(&self.input) {
            if let Ok(mat) = mat_res {
                let style = if match_count % 2 == 0 {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default().bg(Color::LightBlue).fg(Color::Black)
                };

                let styled_match = format!("{}", mat.as_str()).set_style(style);
                spans.push(styled_match);
                match_count += 1;
            }
        }

        return Text::from(Line::from(spans))
    }
}

// --------------------- Tests --------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_set_regex() {
        let mut app = App::default();
        app.update_regex("some regex");
        assert_eq!(app.regex, "some regex");
    }

    #[test]
    fn test_app_set_input() {
        let mut app = App::default();
        app.update_input("some input");
        assert_eq!(app.input, "some input");
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::default();
        app.quit();
        assert_eq!(app.exit, true);
    }

    #[test]
    fn test_app_change_output_type() {
        let mut app = App::default();
        app.output_type = OutputType::Highlight;
        assert_eq!(app.output_type, OutputType::Highlight);
        app.switch_output_type();
        assert_eq!(app.output_type, OutputType::Extract);
    }
}