// カードタイプ情報
// このファイルは static_generator によって自動生成されます

#[derive(Debug, Clone)]
pub struct CardTypeInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub sort_asc: i32,
    pub is_primary: bool,
}

pub const CARD_TYPES: &[CardTypeInfo] = &[
    CardTypeInfo {
        code: "lrig",
        name: "ルリグ",
        sort_asc: 0,
        is_primary: false,
    },
    CardTypeInfo {
        code: "arts",
        name: "アーツ",
        sort_asc: 1,
        is_primary: false,
    },
    CardTypeInfo {
        code: "lrig_assist",
        name: "アシストルリグ",
        sort_asc: 2,
        is_primary: false,
    },
    CardTypeInfo {
        code: "piece",
        name: "ピース",
        sort_asc: 3,
        is_primary: false,
    },
    CardTypeInfo {
        code: "signi",
        name: "シグニ",
        sort_asc: 4,
        is_primary: false,
    },
    CardTypeInfo {
        code: "spell",
        name: "スペル",
        sort_asc: 5,
        is_primary: false,
    },
    CardTypeInfo {
        code: "resona",
        name: "レゾナ",
        sort_asc: 6,
        is_primary: false,
    },
    CardTypeInfo {
        code: "key",
        name: "キー",
        sort_asc: 7,
        is_primary: false,
    },
    CardTypeInfo {
        code: "arts_craft",
        name: "クラフトアーツ",
        sort_asc: 8,
        is_primary: false,
    },
    CardTypeInfo {
        code: "signi_craft",
        name: "クラフトシグニ",
        sort_asc: 9,
        is_primary: false,
    },
    CardTypeInfo {
        code: "spell_craft",
        name: "クラフトスペル",
        sort_asc: 10,
        is_primary: false,
    },
    CardTypeInfo {
        code: "piece_relay",
        name: "リレーピース",
        sort_asc: 11,
        is_primary: false,
    },
    CardTypeInfo {
        code: "piece_craft",
        name: "クラフトピース",
        sort_asc: 12,
        is_primary: false,
    },
    CardTypeInfo {
        code: "resona_craft",
        name: "クラフトレゾナ",
        sort_asc: 13,
        is_primary: false,
    },
    CardTypeInfo {
        code: "token",
        name: "トークン",
        sort_asc: 100,
        is_primary: false,
    },
    CardTypeInfo {
        code: "coin",
        name: "コイン",
        sort_asc: 101,
        is_primary: false,
    },
];

pub const PRIMARY_CARD_TYPES: &[&str] = &[

];

pub const EXTENDED_CARD_TYPES: &[&str] = &[
    "lrig",
    "arts",
    "lrig_assist",
    "piece",
    "signi",
    "spell",
    "resona",
    "key",
    "arts_craft",
    "signi_craft",
    "spell_craft",
    "piece_relay",
    "piece_craft",
    "resona_craft",
    "token",
    "coin"
];

impl CardTypeInfo {
    pub fn find_by_code(code: &str) -> Option<&'static CardTypeInfo> {
        CARD_TYPES.iter().find(|ct| ct.code == code)
    }

    pub fn is_primary_type(code: &str) -> bool {
        PRIMARY_CARD_TYPES.contains(&code)
    }
}
