use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum Color {
    White,
    Blue,
    Red,
    Black,
    Green,
    Colorless,
    Unknown,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "白"),
            Color::Blue => write!(f, "青"),
            Color::Red => write!(f, "赤"),
            Color::Black => write!(f, "黒"),
            Color::Green => write!(f, "緑"),
            Color::Colorless => write!(f, "無"),
            Color::Unknown => write!(f, "?"),
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "w" => Color::White,
            "u" => Color::Blue,
            "b" => Color::Red,
            "r" => Color::Black,
            "g" => Color::Green,
            "l" => Color::Colorless,
            "W" => Color::White,
            "U" => Color::Blue,
            "B" => Color::Red,
            "R" => Color::Black,
            "G" => Color::Green,
            "L" => Color::Colorless,
            "白" => Color::White,
            "青" => Color::Blue,
            "赤" => Color::Red,
            "黒" => Color::Black,
            "緑" => Color::Green,
            "無" => Color::Colorless,
            _ => Color::Unknown,
        }
    }
}

impl Color {
    #[allow(dead_code)]
    pub fn to_bit(&self) -> u64 {
        match self {
            Color::White => 1_u64 << 1,
            Color::Blue => 1 << 2,
            Color::Red => 1 << 3,
            Color::Black => 1 << 4,
            Color::Green => 1 << 5,
            Color::Colorless => 1 << 6,
            Color::Unknown => 1 << 7,
        }
    }
}

#[allow(dead_code)]
pub fn from_bits(bits: u64) -> Vec<Color> {
    let mut colors = Vec::new();

    if bits & 1_u64 << 1 != 0 {
        colors.push(Color::White);
    }

    if bits & 1 << 2 != 0 {
        colors.push(Color::Blue);
    }

    if bits & 1 << 3 != 0 {
        colors.push(Color::Red);
    }
    if bits & 1 << 4 != 0 {
        colors.push(Color::Black);
    }
    if bits & 1 << 5 != 0 {
        colors.push(Color::Green);
    }
    if bits & 1 << 6 != 0 {
        colors.push(Color::Colorless);
    }

    colors
}

#[derive(Debug, Clone, Serialize)]
pub struct Colors(pub Vec<Color>);
impl Display for Colors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", s)
    }
}

impl From<String> for Colors {
    fn from(value: String) -> Self {
        Colors(
            value
                .chars()
                .map(|s| Color::from(s.to_string().as_str()))
                .collect(),
        )
    }
}
