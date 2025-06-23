use regex::Regex;
use serde::de::Unexpected::Char;

pub mod feature;
pub use feature::{CardFeature, BurstFeature};
use crate::BurstFeature::{BlockLrig, BlockSigni, Charge, Defend1, Defend2, Discard, Draw, EraseSkill, Freeze, Guard, Heal, OffenciveDefend, Salvage, Search};
use crate::CardFeature::DiscardOpponent;
// public exports are done at the function level

// フィーチャーのラベル定義を一元管理
pub mod labels {
    use super::feature::CardFeature;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    // 日本語ラベルからCardFeatureへのマッピング
    // Display実装から自動的にラベルを収集して構築
    pub static FEATURE_LABELS: Lazy<HashMap<&'static str, CardFeature>> = Lazy::new(|| {
        CardFeature::create_vec()
            .into_iter()
            .map(|feature| {
                let label = feature.to_string();
                (Box::leak(label.into_boxed_str()) as &'static str, feature)
            })
            .collect()
    });
}

pub struct DetectPattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub features_detected: &'static [CardFeature],
}

pub struct ReplacePattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub replace_to: &'static str,
    pub features_detected: &'static [CardFeature],
}

// BurstFeature用の検出パターン構造体
pub struct BurstDetectPattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub features_detected: &'static [BurstFeature],
}

pub struct BurstReplacePattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub replace_to: &'static str,
    pub features_detected: &'static [BurstFeature],
}

macro_rules! replace_pattern {
    ($pat:expr, $replace:expr, $($feature:expr),*) => {
        ReplacePattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            replace_to: $replace,
            features_detected: &[$($feature),*],
        }
    };
    // 空の機能リスト用の特殊ケース
    ($pat:expr, $replace:expr) => {
        ReplacePattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            replace_to: $replace,
            features_detected: &[],
        }
    };
}

// 置換ぜすテキストをそのまま残すが効果検出もしない、考慮漏れではないことを示すケース`
macro_rules! concerned {
    // ($pat:expr) => {};
    // ($pat:expr,*) => {};
    ($( $x:expr ),*) => {};
}

macro_rules! detect_pattern {
    ($pat:expr, $($feature:expr),*) => {
        DetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            features_detected: &[$($feature),*],
        }
    };
    // 空の機能リスト用の特殊ケース
    ($pat:expr) => {
        DetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            features_detected: &[],
        }
    };
}

// BurstFeature用のマクロ定義
macro_rules! burst_replace_pattern {
    ($pat:expr, $replace:expr, $($feature:expr),*) => {
        BurstReplacePattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            replace_to: $replace,
            features_detected: &[$($feature),*],
        }
    };
    ($pat:expr, $replace:expr) => {
        BurstReplacePattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            replace_to: $replace,
            features_detected: &[],
        }
    };
}

macro_rules! burst_detect_pattern {
    ($pat:expr, $($feature:expr),*) => {
        BurstDetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            features_detected: &[$($feature),*],
        }
    };
    ($pat:expr) => {
        BurstDetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            features_detected: &[],
        }
    };
}

// 置換対象にせず、機能検出もしないテキスト
concerned![
    r"\(シグニの下に置かれるカードは表向きである\)",
    r"\(場に出せなかった場合はトラッシュに置く\)",
    r"\(このピースの後に場に出たシグニにも影響を与える\)",
    r"\(そこにあるすべてをトラッシュに置く。プレイヤーはそこにシグニを配置できない\)",
    r"\(場に出せなかった場合はトラッシュに置く\)",
    r"\(他のシグニより先にアタックしなければならない\)",
    r"\(対戦相手の場にあるカードも参照する\)",
    r"\(両方の【使用条件】を満たさなければならない\)",
    r"\(パワーが0以下になると、移動した後でルールによってバニッシュされる\)",
    r"\(表向きで置く\)",
    r"\(下のカードは場に残す\)",
    r"\(この能力の発動後に場に出たシグニはこの効果の影響を受けない\)"
];

pub const PATTERNS_AMOUNT_R: usize = 78;
pub const PATTERNS_AMOUNT_D: usize = 165;

