#[macro_use]
pub mod feature;
pub mod card;
pub mod color;
pub mod format;

use crate::features;
use crate::wixoss::card::{detect_card_type, CardType};
use crate::wixoss::color::Colors;
use crate::wixoss::feature::CardFeature;
use crate::wixoss::format::Format;

pub use crate::wixoss::card::{
    Arts, ArtsCraft, Key, Lrig, LrigAssist, Piece, PieceRelay, Resona, ResonaCraft, Signi, Spell,
    SpellCraft,
};
use regex::Regex;
use scraper::{Html, Selector};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use sqlx::encode::IsNull::No;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use webapp::models::CreateCard;

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
pub struct Skills {
    value: Vec<String>,
}

impl Skills {
    fn from_vec(skills: Vec<String>) -> Self {
        Self { value: skills }
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
        write!(f, "{}", self.value.join("\n"))
    }
}

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
    klass: OptionString,
    color: Colors,
    level: OptionString,
    cost: OptionString,
    limit: OptionString,
    power: OptionString,
    user: OptionString,

    #[serde(serialize_with = "custom_vec_string_serialize")]
    time: Vec<String>,

    pub story: OptionString,
    format: Format,
    rarity: String,
    skill: Skills,
    features: HashSet<CardFeature>,
}

impl Into<CreateCard> for Card {
    fn into(self) -> CreateCard {
        CreateCard {
            name: self.name,
            code: self.no,
            pronunciation: self.pronounce,
            power: self.power.value,
            cost: self.cost.value,
            level: Some(0),
            limit: Some(0),
            limit_ex: Some(0),
            has_burst: true,
            skill_text: None,
            burst_text: None,
            format: 001_i32,
            story: self.story.value,
            rarity: Some(self.rarity),
            url: None,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}\n{}",
            self.no,
            self.name,
            self.pronounce,
            self.artist,
            self.card_type,
            self.klass,
            self.color,
            self.level,
            self.cost,
            self.limit,
            self.power,
            self.user,
            self.time.join(", "),
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
        let selector_card_data = Selector::parse(".cardData dd").unwrap();

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
            CardType::ArtsCraft => Some(ArtsCraft::from_source(text.to_owned()).into()),
            CardType::ResonaCraft => Some(ResonaCraft::from_source(text.to_owned()).into()),
            CardType::SpellCraft => Some(SpellCraft::from_source(text.to_owned()).into()),
            CardType::Piece => Some(Piece::from_source(text.to_owned()).into()),
            CardType::PieceRelay => Some(PieceRelay::from_source(text.to_owned()).into()),
            CardType::Token => Some(Token::from_source(text.to_owned()).into()),
            _ => None,
        }
    }

    pub fn check_have_feature(&self, card_feature: CardFeature) -> bool {
        println!("feature check: positive {}", card_feature);
        self.features.contains(&card_feature)
    }
}

fn element_to_name_and_pronounce(source: String) -> (String, String) {
    let document = Html::parse_document(&source);

    let br_selector = Selector::parse("br").unwrap();

    let span_selector = Selector::parse("span").unwrap();

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
                let (l, features_detected) = rule_explain_to_feature(line);
                features.extend(features_detected);
                l
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
}

impl From<Token> for Card {
    fn from(val: Token) -> Self {
        Card {
            no: val.no.clone(),
            name: val.name.clone(),
            pronounce: val.pronounce.clone(),
            artist: val.artist.clone(),
            card_type: val.card_type.clone(),
            klass: OptionString::empty(),
            color: val.color.clone(),
            level: OptionString::empty(),
            cost: OptionString::empty(),
            limit: OptionString::empty(),
            power: OptionString::empty(),
            user: OptionString::empty(),
            time: Vec::new(),
            story: OptionString::empty(),
            format: Format::DivaSelection,
            rarity: val.rarity.clone(),
            skill: val.skill.clone(),
            features: val.features.clone(),
        }
    }
}

