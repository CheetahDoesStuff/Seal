use ratatui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct App {
    regex: String,
    input: String,
    output: String,
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

    pub fn get_output(&mut self) -> &str {
        &self.input
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
    fn test_app_get_output() {
        let mut app = App::default();
        app.output = "some output".to_string();
        assert_eq!(app.get_output(), "some output");
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::default();
        app.quit();
        assert_eq!(app.exit, true);
    }
}