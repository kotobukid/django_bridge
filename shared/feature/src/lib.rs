use crate::feature::CardFeature;
use regex::Regex;

pub mod feature;

pub struct DetectPattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub do_replace: bool,
    pub replace_to: &'static str,
    pub features_detected: &'static [CardFeature],
}

macro_rules! detect_pattern {
    ($pat:expr, $do_replace:expr, $replace:expr, $($feature:expr),*) => {
        DetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            do_replace: $do_replace,
            replace_to: $replace,
            features_detected: &[$($feature),*],
        }
    };
    // 空の機能リスト用の特殊ケース
    ($pat:expr, $do_replace:expr, $replace:expr) => {
        DetectPattern {
            pattern: $pat,
            pattern_r: Regex::new($pat).unwrap(),
            do_replace: $do_replace,
            replace_to: $replace,
            features_detected: &[],
        }
    };
}

const PATTERNS_AMOUNT: usize = 175;
pub fn create_detect_patterns() -> [DetectPattern; PATTERNS_AMOUNT] {
    let patterns: [DetectPattern; PATTERNS_AMOUNT] = [
        detect_pattern![
            r"『",
            true,
            ""
        ],
        detect_pattern![
            r"ライフバースト：",
            true,
            "LB:",
            CardFeature::LifeBurst
        ],
        detect_pattern![
            r"【ウィルス】",
            false,
            "",
            CardFeature::Virus
        ],
        detect_pattern![
            r"』",
            true,
            ""
        ],
        detect_pattern![
            r"ライフバースト：",
            true,
            "LB:",
            CardFeature::LifeBurst
        ],
        detect_pattern![
            r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）",
            true,
            "*DAMAGE",
            CardFeature::Damage
        ],
        detect_pattern![
            r"（パワーが０以下のシグニはルールによってバニッシュされる）",
            true,
            "*RULE BANISH POWER ZERO*",
            CardFeature::PowerDown
        ],
        detect_pattern![
            r"（アタックによるダメージでライフクロスを２枚クラッシュする）",
            true,
            "*DOUBLE CRUSH*"
        ],
        detect_pattern![
            r"（２枚以下の場合、それらをすべて選ぶ）",
            true,
            "*TARGET ALL OVER*"
        ],
        detect_pattern![
            r"（このシグニが場に出たとき、あなたのアップ状態の.+をダウンしないかぎり、これをダウンする）",
            true,
            "*HARMONY*"
        ],
        detect_pattern![
            r"【ハーモニー】",
            false,
            "",
            CardFeature::Harmony
        ],
        detect_pattern![
            r"【ウィルス】",
            false,
            "",
            CardFeature::Virus
        ],
        detect_pattern![
            r"（【ウィルス】と同じシグニゾーンにあるシグニは感染状態である）",
            true,
            "*VIRUS*",
            CardFeature::Virus
        ],
        detect_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ダブルクラッシュ】を持つシグニがアタックによってダメージを与えた場合ライフクロスを１枚ではなく２枚クラッシュする）",
            true,
            "*DOUBLE CRUSH && ASSASSIN*"
        ],
        detect_pattern![
            r"（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            true,
            "*LANCER*",
            CardFeature::Lancer
        ],
        detect_pattern![
            r"（【Ｓランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスがある場合はそれを１枚クラッシュする。無い場合は対戦相手にダメージを与える）",
            true,
            "*S LANCER*",
            CardFeature::SLancer
        ],
        detect_pattern![
            r"（このクラフトは効果以外によっては場に出せない）",
            true,
            "*NO STANDARD PUT*",
            CardFeature::Craft
        ],
        detect_pattern![
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            true,
            "*SPELL CRAFT*",
            CardFeature::Craft
        ],
        detect_pattern![
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            true,
            "*SPELL CRAFT GOES REMOVED*",
            CardFeature::Craft
        ],
        detect_pattern![
            r"（《ガードアイコン》を持つシグニは【ガード】を得る）",
            true,
            "*GUARD*",
            CardFeature::Guard
        ],
        detect_pattern![
            r"（複数の【出】能力は好きな順番で発動できる）",
            true,
            "*MULTIPLE CIP*"
        ],
        detect_pattern![
            r"（この条件を満たさなければ場に出せない）",
            true,
            "*RISE LIMITATION*"
        ],
        detect_pattern![
            r"（【チャーム】は裏向きでシグニに付き、１体に１枚までしか付けられない）",
            true,
            "*CHARM*"
        ],
        detect_pattern![
            r"（【ソウル】はシグニに１枚まで付き、そのシグニが場を離れるとルリグトラッシュに置かれる）",
            true,
            "*SOUL*"
        ],
        detect_pattern![
            r"（【チャーム】や【アクセ】、【ソウル】はシグニに付く）",
            true,
            "*CHARM/ACCE/SOUL BELONGS TO SIGNI*"
        ],
        detect_pattern![
            any_num!["（デッキが", "枚以下の場合は置き換えられない）"],
            true,
            "*FEATURE LIMIT DECK DROP*"
        ],
        detect_pattern![
            r"（このカードを手札から捨てることで、ルリグのアタックによるダメージを一度防ぐ）",
            true,
            "*GUARD*"
        ],
        detect_pattern![
            r"（シグニの下に置かれたカードは、そのシグニが場を離れるとルールによってトラッシュに置かれる）",
            true,
            "*GO TO TRASH TOGETHER*"
        ],
        detect_pattern![
            r"（この能力はこのカードがトラッシュにある場合にしか使用できない）",
            true,
            "*ONLY AVAILABLE IN TRASH*"
        ],
        detect_pattern![
            r"（あなたの場に＜.+＞のルリグ３体がいるなら【チーム自】が有効になる）",
            true,
            "*TEAM SKILL*"
        ],
        detect_pattern![
            r"（このスペルを使用する際、使用コストとして追加で.+を支払ってもよい）",
            true,
            "*BET*"
        ],
        detect_pattern![
            r"（【マジックボックス】はシグニゾーン１つにつき１つまで裏向きで設置できる）",
            true,
            "*MAGIC BOX*"
        ],
        detect_pattern![
            r"（【マジックボックス】はシグニゾーン１つにつき１つまで裏向きで設置できる。すでに【マジックボックス】のあるシグニゾーンに設置する場合、元からある【マジックボックス】をトラッシュに置いてから設置する）",
            false,
            "*MAGIC BOX ON BOX*"
        ],
        detect_pattern![
            r"アクセ",
            false,
            "*ACCE*",
            CardFeature::Acce
        ],
        detect_pattern![
            r"（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）",
            true,
            "*ACCE*",
            CardFeature::Acce
        ],
        detect_pattern![
            r"（シグニのパワーを計算する場合、先に基本パワーを適用してプラスやマイナスをする）",
            true,
            "*CALC ORDER*"
        ],
        detect_pattern![
            any_num![
                    "（あなたのルリグの下からカードを合計",
                    "枚ルリグトラッシュに置く）"
            ],
            true,
            "*EXCEED*",
            CardFeature::Exceed
        ],
        detect_pattern![
            any_num!["エクシード", ""],
            false,
            "*EXCEED*",
            CardFeature::Exceed
        ],
        detect_pattern![
            r"（ピースはあなたの場にルリグが３体いないと使用できない）",
            true,
            "*COMMON PIECE*"
        ],
        detect_pattern![
            r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに(合計|合計で)１枚までしか入れられない）",
            true,
            "*TEAM PIECE*"
        ],
        detect_pattern![
            r"（あなたの場にいるルリグ３体がこの条件を満たす）",
            true,
            "*TEAM*"
        ],
        detect_pattern![
            r"（シグニは覚醒すると場にあるかぎり覚醒状態になる）",
            true,
            "*AWAKE*",
            CardFeature::Awake
        ],
        detect_pattern![
            r"（この能力はこのシグニが場にある場合にしか使用できない）",
            true,
            "*AVAILABLE ONLY IN BATTLEFIELD*"
        ],
        detect_pattern![
            r"（グロウしても新しいセンタールリグは能力を得たままである）",
            true,
            "*IN GAME AVAILABLE*"
        ],
        detect_pattern![
            r"（凍結された(ルリグ|シグニ)は次の自分のアップフェイズにアップしない）",
            true,
            "*FROZEN*"
        ],
        detect_pattern![
            r"（凍結されたルリグとシグニは次の自分のアップフェイズにアップしない）",
            true,
            "*FROZEN*"
        ],
        detect_pattern![
            r"（フェゾーネマジックは５種類ある）",
            true,
            "*FESONE MAGIC*"
        ],
        detect_pattern![
            r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）",
            true,
            "*CIP COST*"
        ],
        detect_pattern![
            r"（ゲームを開始する際に、このルリグを表向きにしたとき、このルリグがセンタールリグであるなら、[《コインアイコン》]+を得る）",
            true,
            "*GAIN COINS ON START*",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"（右下に【コイン】を持つルリグがグロウしたとき、それと同じ枚数の[《コインアイコン》]+を得る）",
            true,
            "*GAIN COINS ON GROW*",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"（ゲームを開始する際に、センタールリグでないルリグを表向きにしても《コインアイコン》を得られない）",
            true,
            "*GAIN NO COINS*"
        ],
        detect_pattern![
            r"（プレイヤーが保持できる《コインアイコン》の上限は５枚である）",
            true,
            "*COIN LIMIT*"
        ],
        detect_pattern![
            r"（すでに場に３体以上ある場合は２体になるようにシグニをトラッシュに置く）",
            true,
            "*SIGNI ZONE RESTRICTION*"
        ],
        detect_pattern![
            r"（コストの合計とは、カードの左上のエナコストの数字の合計である。例えばコストが《白×1》《無×1》の場合、コストの合計は２である）",
            true,
            "*TOTAL COST*"
        ],
        detect_pattern![
            r"（コストのない【出】能力は発動しないことを選べない。.+）",
            true,
            "*MUST APPLY CIP*"
        ],
        detect_pattern![
            r"（コストのない【出】能力は発動しないことを選べない。ライフクロスが１枚の場合その１枚をトラッシュに置く）",
            true,
            "*MUST APPLY CIP*"
        ],
        detect_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            true,
            "*ASSASSIN*"
        ],
        detect_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            true,
            "*ASSASSIN OR LANCER*"
        ],
        detect_pattern![
            r"（このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            true,
            "*SELF ASSASSIN*"
        ],
        detect_pattern![
            r"（表記されているパワーとは、元々それに印刷されている値である）",
            true,
            "*BASIC POWER*"
        ],
        detect_pattern![
            r"（あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】１つを消費し、そのダメージを受けない）",
            true,
            "*LRIG BARRIER*"
        ],
        detect_pattern![
            r"（あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】１つを消費し、そのダメージを受けない）",
            true,
            "*SIGNI BARRIER*"
        ],
        detect_pattern![
            r"（あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】１つを消費し、そのダメージを受けない。あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】１つを消費し、そのダメージを受けない）",
            true,
            "*LRIG/SIGNI BARRIER*"
        ],
        detect_pattern![
            r"（(この|それらの)シグニは.+によって対象にされない）",
            true,
            "*SHADOW*"
        ],
        detect_pattern![
            r"ルリグデッキに加える。（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            false,
            "*CRAFT RESONA*",
            CardFeature::Craft
        ],
        detect_pattern![
            r"（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            true,
            "*RANDOM RESONA MUST BE EXPOSED*"
        ],
        detect_pattern![
            r"（レゾナでありクラフトであるシグニはリムーブできず場を離れるとゲームから除外される）",
            true,
            "*RESONA CANT BE REMOVED*"
        ],
        detect_pattern![
            r"（クラフトであるシグニは場を離れるとゲームから除外される）",
            true,
            "*CRAFT SIGNI REMOVED ON LEAVE*"
        ],
        detect_pattern![
            r"（このクラフトの上にあるシグニが場を離れるとこのクラフトはゲームから除外される）",
            true,
            "*TORAMARU GIMMICK*"
        ],
        detect_pattern![
            r"（チェックゾーンにあるカードはターン終了時にトラッシュに置かれる）",
            true,
            "*CHECK ZONE*"
        ],
        detect_pattern![
            r"（あなたの場にいるルリグが１体で、そのルリグがレベル３以上であるかぎり、そのルリグのリミットを＋２する）",
            true,
            "*LIMIT UPPER EFFECTS*"
        ],
        detect_pattern![
            r"（【リミットアッパー】はあなたのルリグゾーンに１つまでしか置けない）",
            true,
            "*ONLY ONE LIMIT UPPER*"
        ],
        detect_pattern![
            r"（あなたのデッキの一番上のカードをエナゾーンに置く）",
            true,
            "*ENER CHARGE*"
        ],
        detect_pattern![
            r"《コインアイコン》を得る",
            false,
            "*GAIN COINS*",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"ガードアイコン",
            true,
            "*GUARD*",
            CardFeature::Guard
        ],
        detect_pattern![
            r"捨てさせる。",
            false,
            "*HAND DESTRUCTION*",
            CardFeature::DiscardOpponent
        ],
        detect_pattern![
            r"各プレイヤーは手札をすべてエナゾーンに置",
            false,
            "*HAND DESTRUCTION*",
            CardFeature::DiscardOpponent, CardFeature::RandomDiscard
        ],
        detect_pattern![
            r"見ないで選び、捨てさせる。",
            false,
            "*RANDOM HAND DESTRUCTION*",
            CardFeature::RandomDiscard
        ],
        detect_pattern![
            r"ダウンする。",
            false,
            "*DOWN*",
            CardFeature::Down
        ],
        detect_pattern![
            r"エナチャージ",
            false,
            "*CHARGE*",
            CardFeature::Charge
        ],
        detect_pattern![
            any_num!["カードを", "枚までエナゾーンに置"],
            false,
            "*CHARGE MANUALLY*",
            CardFeature::Charge
        ],
        detect_pattern![
            r"残りを好きな順番でデッキの一番下に置く",
            false,
            "*BOTTOM CHECK*",
            CardFeature::BottomCheck
        ],
        detect_pattern![
            r"(それ|シグニ)をトラッシュに置",
            false,
            "*TRASH*",
            CardFeature::Trash
        ],
        detect_pattern![
            r"シグニバリア",
            false,
            "*BARRIER SIGNI*",
            CardFeature::Barrier
        ],
        detect_pattern![
            r"ルリグバリア",
            false,
            "*BARRIER LRIG*",
            CardFeature::Barrier
        ],
        // (r"がアタックしたとき", do_remove: false, "*ON ATTACK*", CardFeature::OnAttack]),
        detect_pattern![
            r"アサシン",
            false,
            "*ASSASSIN*",
            CardFeature::Assassin
        ],
        detect_pattern![
            r"シャドウ",
            false,
            "*SHADOW*",
            CardFeature::Shadow
        ],
        detect_pattern![
            r"【マルチエナ】",
            false,
            "*MULTI ENER*",
            CardFeature::DualColorEner
        ],
        detect_pattern![
            r"（エナコストを支払う際、このカードは好きな色１つとして支払える）",
            true,
            "*ALL COLORED ENER*",
            CardFeature::DualColorEner
        ],
        detect_pattern![
            r"（エナコストを支払う際、このカードは.+１つとして支払える）",
            true,
            "*DUAL COLORED ENER*",
            CardFeature::DualColorEner
        ],
        detect_pattern![
            r"（対戦相手のシグニが【シュート】を持つシグニとのバトルによってバニッシュされる場合、エナゾーンに置かれる代わりにトラッシュに置かれる）",
            true,
            "*SHOOT LIKE*"
        ],
        detect_pattern![
            r"チャーム",
            false,
            "*CHARM*",
            CardFeature::Charm
        ],
        detect_pattern![
            r"ダブルクラッシュ",
            false,
            "*DOUBLE CRUSH*",
            CardFeature::DoubleCrush
        ],
        detect_pattern![
            r"トリプルクラッシュ",
            false,
            "*TRIPLE CRUSH*",
            CardFeature::DoubleCrush // ダブクラと統合
        ],
        detect_pattern![
            r"【シュート】",
            false,
            "*SHOOT LIKE*",
            CardFeature::ShootLike
        ],
        detect_pattern![
            r"エナゾーンに置かれる代わりに(トラッシュ|手札|デッキの一番下)",
            false,
            "*SHOOT LIKE*",
            CardFeature::ShootLike
        ],
        detect_pattern![
            r"【ライズ】あなたの",
            false,
            "RISE",
            CardFeature::Rise
        ],
        detect_pattern![
            r"ベット―",
            false,
            "BET",
            CardFeature::BetCoin
        ],
        detect_pattern![
            r"コインアイコン》：",
            false,
            "BET",
            CardFeature::BetCoin
        ],
        detect_pattern![
            r"Sランサー",
            false,
            "*S LANCER*",
            CardFeature::SLancer, CardFeature::Lancer
        ],
        detect_pattern![
            r"Ｓランサー",
            false,
            "*S LANCER*",
            CardFeature::SLancer, CardFeature::Lancer
        ],
        detect_pattern![
            r"【マジックボックス】",
            false,
            "*MAGIC BOX*",
            CardFeature::MagicBox
        ],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それをゲームから除外する"],
            false,
            "*REMOVE SIGNI*",
            CardFeature::RemoveSigni
        ],
        detect_pattern![
            r"バニッシュ",
            false,
            "*BANISH*",
            CardFeature::Banish
        ],
        detect_pattern![
            r"シグニ.+エナゾーンに置",    //todo: 対戦相手の
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手は自分の.?シグニ１体を選びエナゾーンに置",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のパワー.+以下のシグニ１体を対象とし、それをエナゾーンに置",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num!["対戦相手のシグニを", "体(まで|を)対象とし、(それら|それ)をエナゾーンに置"],
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のすべてのシグニをエナゾーンに置",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num!["対戦相手の.+のシグニ", "体を対象とし、それをエナゾーンに置"],
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"支払ってもよい。そうした場合、(それ|それら)をエナゾーンに置",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"支払っても良い。そうした場合、対戦相手は自分のシグニ１体を選びエナゾーンに置",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のシグニ１体を対象とし、それとこのシグニをエナゾーンに",
            false,
            "*ENER OFFENSIVE*",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"シグニをアップ",
            false,
            "*UP*",
            CardFeature::Up
        ],
        detect_pattern![
            any_num!["シグニ", "体を対象とし、(それ|それら)をアップ"],
            false,
            "*UP*",
            CardFeature::Up
        ],
        detect_pattern![
            r"凍結する",
            false,
            "*FREEZE*",
            CardFeature::Freeze
        ],
        detect_pattern![
            r"それらの場所を入れ替え",
            false,
            "*POSITION*",
            CardFeature::Position
        ],
        detect_pattern![
            r"場に出すことができない",
            false,
            "*LIMIT SIGNI*",
            CardFeature::LimitSigni
        ],
        detect_pattern![
            any_num![
                    "対戦相手のシグニ",
                    "体(まで|を)対象とし、(それら|それ)を手札に戻"
                ],
            false,
            "*BOUNCE*",
            CardFeature::Bounce
        ],
        detect_pattern![
             any_num![
                    "対戦相手のパワー",
                    "体(まで|を)対象とし、(それら|それ)を手札に戻"
                ],
            false,
            "*BOUNCE*",
            CardFeature::Bounce
        ],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それを手札に戻"],
            false,
            "BOUNCE",
            CardFeature::Bounce
        ],
        // (    r"手札に加え", do_remove: false, "*SALVAGE*", CardFeature::Salvage]),
        detect_pattern![
            any_num!["ライフクロス", "枚をトラッシュに置"],
            false,
            "*LIFE TRASH*",
            CardFeature::LifeTrash
        ],
        detect_pattern![
            any_num!["エナゾーンからカード", "枚(を|選び).+トラッシュに置"],
            false,
            "*ENER ATTACK*",
            CardFeature::EnerAttack
        ],
        detect_pattern![
            r"ルリグトラッシュに置",
            false,
            "*LRIG TRASH*",
            CardFeature::LrigTrash
        ],
        // (r"アタックフェイズ開始時", do_remove: false, "*ON ATTACK START*", CardFeature::OnAttackStart]),
        detect_pattern![
            r"ライフクロスに加える",
            false,
            "*ADD LIFE*",
            CardFeature::AddLife
        ],
        detect_pattern![
            r"ランサー",
            false,
            "*LANCER*",
            CardFeature::Lancer
        ],
        detect_pattern![
            r"ライフクロスを１枚クラッシュする",
            false,
            "*CRUSH*",
            CardFeature::LifeCrush
        ],
        detect_pattern![
            r"対戦相手のライフクロス１枚をクラッシュする。",
            false,
            "*CRUSH*",
            CardFeature::LifeCrush
        ],
        detect_pattern![
            r"対戦相手にダメージを与える。",
            false,
            "*DAMAGE*",
            CardFeature::Damage
        ],
        detect_pattern![
            r"クラッシュしたとき、",
            false,
            "*ON CRUSH*",
            CardFeature::OnLifeCrush
        ],
        detect_pattern![
            r"クラッシュされ(る場合|たとき|るかトラッシュ|ていた場合)、",
            false,
            "*ON CRUSH*",
            CardFeature::OnLifeCrush
        ],
        detect_pattern![
            r"リコレクトアイコン",
            false,
            "*RECOLLECT*",
            CardFeature::Recollect
        ],
        detect_pattern![
            "（あなたのルリグトラッシュに[（\u{FF10}-\u{FF19}）]枚以上のアーツがあるかぎり《リコレクトアイコン》［[（\u{FF10}-\u{FF19}）]枚以上］に続く文章が有効になる）",
            true,
            "*RECOLLECT*"
        ],
        detect_pattern![
            any_num![r"", "枚見"],
            false,
            "*SEEK*",
            CardFeature::SeekTop
        ],
        detect_pattern![
            r"デッキの一番上に(戻|置)",
            false,
            "TOP",
            CardFeature::TopSet
        ],
        detect_pattern![
            r"のシグニは能力を失う",
            false,
            "*ERASE SKILL*",
            CardFeature::EraseSkill
        ],
        detect_pattern![
            r"それは能力を失う",
            false,
            "*ERASE SKILL*",
            CardFeature::EraseSkill
        ],
        detect_pattern![
            any_num!["シグニを", "体(まで|を)対象とし、ターン終了時まで、それは能力を失う"],
            false,
            "*ERASE SKILL*",
            CardFeature::EraseSkill
        ],
        detect_pattern![
            r"それを《サーバント　ＺＥＲＯ》にする",
            false,
            "*ERASE SKILL / SERVANT ZERO*",
            CardFeature::EraseSkill
        ],
        detect_pattern![
            r"アタックできない",
            false,
            "*NON ATTACKABLE*",
            CardFeature::NonAttackable
        ],
        detect_pattern![
            any_num!["カードを", "枚引"],
            false,
            "*DRAW*",
            CardFeature::Draw
        ],
        detect_pattern![
            any_num!["デッキの上からカードを", "枚トラッシュに置"],
            false,
            "*DROP*",
            CardFeature::Drop
        ],
        detect_pattern![
            any_num![
                    "対戦相手のエナゾーンからカードを",
                    "枚まで対象とし、それらを手札に戻"
                ],
            false,
            "*ENER ATTACK*",
            CardFeature::EnerAttack
        ],
        detect_pattern![
            r"デッキの一番下に置",
            false,
            "*DECK BOUNCE*",
            CardFeature::DeckBounce
        ],
        detect_pattern![
            r"シグニのパワーを＋",
            false,
            "*POWER UP*",
            CardFeature::PowerUp
        ],
        detect_pattern![
            r"このシグニのパワーは＋",
            false,
            "*POWER UP*",
            CardFeature::PowerUp
        ],
        detect_pattern![
            r"(シグニ|それ|それら)のパワーを＋",
            false,
            "*POWER UP*",
            CardFeature::PowerUp
        ],
        detect_pattern![
            r"(シグニ|それ|それら)のパワーを－",
            false,
            "*POWER DOWN*",
            CardFeature::PowerDown
        ],
        detect_pattern![
            r"(シグニ|それ)のパワーをこの方法で.+－",
            false,
            "*POWER DOWN*",
            CardFeature::PowerDown
        ],
        detect_pattern![
            r"ダメージを受けない",
            false,
            "*CANCEL DAMAGE*",
            CardFeature::CancelDamage
        ],
        detect_pattern![
            r"トラッシュからシグニ.+場に出",
            false,
            "*REANIMATE*",
            CardFeature::Reanimate
        ],
        detect_pattern![
            any_num![
                // あなたのトラッシュから黒のシグニ１枚を対象とし、それを場に出す  // TODO
                    "あなたのトラッシュから(シグニ|.+のシグニ)",
                    "枚を対象とし、それを場に出"
                ],
            false,
            "*REANIMATE*",
            CardFeature::Reanimate
        ],
        detect_pattern![
            r"(この|その)ルリグをアップし",
            false,
            "*ADDITIONAL ATTACK*",
            CardFeature::AdditionalAttack
        ],
        detect_pattern![
            r"対戦相手は【ガード】ができない",
            false,
            "*UNGUARDABLE*",
            CardFeature::UnGuardable
        ],
        detect_pattern![
            any_num!["スペル", "枚を.+手札に加え"],
            false,
            "*SALVAGE SPELL*",
            CardFeature::SalvageSpell
        ],
        detect_pattern![
            any_num![
                    "(シグニ|シグニを|シグニをそれぞれ)",
                    "枚(を|まで).+手札に加え"
                ],
            false,
            "SALVAGE SIGNI",
            CardFeature::Salvage
        ],
        detect_pattern![
            any_num!["スペル", "枚をコストを支払わずに使用する"],
            false,
            "FREE SPELL",
            CardFeature::FreeSpell
        ],
        detect_pattern![
            r"このアーツの使用コストは.+減る",
            false,
            "FREE ARTS",
            CardFeature::FreeArts
        ],
        detect_pattern![
            r"このシグニがアタックしたとき.+バニッシュする",
            false,
            "BANISH ON ATTACK",
            CardFeature::BanishOnAttack
        ],
        detect_pattern![
            r"アタックを無効に",    // todo: 攻防あり
            false,
            "ATTACK NO EFFECT",
            CardFeature::AttackNoEffect
        ],
        detect_pattern![
            r"バニッシュされない",
            false,
            "INVULNERABLE",
            CardFeature::Invulnerable
        ],
        detect_pattern![
            r"バニッシュされたとき",
            false,
            "ON BANISH",
            CardFeature::OnBanish
        ],
        detect_pattern![
            r"(ライフバーストを使用することを選んだ場合|ライフバーストの能力化効果の対象になったとき|ライフバーストアイコン》を持っているか、|ライフバーストアイコン》を持つ場合、|ライフバーストが発動する場合、|ライフバーストは発動しない)",
            false,
            "ON BURST",
            CardFeature::OnBurst
        ],
        detect_pattern![
            r"(エクシードのコストとして|あなたがエクシードのコストを支払ったとき、)",
            false,
            "ON EXCEED",
            CardFeature::OnExceed
        ],
        detect_pattern![
            any_num!["手札を", "枚捨ててもよい"],
            false,
            "*HAND COST*",
            CardFeature::HandCost
        ],
        detect_pattern![
            r"アップ状態のルリグ(を好きな数|１体を)ダウンする",
            false,
            "*ASSIST COST*",
            CardFeature::RligDownCost
        ],
        detect_pattern![
            any_num!["アップ状態のルリグ", "体をダウンしてもよい"],
            false,
            "*ASSIST COST*",
            CardFeature::RligDownCost
        ],
        detect_pattern![
            r"このルリグはあなたのルリグトラッシュにあるレベル３の＜.+＞と同じカード名としても扱い、そのルリグの【(自|常)】能力を得る。",
            false,
            "*Inherit*",
            CardFeature::Inherit
        ],
        detect_pattern![
            r"グロウするためのコスト",
            true,
            "*PREVENT GROW COST*",
            CardFeature::PreventGrowCost
        ],
        detect_pattern![
            any_num!["シグニを", "枚まで対象とし、それを場に出す"],
            false,
            "*PUT BLOCKER*",
            CardFeature::PutSigniDefense, CardFeature::PutSigniOffense
        ],
        detect_pattern![
            any_num!["あなたのトラッシュにスペルが", "枚以上あるかぎり"],
            false,
            "*ON SPELL*",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"(あなた|いずれかのプレイヤー)がスペルを使用したとき、",
            false,
            "ON SPELL",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターン、(あなたが次に|次にあなたが)スペルを使用する場合",
            false,
            "ON SPELL",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターンに(あなた|対戦相手)がスペルを使用していた場合、",
            false,
            "ON SPELL",
            CardFeature::OnSpell
        ],
        detect_pattern![
            r"《ディソナアイコン》のスペルを使用したとき、",
            false,
            "ON SPELL",
            CardFeature::OnSpell
        ]
    ];
    patterns
}