use once_cell::sync::Lazy;
use regex::Regex;
use scraper::Selector;

/// WIXOSSカード解析用の静的セレクタとパターン
/// 
/// パフォーマンスのため、初回アクセス時にのみコンパイルされる

// HTMLセレクタ
pub static CARD_MAIN_TABLE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardDetail__mainTable").expect("カードメインテーブルセレクタのパースに失敗")
});

pub static CARD_SUB_TABLE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardDetail__subTable").expect("カードサブテーブルセレクタのパースに失敗")
});

pub static SKILL_TEXTS: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardDetail__skillTexts p").expect("スキルテキストセレクタのパースに失敗")
});

pub static TABLE_ROW: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("tr").expect("テーブル行セレクタのパースに失敗")
});

pub static TABLE_CELL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("td").expect("テーブルセルセレクタのパースに失敗")
});

pub static CARD_IMAGE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardDetail__img img").expect("カード画像セレクタのパースに失敗")
});

pub static CARD_LINK: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("a.c-box").expect("カードリンクセレクタのパースに失敗")
});

// 正規表現パターン
pub static FORMAT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r".*format_(\w+)\.png").expect("フォーマットパターンの正規表現コンパイルに失敗")
});

pub static CARD_NO_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r".*\(([^)]+)\)$").expect("カード番号パターンの正規表現コンパイルに失敗")
});

pub static COLOR_COST_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"【(\w+)】×(\d+)").expect("色コストパターンの正規表現コンパイルに失敗")
});

pub static LIMIT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").expect("リミットパターンの正規表現コンパイルに失敗")
});

pub static LEVEL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"lv(\d+)").expect("レベルパターンの正規表現コンパイルに失敗")
});

pub static POWER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").expect("パワーパターンの正規表現コンパイルに失敗")
});

pub static COIN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\d+)").expect("コインパターンの正規表現コンパイルに失敗")
});

pub static USE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"【(.*?)】").expect("使用タイミングパターンの正規表現コンパイルに失敗")
});

pub static PHASE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"【(.*?)】").expect("フェーズパターンの正規表現コンパイルに失敗")
});

pub static CLASS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"〈([^〉]+)〉").expect("クラスパターンの正規表現コンパイルに失敗")
});

pub static SELECTOR_COST_ITEM: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("span.costItem").expect("コストアイテムセレクタのパースに失敗")
});

pub static SELECTOR_COST_SINGLE: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("span.costSingle").expect("単一コストセレクタのパースに失敗")
});

pub static SELECTOR_COST_COLOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("span.costColor").expect("色コストセレクタのパースに失敗")
});

pub static SELECTOR_COST_IMG: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("span.costImg img").expect("コスト画像セレクタのパースに失敗")
});

pub static SELECTOR_ALT: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("img").expect("画像セレクタのパースに失敗")
});

// カード詳細解析用のセレクタ
pub static CARD_NUM: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardNum").expect("カード番号セレクタのパースに失敗")
});

pub static CARD_NAME: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardName").expect("カード名セレクタのパースに失敗")
});

pub static CARD_RARITY: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardRarity").expect("レアリティセレクタのパースに失敗")
});

pub static CARD_ARTIST: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardImg p span").expect("アーティストセレクタのパースに失敗")
});

pub static CARD_DATA_DD: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardData dd").expect("カードデータセレクタのパースに失敗")
});

pub static CARD_SKILL: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".cardSkill").expect("カードスキルセレクタのパースに失敗")
});

// 汎用HTMLセレクタ
pub static BR_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("br").expect("brセレクタのパースに失敗")
});

pub static SPAN_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("span").expect("spanセレクタのパースに失敗")
});

