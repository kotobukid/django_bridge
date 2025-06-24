use std::fmt::{Display, Formatter};
// 再エクスポート
pub use crate::gen::django_models::{CardDb, CreateCard};
use crate::new_type;

new_type!(Card, CardDb);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用 -> Derefで不要に
        write!(f, "{}", self.name)
    }
}

impl Card {
    pub fn to_custom_string(&self) -> String {
        self.name.to_string()
    }

    pub fn rust_code_head() -> &'static str {
        r###"struct CardStatic {"###
    }

    pub fn to_rust_code(&self) -> String {
        self.to_rust_code_with_klass_bits(0u64)
    }

    pub fn to_rust_code_with_klass_bits(&self, klass_bits: u64) -> String {
        let cost_ = self.cost.clone().unwrap_or("".into());
        let level_: String = self.level.map(|l| l.to_string()).unwrap_or("".into());
        let limit_: String = self.limit.map(|l| l.to_string()).unwrap_or("".into());
        let limit_ex_: String = self.limit_ex.map(|l| l.to_string()).unwrap_or("".into());
        let power_: String = self.power.clone().unwrap_or("".into());
        let skill_text_: String = self.skill_text.clone().unwrap_or("".into());
        let burst_text_: String = self.burst_text.clone().unwrap_or("".into());
        let story_: String = self.story.clone().unwrap_or("".into());
        let rarity_: String = self.rarity.clone().unwrap_or("".into());
        let url_: String = self.url.clone().unwrap_or("".into());
        let ex1_: String = self.ex1.clone().unwrap_or("".into());
        
        // Auto-calculate has_burst based on card_type and burst_text
        let has_burst_calculated = match self.card_type {
            5 | 6 | 10 | 11 => { // シグニ、スペル、クラフト系
                if !burst_text_.is_empty() { 
                    1 // LBあり
                } else { 
                    2 // LBなし
                }
            },
            _ => 0 // 指定なし（ルリグ、アーツなど）
        };

        format!(
            r###"({id}_i32,"{name}","{code}","{pronunciation}",{color}_u32,"{cost}","{level}","{limit}","{limit_ex}","{power}",{has_burst}_u8,"{skill_text}","{burst_text}",{format}_u8,"{story}","{rarity}","{url}",{card_type}_u8,{product}_u8,{timing}_u8,{feature_bits1}_i64,{feature_bits2}_i64,{klass_bits}_u64,{burst_bits}_i64,"{ex1}"),"###,
            id = self.id,
            name = self.name,
            code = self.code,
            pronunciation = self.pronunciation,
            color = self.color,
            cost = cost_,
            level = level_,
            limit = limit_,
            limit_ex = limit_ex_,
            power = power_,
            has_burst = has_burst_calculated,
            skill_text = skill_text_,
            burst_text = burst_text_,
            format = self.format,
            story = story_,
            rarity = rarity_,
            url = url_,
            card_type = self.card_type,
            product = self.product,
            timing = self.timing.unwrap_or(0),
            feature_bits1 = self.feature_bits1,
            feature_bits2 = self.feature_bits2,
            klass_bits = klass_bits,
            burst_bits = self.burst_bits,
            ex1 = ex1_,
        )
    }
}
