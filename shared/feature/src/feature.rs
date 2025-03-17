use serde::Serialize;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

// 大分類を表す FeatureTag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureTag {
    Lethal,    // 最後の1点を取れる
    Offensive, // 攻撃系
    Disturb,   // 妨害系
    Endure,    // 防御系
    Enhance,   // 潤滑系
    Others,    // その他
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum CardFeature {
    DoubleCrush,
    TripleCrush,
    DiscardOpponent,
    RandomDiscard,
    Draw,
    Assassin,
    Freeze,
    Drop,
    OnDrop,
    OnRefresh,
    Lancer,
    SLancer,
    RemoveSigni,
    NonAttackable,
    Down,
    Up,
    Charge,
    EnerAttack,
    Trash,
    Ener, // エナ送り
    PowerUp,
    PowerDown,
    Bounce,
    DeckBounce,
    Salvage,
    LifeBurst,
    Shadow,
    Invulnerable,
    OnSpell,
    OnArts,
    OnPiece,
    OnBanish,
    Banish,
    Guard,
    OnGuard,
    AttackNoEffect,
    // OnAttack,
    // OnAttackStart,
    OnTouch,
    Awake,
    Exceed,
    OnExceed,
    AddLife,
    OnBurst,
    LifeTrash,
    LifeCrush,
    Damage,
    OnLifeCrush,
    Position,
    Vanilla,
    Untouchable,
    TopCheck,
    BottomCheck,
    Barrier,
    MultiEner,
    LrigTrash,
    Charm,
    Craft,
    Acce,
    Rise,
    Recollect,
    SeekTop,
    EraseSkill,
    CancelDamage,
    Reanimate,
    AdditionalAttack,
    UnGuardable,
    SalvageSpell,
    BanishOnAttack,
    Shoot,
    LimitSigni,
    FreeSpell,
    DualColorEner,
    GainCoin,
    BetCoin,
    HandCost,   // 自分自身が捨てる
    AssistCost, // アシストをダウン
    Inherit,    // ルリグトラッシュのルリグを継承
    PreventGrowCost,
    PutSigniDefense,
    PutSigniOffense,
    Harmony,
    MagicBox,
    Virus,

    // team/non dream piece
}

// CardFeature に対応する FeatureTag を取得する
impl CardFeature {
    pub fn tag(&self) -> FeatureTag {
        match self {
            // Lethal
            CardFeature::AdditionalAttack
            | CardFeature::Assassin
            | CardFeature::BanishOnAttack
            | CardFeature::SLancer
            | CardFeature::Damage
            | CardFeature::RemoveSigni
            | CardFeature::UnGuardable
            | CardFeature::LimitSigni => FeatureTag::Lethal,

            // Offensive
            CardFeature::DoubleCrush
            | CardFeature::TripleCrush
            | CardFeature::Lancer
            | CardFeature::LifeCrush
            | CardFeature::LifeTrash
            | CardFeature::Banish
            | CardFeature::DeckBounce
            | CardFeature::Bounce
            | CardFeature::PowerDown
            | CardFeature::Ener
            | CardFeature::Trash
            | CardFeature::Up
            | CardFeature::PutSigniOffense => FeatureTag::Offensive,

            // Disturb
            CardFeature::EnerAttack
            | CardFeature::Shoot
            | CardFeature::EraseSkill
            | CardFeature::Position
            | CardFeature::Drop
            | CardFeature::Freeze
            | CardFeature::RandomDiscard
            | CardFeature::DiscardOpponent
            | CardFeature::Virus => FeatureTag::Disturb,

            // Endure
            CardFeature::Guard
            | CardFeature::Invulnerable
            | CardFeature::Shadow
            | CardFeature::NonAttackable
            | CardFeature::Down
            | CardFeature::OnGuard
            | CardFeature::Barrier
            | CardFeature::Untouchable
            | CardFeature::CancelDamage
            | CardFeature::Vanilla
            | CardFeature::AddLife
            | CardFeature::AttackNoEffect
            | CardFeature::PowerUp
            | CardFeature::PutSigniDefense => FeatureTag::Endure,

            // Enhance
            CardFeature::LifeBurst
            | CardFeature::Draw
            | CardFeature::Salvage
            | CardFeature::SalvageSpell
            | CardFeature::Reanimate
            | CardFeature::SeekTop
            | CardFeature::Recollect
            | CardFeature::MultiEner
            | CardFeature::BottomCheck
            | CardFeature::TopCheck
            | CardFeature::Charge
            | CardFeature::FreeSpell
            | CardFeature::DualColorEner
            | CardFeature::GainCoin
            | CardFeature::BetCoin
            | CardFeature::AssistCost
            | CardFeature::PreventGrowCost => FeatureTag::Enhance,

            CardFeature::Acce
            | CardFeature::Rise
            | CardFeature::Craft
            | CardFeature::Charm
            | CardFeature::LrigTrash
            | CardFeature::OnLifeCrush
            | CardFeature::OnBurst
            | CardFeature::Exceed
            | CardFeature::OnExceed
            | CardFeature::Awake
            | CardFeature::OnTouch
            | CardFeature::OnBanish
            | CardFeature::OnArts
            | CardFeature::OnPiece
            | CardFeature::OnSpell
            | CardFeature::OnRefresh
            | CardFeature::OnDrop
            | CardFeature::HandCost
            | CardFeature::Harmony
            | CardFeature::Inherit
            | CardFeature::MagicBox
            => FeatureTag::Others,
        }
    }
}

