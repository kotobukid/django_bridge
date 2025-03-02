use crate::wixoss::card::CardType;
use crate::wixoss::color::Colors;
use crate::wixoss::feature::CardFeature;
use crate::wixoss::format::Format;
use crate::wixoss::{
    element_to_name_and_pronounce, flatten_break, parse_card_skill, parse_format, parse_story,
    split_by_break, Card, OptionString, Skills, WixossCard,
};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ArtsCraft {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: Colors,
    // level: OptionString,
    cost: OptionString,
    // limit: OptionString,
    // power: OptionString,
    user: OptionString,
    time: Vec<String>,
    story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl From<ArtsCraft> for Card {
    fn from(val: ArtsCraft) -> Self {
        Card {
            no: val.no.clone(),
            name: val.name.clone(),
            pronounce: val.pronounce.clone(),
            artist: val.artist.clone(),
            card_type: val.card_type.clone(),
            klass: OptionString::empty(),
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
        }
    }
}

impl WixossCard for ArtsCraft {
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

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::ArtsCraft,
            color: Colors::from(card_data[2].clone()),
            cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            user: OptionString::from_string(card_data[1].clone()),
            time: split_by_break(card_data[9].clone()),
            story: parse_story(card_data[11].clone().trim().to_string()),
            format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
        }
    }
}

impl Display for ArtsCraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NO.\t:{}", self.no)?;
        writeln!(f, "Name\t:{}", self.name)?;
        writeln!(f, "読み\t:{}", self.pronounce)?;
        writeln!(f, "絵\t:{}", self.artist)?;
        writeln!(f, "Type\t:{}", self.card_type)?;
        writeln!(f, "色\t:{}", self.color)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        writeln!(f, "コスト\t:{}", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        writeln!(f, "ルリグタイプ\t:{}", self.user)?;
        writeln!(f, "タイミング\t:{}", self.time.join(", "))?;
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
