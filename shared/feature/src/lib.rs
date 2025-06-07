use regex::Regex;

pub mod feature;
pub use feature::CardFeature;

pub struct DetectPattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub features_detected: &'static [feature::CardFeature],
}

pub struct ReplacePattern {
    pub pattern: &'static str,
    pub pattern_r: Regex,
    pub replace_to: &'static str,
    pub features_detected: &'static [feature::CardFeature],
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
            feature::CardFeature::LifeBurst
        ],
        replace_pattern![
            r"』",
            ""
        ],
        replace_pattern![
            r"ライフバースト：",
            "LB:",
            feature::CardFeature::LifeBurst
        ],
        replace_pattern![
            r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）",
            "",
            feature::CardFeature::Damage
        ],
        replace_pattern![
            r"（パワーが０以下のシグニはルールによってバニッシュされる）",
            "",
            feature::CardFeature::PowerDown
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
            feature::CardFeature::Lancer
        ],
        replace_pattern![
            r"（【Ｓランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスがある場合はそれを１枚クラッシュする。無い場合は対戦相手にダメージを与える）",
            "*S LANCER*",
            feature::CardFeature::SLancer
        ],
        replace_pattern![
            r"（このクラフトは効果以外によっては場に出せない）",
            "*NO STANDARD PUT*",
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            "*SPELL CRAFT*",
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            "*SPELL CRAFT GOES REMOVED*",
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            "*RESONA CRAFT REMOVED*",
            feature::CardFeature::Craft
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
            feature::CardFeature::Craft
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
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（レゾナでありクラフトであるシグニはリムーブできず場を離れるとゲームから除外される）",
            "*RESONA CANT BE REMOVED*",
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（クラフトであるシグニは場を離れるとゲームから除外される）",
            "*CRAFT SIGNI REMOVED ON LEAVE*",
            feature::CardFeature::Craft
        ],
        replace_pattern![
            r"（このクラフトの上にあるシグニが場を離れるとこのクラフトはゲームから除外される）",
            "*TORAMARU GIMMICK*",
            feature::CardFeature::Craft
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
        detect_pattern![r"【ウィルス】", feature::CardFeature::Virus],
        detect_pattern![r"【ハーモニー】", feature::CardFeature::Harmony],
        detect_pattern![r"【ウィルス】", feature::CardFeature::Virus],
        detect_pattern![
            r"（このクラフトは効果以外によっては場に出せない）",
            feature::CardFeature::Craft
        ],
        detect_pattern![r"覚醒する", feature::CardFeature::Awake],
        detect_pattern![
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            feature::CardFeature::Craft
        ],
        detect_pattern![
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            feature::CardFeature::Craft
        ],
        detect_pattern![r"《ガードアイコン》", feature::CardFeature::Guard],
        detect_pattern![r"アクセ", feature::CardFeature::Acce],
        detect_pattern![any_num!["エクシード", ""], feature::CardFeature::Exceed],
        detect_pattern![
            r"（ゲームを開始する際に、このルリグを表向きにしたとき、このルリグがセンタールリグであるなら、[《コインアイコン》]+を得る）",
            feature::CardFeature::GainCoin
        ],
        detect_pattern![
            r"（右下に【コイン】を持つルリグがグロウしたとき、それと同じ枚数の[《コインアイコン》]+を得る）",
            feature::CardFeature::GainCoin
        ],
        detect_pattern![
            r"ルリグデッキに加える。（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            feature::CardFeature::Craft
        ],
        detect_pattern![r"《コインアイコン》を得る", feature::CardFeature::GainCoin],
        detect_pattern![r"ガードアイコン", feature::CardFeature::Guard],
        detect_pattern![r"捨てさせる。", feature::CardFeature::DiscardOpponent],
        detect_pattern![
            r"各プレイヤーは手札をすべてエナゾーンに置",
            feature::CardFeature::DiscardOpponent,
            feature::CardFeature::RandomDiscard
        ],
        detect_pattern![r"見ないで選び、捨てさせる。", feature::CardFeature::RandomDiscard],
        detect_pattern![r"対象になったとき", feature::CardFeature::OnTouch],
        detect_pattern![r"ダウンする。", feature::CardFeature::Down],
        detect_pattern![r"エナチャージ", feature::CardFeature::Charge],
        detect_pattern![
            any_num!["カードを", "枚までエナゾーンに置"],
            feature::CardFeature::Charge
        ],
        detect_pattern![
            r"残りを好きな順番でデッキの一番下に置く",
            feature::CardFeature::BottomCheck
        ],
        detect_pattern![r"(それ|シグニ)をトラッシュに置", feature::CardFeature::Trash],
        detect_pattern![r"シグニバリア", feature::CardFeature::Barrier],
        detect_pattern![r"ルリグバリア", feature::CardFeature::Barrier],
        // (r"がアタックしたとき", do_remove:  "*ON ATTACK*", feature::CardFeature::OnAttack]),
        detect_pattern![r"アサシン", feature::CardFeature::Assassin],
        detect_pattern![r"シャドウ", feature::CardFeature::Shadow],
        detect_pattern![r"【マルチエナ】", feature::CardFeature::DualColorEner],
        detect_pattern![
            r"（エナコストを支払う際、このカードは好きな色１つとして支払える）",
            feature::CardFeature::DualColorEner
        ],
        detect_pattern![
            r"（エナコストを支払う際、このカードは.+１つとして支払える）",
            feature::CardFeature::DualColorEner
        ],
        detect_pattern![r"チャーム", feature::CardFeature::Charm],
        detect_pattern![r"ダブルクラッシュ", feature::CardFeature::DoubleCrush],
        detect_pattern![
            r"トリプルクラッシュ",
            feature::CardFeature::DoubleCrush // ダブクラと統合
        ],
        detect_pattern![r"【シュート】", feature::CardFeature::ShootLike],
        detect_pattern![
            r"エナゾーンに置かれる代わりに(トラッシュ|手札|デッキの一番下)",
            feature::CardFeature::ShootLike
        ],
        detect_pattern![r"【ライズ】あなたの", feature::CardFeature::Rise],
        detect_pattern![r"ベット―", feature::CardFeature::BetCoin],
        detect_pattern![r"コインアイコン》：", feature::CardFeature::BetCoin],
        detect_pattern![r"Sランサー", feature::CardFeature::SLancer, feature::CardFeature::Lancer],
        detect_pattern![r"Ｓランサー", feature::CardFeature::SLancer, feature::CardFeature::Lancer],
        detect_pattern![r"【マジックボックス】", feature::CardFeature::MagicBox],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それをゲームから除外する"],
            feature::CardFeature::RemoveSigni
        ],
        detect_pattern![r"バニッシュ", feature::CardFeature::Banish],
        detect_pattern![
            r"シグニ.+エナゾーンに置", //todo: 対戦相手の
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手は自分の.?シグニ１体を選びエナゾーンに置",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のパワー.+以下のシグニ１体を対象とし、それをエナゾーンに置",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num![
                "対戦相手のシグニを",
                "体(まで|を)対象とし、(それら|それ)をエナゾーンに置"
            ],
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のすべてのシグニをエナゾーンに置",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            any_num!["対戦相手の.+のシグニ", "体を対象とし、それをエナゾーンに置"],
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"支払ってもよい。そうした場合、(それ|それら)をエナゾーンに置",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"フェゾーネマジックのクラフトから2種類を1枚ずつ公開しルリグデッキに加える",
            feature::CardFeature::Craft
        ],
        detect_pattern![
            r"支払っても良い。そうした場合、対戦相手は自分のシグニ１体を選びエナゾーンに置",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"対戦相手のシグニ１体を対象とし、それとこのシグニをエナゾーンに",
            feature::CardFeature::EnerOffensive
        ],
        detect_pattern![
            r"クラフトの《",
            feature::CardFeature::Craft
        ],
        detect_pattern![r"シグニをアップ", feature::CardFeature::Up],
        detect_pattern![
            any_num!["シグニ", "体を対象とし、(それ|それら)をアップ"],
            feature::CardFeature::Up
        ],
        detect_pattern![r"凍結する", feature::CardFeature::Freeze],
        detect_pattern![r"それらの場所を入れ替え", feature::CardFeature::Position],
        detect_pattern![r"場に出すことができない", feature::CardFeature::LimitSigni],
        detect_pattern![
            any_num![
                "対戦相手のシグニ",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            feature::CardFeature::Bounce
        ],
        detect_pattern![
            any_num![
                "対戦相手のパワー",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            feature::CardFeature::Bounce
        ],
        detect_pattern![
            any_num!["対戦相手のシグニ", "体を対象とし、それを手札に戻"],
            feature::CardFeature::Bounce
        ],
        // (    r"手札に加え", do_remove:  "*SALVAGE*", feature::CardFeature::Salvage]),
        detect_pattern![
            any_num!["ライフクロス", "枚をトラッシュに置"],
            feature::CardFeature::LifeTrash
        ],
        detect_pattern![
            any_num!["エナゾーンからカード", "枚(を|選び).+トラッシュに置"],
            feature::CardFeature::EnerAttack
        ],
        detect_pattern![r"ルリグトラッシュに置", feature::CardFeature::LrigTrash],
        // (r"アタックフェイズ開始時", do_remove:  "*ON ATTACK START*", feature::CardFeature::OnAttackStart]),
        detect_pattern![r"ライフクロスに加える", feature::CardFeature::AddLife],
        detect_pattern![r"ランサー", feature::CardFeature::Lancer],
        detect_pattern![r"ライフクロスを１枚クラッシュする", feature::CardFeature::LifeCrush],
        detect_pattern![
            r"対戦相手のライフクロス１枚をクラッシュする。",
            feature::CardFeature::LifeCrush
        ],
        detect_pattern![r"対戦相手にダメージを与える。", feature::CardFeature::Damage],
        detect_pattern![r"クラッシュしたとき、", feature::CardFeature::OnLifeCrush],
        detect_pattern![
            r"クラッシュされ(る場合|たとき|るかトラッシュ|ていた場合)、",
            feature::CardFeature::OnLifeCrush
        ],
        detect_pattern![r"リコレクトアイコン", feature::CardFeature::Recollect],
        detect_pattern![any_num![r"", "枚見"], feature::CardFeature::SeekTop],
        detect_pattern![r"デッキの一番上に(戻|置)", feature::CardFeature::TopSet],
        detect_pattern![r"のシグニは能力を失う", feature::CardFeature::EraseSkill],
        detect_pattern![r"それは能力を失う", feature::CardFeature::EraseSkill],
        detect_pattern![
            any_num![
                "シグニを",
                "体(まで|を)対象とし、ターン終了時まで、それは能力を失う"
            ],
            feature::CardFeature::EraseSkill
        ],
        detect_pattern![
            r"それを《サーバント　ＺＥＲＯ》にする",
            feature::CardFeature::EraseSkill
        ],
        detect_pattern![r"アタックできない", feature::CardFeature::NonAttackable],
        detect_pattern![any_num!["カードを", "枚引"], feature::CardFeature::Draw],
        detect_pattern![
            any_num!["デッキの上からカードを", "枚トラッシュに置"],
            feature::CardFeature::Drop
        ],
        detect_pattern![
            any_num![
                "対戦相手のエナゾーンからカードを",
                "枚まで対象とし、それらを手札に戻"
            ],
            feature::CardFeature::EnerAttack
        ],
        detect_pattern![r"デッキの一番下に置", feature::CardFeature::DeckBounce],
        detect_pattern![r"シグニのパワーを＋", feature::CardFeature::PowerUp],
        detect_pattern![r"このシグニのパワーは＋", feature::CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを＋", feature::CardFeature::PowerUp],
        detect_pattern![r"(シグニ|それ|それら)のパワーを－", feature::CardFeature::PowerDown],
        detect_pattern![
            r"(シグニ|それ)のパワーをこの方法で.+－",
            feature::CardFeature::PowerDown
        ],
        detect_pattern![r"ダメージを受けない", feature::CardFeature::CancelDamage],
        detect_pattern![r"トラッシュからシグニ.+場に出", feature::CardFeature::Reanimate],
        detect_pattern![
            any_num![
                // あなたのトラッシュから黒のシグニ１枚を対象とし、それを場に出す  // TODO
                "あなたのトラッシュから(シグニ|.+のシグニ)",
                "枚を対象とし、それを場に出"
            ],
            feature::CardFeature::Reanimate
        ],
        detect_pattern![
            r"(この|その)ルリグをアップし",
            feature::CardFeature::AdditionalAttack
        ],
        detect_pattern![r"対戦相手は【ガード】ができない", feature::CardFeature::UnGuardable],
        detect_pattern![
            any_num!["スペル", "枚を.+手札に加え"],
            feature::CardFeature::SalvageSpell
        ],
        detect_pattern![
            any_num![
                "(シグニ|シグニを|シグニをそれぞれ)",
                "枚(を|まで).+手札に加え"
            ],
            feature::CardFeature::Salvage
        ],
        detect_pattern![
            any_num!["スペル", "枚をコストを支払わずに使用する"],
            feature::CardFeature::FreeSpell
        ],
        detect_pattern![r"このアーツの使用コストは.+減る", feature::CardFeature::FreeArts],
        detect_pattern![
            r"このシグニがアタックしたとき.+バニッシュする",
            feature::CardFeature::BanishOnAttack
        ],
        detect_pattern![
            r"アタックを無効に", // todo: 攻防あり
            feature::CardFeature::AttackNoEffect
        ],
        detect_pattern![r"バニッシュされない", feature::CardFeature::Invulnerable],
        detect_pattern![r"バニッシュされたとき", feature::CardFeature::OnBanish],
        detect_pattern![
            r"(ライフバーストを使用することを選んだ場合|ライフバーストの能力化効果の対象になったとき|ライフバースト】を持っているか|ライフバースト】を持つ場合|ライフバーストが発動する場合|ライフバーストは発動しない)",
            feature::CardFeature::OnBurst
        ],
        detect_pattern![
            r"(置かれたライフクロスは|あなたのライフクロスとチェックゾーンにある【ライフバースト】を持たないカードは|ライフバースト】を持つカードを好きな枚数公開|ライフバーストの能力か効果の対象になったとき)",
            feature::CardFeature::OnBurst
        ],
        detect_pattern![
            r"(エクシードのコストとして|あなたがエクシードのコストを支払ったとき、)",
            feature::CardFeature::OnExceed
        ],
        detect_pattern![any_num!["手札を", "枚捨ててもよい"], feature::CardFeature::HandCost],
        detect_pattern![
            r"アップ状態のルリグ(を好きな数|１体を)ダウンする",
            feature::CardFeature::RligDownCost
        ],
        detect_pattern![
            any_num!["アップ状態のルリグ", "体をダウンしてもよい"],
            feature::CardFeature::RligDownCost
        ],
        detect_pattern![
            r"このルリグはあなたのルリグトラッシュにあるレベル３の＜.+＞と同じカード名としても扱い、そのルリグの【(自|常)】能力を得る。",
            feature::CardFeature::Inherit
        ],
        detect_pattern![r"グロウするためのコスト", feature::CardFeature::PreventGrowCost],
        detect_pattern![
            any_num!["シグニを", "枚まで対象とし、それを場に出す"],
            feature::CardFeature::PutSigniDefense,
            feature::CardFeature::PutSigniOffense
        ],
        detect_pattern![
            any_num!["あなたのトラッシュにスペルが", "枚以上あるかぎり"],
            feature::CardFeature::OnSpell
        ],
        detect_pattern![
            r"(あなた|いずれかのプレイヤー)がスペルを使用したとき、",
            feature::CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターン、(あなたが次に|次にあなたが)スペルを使用する場合",
            feature::CardFeature::OnSpell
        ],
        detect_pattern![
            r"このターンに(あなた|対戦相手)がスペルを使用していた場合、",
            feature::CardFeature::OnSpell
        ],
        detect_pattern![
            r"《ディソナアイコン》のスペルを使用したとき、",
            feature::CardFeature::OnSpell
        ],
        detect_pattern![
            r"のアーツを使用していた場合",
            feature::CardFeature::OnArts
        ],
        detect_pattern![
            r"あなたのルリグトラッシュにあるアーツ１枚につき",
            feature::CardFeature::OnArts
        ],
        detect_pattern![
            r"このアーツを使用する際、あなたのルリグデッキから.のアーツ１枚をルリグトラッシュに置いてもよい。",
            feature::CardFeature::OnArts
        ],
        detect_pattern![
            r"このゲームの間にあなたがリレーピースを使用している",
            feature::CardFeature::OnArts
        ],
        detect_pattern![
            r"あなたのルリグデッキにあるピース１枚をゲームから除外する",
            feature::CardFeature::OnArts
        ],
        detect_pattern![
            r"ピースを使用する際、カットインして使用できる",
            feature::CardFeature::OnArts
        ],
        // detect_pattern![ // 同上・特定の1枚のみに同時に存在する条件
        //     r"対戦相手のピース１枚を対象とし",
        //     feature::CardFeature::OnArts
        // ],
        detect_pattern![
            r"このターンにあなたがピースを使用していた場合",
            feature::CardFeature::OnArts
        ],
    ];

    (r_patterns, d_patterns)
}
