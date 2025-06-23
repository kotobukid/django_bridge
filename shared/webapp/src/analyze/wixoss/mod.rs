pub mod card;
pub mod format;
mod selectors;
mod timing;

pub(crate) use crate::analyze::wixoss::card::{detect_card_type, CardType};
use crate::analyze::wixoss::format::Format;
use color::{convert_cost, Colors};
use feature::{create_detect_patterns, DetectPattern, ReplacePattern};

pub use crate::analyze::wixoss::card::{
    Arts, ArtsCraft, Key, Lrig, LrigAssist, Piece, PieceCraft, PieceRelay, Resona, ResonaCraft,
    Signi, SigniCraft, Spell, SpellCraft,
};
use crate::analyze::wixoss::selectors::{
    BR_SELECTOR, CARD_ARTIST, CARD_DATA_DD, CARD_NAME, CARD_NUM, CARD_RARITY, CARD_SKILL,
    SPAN_SELECTOR,
};
use crate::analyze::wixoss::timing::TimingList;
// Removed analyzer dependency to avoid cyclic dependency
// use analyzer::{AnalyzeRule, Analyzer};
use feature::feature::CardFeature;
use models::card::CreateCard;
use rayon::prelude::*;
use regex::Regex;
use scraper::Html;
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub trait WixossCard: Sized {
    fn from_source(source: String) -> Self;
}

// impl Display for dyn WixossCard {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}, {}",
//                &self.no,
//                &self.name,
//         )
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct OptionString {
    value: Option<String>,
}

impl OptionString {
    pub fn from_string(value: String) -> Self {
        // Noneの場合はNoneではなく""空文字
        if value == *"" {
            Self { value: None }
        } else {
            Self { value: Some(value) }
        }
    }

    pub fn empty() -> Self {
        Self { value: None }
    }

    pub fn to_option_integer(&self) -> Option<i32> {
        self.value
            .as_ref()
            .map(|v| v.parse::<i32>().map_err(|_| ()).unwrap_or(-1))
    }
}

impl Serialize for OptionString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.value {
            Some(s) => match s.as_str() {
                "" => serializer.serialize_str(""),
                "-" => serializer.serialize_str(""),
                _ => serializer.serialize_str(s),
            },
            None => serializer.serialize_str(""),
        }
    }
}

impl Display for OptionString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(v) => write!(f, "{}", v),
            None => write!(f, ""),
        }
    }
}

struct OptionInteger {
    value: Option<u32>,
}

#[allow(dead_code)]
impl OptionInteger {
    fn from(value: Option<u32>) -> Self {
        match value {
            Some(v) => Self { value: Some(v) },
            None => Self { value: None },
        }
    }

    fn to_option_string(&self) -> OptionString {
        match &self.value {
            Some(v) => OptionString::from_string(v.to_string()),
            None => OptionString::empty(),
        }
    }
}

