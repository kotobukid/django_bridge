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

macro_rules! to_value {
    ($x:expr) => {
        $x.clone().to_string()
    };
    ($x:expr, $y:expr) => {
        $y
    };
}

impl Card {
    pub fn to_custom_string(&self) -> String {
        self.name.to_string()
    }


    pub fn rust_code_head() -> &'static str {
        r###"struct CardStatic {"###
    }
    pub fn to_rust_code(&self) -> String {
        let cost_ = self.cost.clone().unwrap_or("".into());
        let level_: String = to_value!(self.level.clone(), "".into());
        let limit_: String = to_value!(self.limit.clone(), "".into());
        let limit_ex_: String = to_value!(self.limit_ex.clone(), "".into());
        let power_: String = to_value!(self.power.clone(), "".into());
        let skill_text_: String = self.skill_text.clone().unwrap_or("".into());
        let burst_text_: String = self.burst_text.clone().unwrap_or("".into());
        let story_: String = self.story.clone().unwrap_or("".into());
        let rarity_: String = self.rarity.clone().unwrap_or("".into());
        let url_: String = self.url.clone().unwrap_or("".into());

        format!(
            r###"({id}_i32,"{name}","{code}","{pronunciation}",{color}_u32,"{cost}","{level}","{limit}","{limit_ex}","{power}",{has_burst}_u8,"{skill_text}","{burst_text}",{format}_u8,"{story}","{rarity}","{url}",{card_type}_u8,{product}_u8),"###,
            id = self.id,
            name = self.name,
            code = self.code,
            pronunciation = self.pronunciation,
            color = self.color,
            cost = cost_,
            level =level_,
            limit = limit_,
            limit_ex = limit_ex_,
            power = power_,
            has_burst = self.has_burst,
            skill_text = skill_text_,
            burst_text = burst_text_,
            format = self.format,
            story = story_,
            rarity = rarity_,
            url = url_,
            card_type = self.card_type,
            product = self.product
        )
    }
}
