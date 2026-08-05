use fancy_regex::Regex;
use ratatui::text::Line;

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
pub struct App<'a> {
    regex: String,
    input: String,
    output: Line<'a>,
    pub output_type: OutputType,
    pub exit: bool,
}

impl<'a> App<'a> {
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

    pub fn get_output(&mut self) -> Line<'a> {
        self.output = match self.output_type {
            OutputType::Highlight => self.regex_highlight(),
            OutputType::Extract => Line::from("Todo D:"),
            OutputType::ExtractRaw => Line::from("Todo D:"),
        }
        self.output.clone()
    }

    pub fn switch_output_type(&mut self) {
        self.output_type = self.output_type.next();
    }

    pub fn get_output_highlight(&mut self) -> Line<'a> {
        return "test".into();
    }

    fn regex_highlight(&self) -> Line<'a> {
        let re = Regex::new(&self.input).expect("Invalid regex pattern");
        let mut spans = Vec::new();
        let mut current_idx = 0;
        let mut match_count = 0;



        return Line::from(spans)
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