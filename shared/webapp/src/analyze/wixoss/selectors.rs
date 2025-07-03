use once_cell::sync::Lazy;
use scraper::Selector;

/// WIXOSSカード解析用の静的セレクタとパターン
///
/// パフォーマンスのため、初回アクセス時にのみコンパイルされる

// カード詳細解析用のセレクタ
pub static CARD_NUM: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardNum").expect("カード番号セレクタのパースに失敗"));

pub static CARD_NAME: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardName").expect("カード名セレクタのパースに失敗"));

pub static CARD_RARITY: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardRarity").expect("レアリティセレクタのパースに失敗"));

pub static CARD_ARTIST: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardImg p span").expect("アーティストセレクタのパースに失敗"));

pub static CARD_DATA_DD: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardData dd").expect("カードデータセレクタのパースに失敗"));

pub static CARD_SKILL: Lazy<Selector> =
    Lazy::new(|| Selector::parse(".cardSkill").expect("カードスキルセレクタのパースに失敗"));

// 汎用HTMLセレクタ
pub static BR_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("br").expect("brセレクタのパースに失敗"));

pub static SPAN_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("span").expect("spanセレクタのパースに失敗"));