impl Display for CardFeature {
    #[allow(unreachable_patterns)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            CardFeature::DoubleCrush => "ダブルクラッシュ",
            CardFeature::TripleCrush => "トリプルクラッシュ",
            CardFeature::DiscardOpponent => "手札破壊",
            CardFeature::RandomDiscard => "ランダム手札破壊",
            CardFeature::Draw => "ドロー",
            CardFeature::Assassin => "アサシン",
            CardFeature::Freeze => "凍結",
            CardFeature::Drop => "デッキドロップ",
            CardFeature::OnDrop => "デッキドロップ時",
            CardFeature::OnRefresh => "リフレッシュ時",
            CardFeature::Lancer => "ランサー",
            CardFeature::SLancer => "Sランサー",
            CardFeature::RemoveSigni => "シグニ除外",
            CardFeature::NonAttackable => "アタック不可",
            CardFeature::Down => "ダウン",
            CardFeature::Up => "アップ",
            CardFeature::Charge => "エナチャージ",
            CardFeature::EnerAttack => "エナ破壊",
            CardFeature::Trash => "トラッシュ送り",
            CardFeature::Ener => "エナ送り",
            CardFeature::PowerUp => "パワーアップ",
            CardFeature::PowerDown => "パワーダウン",
            CardFeature::Bounce => "バウンス",
            CardFeature::DeckBounce => "デッキバウンス",
            CardFeature::Salvage => "回収",
            CardFeature::LifeBurst => "ライフバースト",
            CardFeature::Shadow => "シャドウ",
            CardFeature::Invulnerable => "バニッシュされない",
            CardFeature::OnSpell => "スペル使用時",
            CardFeature::OnArts => "アーツ使用時",
            CardFeature::OnPiece => "ピース使用時",
            CardFeature::OnBanish => "バニッシュした時",
            CardFeature::Banish => "バニッシュ",
            CardFeature::Guard => "ガード",
            CardFeature::OnGuard => "ガードした時",
            CardFeature::AttackNoEffect => "アタック無効",
            // CardFeature::OnAttack => "アタック時",
            // CardFeature::OnAttackStart => "アタック開始時",
            CardFeature::OnTouch => "対象になった時",
            CardFeature::Awake => "覚醒",
            CardFeature::Exceed => "エクシード",
            CardFeature::OnExceed => "エクシードした時",
            CardFeature::AddLife => "ライフクロス追加",
            CardFeature::OnBurst => "ライフバースト発動時",
            CardFeature::LifeTrash => "ライフクロストラッシュ送り",
            CardFeature::LifeCrush => "クラッシュ",
            CardFeature::Damage => "ダメージ",
            CardFeature::OnLifeCrush => "クラッシュ時",
            CardFeature::Position => "シグニゾーン移動",
            CardFeature::Vanilla => "能力を持たない",
            CardFeature::Untouchable => "効果を受けない", // アークゲイン
            CardFeature::TopCheck => "トップ確認",
            CardFeature::BottomCheck => "ボトム確認",
            CardFeature::Barrier => "バリア獲得",
            CardFeature::MultiEner => "マルチエナ",
            CardFeature::LrigTrash => "ルリグトラッシュ",
            CardFeature::Charm => "チャーム",
            CardFeature::Craft => "クラフト",
            CardFeature::Acce => "アクセ",
            CardFeature::Rise => "ライズ",
            CardFeature::Recollect => "リコレクト",
            CardFeature::SeekTop => "シーク",
            CardFeature::EraseSkill => "能力消去",
            CardFeature::CancelDamage => "ダメージ無効",
            CardFeature::Reanimate => "トラッシュ場出し",
            CardFeature::AdditionalAttack => "追加アタック",
            CardFeature::UnGuardable => "ガード不可",
            CardFeature::SalvageSpell => "スペル回収",
            CardFeature::BanishOnAttack => "アタック時バニッシュ",
            CardFeature::Shoot => "シュート",
            CardFeature::LimitSigni => "配置禁止",
            CardFeature::FreeSpell => "スペル割引",
            CardFeature::DualColorEner => "多色エナ",
            CardFeature::GainCoin => "コイン獲得",
            CardFeature::BetCoin => "ベット",
            CardFeature::HandCost => "手札コスト",
            CardFeature::AssistCost => "アシストダウン",
            CardFeature::Inherit => "ルリグ能力継承",
            CardFeature::PreventGrowCost => "グロウコスト軽減",
            CardFeature::PutSigniDefense => "ブロッカー場出し",
            CardFeature::PutSigniOffense => "シグニ場出し",
            CardFeature::Harmony => "ハーモニー",
            CardFeature::MagicBox => "マジックボックス",
            CardFeature::Virus => "ウィルス",
            // _ => "未処理フィーチャー",
        };
        write!(f, "{}", label)
    }
}