pub fn create_detect_patterns() -> (
    [ReplacePattern; PATTERNS_AMOUNT_R],
    [DetectPattern; PATTERNS_AMOUNT_D],
) {
    let r_patterns: [ReplacePattern; PATTERNS_AMOUNT_R] = [
        replace_pattern![r"『", ""],
        replace_pattern![r"ライフバースト:", "LB:", CardFeature::LifeBurst],
        replace_pattern![r"』", ""],
        replace_pattern![r"ライフバースト:", "LB:", CardFeature::LifeBurst],
        replace_pattern![
            r"\(対戦相手のライフクロスが1枚以上ある場合、ライフクロス1枚をクラッシュし、0枚の場合、あなたはゲームに勝利する\)",
            "",
            CardFeature::Damage
        ],
        replace_pattern![
            r"\(パワーが0以下のシグニはルールによってバニッシュされる\)",
            "",
            CardFeature::PowerDown
        ],
        replace_pattern![
            r"\(アタックによるダメージでライフクロスを2枚クラッシュする\)",
            "*DOUBLE CRUSH*",
            CardFeature::DoubleCrush
        ],
        replace_pattern![
            r"\(【ダブルクラッシュ】を持つシグニがアタックによってダメージを与えた場合ライフクロスを1枚ではなく2枚クラッシュする\)",
            "*DOUBLE CRUSH*"
        ],
        replace_pattern![
            r"\(2枚以下の場合、それらをすべて選ぶ\)",
            "*TARGET ALL OVER*"
        ],
        replace_pattern![
            r"\(このシグニが場に出たとき、あなたのアップ状態の.+をダウンしないかぎり、これをダウンする\)",
            "*HARMONY*"
        ],
        replace_pattern![
            r"\(【ウィルス】と同じシグニゾーンにあるシグニは感染状態である\)",
            "*VIRUS*",
        ],
        replace_pattern![
            r"\(【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ダブルクラッシュ】を持つシグニがアタックによってダメージを与えた場合ライフクロスを1枚ではなく2枚クラッシュする\)",
            "*DOUBLE CRUSH && ASSASSIN*"
        ],
        replace_pattern![
            r"\(【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを1枚クラッシュする\)",
            "*LANCER*",
            CardFeature::Lancer
        ],
        replace_pattern![
            r"\(【Sランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスがある場合はそれを1枚クラッシュする。無い場合は対戦相手にダメージを与える\)",
            "*S LANCER*",
            CardFeature::SLancer
        ],
        replace_pattern![
            r"\(このクラフトは効果以外によっては場に出せない\)",
            "*NO STANDARD PUT*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(このスペルはあなたのメインフェイズにルリグデッキから使用できる\)",
            "*SPELL CRAFT*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(クラフトであるスペルは、使用後にゲームから除外される\)",
            "*SPELL CRAFT GOES REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(ゲーム終了時にそのレゾナがルリグデッキにあれば公開する\)",
            "*RESONA CRAFT REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(《ガードアイコン》を持つシグニは【ガード】を得る\)",
            "*GUARD*",
        ],
        replace_pattern![
            r"\(複数の【出】能力は好きな順番で発動できる\)",
            "*MULTIPLE CIP*"
        ],
        replace_pattern![
            r"\(この条件を満たさなければ場に出せない\)",
            "*RISE LIMITATION*"
        ],
        replace_pattern![
            r"\(【チャーム】は裏向きでシグニに付き、1体に1枚までしか付けられない\)",
            "*CHARM*"
        ],
        replace_pattern![
            r"\(【ソウル】はシグニに1枚まで付き、そのシグニが場を離れるとルリグトラッシュに置かれる\)",
            "*SOUL*",
            CardFeature::Soul
        ],
        replace_pattern![
            r"\(【チャーム】や【アクセ】、【ソウル】はシグニに付く\)",
            "*CHARM/ACCE/SOUL BELONGS TO SIGNI*"
        ],
        replace_pattern![
            r"\(デッキが\d+枚以下の場合は置き換えられない\)",
            "*FEATURE LIMIT DECK DROP*"
        ],
        replace_pattern![
            r"\(このカードを手札から捨てることで、ルリグのアタックによるダメージを一度防ぐ\)",
            "*GUARD*"
        ],
        replace_pattern![
            r"\(シグニの下に置かれたカードは、そのシグニが場を離れるとルールによってトラッシュに置かれる\)",
            "*GO TO TRASH TOGETHER*"
        ],
        replace_pattern![
            r"\(この能力はこのカードがトラッシュにある場合にしか使用できない\)",
            "*ONLY AVAILABLE IN TRASH*"
        ],
        replace_pattern![
            r"\(あなたの場に\<.+\>のルリグ3体がいるなら【チーム自】が有効になる\)",
            "*TEAM SKILL*"
        ],
        replace_pattern![
            r"\(このスペルを使用する際、使用コストとして追加で.+を支払ってもよい\)",
            "*BET*"
        ],
        replace_pattern![
            r"\(【マジックボックス】はシグニゾーン1つにつき1つまで裏向きで設置できる\)",
            "*MAGIC BOX*"
        ],
        replace_pattern![
            r"\(【マジックボックス】はシグニゾーン1つにつき1つまで裏向きで設置できる。すでに【マジックボックス】のあるシグニゾーンに設置する場合、元からある【マジックボックス】をトラッシュに置いてから設置する\)",
            "*MAGIC BOX ON BOX*"
        ],
        replace_pattern![
            r"\(【アクセ】はシグニ1体に1枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される\)",
            "*ACCE*"
        ],
        replace_pattern![
            r"\(シグニのパワーを計算する場合、先に基本パワーを適用してプラスやマイナスをする\)",
            "*CALC ORDER*"
        ],
        replace_pattern![
            r"\(ピースはあなたの場にルリグが3体いないと使用できない\)",
            "*COMMON PIECE*"
        ],
        replace_pattern![
            r"\(ピースはあなたの場にルリグが3体いると使用できる\)",
            "*COMMON PIECE*"
        ],
        replace_pattern![
            r"\(あなたのルリグの下からカードを合計\d+枚ルリグトラッシュに置く\)",
            ""
        ],
        replace_pattern![
            r"\(【チーム】または【ドリームチーム】を持つピースはルリグデッキに(合計|合計で)1枚までしか入れられない\)",
            "*TEAM PIECE*"
        ],
        replace_pattern![r"\(あなたの場にいるルリグ3体がこの条件を満たす\)", "*TEAM*"],
        replace_pattern![
            r"\(シグニは覚醒すると場にあるかぎり覚醒状態になる\)",
            "*AWAKE*",
        ],
        replace_pattern![
            r"\(この能力はこのシグニが場にある場合にしか使用できない\)",
            "*AVAILABLE ONLY IN BATTLEFIELD*"
        ],
        replace_pattern![
            r"\(グロウしても新しいセンタールリグは能力を得たままである\)",
            "*IN GAME AVAILABLE*"
        ],
        replace_pattern![
            r"\(凍結された(ルリグ|シグニ)は次の自分のアップフェイズにアップしない\)",
            "*FROZEN*"
        ],
        replace_pattern![
            r"\(凍結されたルリグとシグニは次の自分のアップフェイズにアップしない\)",
            "*FROZEN*"
        ],
        replace_pattern![
            r"\(フェゾーネマジックは5種類ある\)",
            "*FESONE MAGIC*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(【出】能力の:の左側はコストである。コストを支払わず発動しないことを選んでもよい\)",
            "*CIP COST*"
        ],
        replace_pattern![
            r"\(ゲームを開始する際に、センタールリグでないルリグを表向きにしても《コインアイコン》を得られない\)",
            "*GAIN NO COINS*"
        ],
        replace_pattern![
            r"\(プレイヤーが保持できる《コインアイコン》の上限は5枚である\)",
            "*COIN LIMIT*"
        ],
        replace_pattern![
            r"\(すでに場に3体以上ある場合は2体になるようにシグニをトラッシュに置く\)",
            "*SIGNI ZONE RESTRICTION*"
        ],
        replace_pattern![
            r"\(コストの合計とは、カードの左上のエナコストの数字の合計である。例えばコストが《白×1》《無×1》の場合、コストの合計は2である\)",
            "*TOTAL COST*"
        ],
        replace_pattern![
            r"\(コストのない【出】能力は発動しないことを選べない\)",
            "*MUST APPLY CIP*"
        ],
        replace_pattern![
            r"\(コストのない【出】能力は発動しないことを選べない。ライフクロスが1枚の場合その1枚をトラッシュに置く\)",
            "*MUST APPLY CIP*"
        ],
        replace_pattern![
            r"\(【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える\)",
            "*ASSASSIN*"
        ],
        replace_pattern![
            r"\(【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを1枚クラッシュする\)",
            "*ASSASSIN OR LANCER*"
        ],
        replace_pattern![
            r"\(このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える\)",
            "*SELF ASSASSIN*"
        ],
        replace_pattern![
            r"\(表記されているパワーとは、元々それに印刷されている値である\)",
            "*BASIC POWER*"
        ],
        replace_pattern![
            r"\(あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】1つを消費し、そのダメージを受けない\)",
            "*LRIG BARRIER*"
        ],
        replace_pattern![
            r"\(あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】1つを消費し、そのダメージを受けない\)",
            "*SIGNI BARRIER*"
        ],
        replace_pattern![
            r"\(あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】1つを消費し、そのダメージを受けない。あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】1つを消費し、そのダメージを受けない\)",
            "*LRIG/SIGNI BARRIER*"
        ],
        replace_pattern![
            r"\((この|それらの)シグニは.+によって対象にされない\)",
            "*SHADOW*"
        ],
        replace_pattern![
            r"\(【シャドウ\(.+\)】を持つシグニは対戦相手の.*によって対象にされない\)",
            "*LIMITED SHADOW*",
            CardFeature::Shadow
        ],
        replace_pattern![
            r"\(【シャドウ】を持つシグニは対戦相手によって対象にされない\)",
            "*SHADOW*",
            CardFeature::Shadow
        ],
        replace_pattern![
            r"\(ゲーム終了時にそのレゾナがルリグデッキにあれば公開する\)",
            "*RANDOM RESONA MUST BE EXPOSED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(レゾナでありクラフトであるシグニはリムーブできず場を離れるとゲームから除外される\)",
            "*RESONA CANT BE REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(クラフトであるシグニは場を離れるとゲームから除外される\)",
            "*CRAFT SIGNI REMOVED ON LEAVE*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(このクラフトの上にあるシグニが場を離れるとこのクラフトはゲームから除外される\)",
            "*TORAMARU GIMMICK*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"\(チェックゾーンにあるカードはターン終了時にトラッシュに置かれる\)",
            "*CHECK ZONE*"
        ],
        replace_pattern![
            r"\(あなたの場にいるルリグが1体で、そのルリグがレベル3以上であるかぎり、そのルリグのリミットを+2する\)",
            "*LIMIT UPPER EFFECTS*",
            CardFeature::EnhanceLimit
        ],
        replace_pattern![
            r"\(【リミットアッパー】はあなたのルリグゾーンに1つまでしか置けない\)",
            "*ONLY ONE LIMIT UPPER*",
            CardFeature::EnhanceLimit
        ],
        replace_pattern![
            r"\(あなたのデッキの一番上のカードをエナゾーンに置く\)",
            "*ENER CHARGE*"
        ],
        replace_pattern![
            r"\(対戦相手のシグニが【シュート】を持つシグニとのバトルによってバニッシュされる場合、エナゾーンに置かれる代わりにトラッシュに置かれる\)",
            "*SHOOT LIKE*"
        ],
        replace_pattern![
            r"\(あなたのルリグトラッシュに[\(0-9\)]枚以上のアーツがあるかぎり《リコレクトアイコン》\[[\(0-9\)]枚以上\]に続く文章が有効になる\)",
            "*RECOLLECT*"
        ],
        replace_pattern![
            r"\(《相手ターン》の能力は、対戦相手のターンの間にのみ有効になる\)",
            "*OPPONENT TURN*"
        ],
        replace_pattern![
            r"\(エナコストを支払う際、このカードは.+1つとして支払える\)",
            "*MULTI ENER*",
            CardFeature::DualColorEner
        ],
        replace_pattern![r"\(その生徒の【絆】能力が有効になる\)", "*BOND*"],
        replace_pattern![
            r"\(カード名1つを宣言する。宣言されたカード名のカードの【絆】能力が有効になる\)",
            "*BOND*"
        ],
        replace_pattern![
            r"\(生徒とは、ブルーアーカイブにおけるキャラクターのことです。生徒との絆を獲得すると、その生徒のカードが持つ【絆】能力が有効になります。場や手札にいない生徒との絆も獲得できます。ルリグである生徒との絆を獲得した場合は、その生徒のすべてのカードの【絆】能力が有効になります。生徒との絆の数に上限はなく、失われることはありません\!\)",
            "*BOND*"
        ],
        replace_pattern![r"\(無色は色に含まれない\)", "*NO COLOR MEANS NO COLOR*"],
    ];

    let d_patterns: [DetectPattern; PATTERNS_AMOUNT_D] = [
        detect_pattern![r"【ウィルス】", CardFeature::Virus],
        detect_pattern![r"感染状態", CardFeature::Virus],
        detect_pattern![r"【ハーモニー】", CardFeature::Harmony],
        detect_pattern![r"【ウィルス】", CardFeature::Virus],
        detect_pattern![
            r"\(このクラフトは効果以外によっては場に出せない\)",
            CardFeature::Craft
        ],
        detect_pattern![r"覚醒する", CardFeature::Awake],
        detect_pattern![
            r"\(このスペルはあなたのメインフェイズにルリグデッキから使用できる\)",
            CardFeature::Craft
        ],
        detect_pattern![
            r"\(クラフトであるスペルは、使用後にゲームから除外される\)",
            CardFeature::Craft
        ],
        detect_pattern![r"《ガードアイコン》", CardFeature::Guard],
        detect_pattern![r"【アクセ】", CardFeature::Acce],
        detect_pattern![r"《アクセアイコン》", CardFeature::Acce],
        detect_pattern![r"アクセされてい", CardFeature::Acce], // されている・されていた
        detect_pattern![r"アクセされたとき", CardFeature::Acce],
        detect_pattern![r"アクセするための", CardFeature::Acce],
        detect_pattern![r"エクシード\d+", CardFeature::Exceed],
        detect_pattern![
            r"\(ゲームを開始する際に、このルリグを表向きにしたとき、このルリグがセンタールリグであるなら、[《コインアイコン》]+を得る\)",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"\(右下に【コイン】を持つルリグがグロウしたとき、それと同じ枚数の《コインアイコン》を得る\)", //　この記法では単数
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"ルリグデッキに加える。\(ゲーム終了時にそのレゾナがルリグデッキにあれば公開する\)",
            CardFeature::Craft
        ],
        detect_pattern![r"《コインアイコン》を得る", CardFeature::GainCoin],
        detect_pattern![r"ガードアイコン", CardFeature::Guard],
        detect_pattern![r"捨てさせる。", CardFeature::DiscardOpponent],
        detect_pattern![r"対戦相手は手札を\d+枚捨て", CardFeature::DiscardOpponent],
        detect_pattern![
            r"各プレイヤーは手札をすべてエナゾーンに置",
            CardFeature::DiscardOpponent,
            CardFeature::RandomDiscard
        ],
        detect_pattern![r"見ないで選び、捨てさせる。", CardFeature::RandomDiscard],
        detect_pattern![r"対戦相手の手札を見て", CardFeature::RandomDiscard],
        detect_pattern![
            // コードラビラント・ヨグソトス専用
            r"手札を3枚まで見ないで選び、それらを見て1枚をデッキの一番下に置く",
            CardFeature::RandomDiscard
        ],
        detect_pattern![r"対象になったとき", CardFeature::OnTouch],
        detect_pattern![r"ダウンする。", CardFeature::Down],
        detect_pattern![r"エナチャージ", CardFeature::Charge],
        detect_pattern![r"カードを\d+枚までエナゾーンに置", CardFeature::Charge],
        detect_pattern![
            r"残りを好きな順番でデッキの一番下に置く",
            CardFeature::BottomCheck
        ],
        detect_pattern![r"(それ|シグニ)をトラッシュに置", CardFeature::Trash],
        detect_pattern![r"シグニバリア", CardFeature::Barrier],
        detect_pattern![r"ルリグバリア", CardFeature::Barrier],
        // (r"がアタックしたとき", do_remove:  "*ON ATTACK*", CardFeature::OnAttack]),
        detect_pattern![r"アサシン", CardFeature::Assassin],
        detect_pattern![r"【リミットアッパー】", CardFeature::EnhanceLimit],
        detect_pattern![r"それのリミットを\+1", CardFeature::EnhanceLimit],
        detect_pattern![
            r"あなたのグロウフェイズ開始時、このゲームの間、あなたの場にいる《夢限 -Q-》のリミットを\+1する",
            CardFeature::EnhanceLimit
        ],
        detect_pattern![r"【シャドウ】", CardFeature::Shadow],
        detect_pattern![r"【シャドウ\(.+\)】", CardFeature::Shadow],
        detect_pattern![r"【マルチエナ】", CardFeature::DualColorEner],
        detect_pattern![
            r"\(エナコストを支払う際、このカードは好きな色1つとして支払える\)",
            CardFeature::DualColorEner
        ],
        detect_pattern![r"チャーム", CardFeature::Charm],
        detect_pattern![r"ダブルクラッシュ", CardFeature::DoubleCrush],
        detect_pattern![
            r"トリプルクラッシュ",
            CardFeature::DoubleCrush // ダブクラと統合
        ],
        detect_pattern![r"【シュート】", CardFeature::ShootLike],
        detect_pattern![
            r"エナゾーンに置かれる代わりに(トラッシュ|手札|デッキの一番下)",
            CardFeature::ShootLike
        ],
        detect_pattern![r"【ライズ】あなたの", CardFeature::Rise],
        detect_pattern![r"ベット―", CardFeature::BetCoin],
        detect_pattern![r"コインアイコン", CardFeature::BetCoin],
        detect_pattern![r"Sランサー", CardFeature::SLancer, CardFeature::Lancer],
        detect_pattern![r"Sランサー", CardFeature::SLancer, CardFeature::Lancer],
        detect_pattern![r"【マジックボックス】", CardFeature::MagicBox],
        detect_pattern![
            r"対戦相手のシグニ\d+体を対象とし、それをゲームから除外する",
            CardFeature::RemoveSigni
        ],
        detect_pattern![r"バニッシュ", CardFeature::Banish],
        detect_pattern![
            r"シグニ.+エナゾーンに置", //todo: 対戦相手の
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手は自分の.?シグニ1体を選びエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のパワー.+以下のシグニ1体を対象とし、それをエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のシグニを\d+体(まで|を)対象とし、(それら|それ)をエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のすべてのシグニをエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手の.+のシグニ\d+体を対象とし、それをエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"支払ってもよい。そうした場合、(それ|それら)をエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"フェゾーネマジックのクラフトから2種類を1枚ずつ公開しルリグデッキに加える",
            CardFeature::Craft
        ],
        detect_pattern![
            r"支払っても良い。そうした場合、対戦相手は自分のシグニ1体を選びエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のシグニ1体を対象とし、それとこのシグニをエナゾーンに",
            CardFeature::EnerOffensive
        ],
        detect_pattern![r"クラフトの《", CardFeature::Craft],
        detect_pattern![
            r"あなたのルリグデッキに《コードイート ケチャチャ》",
            CardFeature::Craft
        ], // メル=チアーズ専用
        detect_pattern![r"フェゾーネマジックのクラフト", CardFeature::Craft],
        detect_pattern![r"シグニをアップ", CardFeature::Up],
        detect_pattern![
            r"シグニ\d+体を対象とし、(それ|それら)をアップ",
            CardFeature::Up
        ],
        detect_pattern![r"凍結する", CardFeature::Freeze],
        detect_pattern![r"凍結状態", CardFeature::Freeze],
        detect_pattern![r"それらの場所を入れ替え", CardFeature::Position],
        detect_pattern![r"場に出すことができない", CardFeature::LimitSigni],
        detect_pattern![r"シグニゾーン1つに配置する", CardFeature::Position],
        detect_pattern![r"シグニゾーン1つを消す", CardFeature::LimitSigni],
        detect_pattern![r"配置しなければ", CardFeature::Position], // ギロッポン
        detect_pattern![r"新たに配置できない", CardFeature::LimitSigni],
        detect_pattern![r"新たに場に出せない", CardFeature::LimitSigni],
        detect_pattern![r"それらの場所を入れ替", CardFeature::Position],
        detect_pattern![
            r"対戦相手のシグニ\d+体(まで|を)対象とし、(それら|それ)を手札に戻",
            CardFeature::Bounce
        ],
        detect_pattern![
            r"対戦相手のパワー\d+.*\d+体(まで|を)対象とし、(それら|それ)を手札に戻",
            CardFeature::Bounce
        ],
        detect_pattern![
            r"対戦相手のシグニ\d+体を対象とし、それを手札に戻",
            CardFeature::Bounce
        ],
        // (    r"手札に加え", do_remove:  "*SALVAGE*", CardFeature::Salvage]),
        detect_pattern![r"ライフクロス\d+枚をトラッシュに置", CardFeature::LifeTrash],
        detect_pattern![
            r"エナゾーンからカード\d+枚(を|選び).+トラッシュに置",
            CardFeature::EnerAttack
        ],
        detect_pattern![r"ルリグトラッシュに置", CardFeature::LrigTrash],
        // (r"アタックフェイズ開始時", do_remove:  "*ON ATTACK START*", CardFeature::OnAttackStart]),
        detect_pattern![r"ライフクロスに加える", CardFeature::AddLife],
        detect_pattern![r"ランサー", CardFeature::Lancer],
        detect_pattern![r"ライフクロスを1枚クラッシュする", CardFeature::LifeCrush],
        detect_pattern![
            r"対戦相手のライフクロス1枚をクラッシュする。",
            CardFeature::LifeCrush
        ],
        detect_pattern![r"対戦相手にダメージを与える。", CardFeature::Damage],
        detect_pattern![r"クラッシュしたとき、", CardFeature::OnLifeCrush],
        detect_pattern![
            r"クラッシュされ(る場合|たとき|るかトラッシュ|ていた場合)、",
            CardFeature::OnLifeCrush
        ],
        detect_pattern![r"リコレクトアイコン", CardFeature::Recollect],

        detect_pattern![r"あなたのデッキの上からカードを\d+枚見(る。|て)", CardFeature::SeekTop],
        detect_pattern![r"あなたのデッキの一番上を見", CardFeature::SeekTop],
        detect_pattern![r"あなたのデッキの一番上を公開する", CardFeature::SeekTop],
        detect_pattern![r"あなたは自分のデッキの上からカードを\d+枚見る", CardFeature::SeekTop],
        detect_pattern![r"あなたのデッキの上から、カードを\d+枚トラッシュに置きカードを\d+枚見る", CardFeature::SeekTop],   // アト//メモリア
        detect_pattern![r"デッキの上からカードを\d+枚見て", CardFeature::SeekTop],

        detect_pattern![r"デッキの一番上に(戻|置)", CardFeature::TopSet],
        detect_pattern![r"のシグニは能力を失う", CardFeature::EraseSkill],
        detect_pattern![r"それは能力を失う", CardFeature::EraseSkill],
        detect_pattern![
            r"シグニを\d+体(まで|を)対象とし、ターン終了時まで、それは能力を失う",
            CardFeature::EraseSkill
        ],
        detect_pattern![r"それを《サーバント ZERO》にする", CardFeature::EraseSkill],
        detect_pattern![r"アタックできない", CardFeature::NonAttackable],
        detect_pattern![r"カードを\d+枚引", CardFeature::Draw],
        detect_pattern![
            r"デッキの上からカードを\d+枚トラッシュに置",
            CardFeature::Drop
        ],
        detect_pattern![
            r"対戦相手のエナゾーンからカードを\d+枚まで対象とし、それらを手札に戻",
            CardFeature::EnerAttack
        ],
        detect_pattern![r"デッキの一番下に置", CardFeature::DeckBounce],
        detect_pattern![r"シグニのパワーを\+", CardFeature::PowerUp],
        detect_pattern![r"のパワーを\+", CardFeature::PowerUp], // 範囲が広く検討の余地あり
        detect_pattern![r"このシグニのパワーは\+", CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを\+", CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを\-", CardFeature::PowerDown],
        detect_pattern![
            r"(シグニ|それ)のパワーをこの方法で.+\-", // この+は正規表現の記法
            CardFeature::PowerDown
        ],
        detect_pattern![r"ダメージを受けない", CardFeature::CancelDamage],
        detect_pattern![r"トラッシュからシグニ.+場に出", CardFeature::Reanimate],
        detect_pattern![
            // あなたのトラッシュから黒のシグニ1枚を対象とし、それを場に出す  // TODO
            r"あなたのトラッシュから(シグニ|.+のシグニ)\d+枚を対象とし、それを場に出",
            CardFeature::Reanimate
        ],
        detect_pattern![
            r"(この|その)ルリグをアップし",
            CardFeature::AdditionalAttack
        ],
        detect_pattern![r"対戦相手は【ガード】ができない", CardFeature::UnGuardable],
        detect_pattern![
            r"を支払わないかぎり【ガード】ができない",
            CardFeature::UnGuardable
        ],
        detect_pattern![r"スペル\d+枚を.+手札に加え", CardFeature::SalvageSpell],
        detect_pattern![
            r"あなたのトラッシュから.?(シグニ|シグニを|シグニをそれぞれ)\d+枚(を|まで).+手札に加え",
            CardFeature::Salvage
        ],
        detect_pattern![
            r"あなたのトラッシュから.?(シグニ|シグニを|シグニをそれぞれ)\d+枚(を|まで)対象とし、それを手札に加え",
            CardFeature::Salvage
        ],
        detect_pattern![
            r"スペル\d+枚をコストを支払わずに使用する",
            CardFeature::FreeSpell
        ],
        detect_pattern![r"このアーツの使用コストは.+減る", CardFeature::FreeArts],
        detect_pattern![
            r"このシグニがアタックしたとき.+バニッシュする",
            CardFeature::BanishOnAttack
        ],
        detect_pattern![
            r"アタックを無効に", // todo: 攻防あり
            CardFeature::AttackNoEffect
        ],
        detect_pattern![r"バニッシュされない", CardFeature::Invulnerable],
        detect_pattern![r"バニッシュされたとき", CardFeature::OnBanish],
        detect_pattern![
            r"(ライフバーストを使用することを選んだ場合|ライフバーストの能力化効果の対象になったとき|ライフバースト】を持っているか|ライフバースト】を持つ場合|ライフバーストが発動する場合|ライフバーストは発動しない)",
            CardFeature::OnBurst
        ],
        detect_pattern![
            r"(置かれたライフクロスは|あなたのライフクロスとチェックゾーンにある【ライフバースト】を持たないカードは|ライフバースト】を持つカードを好きな枚数公開|ライフバーストの能力か効果の対象になったとき)",
            CardFeature::OnBurst
        ],
        detect_pattern![
            r"(エクシードのコストとして|あなたがエクシードのコストを支払ったとき、)",
            CardFeature::OnExceed
        ],
        detect_pattern![r"手札を\d+枚捨ててもよい", CardFeature::HandCost],
        detect_pattern![
            r"アップ状態のルリグ(を好きな数|1体を)ダウンする",
            CardFeature::RligDownCost
        ],
        detect_pattern![
            r"アップ状態のルリグ\d+体をダウンしてもよい",
            CardFeature::RligDownCost
        ],
        detect_pattern![
            r"このルリグはあなたのルリグトラッシュにあるレベル3の\<.+\>と同じカード名としても扱い、そのルリグの【(自|常)】能力を得る。",
            CardFeature::Inherit
        ],
        detect_pattern![r"グロウするためのコスト", CardFeature::PreventGrowCost],
        detect_pattern![
            r"シグニを\d+枚まで対象とし、それを場に出す",
            CardFeature::PutSigniDefense,
            CardFeature::PutSigniOffense
        ],
        detect_pattern![
            r"あなたのトラッシュにスペルが\d+枚以上あるかぎり",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"(あなた|いずれかのプレイヤー)がスペルを使用したとき、",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターン、(あなたが次に|次にあなたが)スペルを使用する場合",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターンに(あなた|対戦相手)がスペルを使用していた場合、",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"《ディソナアイコン》のスペルを使用したとき、",
            CardFeature::OnSpell
        ],
        detect_pattern![r"のアーツを使用していた場合", CardFeature::OnArts],
        detect_pattern![
            r"あなたのルリグトラッシュにあるアーツ1枚につき",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"このアーツを使用する際、あなたのルリグデッキから.のアーツ1枚をルリグトラッシュに置いてもよい。",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"このゲームの間にあなたがリレーピースを使用している",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"あなたのルリグデッキにあるピース1枚をゲームから除外する",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"ピースを使用する際、カットインして使用できる",
            CardFeature::OnArts
        ],
        // detect_pattern![ // 同上・特定の1枚のみに同時に存在する条件
        //     r"対戦相手のピース1枚を対象とし",
        //     CardFeature::OnArts
        // ],
        detect_pattern![
            r"このターンにあなたがピースを使用していた場合",
            CardFeature::OnArts
        ],
        detect_pattern![r"【ライフバースト】", CardFeature::LifeBurst],
        detect_pattern![
            r"このカードが【ソウル】として付いているシグニ",
            CardFeature::Soul
        ],
        detect_pattern![
            r"このルリグの下からカード１枚をそれの【ソウル】にする",
            CardFeature::Soul
        ],
        detect_pattern![r"【ソウル】が付いているあなたのシグニ", CardFeature::Soul],
        detect_pattern![
            r"あなたのルリグトラッシュからルリグ１枚をそれの【ソウル】にする",
            CardFeature::Soul
        ],
        detect_pattern![r"あなたの場に【ソウル】があり", CardFeature::Soul],
        detect_pattern![
            r"このシグニ(に|は)【ソウル】が付いている(場合|かぎり)",
            CardFeature::Soul
        ],
        detect_pattern![
            r"あなたのシグニ１体に【ソウル】が付いたとき",
            CardFeature::Soul
        ],
        detect_pattern![r"このシグニに【ソウル】が付いたとき", CardFeature::Soul],
        detect_pattern![r"<プリパラ>", CardFeature::Pripara],
        detect_pattern![r"<電音部>", CardFeature::Denonbu],
        detect_pattern![r"<ブルアカ>", CardFeature::BlueArchive],
        detect_pattern![r"<バーチャル>", CardFeature::Nijisanji],
    ];

    (r_patterns, d_patterns)
}

