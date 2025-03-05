use std::collections::HashMap;
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
            Color::White => 1_u64 << 1, // 2
            Color::Blue => 1 << 2,      // 4
            Color::Red => 1 << 3,       // 8
            Color::Black => 1 << 4,     // 16
            Color::Green => 1 << 5,     // 32
            Color::Colorless => 1 << 6, // 64
            Color::Unknown => 1 << 7,   // 128
                                         // Color::White && Color::Red -> 10
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

impl Colors {
    pub fn to_bitset(&self) -> i32 {
        let mut bits = 0_u64;
        for c in &self.0 {
            bits |= c.to_bit();
        }
        bits.to_string().parse::<i32>().unwrap()
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

pub fn convert_cost(cost_string: &str) -> Result<String, String> {
    // 色とその対応関係を定義
    let color_source = [
        ('w', "白"),
        ('u', "青"),
        ('r', "赤"),
        ('k', "黒"),
        ('g', "緑"),
        ('l', "無"),
        ('x', "?"),
    ];

    let mut natural_to_internal = HashMap::new();
    for (internal, natural) in &color_source {
        natural_to_internal.insert(*natural, *internal);
    }

    // 全角数字から半角数字への変換テーブル
    let full_to_half_digits = [
        ('０', '0'),
        ('１', '1'),
        ('２', '2'),
        ('３', '3'),
        ('４', '4'),
        ('５', '5'),
        ('６', '6'),
        ('７', '7'),
        ('８', '8'),
        ('９', '9'),
    ];

    let mut digit_map = HashMap::new();
    for (full, half) in full_to_half_digits {
        digit_map.insert(full, half);
    }

    // 全角数字を半角数字に置換
    let normalized_input: String = cost_string
        .chars()
        .map(|c| *digit_map.get(&c).unwrap_or(&c)) // 全角なら対応する半角に、対応がなければそのまま
        .collect();

    let mut result = String::new();
    let mut chars = normalized_input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '《' {
            // 色の開始を検知 (例: "白")
            let mut color = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == '》' {
                    chars.next(); // '》' を消費
                    break;
                }
                color.push(next_c);
                chars.next();
            }

            // 対応する内部表記に変換
            let internal = natural_to_internal.get(color.as_str())
                .ok_or_else(|| format!("Unexpected color '{}'", color))?;

            // 次が "×" か確認
            if chars.next() != Some('×') {
                return Err("Invalid input, expected '×' after color".to_string());
            }

            // 数量を取得
            let mut count = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit() {
                    count.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            let count: usize = count.parse().map_err(|_| "Failed to parse count".to_string())?;
            result.push(*internal);
            result.push_str(&count.to_string());
        }
    }

    Ok(result)
}

// テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_cost() {
        let cases = vec![
            ("《白》×３《無》×１", Ok("w3l1".to_string())),
            ("《青》×２《赤》×１", Ok("u2r1".to_string())),
            ("《黒》×４《緑》×２", Ok("k4g2".to_string())),
            ("《無》×５", Ok("l5".to_string())),
            ("《白》×１《青》×１《赤》×１", Ok("w1u1r1".to_string())),
            ("《黄》×1", Err("Unexpected color '黄'".to_string())),
            // ascii混在
            ("《白》×3《無》×１", Ok("w3l1".to_string())),
            ("《青》×２《赤》×1", Ok("u2r1".to_string())),
            ("《黒》×4《緑》×２", Ok("k4g2".to_string())),
            ("《無》×5", Ok("l5".to_string())),
            ("《白》×1《青》×１《赤》×１", Ok("w1u1r1".to_string())),
            ("《黄》×１", Err("Unexpected color '黄'".to_string())),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_cost(input), expected);
        }
    }
}
