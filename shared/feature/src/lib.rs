use crate::feature::CardFeature;
use regex::Regex;

pub mod feature;

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

const PATTERNS_AMOUNT_R: usize = 68;
const PATTERNS_AMOUNT_D: usize = 124;

pub fn create_detect_patterns() -> (
    [ReplacePattern; PATTERNS_AMOUNT_R],
    [DetectPattern; PATTERNS_AMOUNT_D],
) {
    let r_patterns: [ReplacePattern; PATTERNS_AMOUNT_R] = [
        replace_pattern![
            r"『",
            ""
        ],
        replace_pattern![
            r"ライフバースト：",
            "LB:",
            CardFeature::LifeBurst
        ],
        replace_pattern![
            r"』",
            ""
        ],
        replace_pattern![
            r"ライフバースト：",
            "LB:",
            CardFeature::LifeBurst
        ],
        replace_pattern![
            r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）",
            "",
            CardFeature::Damage
        ],
        replace_pattern![
            r"（パワーが０以下のシグニはルールによってバニッシュされる）",
            "",
            CardFeature::PowerDown
        ],
        replace_pattern![
            r"（アタックによるダメージでライフクロスを２枚クラッシュする）",
            "*DOUBLE CRUSH*"
        ],
        replace_pattern![
            r"（２枚以下の場合、それらをすべて選ぶ）",
            "*TARGET ALL OVER*"
        ],
        replace_pattern![
            r"（このシグニが場に出たとき、あなたのアップ状態の.+をダウンしないかぎり、これをダウンする）",
            "*HARMONY*"
        ],
        replace_pattern![
            r"（【ウィルス】と同じシグニゾーンにあるシグニは感染状態である）",
            "*VIRUS*",
        ],
        replace_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ダブルクラッシュ】を持つシグニがアタックによってダメージを与えた場合ライフクロスを１枚ではなく２枚クラッシュする）",
            "*DOUBLE CRUSH && ASSASSIN*"
        ],
        replace_pattern![
            r"（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            "*LANCER*",
            CardFeature::Lancer
        ],
        replace_pattern![
            r"（【Ｓランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスがある場合はそれを１枚クラッシュする。無い場合は対戦相手にダメージを与える）",
            "*S LANCER*",
            CardFeature::SLancer
        ],
        replace_pattern![
            r"（このクラフトは効果以外によっては場に出せない）",
            "*NO STANDARD PUT*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            "*SPELL CRAFT*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            "*SPELL CRAFT GOES REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            "*RESONA CRAFT REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（《ガードアイコン》を持つシグニは【ガード】を得る）",
            "*GUARD*",
        ],
        replace_pattern![
            r"（複数の【出】能力は好きな順番で発動できる）",
            "*MULTIPLE CIP*"
        ],
        replace_pattern![
            r"（この条件を満たさなければ場に出せない）",
            "*RISE LIMITATION*"
        ],
        replace_pattern![
            r"（【チャーム】は裏向きでシグニに付き、１体に１枚までしか付けられない）",
            "*CHARM*"
        ],
        replace_pattern![
            r"（【ソウル】はシグニに１枚まで付き、そのシグニが場を離れるとルリグトラッシュに置かれる）",
            "*SOUL*"
        ],
        replace_pattern![
            r"（【チャーム】や【アクセ】、【ソウル】はシグニに付く）",
            "*CHARM/ACCE/SOUL BELONGS TO SIGNI*"
        ],
        replace_pattern![
            any_num!["（デッキが", "枚以下の場合は置き換えられない）"],
            "*FEATURE LIMIT DECK DROP*"
        ],
        replace_pattern![
            r"（このカードを手札から捨てることで、ルリグのアタックによるダメージを一度防ぐ）",
            "*GUARD*"
        ],
        replace_pattern![
            r"（シグニの下に置かれたカードは、そのシグニが場を離れるとルールによってトラッシュに置かれる）",
            "*GO TO TRASH TOGETHER*"
        ],
        replace_pattern![
            r"（この能力はこのカードがトラッシュにある場合にしか使用できない）",
            "*ONLY AVAILABLE IN TRASH*"
        ],
        replace_pattern![
            r"（あなたの場に＜.+＞のルリグ３体がいるなら【チーム自】が有効になる）",
            "*TEAM SKILL*"
        ],
        replace_pattern![
            r"（このスペルを使用する際、使用コストとして追加で.+を支払ってもよい）",
            "*BET*"
        ],
        replace_pattern![
            r"（【マジックボックス】はシグニゾーン１つにつき１つまで裏向きで設置できる）",
            "*MAGIC BOX*"
        ],
        replace_pattern![
            r"（【マジックボックス】はシグニゾーン１つにつき１つまで裏向きで設置できる。すでに【マジックボックス】のあるシグニゾーンに設置する場合、元からある【マジックボックス】をトラッシュに置いてから設置する）",
            "*MAGIC BOX ON BOX*"
        ],
        replace_pattern![
            r"（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）",
            "*ACCE*"
        ],
        replace_pattern![
            r"（シグニのパワーを計算する場合、先に基本パワーを適用してプラスやマイナスをする）",
            "*CALC ORDER*"
        ],
        replace_pattern![
            r"（ピースはあなたの場にルリグが３体いないと使用できない）",
            "*COMMON PIECE*"
        ],
        replace_pattern![
            any_num![
                    "（あなたのルリグの下からカードを合計",
                    "枚ルリグトラッシュに置く）"
            ],
            ""
        ],
        replace_pattern![
            r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに(合計|合計で)１枚までしか入れられない）",
            "*TEAM PIECE*"
        ],
        replace_pattern![
            r"（あなたの場にいるルリグ３体がこの条件を満たす）",
            "*TEAM*"
        ],
        replace_pattern![
            r"（シグニは覚醒すると場にあるかぎり覚醒状態になる）",
            "*AWAKE*",
        ],
        replace_pattern![
            r"（この能力はこのシグニが場にある場合にしか使用できない）",
            "*AVAILABLE ONLY IN BATTLEFIELD*"
        ],
        replace_pattern![
            r"（グロウしても新しいセンタールリグは能力を得たままである）",
            "*IN GAME AVAILABLE*"
        ],
        replace_pattern![
            r"（凍結された(ルリグ|シグニ)は次の自分のアップフェイズにアップしない）",
            "*FROZEN*"
        ],
        replace_pattern![
            r"（凍結されたルリグとシグニは次の自分のアップフェイズにアップしない）",
            "*FROZEN*"
        ],
        replace_pattern![
            r"（フェゾーネマジックは５種類ある）",
            "*FESONE MAGIC*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）",
            "*CIP COST*"
        ],
        replace_pattern![
            r"（ゲームを開始する際に、センタールリグでないルリグを表向きにしても《コインアイコン》を得られない）",
            "*GAIN NO COINS*"
        ],
        replace_pattern![
            r"（プレイヤーが保持できる《コインアイコン》の上限は５枚である）",
            "*COIN LIMIT*"
        ],
        replace_pattern![
            r"（すでに場に３体以上ある場合は２体になるようにシグニをトラッシュに置く）",
            "*SIGNI ZONE RESTRICTION*"
        ],
        replace_pattern![
            r"（コストの合計とは、カードの左上のエナコストの数字の合計である。例えばコストが《白×1》《無×1》の場合、コストの合計は２である）",
            "*TOTAL COST*"
        ],
        replace_pattern![
            r"（コストのない【出】能力は発動しないことを選べない。.+）",
            "*MUST APPLY CIP*"
        ],
        replace_pattern![
            r"（コストのない【出】能力は発動しないことを選べない。ライフクロスが１枚の場合その１枚をトラッシュに置く）",
            "*MUST APPLY CIP*"
        ],
        replace_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            "*ASSASSIN*"
        ],
        replace_pattern![
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            "*ASSASSIN OR LANCER*"
        ],
        replace_pattern![
            r"（このシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            "*SELF ASSASSIN*"
        ],
        replace_pattern![
            r"（表記されているパワーとは、元々それに印刷されている値である）",
            "*BASIC POWER*"
        ],
        replace_pattern![
            r"（あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】１つを消費し、そのダメージを受けない）",
            "*LRIG BARRIER*"
        ],
        replace_pattern![
            r"（あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】１つを消費し、そのダメージを受けない）",
            "*SIGNI BARRIER*"
        ],
        replace_pattern![
            r"（あなたが次にシグニからダメージを受ける場合、代わりに【シグニバリア】１つを消費し、そのダメージを受けない。あなたが次にルリグからダメージを受ける場合、代わりに【ルリグバリア】１つを消費し、そのダメージを受けない）",
            "*LRIG/SIGNI BARRIER*"
        ],
        replace_pattern![
            r"（(この|それらの)シグニは.+によって対象にされない）",
            "*SHADOW*"
        ],
        replace_pattern![
            r"（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            "*RANDOM RESONA MUST BE EXPOSED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（レゾナでありクラフトであるシグニはリムーブできず場を離れるとゲームから除外される）",
            "*RESONA CANT BE REMOVED*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（クラフトであるシグニは場を離れるとゲームから除外される）",
            "*CRAFT SIGNI REMOVED ON LEAVE*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（このクラフトの上にあるシグニが場を離れるとこのクラフトはゲームから除外される）",
            "*TORAMARU GIMMICK*",
            CardFeature::Craft
        ],
        replace_pattern![
            r"（チェックゾーンにあるカードはターン終了時にトラッシュに置かれる）",
            "*CHECK ZONE*"
        ],
        replace_pattern![
            r"（あなたの場にいるルリグが１体で、そのルリグがレベル３以上であるかぎり、そのルリグのリミットを＋２する）",
            "*LIMIT UPPER EFFECTS*"
        ],
        replace_pattern![
            r"（【リミットアッパー】はあなたのルリグゾーンに１つまでしか置けない）",
            "*ONLY ONE LIMIT UPPER*"
        ],
        replace_pattern![
            r"（あなたのデッキの一番上のカードをエナゾーンに置く）",
            "*ENER CHARGE*"
        ],
        replace_pattern![
            r"（対戦相手のシグニが【シュート】を持つシグニとのバトルによってバニッシュされる場合、エナゾーンに置かれる代わりにトラッシュに置かれる）",
            "*SHOOT LIKE*"
        ],
        replace_pattern![
            "（あなたのルリグトラッシュに[（\u{FF10}-\u{FF19}）]枚以上のアーツがあるかぎり《リコレクトアイコン》［[（\u{FF10}-\u{FF19}）]枚以上］に続く文章が有効になる）",
            "*RECOLLECT*"
        ],
    ];

    let d_patterns: [DetectPattern; PATTERNS_AMOUNT_D] = [
        detect_pattern![r"【ウィルス】", CardFeature::Virus],
        detect_pattern![r"【ハーモニー】", CardFeature::Harmony],
        detect_pattern![r"【ウィルス】", CardFeature::Virus],
        detect_pattern![
            r"（このクラフトは効果以外によっては場に出せない）",
            CardFeature::Craft
        ],
        detect_pattern![r"覚醒する", CardFeature::Awake],
        detect_pattern![
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            CardFeature::Craft
        ],
        detect_pattern![
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            CardFeature::Craft
        ],
        detect_pattern![r"《ガードアイコン》", CardFeature::Guard],
        detect_pattern![r"アクセ", CardFeature::Acce],
        detect_pattern![any_num!["エクシード", ""], CardFeature::Exceed],
        detect_pattern![
            r"（ゲームを開始する際に、このルリグを表向きにしたとき、このルリグがセンタールリグであるなら、[《コインアイコン》]+を得る）",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"（右下に【コイン】を持つルリグがグロウしたとき、それと同じ枚数の[《コインアイコン》]+を得る）",
            CardFeature::GainCoin
        ],
        detect_pattern![
            r"ルリグデッキに加える。（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            CardFeature::Craft
        ],
        detect_pattern![r"《コインアイコン》を得る", CardFeature::GainCoin],
        detect_pattern![r"ガードアイコン", CardFeature::Guard],
        detect_pattern![r"捨てさせる。", CardFeature::DiscardOpponent],
        detect_pattern![
            r"各プレイヤーは手札をすべてエナゾーンに置",
            CardFeature::DiscardOpponent,
            CardFeature::RandomDiscard
        ],
        detect_pattern![r"見ないで選び、捨てさせる。", CardFeature::RandomDiscard],
        detect_pattern![r"対象になったとき", CardFeature::OnTouch],
        detect_pattern![r"ダウンする。", CardFeature::Down],
        detect_pattern![r"エナチャージ", CardFeature::Charge],
        detect_pattern![
            any_num!["カードを", "枚までエナゾーンに置"],
            CardFeature::Charge
        ],
        detect_pattern![
            r"残りを好きな順番でデッキの一番下に置く",
            CardFeature::BottomCheck
        ],
        detect_pattern![r"(それ|シグニ)をトラッシュに置", CardFeature::Trash],
        detect_pattern![r"シグニバリア", CardFeature::Barrier],
        detect_pattern![r"ルリグバリア", CardFeature::Barrier],
        // (r"がアタックしたとき", do_remove:  "*ON ATTACK*", CardFeature::OnAttack]),
        detect_pattern![r"アサシン", CardFeature::Assassin],
        detect_pattern![r"シャドウ", CardFeature::Shadow],
        detect_pattern![r"【マルチエナ】", CardFeature::DualColorEner],
        detect_pattern![
            r"（エナコストを支払う際、このカードは好きな色１つとして支払える）",
            CardFeature::DualColorEner
        ],
        detect_pattern![
            r"（エナコストを支払う際、このカードは.+１つとして支払える）",
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
        detect_pattern![r"コインアイコン》：", CardFeature::BetCoin],
        detect_pattern![r"Sランサー", CardFeature::SLancer, CardFeature::Lancer],
        detect_pattern![r"Ｓランサー", CardFeature::SLancer, CardFeature::Lancer],
        detect_pattern![r"【マジックボックス】", CardFeature::MagicBox],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それをゲームから除外する"],
            CardFeature::RemoveSigni
        ],
        detect_pattern![r"バニッシュ", CardFeature::Banish],
        detect_pattern![
            r"シグニ.+エナゾーンに置", //todo: 対戦相手の
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手は自分の.?シグニ１体を選びエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のパワー.+以下のシグニ１体を対象とし、それをエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num![
                "対戦相手のシグニを",
                "体(まで|を)対象とし、(それら|それ)をエナゾーンに置"
            ],
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のすべてのシグニをエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num!["対戦相手の.+のシグニ", "体を対象とし、それをエナゾーンに置"],
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
            r"支払っても良い。そうした場合、対戦相手は自分のシグニ１体を選びエナゾーンに置",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のシグニ１体を対象とし、それとこのシグニをエナゾーンに",
            CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"クラフトの《",
            CardFeature::Craft
        ],
        detect_pattern![r"シグニをアップ", CardFeature::Up],
        detect_pattern![
            any_num!["シグニ", "体を対象とし、(それ|それら)をアップ"],
            CardFeature::Up
        ],
        detect_pattern![r"凍結する", CardFeature::Freeze],
        detect_pattern![r"それらの場所を入れ替え", CardFeature::Position],
        detect_pattern![r"場に出すことができない", CardFeature::LimitSigni],
        detect_pattern![
            any_num![
                "対戦相手のシグニ",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            CardFeature::Bounce
        ],
        detect_pattern![
            any_num![
                "対戦相手のパワー",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            CardFeature::Bounce
        ],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それを手札に戻"],
            CardFeature::Bounce
        ],
        // (    r"手札に加え", do_remove:  "*SALVAGE*", CardFeature::Salvage]),
        detect_pattern![
            any_num!["ライフクロス", "枚をトラッシュに置"],
            CardFeature::LifeTrash
        ],
        detect_pattern![
            any_num!["エナゾーンからカード", "枚(を|選び).+トラッシュに置"],
            CardFeature::EnerAttack
        ],
        detect_pattern![r"ルリグトラッシュに置", CardFeature::LrigTrash],
        // (r"アタックフェイズ開始時", do_remove:  "*ON ATTACK START*", CardFeature::OnAttackStart]),
        detect_pattern![r"ライフクロスに加える", CardFeature::AddLife],
        detect_pattern![r"ランサー", CardFeature::Lancer],
        detect_pattern![r"ライフクロスを１枚クラッシュする", CardFeature::LifeCrush],
        detect_pattern![
            r"対戦相手のライフクロス１枚をクラッシュする。",
            CardFeature::LifeCrush
        ],
        detect_pattern![r"対戦相手にダメージを与える。", CardFeature::Damage],
        detect_pattern![r"クラッシュしたとき、", CardFeature::OnLifeCrush],
        detect_pattern![
            r"クラッシュされ(る場合|たとき|るかトラッシュ|ていた場合)、",
            CardFeature::OnLifeCrush
        ],
        detect_pattern![r"リコレクトアイコン", CardFeature::Recollect],
        detect_pattern![any_num![r"", "枚見"], CardFeature::SeekTop],
        detect_pattern![r"デッキの一番上に(戻|置)", CardFeature::TopSet],
        detect_pattern![r"のシグニは能力を失う", CardFeature::EraseSkill],
        detect_pattern![r"それは能力を失う", CardFeature::EraseSkill],
        detect_pattern![
            any_num![
                "シグニを",
                "体(まで|を)対象とし、ターン終了時まで、それは能力を失う"
            ],
            CardFeature::EraseSkill
        ],
        detect_pattern![
            r"それを《サーバント　ＺＥＲＯ》にする",
            CardFeature::EraseSkill
        ],
        detect_pattern![r"アタックできない", CardFeature::NonAttackable],
        detect_pattern![any_num!["カードを", "枚引"], CardFeature::Draw],
        detect_pattern![
            any_num!["デッキの上からカードを", "枚トラッシュに置"],
            CardFeature::Drop
        ],
        detect_pattern![
            any_num![
                "対戦相手のエナゾーンからカードを",
                "枚まで対象とし、それらを手札に戻"
            ],
            CardFeature::EnerAttack
        ],
        detect_pattern![r"デッキの一番下に置", CardFeature::DeckBounce],
        detect_pattern![r"シグニのパワーを＋", CardFeature::PowerUp],
        detect_pattern![r"このシグニのパワーは＋", CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを＋", CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを－", CardFeature::PowerDown],
        detect_pattern![
            r"(シグニ|それ)のパワーをこの方法で.+－",
            CardFeature::PowerDown
        ],
        detect_pattern![r"ダメージを受けない", CardFeature::CancelDamage],
        detect_pattern![r"トラッシュからシグニ.+場に出", CardFeature::Reanimate],
        detect_pattern![
            any_num![
                // あなたのトラッシュから黒のシグニ１枚を対象とし、それを場に出す  // TODO
                "あなたのトラッシュから(シグニ|.+のシグニ)",
                "枚を対象とし、それを場に出"
            ],
            CardFeature::Reanimate
        ],
        detect_pattern![
            r"(この|その)ルリグをアップし",
            CardFeature::AdditionalAttack
        ],
        detect_pattern![r"対戦相手は【ガード】ができない", CardFeature::UnGuardable],
        detect_pattern![
            any_num!["スペル", "枚を.+手札に加え"],
            CardFeature::SalvageSpell
        ],
        detect_pattern![
            any_num![
                "(シグニ|シグニを|シグニをそれぞれ)",
                "枚(を|まで).+手札に加え"
            ],
            CardFeature::Salvage
        ],
        detect_pattern![
            any_num!["スペル", "枚をコストを支払わずに使用する"],
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
        detect_pattern![any_num!["手札を", "枚捨ててもよい"], CardFeature::HandCost],
        detect_pattern![
            r"アップ状態のルリグ(を好きな数|１体を)ダウンする",
            CardFeature::RligDownCost
        ],
        detect_pattern![
            any_num!["アップ状態のルリグ", "体をダウンしてもよい"],
            CardFeature::RligDownCost
        ],
        detect_pattern![
            r"このルリグはあなたのルリグトラッシュにあるレベル３の＜.+＞と同じカード名としても扱い、そのルリグの【(自|常)】能力を得る。",
            CardFeature::Inherit
        ],
        detect_pattern![r"グロウするためのコスト", CardFeature::PreventGrowCost],
        detect_pattern![
            any_num!["シグニを", "枚まで対象とし、それを場に出す"],
            CardFeature::PutSigniDefense,
            CardFeature::PutSigniOffense
        ],
        detect_pattern![
            any_num!["あなたのトラッシュにスペルが", "枚以上あるかぎり"],
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
        detect_pattern![
            r"のアーツを使用していた場合",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"あなたのルリグトラッシュにあるアーツ１枚につき",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"このアーツを使用する際、あなたのルリグデッキから.のアーツ１枚をルリグトラッシュに置いてもよい。",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"このゲームの間にあなたがリレーピースを使用している",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"あなたのルリグデッキにあるピース１枚をゲームから除外する",
            CardFeature::OnArts
        ],
        detect_pattern![
            r"ピースを使用する際、カットインして使用できる",
            CardFeature::OnArts
        ],
        // detect_pattern![ // 同上・特定の1枚のみに同時に存在する条件
        //     r"対戦相手のピース１枚を対象とし",
        //     CardFeature::OnArts
        // ],
        detect_pattern![
            r"このターンにあなたがピースを使用していた場合",
            CardFeature::OnArts
        ],
    ];

    (r_patterns, d_patterns)
}
