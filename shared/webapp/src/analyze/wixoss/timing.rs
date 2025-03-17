use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum Timing {
    Main = 1 << 1,
    Attack = 1 << 2,
    Cutin = 1 << 3,
    Unknown = 1 << 0,
}

impl Timing {
    fn from_string(s: String) -> Self {
        match s.as_str() {
            "メインフェイズ" => Timing::Main,
            "アタックフェイズ" => Timing::Attack,
            "スペルカットイン" => Timing::Cutin,
            _ => Timing::Unknown,
        }
    }
}

impl Display for Timing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Timing::Main => "メインフェイズ",
            Timing::Attack => "アタックフェイズ",
            Timing::Cutin => "スペルカットイン",
            _ => "",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Default)]
pub struct TimingList {
    values: Vec<Timing>,
}

impl TimingList {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn to_bitset(&self) -> i32 {
        let mut bitset = 0;
        for timing in &self.values {
            bitset |= timing.clone() as u32;
        }
        bitset.to_string().parse::<i32>().unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn add(&mut self, timing: Timing) {
        self.values.push(timing);
    }

    pub fn from_vec_string(vec_string: Vec<String>) -> Self {
        let mut values = vec![];
        for timing_str in vec_string {
            let timing = Timing::from_string(timing_str);
            values.push(timing);
        }
        Self { values }
    }
}

impl Display for TimingList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .values
            .iter()
            .map(|timing| timing.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", s)
    }
}
