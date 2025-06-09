use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

macro_rules! define_features {
    (
        $(
            $feature:ident => { tag: $tag:ident, bit_shift: ($shift1:expr, $shift2:expr), label: $label:expr },
        )*
    ) => {
        // 1. CardFeature enumの定義
        #[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
        pub enum CardFeature {
            $($feature),*
        }

        // 2. 各フィーチャのタグを返す関数
        impl CardFeature {
            pub fn tag(&self) -> FeatureTag {
                match self {
                    $(
                        CardFeature::$feature => FeatureTag::$tag,
                    )*
                }
            }

            // 3. 各フィーチャのビットシフト値を返す関数
            pub fn to_bit_shifts(&self) -> (i64, i64) {
                match self {
                    $(
                        CardFeature::$feature => ($shift1, $shift2),
                    )*
                }
            }

            pub fn create_vec() -> Vec<CardFeature> {
                    vec![
                        $(
                            CardFeature::$feature,
                        )*
                    ]
                }
            }

        // 各フィーチャの文字列表現を返す関数
        impl Display for CardFeature {
            #[allow(unreachable_patterns)]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let label = match self {
                    $(
                    CardFeature::$feature => $label,
                    )*
                };
                write!(f, "{}", label)
            }
        }
    };
}

// 大分類を表す FeatureTag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureTag {
    Lethal,    // 最後の1点を取れる
    Offensive, // 攻撃系
    Disturb,   // 妨害系
    Endure,    // 防御系
    Enhance,   // 資源系
    Unique,    // 固有系
    Others,    // その他
}

impl Display for FeatureTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            FeatureTag::Lethal => "01リーサル",
            FeatureTag::Offensive => "02攻撃系",
            FeatureTag::Disturb => "04妨害系",
            FeatureTag::Endure => "03防御系",
            FeatureTag::Enhance => "05資源系",
            FeatureTag::Unique => "06固有系",
            FeatureTag::Others => "07その他",
        };
        write!(f, "{}", label)
    }
}

#[macro_export]
macro_rules! features {
        ($($feature:expr),* $(,)?) => {
            {
                #[allow(unused_mut)]
                let mut set = HashSet::new();
                $(
                    set.insert($feature);
                )*
                set
            }
        };
    }

// const ANY_NUM: &str = r"[（\u{FF10}-\u{FF19}）]";
#[macro_export]
macro_rules! any_num {
    // 引数が1つの場合: 頭に連結するケース
    ($pattern:expr) => {
        concat!(r"[（\u{FF10}-\u{FF19}）]+", $pattern) // rawリテラル対応
    };

    // 引数が2つの場合: "ANY_NUM" を指定したリテラルで挟む
    ($pattern_head:expr, $pattern_tail:expr) => {{
        concat!(
            concat![$pattern_head, r"[（\u{FF10}-\u{FF19}）]+"],
            $pattern_tail
        )
    }};
}

