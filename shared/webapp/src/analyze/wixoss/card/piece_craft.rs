use crate::analyze::wixoss::card::CardType;
use crate::analyze::wixoss::color::Colors;
use feature::feature::{CardFeature, HashSetToBits};
use crate::analyze::wixoss::format::Format;
use crate::analyze::wixoss::{
    element_to_name_and_pronounce, flatten_break, parse_card_skill, parse_format, parse_story,
    split_by_break, Card, OptionString, Skills, WixossCard,
};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::analyze::wixoss::timing::TimingList;

#[derive(Debug)]
pub struct PieceCraft {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    color: Colors,
    // level: Option<i32>,
    cost: OptionString,
    // limit: Option<String>,
    // power: Option<String>,
    user: OptionString,
    time: TimingList,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
    feature_bits1: i64,
    feature_bits2: i64,
}

impl From<PieceCraft> for Card {
    fn from(val: PieceCraft) -> Self {
        Card {
            no: val.no.clone(),
            name: val.name.clone(),
            pronounce: val.pronounce.clone(),
            artist: val.artist.clone(),
            card_type: val.card_type.clone(),
            klass: None,
            color: val.color.clone(),
            level: OptionString::empty(),
            cost: val.cost.clone(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: val.user.clone(),
            time: val.time.clone(),
            story: val.story.clone(),
            format: val.format.clone(),
            rarity: val.rarity.clone(),
            skill: val.skill.clone(),
            features: val.features.clone(),
            feature_bits1: val.feature_bits1,
            feature_bits2: val.feature_bits2,
        }
    }
}

impl WixossCard for PieceCraft {
    fn from_source(source: String) -> Self {
        let document: Html = Html::parse_document(&source);

        let selector_card_num = Selector::parse(".cardNum").unwrap();
        let card_no = match document.select(&selector_card_num).next() {
            Some(card_no) => card_no.inner_html(),
            None => "unknown".into(),
        };

        let selector_card_name = Selector::parse(".cardName").unwrap();
        let card_name = match document.select(&selector_card_name).next() {
            Some(card_name) => element_to_name_and_pronounce(card_name.inner_html()),
            None => ("unknown".into(), "unknown".into()),
        };

        let selector_rarity = Selector::parse(".cardRarity").unwrap();
        let card_rarity = match document.select(&selector_rarity).next() {
            Some(card_rarity) => card_rarity.inner_html(),
            None => "unknown rarity".into(),
        };

        let selector_artist = Selector::parse(".cardImg p span").unwrap();
        let artist = match document.select(&selector_artist).next() {
            Some(artist) => artist.inner_html(),
            None => "unknown artist".into(),
        };

        let selector_card_data = Selector::parse(".cardData dd").unwrap();

        let mut card_data: Vec<String> = Vec::new();
        for element in document.select(&selector_card_data) {
            card_data.push(element.inner_html());
        }

        let selector_card_skill = Selector::parse(".cardSkill").unwrap();
        let mut card_skills: Vec<String> = Vec::new();
        for element in document.select(&selector_card_skill) {
            card_skills.push(element.inner_html());
        }

        let (skill, features) = parse_card_skill(card_skills.clone());
        let feature_bits = features.to_bits();

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::PieceCraft,
            color: Colors::from(card_data[2].clone()),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[8].clone()),
            time: TimingList::from_vec_string(split_by_break(card_data[9].clone())),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
            feature_bits1: feature_bits.0,
            feature_bits2: feature_bits.1,
        }
    }
}

impl Display for PieceCraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NO.\t:{}", self.no)?;
        writeln!(f, "Name\t:{}", self.name)?;
        writeln!(f, "読み\t:{}", self.pronounce)?;
        writeln!(f, "絵\t:{}", self.artist)?;
        writeln!(f, "Type\t:{}", self.card_type)?;
        // write!(f, "種族\t:{}\n", self.klass)?;
        writeln!(f, "色\t:{}", self.color)?;
        writeln!(f, "コスト\t:{}", self.cost)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        writeln!(f, "限定\t:{}", self.user)?;
        writeln!(f, "ストーリー\t:{}", self.story)?;
        writeln!(f, "フォーマット\t:{}", self.format)?;
        writeln!(f, "レアリティ\t:{}", self.rarity)?;
        writeln!(f, "テキスト({})\t:{}", self.skill.value.len(), self.skill)?;
        writeln!(
            f,
            "フィーチャー({})\t:{:?}",
            self.features.len(),
            self.features
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        write!(f, "")
    }
}