impl WixossCard for Token {
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

fn rule_explain_to_feature(text: String) -> (String, Vec<CardFeature>) {
    let text = replace_img_with_alt(text);

    let mut features: Vec<CardFeature> = Vec::new();

    let remove_patterns: Vec<(&str, bool, &str, HashSet<CardFeature>)> = vec![
        (r"『", true, "", features![]), // アクセのみ？
        (r"』", true, "", features![]), // アクセのみ？
        (
            r"ライフバースト：",
            true,
            "LB:",
            features![CardFeature::LifeBurst],
        ),
        (
            r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）",
            true,
            "",
            features![CardFeature::Damage],
        ),
        (
            r"（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            true,
            "",
            features![CardFeature::Lancer],
        ),
        (
            r"（このクラフトは効果以外によっては場に出せない）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (r"アクセ", false, "*ACCE*", features![CardFeature::Acce]),
        (
            r"（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）",
            true,
            "",
            features![CardFeature::Acce],
        ),
        (
            r"（あなたのルリグの下からカードを合計４枚ルリグトラッシュに置く）",
            true,
            "*EXCEED*",
            features![CardFeature::Exceed],
        ),
        (
            r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに合計１枚までしか入れられない）",
            true,
            "*DREAM TEAM*",
            features![],
        ),
        (
            r"（あなたの場にいるルリグ３体がこの条件を満たす）",
            true,
            "*TEAM*",
            features![],
        ),
        (
            r"（シグニは覚醒すると場にあるかぎり覚醒状態になる）",
            true,
            "*AWAKE*",
            features![CardFeature::Awake],
        ),
        (
            r"（凍結されたシグニは次の自分のアップフェイズにアップしない）",
            true,
            "*FROZEN*",
            features![CardFeature::Freeze],
        ),
        (
            r"（フェゾーネマジックは５種類ある）",
            true,
            "*FESONE MAGIC*",
            features![],
        ),
        (
            r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）",
            true,
            "*CIP COST*",
            features![],
        ),
        (
            r"ガードアイコン",
            true,
            "ガード",
            features![CardFeature::Guard],
        ),
        (
            r"捨てさせる。",
            false,
            "*HAND DESTRUCTION*",
            features![CardFeature::DiscardOpponent],
        ),
        (
            r"見ないで選び、捨てさせる。",
            false,
            "*RANDOM HAND DESTRUCTION*",
            features![CardFeature::RandomDiscard],
        ),
        (
            r"ダウンする。",
            false,
            "*DOWN*",
            features![CardFeature::Down],
        ),
        (
            r"エナチャージ",
            false,
            "*CHARGE*",
            features![CardFeature::Charge],
        ),
        (
            r"残りを好きな順番でデッキの一番下に置く",
            false,
            "*BOTTOM CHECK*",
            features![CardFeature::BottomCheck],
        ),
        (
            r"それをトラッシュに置",
            false,
            "*TRASH*",
            features![CardFeature::Trash],
        ),
        (
            r"シグニバリア",
            false,
            "*BARRIER*",
            features![CardFeature::Barrier],
        ),
        (
            r"ルリグバリア",
            false,
            "*BARRIER*",
            features![CardFeature::Barrier],
        ),
        // (r"がアタックしたとき", false, "*ON ATTACK*", features![CardFeature::OnAttack]),
        (
            r"アサシン",
            false,
            "*ASSASSIN*",
            features![CardFeature::Assassin],
        ),
        (
            r"シャドウ",
            false,
            "*SHADOW*",
            features![CardFeature::Shadow],
        ),
        (
            r"【マルチエナ】",
            false,
            "*MULTI ENER*",
            features![CardFeature::MultiEner],
        ),
        (r"チャーム", false, "*CHARM*", features![CardFeature::Charm]),
        (
            r"ダブルクラッシュ",
            false,
            "*DOUBLE CRUSH*",
            features![CardFeature::DoubleCrush],
        ),
        (
            r"トリプルクラッシュ",
            false,
            "*TRIPLE CRUSH*",
            features![CardFeature::TripleCrush],
        ),
        (
            r"Sランサー",
            false,
            "*S LANCER*",
            features![CardFeature::SLancer],
        ),
        (
            r"Ｓランサー",
            false,
            "*S LANCER*",
            features![CardFeature::SLancer],
        ),
        (
            r"バニッシュ",
            false,
            "*BANISH*",
            features![CardFeature::Banish],
        ),
        (
            r"凍結する",
            false,
            "*FREEZE*",
            features![CardFeature::Freeze],
        ),
        (
            r"対戦相手のシグニを[（\u{FF10}-\u{FF19}）]+体まで対象とし、それらを手札に戻",
            false,
            "*BOUNCE*",
            features![CardFeature::Bounce],
        ),
        (
            r"対戦相手のシグニ[（\u{FF10}-\u{FF19}）]+体を対象とし、それを手札に戻",
            false,
            "BOUNCE",
            features![CardFeature::Bounce],
        ),
        // (r"手札に加え", false, "*SALVAGE*", features![CardFeature::Salvage]),
        (
            r"ライフクロス[（\u{FF10}-\u{FF19}）]+枚をトラッシュに置",
            false,
            "*LIFE TRASH*",
            features![CardFeature::LifeTrash],
        ),
        (
            r"エナゾーンからカード[（\u{FF10}-\u{FF19}）]+枚を.+トラッシュに置",
            false,
            "*ENER ATTACK*",
            features![CardFeature::EnerAttack],
        ),
        (
            r"ルリグトラッシュに置",
            false,
            "*LRIG TRASH*",
            features![CardFeature::LrigTrash],
        ),
        // (r"アタックフェイズ開始時", false, "*ON ATTACK START*", features![CardFeature::OnAttackStart]),
        (
            r"ライフクロスに加える",
            false,
            "*ADD LIFE*",
            features![CardFeature::AddLife],
        ),
        (
            r"ランサー",
            false,
            "*LANCER*",
            features![CardFeature::Lancer],
        ),
        (
            r"ライフクロスを１枚クラッシュする",
            false,
            "*CRUSH*",
            features![CardFeature::LifeCrush],
        ),
        (
            r"対戦相手のライフクロス１枚をクラッシュする。",
            false,
            "*CRUSH*",
            features![CardFeature::LifeCrush],
        ),
        (
            r"対戦相手にダメージを与える。",
            false,
            "*DAMAGE*",
            features![CardFeature::Damage],
        ),
        (
            r"リコレクトアイコン",
            false,
            "*RECOLLECT*",
            features![CardFeature::Recollect],
        ),
        (r"枚見", false, "*SEEK*", features![CardFeature::SeekTop]),
        (
            r"能力を失う",
            false,
            "*ERASE SKILL*",
            features![CardFeature::EraseSkill],
        ),
        (
            r"アタックできない",
            false,
            "*NON ATTACKABLE*",
            features![CardFeature::NonAttackable],
        ),
        (
            r"カードを[（\u{FF10}-\u{FF19}）]+枚引",
            false,
            "*DRAW*",
            features![CardFeature::Draw],
        ),
        (
            r"デッキの上からカードを[（\u{FF10}-\u{FF19}）]+枚トラッシュに置",
            false,
            "*DROP*",
            features![CardFeature::Drop],
        ),
        (
            r"対戦相手のエナゾーンからカードを[（\u{FF10}-\u{FF19}）]+枚まで対象とし、それらを手札に戻",
            false,
            "*ENER ATTACK*",
            features![CardFeature::EnerAttack],
        ),
        (
            r"デッキの一番下に置",
            false,
            "*DECK BOUNCE*",
            features![CardFeature::DeckBounce],
        ),
        (
            r"シグニのパワーを＋",
            false,
            "*POWER UP*",
            features![CardFeature::PowerUp],
        ),
        (
            r"(シグニ|それ)のパワーを－",
            false,
            "*POWER DOWN*",
            features![CardFeature::PowerDown],
        ),
        (
            r"ダメージを受けない",
            false,
            "*CANCEL DAMAGE*",
            features![CardFeature::CancelDamage],
        ),
        (
            r"トラッシュからシグニ.+場に出",
            false,
            "*REANIMATE*",
            features![CardFeature::Reanimate],
        ),
        (
            r"このルリグをアップし",
            false,
            "*ADDITIONAL ATTACK*",
            features![CardFeature::AdditionalAttack],
        ),
        (
            r"対戦相手は【ガード】ができない",
            false,
            "*UNGUARDABLE*",
            features![CardFeature::UnGuardable],
        ),
        (
            r"スペル[（\u{FF10}-\u{FF19}）]+枚を.+手札に加え",
            false,
            "*SALVAGE SPELL*",
            features![CardFeature::SalvageSpell],
        ),
        (
            r"シグニ[（\u{FF10}-\u{FF19}）]+枚を.+手札に加え",
            false,
            "*SALVAGE SIGNI*",
            features![CardFeature::Salvage],
        ),
        (
            r"このシグニがアタックしたとき.+バニッシュする",
            false,
            "*BANISH ON ATTACK*",
            features![CardFeature::BanishOnAttack],
        ),
    ];

    let replaced_text = remove_patterns.iter().fold(text, |current_text, pat| {
        let re = Regex::new(pat.0).unwrap();

        if re.is_match(&current_text) {
            features.extend(pat.3.iter().cloned());
        }

        if pat.1 {
            re.replace_all(&current_text, pat.2).to_string()
        } else {
            current_text
        }
    });

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
        OptionString::from_string("ディソナ".into())
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