// BurstFeature検出パターンを作成する関数
pub fn create_burst_detect_patterns() -> (Vec<BurstReplacePattern>, Vec<BurstDetectPattern>) {
    let r_patterns = vec![
        // 基本的な置換パターン（今後拡張予定）
        burst_replace_pattern![r"ライフバースト:", "LB:"],
        burst_replace_pattern![r"\(パワーが0以下のシグニはルールによってバニッシュされる\)", ""],
        burst_replace_pattern![r"\(凍結されたシグニは次の自分のアップフェイズにアップしない\)", "", Freeze],
        burst_replace_pattern![r"\(あなたのデッキの上からカードを\d+枚エナゾーンに置く\)", "", Charge],
        burst_replace_pattern![r"\(あなたのデッキの一番上のカードをエナゾーンに置く\)", "", Charge],
        burst_replace_pattern![r"\(《ガードアイコン》を持つシグニは【ガード】を得る\)", "", Guard],
    ];

    let d_patterns = vec![
        burst_detect_pattern![r"【エナチャージ\d+】", Charge],
        burst_detect_pattern![r"【エナチャージ1】をする。このターン、あなたは対戦相手のレベル3以下のシグニによってダメージを受けない。", Charge, BlockSigni],
        burst_detect_pattern![r"【エナチャージ1】をする。このターン、次にあなたがシグニからダメージを受ける場合、代わりにダメージを受けない。", Charge, BlockSigni],
        burst_detect_pattern![r"【エナチャージ1】をする。このターン、次にあなたがシグニによってダメージを受ける場合、代わりにダメージを受けない。", Charge, BlockSigni],
        burst_detect_pattern![r"【エナチャージ1】をする。このターン、次にあなたがルリグによってダメージを受ける場合、代わりにダメージを受けない。", Charge, BlockLrig],
        burst_detect_pattern![r"【エナチャージ1】をする。このターン、次にシグニがアタックしたとき、そのアタックを無効にする。", Charge, BlockSigni],
        burst_detect_pattern![r"【エナチャージ1】をする。その後、あなたのエナゾーンからシグニを1枚まで対象とし、それを手札に加えるか場に出す。", Charge, Draw, BlockSigni, Defend1, Guard],
        burst_detect_pattern![r"【エナチャージ2】をする。その後、あなたのエナゾーンからシグニを1枚まで対象とし、それを手札に加える。", Charge, Draw, Guard],
        burst_detect_pattern![r"【エナチャージ1】をする。その後、あなたのエナゾーンから.*のシグニを1枚まで対象とし、それを手札に加えるか場に出す。", Charge, Draw, BlockSigni, Defend1],
        burst_detect_pattern![r"あなたのデッキからスペル1枚を探して公開し手札に加え、デッキをシャッフルする。", Search, Draw],
        burst_detect_pattern![r"あなたのデッキから.*のシグニ1枚を探して公開し手札に加え、デッキをシャッフルする。", Search, Draw],
        burst_detect_pattern![r"あなたのデッキから.*のシグニ1枚を探して公開し手札に加え、デッキをシャッフルする。", Search, Draw],
        burst_detect_pattern![r"あなたのデッキから.*のシグニ1枚を探して公開し手札に加えるか場に出し、デッキをシャッフルする。", Search, Draw, Defend1],
        burst_detect_pattern![r"あなたのデッキから.*のシグニ1枚を探して公開し手札に加え、デッキをシャッフルする。", Search, Draw],
        burst_detect_pattern![r"あなたのデッキの一番上と一番下を見る。その中からシグニを1枚まで場に出し、残りを手札に加える。", Search, Draw, Defend1],
        burst_detect_pattern![r"あなたのデッキの一番上のカードをエナゾーンに置く。その後、あなたのエナゾーンからカードを1枚まで対象とし、それを手札に加える。", Charge, Guard, Draw],
        burst_detect_pattern![r"あなたのデッキの上からカードを10枚見る。その中からカードを1枚まで手札に加え、残りをデッキに加えてシャッフルする。", Search, Guard, Draw],
        burst_detect_pattern![r"あなたのデッキの上からカードを3枚トラッシュに置く。その後、あなたのトラッシュから無色ではないシグニ1枚を対象とし、それを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのデッキの上からカードを3枚見る。その中からシグニを2枚まで公開し手札に加え、残りを好きな順番でデッキの一番下に置く。", Search],
        burst_detect_pattern![r"あなたのデッキの上からカードを3枚見る。その中からシグニ1枚を公開し手札に加えるか場に出し、残りを好きな順番でデッキの一番下に置く。", Search, Guard, Draw, Defend1],
        burst_detect_pattern![r"あなたのデッキの上からカードを5枚見る。その中からカードを3枚まで手札に加え、残りを好きな順番でデッキの一番下に置く。", Search, Guard, Draw],
        burst_detect_pattern![r"あなたのデッキの枚数10枚につきカードを１枚引く。", Draw],
        burst_detect_pattern![r"あなたのデッキをシャッフルし一番上のカードをライフクロスに加える。", Heal, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから、対象のレベル２の.*のシグニ1枚を手札に加えて対象のレベル1の.*のシグニ1枚を場に出す。", Defend1],
        burst_detect_pattern![r"あなたのトラッシュから《ガードアイコン》を持たないシグニを2枚まで対象とし、それらを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから《ガードアイコン》を持たないシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから《ガードアイコン》を持たないレベル2以下のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから《ガードアイコン》を持つシグニ1枚を対象とし、それを手札に加える。カードを１枚引く。", Salvage, Guard, Draw],
        burst_detect_pattern![r"あなたのトラッシュから《ディソナアイコン》のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから【ライフバースト】を持たないカード1枚を対象とし、それをライフクロスに加える。", Heal, Defend1],
        burst_detect_pattern![r"あなたのトラッシュからカード1枚を対象とし、それを手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"あなたのトラッシュからシグニとスペルをそれぞれ1枚まで対象とし、それらを手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"あなたのトラッシュからシグニを2枚まで対象とし、それらを手札に加える。手札を1枚捨てる。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュからシグニ1枚を対象とし、それを手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"あなたのトラッシュからシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュからシグニ1枚を手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"あなたのトラッシュからスペルを2枚まで対象とし、それらを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ１枚を対象とし、それを場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニを2枚まで対象とし、それらを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加える。対戦相手のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-3000する。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニを2枚まで対象とし、それらを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加えるか場に出す。", Salvage, Defend1],
        burst_detect_pattern![r"あなたのトラッシュから.*のシグニ1枚を対象とし、それを手札に加える。", Salvage],
        burst_detect_pattern![r"あなたの場に白のルリグが2体以上いる場合、対戦相手のパワー10000以下のシグニ1体を対象とし、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"あなたの手札が4枚以上ある場合、対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"このターンにアタックした対戦相手のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend],
        burst_detect_pattern![r"どちらか1つを選ぶ。?好きな生徒1人との絆を獲得する。?あなたのトラッシュからシグニ1枚を対象とし、それを手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"どちらか1つを選ぶ。?対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。?【エナチャージ1】", OffenciveDefend, Defend1, Charge],
        burst_detect_pattern![r"どちらか1つを選ぶ。?対戦相手のアップ状態のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-15000する。?カードを1枚引く。", OffenciveDefend, Draw, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。?対戦相手のシグニを2体まで対象とし、それらをダウンする。?カードを1枚引く。", BlockSigni, Defend2],
        burst_detect_pattern![r"どちらか1つを選ぶ。①カードを1枚引く。②【エナチャージ1】", Draw, Charge],
        burst_detect_pattern![r"どちらか1つを選ぶ。①カードを1枚引く。②対戦相手のアップ状態のシグニ1体を対象とし、それを手札に戻す。", Draw, OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①カードを2枚引く。②あなたのトラッシュから《ガードアイコン》を持たないシグニ1枚を対象とし、それを手札に加えるか場に出す。", Draw, Salvage, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①ターン終了時まで、あなたのすべてのシグニのパワーを\+10000する。②あなたのデッキの上からカードを3枚見る。その中からシグニ1枚を公開し手札に加えるか場に出し、残りを好きな順番でデッキの一番下に置く。",
            Draw, Salvage, Defend1
        ],
        burst_detect_pattern![r"どちらか1つを選ぶ。①ターン終了時まで、対戦相手のすべてのシグニは能力を失う。②カードを2枚引く。", Draw, BurstFeature::EraseSkill],
        burst_detect_pattern![r"どちらか1つを選ぶ。①好きな生徒1人との絆を獲得する。②あなたのトラッシュからシグニ1枚を対象とし、それを手札に加える。", Salvage, Guard],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。②【エナチャージ1】", Charge, OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。②【エナチャージ1】", Charge, OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。②あなたか対戦相手のデッキの上からカードを4枚トラッシュに置く。", OffenciveDefend, Defend1], // todo new skill?
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。②カードを1枚引く。", Draw, OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それを手札に戻す。②あなたのデッキの上からカードを4枚見る。その中からカードを2枚まで手札に加え、残りを好きな順番でデッキの一番下に置く。", OffenciveDefend, Defend1, Draw],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それを手札に戻す。②カードを1枚引く。", OffenciveDefend, Defend1, Draw],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、それを手札に戻す。②対戦相手の手札を1枚見ないで選び、捨てさせる。", OffenciveDefend, Defend1, Discard],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のアップ状態のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-15000する。②カードを1枚引く。", OffenciveDefend, Draw, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のシグニを2体まで対象とし、それらをダウンする。②カードを1枚引く。", BlockSigni, Defend2, Draw],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のシグニ1体を対象とし、それをダウンする。対戦相手は手札を1枚捨てる。②カードを2枚引く。", BlockSigni, Defend1, Discard, Draw],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-5000する。②対戦相手のシグニ1体を対象とし、《コインアイコン》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-12000する。",
            OffenciveDefend, Defend1
        ],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-8000する。②あなたのトラッシュから《ガードアイコン》を持たないシグニ1枚を対象とし、それを手札に加える。", OffenciveDefend, Salvage, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のシグニ1体を対象とし、手札を2枚捨ててもよい。そうした場合、それをバニッシュする。②対戦相手の手札を1枚見ないで選び、捨てさせる。", OffenciveDefend, Discard, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のパワー12000以上のシグニ1体を対象とし、それをバニッシュする。②対戦相手のパワー5000以上のシグニ1体を対象とし、《コインアイコン》を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のパワー3000以下のシグニ1体を対象とし、それをデッキの一番下に置く。②対戦相手のパワー8000以下のシグニ1体を対象とし、《コインアイコン》を支払ってもよい。そうした場合、それをデッキの一番下に置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のパワー5000以下のシグニ1体を対象とし、それをバニッシュする。②対戦相手のシグニ1体を対象とし、ターン終了時まで、それは「【常】：アタックできない。」を得る。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のパワー5000以下のシグニ1体を対象とし、それをバニッシュする。②対戦相手のパワー12000以下のシグニ1体を対象とし、《コインアイコン》を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のパワー5000以下のシグニ1体を対象とし、それを手札に戻す。②対戦相手のパワー12000以下のシグニ1体を対象とし、《コインアイコン》を支払ってもよい。そうした場合、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"どちらか1つを選ぶ。①対戦相手のレベル1のシグニ1体を対象とし、それをバニッシュする。②【エナチャージ2】", OffenciveDefend, Defend1, Charge],
        burst_detect_pattern![r"どちらか1つ選ぶ。①カードを1枚引く。②【エナチャージ１】", Draw, Charge],
        burst_detect_pattern![r"カードを1枚引き、【エナチャージ1】をする。", Draw, Charge],
        burst_detect_pattern![r"カードを1枚引き【エナチャージ2】をする。", Draw, Charge],
        burst_detect_pattern![r"カードを1枚引く。", Draw],
        burst_detect_pattern![r"カードを1枚引く。このターン、あなたの手札にあるシグニは《ガードアイコン》を得る。", Draw, Guard],
        burst_detect_pattern![r"カードを1枚引く。その後、あなたのライフクロス1枚を手札に加えてもよい。そうした場合、あなたの手札を1枚ライフクロスに加える。", Draw, Heal],
        burst_detect_pattern![r"カードを1枚引く。対戦相手は手札を1枚捨てる。", Draw, Discard],
        burst_detect_pattern![r"カードを2枚引く。対戦相手は手札を1枚捨てる。", Draw, Discard],
        burst_detect_pattern![r"カードを2枚引き【エナチャージ1】をする。", Draw, Charge],
        burst_detect_pattern![r"カードを2枚引く。", Draw],
        burst_detect_pattern![r"カードを2枚引く。あなたの手札から.+のシグニを1枚まで場に出す。", Draw, Defend1],
        burst_detect_pattern![r"カードを2枚引く。このターン、あなたの手札にあるシグニは《ガードアイコン》を得る。", Draw, Guard],
        burst_detect_pattern![r"カードを2枚捨て、カードを3枚引く。", Draw],
        burst_detect_pattern![r"カードを3枚引き、手札を1枚捨てる。", Draw],
        burst_detect_pattern![r"カードを3枚引く。", Draw],
        burst_detect_pattern![r"ターン終了時まで、対戦相手のすべてのシグニは能力を失う。カードを1枚引く。", Draw, EraseSkill],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、それをデッキの一番下に置く。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、それをトラッシュに置く。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、それを手札に戻す。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-15000する。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のアップ状態のシグニ1体を対象とし、手札を1枚捨ててもよい。そうした場合、それをデッキの一番下に置く。", OffenciveDefend],
        burst_detect_pattern![r"対戦相手のエナゾーンからカード1枚を対象とし、それをトラッシュに置く。カードを1枚引く。", Draw],   // todo new skill?
        burst_detect_pattern![r"対戦相手のシグニを2体まで対象とし、それらをダウンする。", BlockSigni, Defend2],
        burst_detect_pattern![r"対戦相手のシグニを2体まで対象とし、それらをダウンする。カードを1枚引く。", BlockSigni, Draw, Defend2],
        burst_detect_pattern![r"対戦相手のシグニを2体まで対象とし、ターン終了時まで、それらは「【常】：アタックできない。」を得る。カードを1枚引く。", Draw, Defend2],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、.*を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《無》を支払ってもよい。そうした場合、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《無》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-12000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《無》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-15000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《青》《青》を支払ってもよい。そうした場合、それをデッキの一番下に置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《黒》《無》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-12000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《黒》《無》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-12000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、《黒》を支払ってもよい。そうした場合、ターン終了時まで、それのパワーを\-8000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、それをエナゾーンに置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、それをダウンし凍結する。カードを１枚引く。", Defend1, BlockSigni, Freeze, Draw],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、それをダウンし凍結する。対戦相手の手札を1枚(捨てる|見ないで選び、捨てさせる)。", Defend1, BlockSigni, Freeze, Discard],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、ターン終了時まで、それのパワーをあなたのトラッシュにある黒のカード1枚につき\-1000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、ターン終了時まで、それのパワーを\-\d+する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、ターン終了時まで、それは「【常】:アタックできない。」を得る。カードを１枚引く。", Defend1, BlockSigni, Draw],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、対戦相手が《無》《無》を支払わないかぎり、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、手札を1枚捨ててもよい。そうした場合、ターン終了時まで、それのパワーを\-12000する。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、手札を2枚捨ててもよい。そうした場合、それをデッキの一番下に置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のシグニ1体を対象とし、手札を2枚捨ててもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のセンタールリグかシグニ1体を対象とし、ターン終了時まで、それは「【常】:アタックできない。」を得る。", Defend1, BlockSigni, BlockLrig],
        burst_detect_pattern![r"対戦相手のセンタールリグとすべてのシグニをダウンする。", Defend2, BlockSigni, BlockLrig],
        burst_detect_pattern![r"対戦相手のパワー10000以下のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー10000以下のシグニ1体を対象とし、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー10000以下のシグニ1体を対象とし、対戦相手が《無》《無》《無》を支払わないかぎり、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー12000以下のシグニ1体を対象とし、.+を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー12000以下のシグニ1体を対象とし、手札を１枚捨ててもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー5000以上のシグニ1体を対象とし、《無》を支払ってもよい。そうした場合、それをエナゾーンに置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー7000以上のシグニ1体を対象とし、《無》を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー7000以上のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー8000以下のシグニ1体を対象とし、《白》を支払ってもよい。そうした場合、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー8000以下のシグニ1体を対象とし、《赤》を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー8000以下のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー8000以下のシグニ1体を対象とし、それを手札に戻す。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー10000以下のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー12000以下のシグニ1体を対象とし、《赤》《無》を支払ってもよい。そうした場合、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー13000以下のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のパワー5000以上のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のルリグかシグニ1体を対象とする。このターン、それがアタックしたとき、対戦相手が手札を３枚捨てないかぎり、そのアタックを無効にする。", Defend1, BlockSigni, BlockLrig],
        burst_detect_pattern![r"対戦相手のルリグ1体と対戦相手のシグニ1体を対象とし、それらを凍結する。", Freeze, Defend1, Defend2, BlockLrig, BlockSigni],
        burst_detect_pattern![r"対戦相手のルリグ\d+体を対象とし、それをダウンする。", BlockLrig, Defend1],
        burst_detect_pattern![r"対戦相手のルリグ1体を対象とし、それを凍結する。", Freeze],
        burst_detect_pattern![r"対戦相手のルリグ1体を対象とし、ターン終了時まで、それは「【常】:アタックできない。」を得る。", BlockLrig, Defend1],
        burst_detect_pattern![r"対戦相手のレベル2以上のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のレベル2以下のシグニ1体を対象とし、それをデッキの一番下に置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のレベル2以下のシグニ1体を対象とし、それをトラッシュに置く。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手のレベル2以下のシグニ1体を対象とし、それをバニッシュする。", OffenciveDefend, Defend1],
        burst_detect_pattern![r"対戦相手の手札を見て1枚選び、捨てさせる。", Discard],
        burst_detect_pattern![r"対戦相手は自分のシグニ1体を選びデッキの一番下に置く。", OffenciveDefend, Defend1, BlockSigni],
        burst_detect_pattern![r"対戦相手は自分のシグニ1体を選びトラッシュに置く。", OffenciveDefend, Defend1, BlockSigni],
        burst_detect_pattern![r"手札を1枚捨て、カードを3枚引く。\(手札を捨てられなくてもカードを引ける\)", Draw],
        burst_detect_pattern![r"数字1つを宣言する。あなたのデッキの上からカードを5枚公開する。その中から宣言した数字と同じレベルを持つシグニを２枚まで手札に加え、残りをシャッフルしてデッキの一番下に置く。", Draw],

        // selectエラー
        // burst_detect_pattern![r"あなたのアタックフェイズ開始時、あなたの場に<水獣>のシグニがある場合、カードを1枚引き、対戦相手のデッキの一番上を公開する。公開したそのカードが【ライフバースト】を持つ場合、カードを１枚引く。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》リクエスト《青×0》：このターンと次のターンの間、あなたのすべての領域にあるカードは【ライフバースト】「カードを１枚引く。その後、対戦相手のシグニ１体を対象とし、手札を２枚捨ててもよい。そうした場合、それをダウンする。」を得る。"],
        // burst_detect_pattern![r"あなたのシグニ1体を対象とし、それをバニッシュする。そうした場合、あなたのデッキから【ライフバースト】を持つ無色ではないカード1枚を探して公開し手札に加え、デッキをシャッフルする。"],
        // burst_detect_pattern![r"あなたのメインフェイズの間、あなたのレベル２以下の＜悪魔＞のシグニ１体がコストか効果によって場からトラッシュに置かれたとき、あなたのトラッシュからそのシグニより高いレベルを持つ＜悪魔＞のシグニを１枚まで対象とし、それを場に出す。"],
        // burst_detect_pattern![r"あなたのライフクロスとチェックゾーンにある【ライフバースト】を持たないカードは【ライフバースト】「どちらか１つを選ぶ。①カードを１枚引く。②【エナチャージ１】」を得る。"],
        // burst_detect_pattern![r"あなたのルリグ1体を対象とし、ターン終了時まで、それは「【自】《ターン1回》：このルリグがアタックしたとき、あなたのシグニを2体まで場からトラッシュに置く。その後、あなたのトラッシュからこの方法でトラッシュに置いたシグニ1体につき【ライフバースト】を持たないカード１枚を対象とし、それらをライフクロスに加える。」を得る。"],
        // burst_detect_pattern![r"このシグニがアタックしたとき、あなたの場に《ちより　第三章》がいる場合、このシグニと同じシグニゾーンにある【マジックボックス】１つを表向きにしトラッシュに置いてもよい。その後、そのカードが【ライフバースト】を持つ場合、対戦相手のシグニ１体を対象とし、それをバニッシュする。【ライフバースト】を持たない場合、このアタックを無効にし、対戦相手が《無》《無》《無》《無》《無》を支払わないかぎり、対戦相手にダメージを与える。"],
        // burst_detect_pattern![r"このシグニがアタックしたとき、このシグニと同じシグニゾーンにある【マジックボックス】1つを表向きにしトラッシュに置いてもよい。そのカードが【ライフバースト】を持つ場合、ターン終了時まで、このシグニは【ランサー（パワー5000以下のシグニ）】を得る。【ライフバースト】を持たない場合、このアタックを無効にし、【エナチャージ３】をする。"],
        // burst_detect_pattern![r"このシグニがアタックしたとき、このシグニと同じシグニゾーンにある【マジックボックス】1つを表向きにしトラッシュに置いてもよい。その後、そのカードが【ライフバースト】を持つ場合、対戦相手のパワー5000以下のシグニ１体を対象とし、それをバニッシュする。【ライフバースト】を持たない場合、このアタックを無効にし、対戦相手のエナゾーンから対戦相手のセンタールリグと共通する色を持たないカードを３枚まで対象とし、それらをトラッシュに置く。"],
        // burst_detect_pattern![r"このシグニがアタックしたとき、このシグニと同じシグニゾーンにある【マジックボックス】1つを表向きにしトラッシュに置いてもよい。その後、そのカードが【ライフバースト】を持つ場合、対戦相手のパワー8000以下のシグニ１体を対象とし、それをバニッシュする。【ライフバースト】を持たない場合、このアタックを無効にし、あなたのルリグ１体を対象とし、ターン終了時まで、それは「【常】：対戦相手は追加で《無》《無》《無》を支払わないかぎり【ガード】ができない。」を得る。"],
        // burst_detect_pattern![r"このシグニのパワーはあなたの手札2枚につき\+1000される。"],
        // burst_detect_pattern![r"このターン、1枚目と2枚目にあなたのチェックゾーンに置かれたライフクロスは【ライフバースト】「どちらか1つを選ぶ。①対戦相手のシグニ1体を対象とし、それをダウンする。②カードを2枚引く。」を得る。"],
        // burst_detect_pattern![r"このルリグはあなたのルリグトラッシュにあるレベル3の＜アルフォウ＞と同じカード名としても扱い、そのルリグの【自】能力を得る。"],
        // burst_detect_pattern![r"ターン終了時まで、このルリグは「【自】：対戦相手のシグニかルリグ１体がアタックしたとき、あなたと対戦相手は自分のデッキの一番上を公開し、そのカードをデッキの一番下に置く。この方法で公開されたカードがどちらも【ライフバースト】を持っているか、どちらも【ライフバースト】を持っていない場合、そのアタックを無効にする。」を得る。"],
        // burst_detect_pattern![r"【出】《緑》《無》《無》《無》：あなたのトラッシュから【ライフバースト】を持たないカード１枚を対象とし、それをライフクロスに加える。"],
        // burst_detect_pattern![r"【出】《黒》《無》《無》《無》：あなたのトラッシュから【ライフバースト】を持たないカード１枚を対象とし、それをライフクロスに加える。"],
        // burst_detect_pattern![r"【絆出】：あなたのトラッシュから＜ブルアカ＞のカード１枚を対象とし、それを手札に加える。"],
        // burst_detect_pattern![r"【起】《ターン１回》《黒×0》：１～３の数字１つを宣言する。あなたのデッキの上からカードを宣言した数字に等しい枚数トラッシュに置く。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》手札を２枚捨てる：ターン終了時まで、このルリグは「【自】《ターン１回》：このルリグがアタックしたとき、あなたのトラッシュから【ライフバースト】を持たないカード１枚を対象とし、それをライフクロスに加える。」を得る。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》デザイア　シグニ１体を場からトラッシュに置く：あなたのトラッシュから【ライフバースト】を持たないカード１枚を対象とし、それをライフクロスに加える。"],
        // burst_detect_pattern![r"【出】：対戦相手のライフクロスの一番上を公開する。あなたはそのカードと対戦相手のデッキの一番上のカードを入れ替えてもよい。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》《青×0》：あなたのライフクロス１枚をクラッシュする。そうした場合、あなたのデッキをシャッフルし一番上のカードをライフクロスに加える。"],
        // burst_detect_pattern![r"【出】：場に他の＜トリック＞のシグニがない場合、このシグニをダウンする。"],
        // burst_detect_pattern![r"【自】：このシグニがアタックしたとき、あなたのデッキの上からカードを３枚見る。その中から【ライフバースト】を持つカードを好きな枚数公開し手札に加え、残りを好きな順番でデッキの一番下に置く。"],
        // burst_detect_pattern![r"【出】エクシード４：あなたか対戦相手のデッキの上からカードを６枚トラッシュに置く。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》ジェラシー《黒×0》：《リコレクトアイコン》［４枚以上］あなたのトラッシュから【ライフバースト】を持たないすべてのカードをデッキに加えてシャッフルする。ターン終了時まで、対戦相手のすべてのシグニのパワーをこの方法でデッキに加えたカード１枚につき－1000する。"],
        // burst_detect_pattern![r"【起】《ターン１回》ライフクロス１枚をクラッシュする：カードを２枚引く。"],
        // burst_detect_pattern![r"【起】《ゲーム１回》《赤×0》：あなたのデッキをシャッフルし一番上のカードを公開し手札に加える。その後、あなたのシグニ１体を対象とし、この方法で公開されたカードが【ライフバースト】を持つ場合、ターン終了時まで、それは【アサシン】を得る。公開されたカードが【ライフバースト】を持たない場合、ターン終了時まで、それは【ダブルクラッシュ】を得る。"],
        // burst_detect_pattern![r"【出】《黒》《無》《無》《無》：あなたのトラッシュから【ライフバースト】を持たないカード１枚を対象とし、それをライフクロスに加える。"],
        // burst_detect_pattern![r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）"],
    ];

    (r_patterns, d_patterns)
}