define_features! {
    DoubleCrush => { tag: Offensive, bit_shift: (1, 0), label: "ダブルクラッシュ" },
    // TripleCrush => { tag: Offensive, bit_shift: (2, 0), label: "トリプルクラッシュ" },
    DiscardOpponent => { tag: Disturb, bit_shift: (3, 0), label: "ハンデス" },
    RandomDiscard => { tag: Disturb, bit_shift: (4, 0), label: "ハンデス(強)" },
    Draw => { tag: Enhance, bit_shift: (5, 0), label: "ドロー" },
    Assassin => { tag: Lethal, bit_shift: (6, 0), label: "アサシン" },
    Freeze => { tag: Disturb, bit_shift: (7, 0), label: "凍結" },
    Drop => { tag: Offensive, bit_shift: (8, 0), label: "デッキ落下" },
    OnDrop => { tag: Offensive, bit_shift: (9, 0), label: "デッキ落下時" },
    OnRefresh => { tag: Offensive, bit_shift: (10, 0), label: "リフレッシュ時" },
    Lancer => { tag: Offensive, bit_shift: (11, 0), label: "ランサー" },
    SLancer => { tag: Lethal, bit_shift: (12, 0), label: "Sランサー" },
    RemoveSigni => { tag: Offensive, bit_shift: (13, 0), label: "シグニ除外" },
    NonAttackable => { tag: Endure, bit_shift: (14, 0), label: "アタック不可" },
    Down => { tag: Endure, bit_shift: (15, 0), label: "ダウン" },
    Up => { tag: Offensive, bit_shift: (16, 0), label: "シグニアップ" },
    Charge => { tag: Enhance, bit_shift: (17, 0), label: "エナチャージ" },
    EnerAttack => { tag: Disturb, bit_shift: (18, 0), label: "エナ破壊" },
    Trash => { tag: Offensive, bit_shift: (19, 0), label: "トラッシュ送り" },
    EnerOffensive => { tag: Offensive, bit_shift: (20, 0), label: "エナ送り" }, // エナ送り
    PowerUp => { tag: Endure, bit_shift: (21, 0), label: "パワーアップ" },
    PowerDown => { tag: Offensive, bit_shift: (22, 0), label: "パワーダウン" },
    Bounce => { tag: Offensive, bit_shift: (23, 0), label: "バウンス" },
    DeckBounce => { tag: Offensive, bit_shift: (24, 0), label: "デッキバウンス" },
    Salvage => { tag: Enhance, bit_shift: (25, 0), label: "トラッシュ回収" },
    LifeBurst => { tag: Endure, bit_shift: (26, 0), label: "ライフバースト" },
    Shadow => { tag: Endure, bit_shift: (27, 0), label: "シャドウ" },
    Invulnerable => { tag: Endure, bit_shift: (28, 0), label: "バニッシュ耐性" },
    OnSpell => { tag: Others, bit_shift: (29, 0), label: "スペル参照" },
    OnArts => { tag: Others, bit_shift: (31, 0), label: "アーツ・ピース参照" },
    // OnPiece => { tag: Others, bit_shift: (31, 0), label: "ピース使用時" },
    OnBanish => { tag: Endure, bit_shift: (32, 0), label: "被バニッシュ時" },
    Banish => { tag: Offensive, bit_shift: (33, 0), label: "バニッシュ" },
    Guard => { tag: Endure, bit_shift: (34, 0), label: "ガード" },
    OnGuard => { tag: Enhance, bit_shift: (35, 0), label: "ガード時" },
    AttackNoEffect => { tag: Endure, bit_shift: (36, 0), label: "アタック無効" },
    // OnAttack => { tag: Others, bit_shift: (2, 0), label: "アタック時" },
    // OnAttackStart => { tag: Others, bit_shift: (2, 0), label: "アタック開始時" },
    OnTouch => { tag: Others, bit_shift: (37, 0), label: "被対象時" },
    Awake => { tag: Others, bit_shift: (38, 0), label: "覚醒" },
    Exceed => { tag: Enhance, bit_shift: (39, 0), label: "エクシード" },
    OnExceed => { tag: Others, bit_shift: (40, 0), label: "エクシード時" },
    AddLife => { tag: Endure, bit_shift: (41, 0), label: "ライフクロス追加" },
    OnBurst => { tag: Others, bit_shift: (42, 0), label: "バースト参照" },
    LifeTrash => { tag: Offensive, bit_shift: (43, 0), label: "ライフトラッシュ" },
    LifeCrush => { tag: Offensive, bit_shift: (44, 0), label: "ライフクラッシュ" },
    Damage => { tag: Lethal, bit_shift: (45, 0), label: "ダメージ" },
    OnLifeCrush => { tag: Others, bit_shift: (46, 0), label: "クラッシュ時" },
    Position => { tag: Disturb, bit_shift: (47, 0), label: "シグニゾーン移動" },
    Vanilla => { tag: Endure, bit_shift: (48, 0), label: "バニラ" },
    // Untouchable => { tag: Others, bit_shift: (49, 0), label: "不可触" },    //旧アークゲイン
    TopSet => { tag: Enhance, bit_shift: (50, 0), label: "トップ操作" },
    BottomCheck => { tag: Enhance, bit_shift: (51, 0), label: "ボトム操作" },
    Barrier => { tag: Endure, bit_shift: (52, 0), label: "バリア" },
    // MultiEner => { tag: Enhance, bit_shift: (53, 0), label: "マルチエナ" },
    LrigTrash => { tag: Enhance, bit_shift: (54, 0), label: "ルリグトラッシュ参照" },
    Charm => { tag: Unique, bit_shift: (55, 0), label: "チャーム" },
    Craft => { tag: Unique, bit_shift: (56, 0), label: "クラフト" },
    Acce => { tag: Unique, bit_shift: (57, 0), label: "アクセ" },
    Rise => { tag: Unique, bit_shift: (58, 0), label: "ライズ" },
    Recollect => { tag: Enhance, bit_shift: (59, 0), label: "リコレクト" },
    SeekTop => { tag: Enhance, bit_shift: (60, 0), label: "シーク" },
    EraseSkill => { tag: Others, bit_shift: (61, 0), label: "能力消去" },
    CancelDamage => { tag: Endure, bit_shift: (0, 1), label: "ダメージ無効" },
    Reanimate => { tag: Endure, bit_shift: (0, 2), label: "トラッシュ場出し" },
    AdditionalAttack => { tag: Lethal, bit_shift: (0, 3), label: "追加アタック" },
    UnGuardable => { tag: Lethal, bit_shift: (0, 4), label: "ガード不可" },
    SalvageSpell => { tag: Enhance, bit_shift: (0, 5), label: "スペル回収" },
    BanishOnAttack => { tag: Lethal, bit_shift: (0, 6), label: "アタック時バニッシュ" },
    ShootLike => { tag: Disturb, bit_shift: (0, 7), label: "バニッシュ代替" },
    LimitSigni => { tag: Lethal, bit_shift: (0, 8), label: "シグニゾーン制限" },
    FreeSpell => { tag: Enhance, bit_shift: (0, 9), label: "スペルコスト軽減" },
    DualColorEner => { tag: Enhance, bit_shift: (0, 10), label: "複数色エナ" },
    GainCoin => { tag: Unique, bit_shift: (0, 11), label: "コイン獲得" },
    BetCoin => { tag: Others, bit_shift: (0, 12), label: "ベット/コイン使用" },
    HandCost => { tag: Enhance, bit_shift: (0, 13), label: "手札コスト" },   // 自分自身が捨てる
    RligDownCost => { tag: Enhance, bit_shift: (0, 14), label: "ルリグダウンコスト" }, // ルリグをダウン
    Inherit => { tag: Others, bit_shift: (0, 15), label: "Lv3継承" },    // ルリグトラッシュのルリグを継承
    PreventGrowCost => { tag: Enhance, bit_shift: (0, 16), label: "グロウコスト軽減" },
    PutSigniDefense => { tag: Endure, bit_shift: (0, 17), label: "ブロッカー場出し" },
    PutSigniOffense => { tag: Offensive, bit_shift: (0, 18), label: "アタッカー場出し" },
    Harmony => { tag: Unique, bit_shift: (0, 19), label: "ハーモニー" },
    MagicBox => { tag: Unique, bit_shift: (0, 20), label: "マジックボックス" },
    Virus => { tag: Unique, bit_shift: (0, 21), label: "ウィルス" },
    FreeArts => { tag: Enhance, bit_shift: (0, 22), label: "アーツコスト軽減" },
}

