use serde::Serialize;
use std::collections::HashMap;
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

/// カード色のテーマカラー定義
#[derive(Debug, Clone, Serialize)]
pub struct ColorTheme {
    /// ベース色（メインカラー）
    pub base: &'static str,
    /// 強調色（ボーダーやアクセントに使用）
    pub accent: &'static str,
    /// うっすら色（背景色に使用）
    pub light: &'static str,
}

impl Color {
    /// 各色のテーマカラーを返す
    pub fn theme(&self) -> ColorTheme {
        match self {
            Color::White => ColorTheme {
                base: "#fff1b4",
                accent: "#f5d872",  // より濃いクリーム色
                light: "#fffdf0",   // より薄いクリーム色
            },
            Color::Blue => ColorTheme {
                base: "#b4ceff",
                accent: "#6b9eff",  // より濃い青
                light: "#e6f0ff",   // より薄い青
            },
            Color::Red => ColorTheme {
                base: "#ffb4b4",
                accent: "#ff7a7a",  // より濃い赤
                light: "#ffe6e6",   // より薄い赤
            },
            Color::Black => ColorTheme {
                base: "rgb(176, 150, 255)",
                accent: "rgb(139, 101, 255)",  // より濃い紫
                light: "rgb(225, 217, 255)",   // より薄い紫
            },
            Color::Green => ColorTheme {
                base: "#ccffb4",
                accent: "#8eff66",  // より濃い緑
                light: "#e8ffe0",   // より薄い緑
            },
            Color::Colorless => ColorTheme {
                base: "#cfcfcf",
                accent: "#a0a0a0",  // より濃いグレー
                light: "#f0f0f0",   // より薄いグレー
            },
            Color::Unknown => ColorTheme {
                base: "#ffffff",
                accent: "#cccccc",
                light: "#fafafa",
            },
        }
    }

    #[allow(dead_code)]
    pub fn to_bit(&self) -> i32 {
        match self {
            Color::White => 1_i32 << 1, // 2
            Color::Blue => 1 << 2,      // 4
            Color::Red => 1 << 3,       // 8
            Color::Black => 1 << 4,     // 16
            Color::Green => 1 << 5,     // 32
            Color::Colorless => 1 << 6, // 64
            Color::Unknown => 1 << 7,   // 128
                                         // Color::White && Color::Red -> 10
        }
    }

    // 指定した `Color` を CSS のカラーコードに変換する関数
    pub fn to_css_color_code(&self) -> &'static str {
        match self {
            Color::White => "#fff1b4",
            Color::Blue => "#b4ceff",
            Color::Red => "#ffb4b4",
            Color::Black => "rgb(176, 150, 255)",
            Color::Green => "#ccffb4",
            Color::Colorless => "#cfcfcf",
            _ => "#ffffff", // デフォルト色
        }
    }
}

#[allow(dead_code)]
pub fn from_bits(bits: i32) -> Vec<Color> {
    let mut colors = Vec::new();

    if bits & 1_i32 << 1 != 0 {
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
        let mut bits = 0_i32;
        for c in &self.0 {
            bits |= c.to_bit();
        }
        bits.to_string().parse::<i32>().unwrap()
    }

    /// 与えられたビットセット (i32) から CSSグラデーション定義を生成するメソッド
    pub fn bits_to_gradient(bits: i32) -> String {
        // ビットセットから有効な色を取得
        let colors = from_bits(bits);

        if colors.is_empty() {
            return "".to_string(); // 空の場合
        }

        let offset: usize = 10; // グラデーションの両端の余白
        let width_1 = if colors.len() > 1 {
            (100 - (offset * 2)) / (colors.len() - 1)
        } else {
            return format!(
                "background-color: {};",
                colors.first().unwrap().to_css_color_code()
            );
        };

        // 各色のグラデーションコードを生成
        let gradient_code: String = colors
            .iter()
            .enumerate()
            .map(|(i, color)| {
                let color_code = color.to_css_color_code();
                format!("{} {}%", color_code, i * width_1 + offset)
            })
            .collect::<Vec<String>>()
            .join(",");

        let result = format!("background: linear-gradient(to right, {});", gradient_code);

        result
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
        ('c', "コイン"),
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
            let internal = if color.contains("コイン") {
                // コインアイコンの場合
                'c'
            } else {
                *natural_to_internal
                    .get(color.as_str())
                    .ok_or_else(|| format!("Unexpected color '{}'", color))?
            };

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

            let count: usize = count
                .parse()
                .map_err(|_| "Failed to parse count".to_string())?;
            result.push(internal);
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
            ("《無》×０", Ok("l0".to_string())),
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
            ("《コインアイコン》×１", Ok("c1".to_string())),
            ("《コインアイコン》×２", Ok("c2".to_string())),
            ("《黄》×１", Err("Unexpected color '黄'".to_string())),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_cost(input), expected);
        }
    }
}
