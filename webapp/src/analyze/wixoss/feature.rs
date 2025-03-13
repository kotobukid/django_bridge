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
    // DeckAttack,
    OnDrop,
    OnRefresh,
    Lancer,
    SLancer,
    Penetrate,
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
    // アークゲイン
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
            | CardFeature::Penetrate    // ガード不可
            | CardFeature::UnGuardable    // ガード不可  todo　確認
                => FeatureTag::Lethal,

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
                => FeatureTag::Offensive,

            // Disturb
            CardFeature::EnerAttack
            | CardFeature::Shoot
            | CardFeature::EraseSkill
            | CardFeature::Position
            | CardFeature::Drop
            | CardFeature::Freeze
            | CardFeature::RandomDiscard
            | CardFeature::DiscardOpponent
                => FeatureTag::Disturb,

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
                => FeatureTag::Endure,

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
                => FeatureTag::Enhance,

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
            CardFeature::Drop => "デッキドロップ", // DeckAttack,
            CardFeature::OnDrop => "デッキドロップ時",
            CardFeature::OnRefresh => "リフレッシュ時",
            CardFeature::Lancer => "ランサー",
            CardFeature::SLancer => "Sランサー",
            CardFeature::Penetrate => "ガード不可",
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
            // _ => "未処理フィーチャー",
        };
        write!(f, "{}", label)
    }
}

impl CardFeature {
    pub fn to_bit(&self) -> (i64, i64) {
        match self {
            CardFeature::DoubleCrush => (1_i64, 0),
            CardFeature::TripleCrush => (1 << 1, 0),
            CardFeature::DiscardOpponent => (1 << 2, 0),
            CardFeature::RandomDiscard => (1 << 3, 0),
            CardFeature::Draw => (1 << 4, 0),
            CardFeature::Assassin => (1 << 5, 0),
            CardFeature::Freeze => (1 << 6, 0),
            CardFeature::Drop => (1 << 7, 0),
            // CardFeature::// DeckAttack => (1 << , 0),
            CardFeature::OnDrop => (1 << 8, 0),
            CardFeature::OnRefresh => (1 << 9, 0),
            CardFeature::Lancer => (1 << 10, 0),
            CardFeature::SLancer => (1 << 11, 0),
            CardFeature::Penetrate => (1 << 12, 0),
            CardFeature::NonAttackable => (1 << 13, 0),
            CardFeature::Down => (1 << 14, 0),
            CardFeature::Up => (1 << 15, 0),
            CardFeature::Charge => (1 << 16, 0),
            CardFeature::EnerAttack => (1 << 17, 0),
            CardFeature::Trash => (1 << 18, 0),
            CardFeature::Ener => (1 << 19, 0),
            CardFeature::PowerUp => (1 << 20, 0),
            CardFeature::PowerDown => (1 << 21, 0),
            CardFeature::Bounce => (1 << 22, 0),
            CardFeature::DeckBounce => (1 << 23, 0),
            CardFeature::Salvage => (1 << 24, 0),
            CardFeature::LifeBurst => (1 << 25, 0),
            CardFeature::Shadow => (1 << 26, 0),
            CardFeature::Invulnerable => (1 << 27, 0),
            CardFeature::OnSpell => (1 << 28, 0),
            CardFeature::OnArts => (1 << 29, 0),
            CardFeature::OnPiece => (1 << 30, 0),
            CardFeature::OnBanish => (1 << 31, 0),
            CardFeature::Banish => (1 << 32, 0),
            CardFeature::Guard => (1 << 33, 0),
            CardFeature::OnGuard => (1 << 34, 0),
            CardFeature::AttackNoEffect => (1 << 35, 0),
            // CardFeature::OnAttack => (1 << 36, 0),
            // CardFeature::OnAttackStart => (1 << 37, 0),
            CardFeature::OnTouch => (1 << 38, 0),
            CardFeature::Awake => (1 << 39, 0),
            CardFeature::Exceed => (1 << 40, 0),
            CardFeature::OnExceed => (1 << 41, 0),
            CardFeature::AddLife => (1 << 42, 0),
            CardFeature::OnBurst => (1 << 43, 0),
            CardFeature::LifeTrash => (1 << 44, 0),
            CardFeature::LifeCrush => (1 << 45, 0),
            CardFeature::Damage => (1 << 46, 0),
            CardFeature::OnLifeCrush => (1 << 47, 0),
            CardFeature::Position => (1 << 48, 0),
            CardFeature::Vanilla => (1 << 49, 0),
            CardFeature::Untouchable => (1 << 50, 0),
            // CardFeature::// アークゲイ => (1 << , 0),
            CardFeature::TopCheck => (1 << 51, 0),
            CardFeature::BottomCheck => (1 << 52, 0),
            CardFeature::Barrier => (1 << 53, 0),
            CardFeature::MultiEner => (1 << 54, 0),
            CardFeature::LrigTrash => (1 << 55, 0),
            CardFeature::Charm => (1 << 56, 0),
            CardFeature::Craft => (1 << 57, 0),
            CardFeature::Acce => (1 << 58, 0),
            CardFeature::Rise => (1 << 59, 0),
            CardFeature::Recollect => (1 << 60, 0),
            CardFeature::SeekTop => (1 << 61, 0),
            CardFeature::EraseSkill => (1 << 62, 0),

            // i64 なので63ビット使用可能、0から62で63個
            CardFeature::CancelDamage => (0, 1_i64 << 0),
            CardFeature::Reanimate => (0, 1 << 1),
            CardFeature::AdditionalAttack => (0, 1 << 2),
            CardFeature::UnGuardable => (0, 1 << 3),
            CardFeature::SalvageSpell => (0, 1 << 4),
            CardFeature::BanishOnAttack => (0, 1 << 5),
            CardFeature::Shoot => (0, 1 << 6),
        }
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