const NO_FEATURE_FOUND_SHIFT: i64 = 0_i64;

impl CardFeature {
    // (bits1, bits2)
    pub fn to_bit(&self) -> (i64, i64) {
        let bit1: i64 = 1_i64
            << match self {
                CardFeature::DoubleCrush => 1,
                CardFeature::TripleCrush => 2,
                CardFeature::DiscardOpponent => 3,
                CardFeature::RandomDiscard => 4,
                CardFeature::Draw => 5,
                CardFeature::Assassin => 6,
                CardFeature::Freeze => 7,
                CardFeature::Drop => 8,
                CardFeature::OnDrop => 9,
                CardFeature::OnRefresh => 10,
                CardFeature::Lancer => 11,
                CardFeature::SLancer => 12,
                CardFeature::RemoveSigni => 13,
                CardFeature::NonAttackable => 14,
                CardFeature::Down => 15,
                CardFeature::Up => 16,
                CardFeature::Charge => 17,
                CardFeature::EnerAttack => 18,
                CardFeature::Trash => 19,
                CardFeature::Ener => 20,
                CardFeature::PowerUp => 21,
                CardFeature::PowerDown => 22,
                CardFeature::Bounce => 23,
                CardFeature::DeckBounce => 24,
                CardFeature::Salvage => 25,
                CardFeature::LifeBurst => 26,
                CardFeature::Shadow => 27,
                CardFeature::Invulnerable => 28,
                CardFeature::OnSpell => 29,
                CardFeature::OnArts => 31,
                CardFeature::OnPiece => 31,
                CardFeature::OnBanish => 32,
                CardFeature::Banish => 33,
                CardFeature::Guard => 34,
                CardFeature::OnGuard => 35,
                CardFeature::AttackNoEffect => 36,
                // 37,
                CardFeature::OnTouch => 38,
                CardFeature::Awake => 39,
                CardFeature::Exceed => 40,
                CardFeature::OnExceed => 41,
                CardFeature::AddLife => 42,
                CardFeature::OnBurst => 43,
                CardFeature::LifeTrash => 44,
                CardFeature::LifeCrush => 45,
                CardFeature::Damage => 46,
                CardFeature::OnLifeCrush => 47,
                CardFeature::Position => 48,
                CardFeature::Vanilla => 49,
                CardFeature::Untouchable => 50,
                CardFeature::TopCheck => 51,
                CardFeature::BottomCheck => 52,
                CardFeature::Barrier => 53,
                CardFeature::MultiEner => 54,
                CardFeature::LrigTrash => 55,
                CardFeature::Charm => 56,
                CardFeature::Craft => 57,
                CardFeature::Acce => 58,
                CardFeature::Rise => 59,
                CardFeature::Recollect => 60,
                CardFeature::SeekTop => 61,
                CardFeature::EraseSkill => 62,
                _ => NO_FEATURE_FOUND_SHIFT,
            };
        // i64 なので63ビット使用可能、0から62で63個
        let bit2: i64 = 1_i64
            << match self {
                CardFeature::CancelDamage => 1,
                CardFeature::Reanimate => 2,
                CardFeature::AdditionalAttack => 3,
                CardFeature::UnGuardable => 4,
                CardFeature::SalvageSpell => 5,
                CardFeature::BanishOnAttack => 6,
                CardFeature::Shoot => 7,
                CardFeature::LimitSigni => 8,
                CardFeature::FreeSpell => 9,
                CardFeature::DualColorEner => 10,
                CardFeature::GainCoin => 11,
                CardFeature::BetCoin => 12,
                CardFeature::HandCost => 13,
                CardFeature::AssistCost => 14,
                CardFeature::Inherit => 15,
                CardFeature::PreventGrowCost => 16,
                CardFeature::PutSigniDefense => 17,
                CardFeature::PutSigniOffense => 18, // 手作業マークが必要そう
                CardFeature::Harmony => 19,
                CardFeature::MagicBox => 20,
                CardFeature::Virus => 21,
                _ => NO_FEATURE_FOUND_SHIFT,
            };

        (bit1, bit2)
    }
}

pub trait HashSetToBits {
    fn to_bits(&self) -> (i64, i64);
}

impl HashSetToBits for HashSet<CardFeature> {
    fn to_bits(&self) -> (i64, i64) {
        let mut bits = (0, 0);
        for feature in self {
            let (bit1, bit2) = feature.to_bit();
            bits.0 |= bit1;
            bits.1 |= bit2;
        }
        bits
    }
}
