use crossterm::style::{Color, Colors};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyledChar {
    pub character: char,
    pub bold: bool,
    pub colour: Colors,
}

impl From<char> for StyledChar {
    fn from(value: char) -> Self {
        Self {
            character: value,
            bold: false,
            colour: Colors::new(Color::Reset, Color::Reset),
        }
    }
}

pub fn add_style_to_string(
    value: &str,
    fg_colour: Color,
    bg_colour: Color,
    bold: bool,
) -> Vec<StyledChar> {
    value
        .chars()
        .map(|c| StyledChar {
            character: c,
            bold,
            colour: Colors::new(fg_colour, bg_colour),
        })
        .collect()
}
