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
            | CardFeature::Up => FeatureTag::Offensive,

            // Disturb
            CardFeature::EnerAttack
            | CardFeature::Shoot
            | CardFeature::EraseSkill
            | CardFeature::Position
            | CardFeature::Drop
            | CardFeature::Freeze
            | CardFeature::RandomDiscard
            | CardFeature::DiscardOpponent => FeatureTag::Disturb,

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
            | CardFeature::PowerUp => FeatureTag::Endure,

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
            | CardFeature::AssistCost => FeatureTag::Enhance,

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
            | CardFeature::Inherit => FeatureTag::Others,
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
            // _ => "未処理フィーチャー",
        };
        write!(f, "{}", label)
    }
}

impl CardFeature {
    // (bits1, bits2)
    pub fn to_bit(&self) -> (i64, i64) {
        let bit1: i64 = match self {
            CardFeature::DoubleCrush => 1_i64,
            CardFeature::TripleCrush => 1 << 1,
            CardFeature::DiscardOpponent => 1 << 2,
            CardFeature::RandomDiscard => 1 << 3,
            CardFeature::Draw => 1 << 4,
            CardFeature::Assassin => 1 << 5,
            CardFeature::Freeze => 1 << 6,
            CardFeature::Drop => 1 << 7,
            CardFeature::OnDrop => 1 << 8,
            CardFeature::OnRefresh => 1 << 9,
            CardFeature::Lancer => 1 << 10,
            CardFeature::SLancer => 1 << 11,
            CardFeature::RemoveSigni => 1 << 12,
            CardFeature::NonAttackable => 1 << 13,
            CardFeature::Down => 1 << 14,
            CardFeature::Up => 1 << 15,
            CardFeature::Charge => 1 << 16,
            CardFeature::EnerAttack => 1 << 17,
            CardFeature::Trash => 1 << 18,
            CardFeature::Ener => 1 << 19,
            CardFeature::PowerUp => 1 << 20,
            CardFeature::PowerDown => 1 << 21,
            CardFeature::Bounce => 1 << 22,
            CardFeature::DeckBounce => 1 << 23,
            CardFeature::Salvage => 1 << 24,
            CardFeature::LifeBurst => 1 << 25,
            CardFeature::Shadow => 1 << 26,
            CardFeature::Invulnerable => 1 << 27,
            CardFeature::OnSpell => 1 << 28,
            CardFeature::OnArts => 1 << 29,
            CardFeature::OnPiece => 1 << 30,
            CardFeature::OnBanish => 1 << 31,
            CardFeature::Banish => 1 << 32,
            CardFeature::Guard => 1 << 33,
            CardFeature::OnGuard => 1 << 34,
            CardFeature::AttackNoEffect => 1 << 35,
            // 36,
            // 37,
            CardFeature::OnTouch => 1 << 38,
            CardFeature::Awake => 1 << 39,
            CardFeature::Exceed => 1 << 40,
            CardFeature::OnExceed => 1 << 41,
            CardFeature::AddLife => 1 << 42,
            CardFeature::OnBurst => 1 << 43,
            CardFeature::LifeTrash => 1 << 44,
            CardFeature::LifeCrush => 1 << 45,
            CardFeature::Damage => 1 << 46,
            CardFeature::OnLifeCrush => 1 << 47,
            CardFeature::Position => 1 << 48,
            CardFeature::Vanilla => 1 << 49,
            CardFeature::Untouchable => 1 << 50,
            CardFeature::TopCheck => 1 << 51,
            CardFeature::BottomCheck => 1 << 52,
            CardFeature::Barrier => 1 << 53,
            CardFeature::MultiEner => 1 << 54,
            CardFeature::LrigTrash => 1 << 55,
            CardFeature::Charm => 1 << 56,
            CardFeature::Craft => 1 << 57,
            CardFeature::Acce => 1 << 58,
            CardFeature::Rise => 1 << 59,
            CardFeature::Recollect => 1 << 60,
            CardFeature::SeekTop => 1 << 61,
            CardFeature::EraseSkill => 1 << 62,
            _ => 0_i64,
        };
        // i64 なので63ビット使用可能、0から62で63個
        let bit2: i64 = match self {
            CardFeature::CancelDamage => 1_i64 << 0,
            CardFeature::Reanimate => 1 << 1,
            CardFeature::AdditionalAttack => 1 << 2,
            CardFeature::UnGuardable => 1 << 3,
            CardFeature::SalvageSpell => 1 << 4,
            CardFeature::BanishOnAttack => 1 << 5,
            CardFeature::Shoot => 1 << 6,
            CardFeature::LimitSigni => 1 << 7,
            CardFeature::FreeSpell => 1 << 8,
            CardFeature::DualColorEner => 1 << 9,
            CardFeature::GainCoin => 1 << 10,
            CardFeature::BetCoin => 1 << 11,
            CardFeature::HandCost => 1 << 12,
            CardFeature::AssistCost => 1 << 13,
            CardFeature::Inherit => 1 << 14,
            _ => 0_i64,
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