impl Display for OptionInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value {
            None => write!(f, ""),
            Some(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CardSkill {
    Normal(String),
    LifeBurst(String),
}

const SKILL_PREFIX_NORMAL: &str = "N:";
const SKILL_PREFIX_LB: &str = "LB:";

impl CardSkill {
    fn from_string(s: String) -> Self {
        if s.starts_with(SKILL_PREFIX_LB) {
            Self::LifeBurst(s.replace(SKILL_PREFIX_LB, ""))
        } else if s.starts_with(SKILL_PREFIX_NORMAL) {
            Self::Normal(s.replace(SKILL_PREFIX_NORMAL, ""))
        } else {
            Self::Normal(s)
        }
    }
}

impl Serialize for CardSkill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for CardSkill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CardSkill::Normal(text) => write!(f, "{}{}", SKILL_PREFIX_NORMAL, text),
            CardSkill::LifeBurst(text) => write!(f, "{}{}", SKILL_PREFIX_LB, text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Skills {
    value: Vec<CardSkill>,
}

impl Skills {
    fn from_vec(skills: Vec<String>) -> Self {
        let value = skills.into_iter().map(CardSkill::from_string).collect();
        Skills { value }
    }

    pub fn as_vec(&self) -> Vec<String> {
        self.value.iter().map(|skill| skill.to_string()).collect()
    }
    pub fn get_normal_skills(&self) -> Vec<String> {
        self.value
            .iter()
            .filter_map(|skill| {
                if let CardSkill::Normal(text) = skill {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_life_burst_skills(&self) -> Vec<String> {
        self.value
            .iter()
            .filter_map(|skill| {
                if let CardSkill::LifeBurst(text) = skill {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Serialize for Skills {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
        for e in &self.value {
            seq.serialize_element(e)?;
        }
        seq.end()

        // let joined = self.value.join(",");
        // serializer.serialize_str(&joined)
    }
}

impl Display for Skills {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res: String = self.value.iter().map(|s| s.to_string()).collect();
        write!(f, "{res}")
    }
}

#[allow(dead_code)]
fn custom_vec_string_serialize<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = value
        .iter()
        .filter_map(|t| if t != "-" { Some(t.as_str()) } else { None })
        .collect::<Vec<&str>>()
        .join(", ");

    serializer.serialize_str(&joined)

    // let mut seq = serializer.serialize_seq(Some(value.len()))?;
    // for e in value.iter() {
    //     seq.serialize_element(e)?;
    // }
    // seq.end()
}

#[derive(Debug, Serialize)]
pub struct Card {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    pub card_type: CardType,
    pub klass: Option<i64>,
    color: Colors,
    level: OptionString,
    cost: OptionString,
    limit: OptionString,
    power: OptionString,
    user: OptionString,

    time: TimingList,

    pub story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
    feature_bits1: i64,
    feature_bits2: i64,
    ex1: OptionString,
}

impl From<Card> for CreateCard {
    fn from(val: Card) -> Self {
        let burst = val.burst();
        let normal_skills = val.skill.get_normal_skills();
        let life_burst_skills = val.skill.get_life_burst_skills();
        let card_number = val.no;
        let cost = convert_cost(&val.cost.value.unwrap_or("".to_string())).unwrap_or_default();
        CreateCard {
            name: val.name,
            code: card_number.clone(),
            pronunciation: val.pronounce,
            color: val.color.to_bitset(),
            power: val.power.value,
            has_burst: burst,
            cost: Some(cost),
            level: val.level.to_option_integer(),
            limit: match val.card_type {
                CardType::Lrig => val.limit.to_option_integer(),
                CardType::LrigAssist => val.limit.to_option_integer(),
                _ => None,
            },
            limit_ex: match val.card_type {
                CardType::Signi => val.limit.to_option_integer(),
                CardType::SigniCraft => val.limit.to_option_integer(),
                CardType::Resona => val.limit.to_option_integer(),
                CardType::ResonaCraft => val.limit.to_option_integer(),
                _ => None,
            },
            burst_text: Some(life_burst_skills.join("\n")),
            format: match val.format {
                Format::AllStar => 111_i32,
                Format::KeySelection => 11_i32,
                Format::DivaSelection => 11_i32,
            },
            story: val.story.value,
            rarity: Some(val.rarity),
            timing: Some(val.time.to_bitset()),
            card_type: val.card_type.to_db_id(), // 検出されたcard_typeを使用
            product: 0,   // default
            url: None,
            skill_text: Some(normal_skills.join("\n")),
            feature_bits1: val.feature_bits1,
            feature_bits2: val.feature_bits2,
            burst_bits: 0, // Note: This conversion path doesn't implement burst feature detection - analyzer path does
            ex1: val.ex1.value,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let k = self.klass.unwrap_or(0);
        write!(
            f,
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}\n{}",
            self.no,
            self.name,
            self.pronounce,
            self.artist,
            self.card_type,
            k,
            self.color,
            self.level,
            self.cost,
            self.limit,
            self.power,
            self.user,
            self.time,
            self.story,
            self.format,
            self.rarity,
            self.skill,
            self.features
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Card {
    pub fn detect_card_type(text: &str) -> CardType {
        let document: Html = Html::parse_document(text);
        let selector_card_data = &*CARD_DATA_DD;

        let mut card_data: Vec<String> = Vec::new();
        for element in document.select(&selector_card_data) {
            card_data.push(element.inner_html());
        }

        let text = card_data[0].clone();
        detect_card_type(&text)
    }

    pub fn card_from_html(text: &str) -> Option<Self> {
        match Self::detect_card_type(text) {
            CardType::Lrig => Some(Lrig::from_source(text.to_owned()).into()),
            CardType::LrigAssist => Some(LrigAssist::from_source(text.to_owned()).into()),
            CardType::Arts => Some(Arts::from_source(text.to_owned()).into()),
            CardType::Key => Some(Key::from_source(text.to_owned()).into()),
            CardType::Signi => Some(Signi::from_source(text.to_owned()).into()),
            CardType::Spell => Some(Spell::from_source(text.to_owned()).into()),
            CardType::Resona => Some(Resona::from_source(text.to_owned()).into()),
            CardType::SigniCraft => Some(SigniCraft::from_source(text.to_owned()).into()),
            CardType::ArtsCraft => Some(ArtsCraft::from_source(text.to_owned()).into()),
            CardType::ResonaCraft => Some(ResonaCraft::from_source(text.to_owned()).into()),
            CardType::SpellCraft => Some(SpellCraft::from_source(text.to_owned()).into()),
            CardType::Piece => Some(Piece::from_source(text.to_owned()).into()),
            CardType::PieceRelay => Some(PieceRelay::from_source(text.to_owned()).into()),
            CardType::PieceCraft => Some(PieceCraft::from_source(text.to_owned()).into()),
            CardType::Token => Some(Token::from_source(text.to_owned()).into()),
            _ => None,
        }
    }

    pub fn check_have_feature(&self, card_feature: CardFeature) -> bool {
        println!("feature check: positive {}", card_feature);
        self.features.contains(&card_feature)
    }

    pub fn burst(&self) -> i32 {
        if self.card_type == CardType::Signi || self.card_type == CardType::Spell {
            match self.skill.get_life_burst_skills().is_empty() {
                true => 1,
                false => 2,
            }
        } else {
            0
        }
    }

    pub fn get_skill_texts(&self) -> (Vec<CardSkill>, Vec<CardSkill>) {
        let mut skill_texts: Vec<CardSkill> = Vec::new();
        let mut burst_texts: Vec<CardSkill> = Vec::new();

        for skill in self.skill.value.iter() {
            let skill_text = skill.to_string();
            let (skill_text_replaced, features_detected) = rule_explain_to_feature(skill_text);

            if self.features.contains(&CardFeature::LifeBurst) {
                if features_detected.contains(&CardFeature::LifeBurst) {
                    burst_texts.push(CardSkill::LifeBurst(skill_text_replaced));
                } else {
                    skill_texts.push(CardSkill::Normal(skill_text_replaced));
                }
            }
        }

        (skill_texts, burst_texts)
    }
}

fn element_to_name_and_pronounce(source: String) -> (String, String) {
    let document = Html::parse_document(&source);

    let br_selector = &*BR_SELECTOR;

    let span_selector = &*SPAN_SELECTOR;

    let mut name = String::new();
    let mut pronounce = String::new();

    if let Some(br_element) = document.select(&br_selector).next() {
        if let Some(text_node) = br_element.prev_sibling() {
            name = text_node.value().as_text().unwrap().to_string();
        }
    }

    if let Some(span_element) = document.select(&span_selector).next() {
        pronounce = span_element.inner_html();
    }

    let re_head = Regex::new(r"^＜").unwrap();
    let re_tail = Regex::new(r"＞$").unwrap();

    (
        name,
        re_tail
            .replace(&re_head.replace(&pronounce, ""), "")
            .to_string(),
    )
}

fn parse_card_skill(source: Vec<String>) -> (Skills, HashSet<CardFeature>) {
    let re_br = Regex::new(r"<br\s?>").unwrap();
    let mut features: HashSet<CardFeature> = HashSet::new();
    let mut all_skills: Vec<String> = Vec::new();

    for s in source {
        let new_html = wrap_by_gainskill(s);

        let skills_for_this_string: Vec<String> = re_br
            .replace_all(&new_html, "\n")
            .split('\n')
            .map(|line| line.trim().to_string())
            .map(|line| {
                let (line_replaced, features_detected) = rule_explain_to_feature(line);
                features.extend(features_detected);
                line_replaced
            })
            .filter(|line| !line.is_empty()) // 空の行を除去
            .collect();

        all_skills.extend(skills_for_this_string);
    }

    (Skills::from_vec(all_skills), features)
}

#[derive(Debug)]
pub struct Token {
    no: String,
    name: String,
    pronounce: String,
    artist: String,
    card_type: CardType,
    // klass: OptionString,
    color: Colors,
    // level: OptionString,
    // cost: OptionString,
    // limit: OptionString,
    // power: OptionString,
    // user: OptionString,
    // time: Vec<String>,
    // story: OptionString,
    // format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
    feature_bits1: i64,
    feature_bits2: i64,
    ex1: OptionString,
}

impl From<Token> for Card {
    fn from(val: Token) -> Self {
        Card {
            no: val.no.clone(),
            name: val.name.clone(),
            pronounce: val.pronounce.clone(),
            artist: val.artist.clone(),
            card_type: val.card_type.clone(),
            klass: None,
            color: val.color.clone(),
            level: OptionString::empty(),
            cost: OptionString::empty(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: OptionString::empty(),
            time: TimingList::new(),
            story: OptionString::empty(),
            format: Format::DivaSelection,
            rarity: val.rarity.clone(),
            skill: val.skill.clone(),
            features: val.features.clone(),
            feature_bits1: val.feature_bits1,
            feature_bits2: val.feature_bits2,
            ex1: val.ex1.clone(),
        }
    }
}

impl WixossCard for Token {
    fn from_source(source: String) -> Self {
        let document: Html = Html::parse_document(&source);

        let card_no = match document.select(&CARD_NUM).next() {
            Some(card_no) => card_no.inner_html(),
            None => "unknown".into(),
        };

        let card_name = match document.select(&CARD_NAME).next() {
            Some(card_name) => element_to_name_and_pronounce(card_name.inner_html()),
            None => ("unknown".into(), "unknown".into()),
        };
        let card_rarity = match document.select(&CARD_RARITY).next() {
            Some(card_rarity) => card_rarity.inner_html(),
            None => "unknown rarity".into(),
        };

        let artist = match document.select(&CARD_ARTIST).next() {
            Some(artist) => artist.inner_html(),
            None => "unknown artist".into(),
        };

        let mut card_data: Vec<String> = Vec::new();
        for element in document.select(&CARD_DATA_DD) {
            card_data.push(element.inner_html());
        }

        let mut card_skills: Vec<String> = Vec::new();
        for element in document.select(&CARD_SKILL) {
            card_skills.push(element.inner_html());
        }

        let (skill, features) = parse_card_skill(card_skills.clone());

        Self {
            no: card_no,
            name: card_name.0,
            pronounce: card_name.1,
            artist,
            card_type: CardType::Token,
            // klass: OptionString::from_string(card_data[1].clone()),
            color: Colors::from(card_data[2].clone()),
            // cost: OptionString::from_string(flatten_break(card_data[5].clone())),
            // level: OptionString::from_string(card_data[3].clone()),
            // power: OptionString::from_string(card_data[7].clone()),
            // user: OptionString::from_string(card_data[8].clone()),
            // time: split_by_break(card_data[9].clone()),
            // story: parse_story(card_data[11].clone().trim().to_string()),
            // format: parse_format(card_data[10].clone()),
            rarity: card_rarity,
            skill,
            features,
            feature_bits1: 0,
            feature_bits2: 0,
            ex1: OptionString::empty(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NO.\t:{}", self.no)?;
        writeln!(f, "Name\t:{}", self.name)?;
        writeln!(f, "読み\t:{}", self.pronounce)?;
        writeln!(f, "絵\t:{}", self.artist)?;
        writeln!(f, "Type\t:{}", self.card_type)?;
        writeln!(f, "色\t:{}", self.color)?;
        // write!(f, "種族\t:{}\n", self.klass)?;
        // write!(f, "レベル\t:{}\n", self.level)?;
        // write!(f, "コスト\t:{}\n", self.cost)?;
        // write!(f, "リミット\t:{}\n", self.limit)?;
        // write!(f, "パワー\t:{}\n", self.power)?;
        // write!(f, "ルリグタイプ\t:{}\n", self.user)?;
        // write!(f, "タイミング\t:{}\n", self.time.join(", "))?;
        // write!(f, "ストーリー\t:{}\n", self.story)?;
        // write!(f, "フォーマット\t:{}\n", self.format)?;
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

fn wrap_by_gainskill(html: String) -> String {
    let replaced = html.replace("<div class=\"card_ability_add_border\">", "\n<gainskill>");
    let replaced = replaced.replace("</div>", "</gainskill>\n");
    let re = Regex::new(r"(<br>)?\n?</gainskill>").unwrap();
    let replaced = re.replace_all(&replaced, "</gainskill>").to_string();
    replaced
}

/// A rule that detects card features in text
struct CardFeatureRule {
    // This rule doesn't store any patterns, it uses create_detect_patterns() directly
}

impl CardFeatureRule {
    fn new() -> Self {
        Self {}
    }

    // Apply replace patterns and return the replaced text
    fn apply_replace(&self, text: &str) -> String {
        let (replace_patterns, _) = &create_detect_patterns();

        replace_patterns.iter().fold(
            text.to_string(),
            |current_text: String, pat: &ReplacePattern| {
                let re = &pat.pattern_r;
                let replaced = re.replace_all(&current_text, pat.replace_to).to_string();
                replaced
            },
        )
    }
    
    // Direct feature detection without trait
    fn detect_features(&self, text: &str) -> HashSet<CardFeature> {
        let mut features = HashSet::new();

        let (replace_patterns, detect_patterns) = &create_detect_patterns();

        // First check replace patterns
        for pat in replace_patterns {
            let re = &pat.pattern_r;
            if re.is_match(text) {
                features.extend(pat.features_detected.iter().cloned());
            }
        }

        // Then check detect patterns using parallel iterator
        let detected_features: HashSet<CardFeature> = detect_patterns
            .par_iter()
            .filter_map(|pat: &DetectPattern| {
                let re = &pat.pattern_r;

                // Only if the pattern matches
                if re.is_match(text) {
                    Some(
                        pat.features_detected
                            .iter()
                            .cloned()
                            .collect::<HashSet<CardFeature>>(),
                    )
                } else {
                    None
                }
            })
            .reduce(HashSet::new, |mut acc, detected| {
                acc.extend(detected); // Combine results from each thread
                acc
            });

        // Combine all detected features
        features.extend(detected_features);

        features
    }
}

fn rule_explain_to_feature(text: String) -> (String, HashSet<CardFeature>) {
    let text = replace_img_with_alt(text);

    // Create a single rule that handles both replace and detect patterns
    let rule = CardFeatureRule::new();

    // Apply replace patterns to get the replaced text
    let replaced_text = rule.apply_replace(&text);

    // Detect features directly without using Analyzer trait
    let features = rule.detect_features(&replaced_text);

    (replaced_text, features)
}

fn replace_img_with_alt(html: String) -> String {
    let re = Regex::new(r#"<img[^>]*alt="([^"]*)"[^>]*>"#).unwrap();
    let replaced = re.replace_all(&html, |caps: &regex::Captures| {
        let alt_text = &caps[1];
        alt_text.replace("2》", "》")
    });
    replaced.into_owned()
}

fn parse_story(html: String) -> OptionString {
    if html.contains(r#"class="cardData_story_img""#) {
        OptionString::from_string("dissona".into())
    } else {
        OptionString::empty()
    }
}

fn split_by_break(html: String) -> Vec<String> {
    html.replace('\n', "")
        .split("<br>")
        .map(|s| s.to_string())
        .collect()
}

fn flatten_break(html: String) -> String {
    html.replace('\n', "").replace("<br>", "")
}

fn parse_format(html: String) -> Format {
    match html.as_str() {
        _ if html.contains("ディーヴァアイコン") => Format::DivaSelection,
        _ if html.contains("キーアイコン") => Format::KeySelection,
        _ => Format::AllStar,
    }
}
