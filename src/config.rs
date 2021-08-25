#[derive(Clone, PartialEq)]
pub enum ReadStyle {
    Default, // spits out text to console
    Top, // spits out text to console; moves cursor to top of file
    LineByLine // makes the user press enter to print each line
}
impl ReadStyle {
    pub fn from_string(string: &str) -> Self {
        let lower: &str = &string.to_lowercase();
        match lower {
            "lbl" | "line_by_line" => Self::LineByLine,
            "t" | "top" => Self::Top,
            _ => Self::Default,
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub path: String,
    pub read_style: ReadStyle,
    pub numbered_lines: bool,
    pub max_lines: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            path: String::from("."),
            read_style: ReadStyle::Default,
            numbered_lines: false,
            max_lines: 100
        }
    }
}