pub trait HashSetToBits {
    fn to_bits(&self) -> (i64, i64);
}

impl HashSetToBits for HashSet<CardFeature> {
    fn to_bits(&self) -> (i64, i64) {
        let mut bits = (0, 0);
        for feature in self {
            let (shift1, shift2) = feature.to_bit_shifts();
            bits.0 |= 1_i64 << shift1;
            bits.1 |= 1_i64 << shift2;
        }
        bits
    }
}

// ユーザーが見る形式のデータを定義
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportedCardFeature {
    pub name: String,          // Feature名（例: "DoubleCrush"）
    pub bit_shift: (i64, i64), // ビットシフト値
    pub tag: FeatureTag,       // タグカテゴリ
}

// タグとその機能群をマッピング
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportedFeatureGroup {
    pub tag_name: String,                   // タグ名（例: "Lethal"）
    pub features: Vec<ExportedCardFeature>, // 該当する機能のリスト
}
// Export用の方法を実装
impl CardFeature {
    pub fn export(&self) -> ExportedCardFeature {
        ExportedCardFeature {
            name: format!("{}", self), // Enum名を文字列化
            bit_shift: self.to_bit_shifts(),    // to_bit_shift の結果を使用
            tag: self.tag(),             // タグカテゴリを取得
        }
    }
}
pub fn export_features() -> HashMap<String, Vec<ExportedCardFeature>> {
    let mut feature_map: HashMap<String, Vec<ExportedCardFeature>> = HashMap::new();

    let all_features = CardFeature::create_vec();

    // 分類して値をマッピング
    for feature in all_features.into_iter() {
        let exported = feature.export();

        feature_map
            .entry(exported.tag.to_string()) // タグごとに分類
            .or_default()
            .push(exported);
    }

    feature_map
}
