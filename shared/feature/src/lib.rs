use std::collections::HashSet;
use crate::feature::CardFeature;

pub mod feature;

pub fn create_remove_patterns<'a>() -> Vec<(&'a str, bool, &'a str, HashSet<CardFeature>)> {
    vec![
    // let remove_patterns: Vec<(&str, bool, &str, HashSet<CardFeature>)> = vec![
        (r"『", true, "", features![]), // アクセのみ？
        (r"』", true, "", features![]), // アクセのみ？
        (
            r"ライフバースト：",
            true,
            "LB:",
            features![CardFeature::LifeBurst],
        ),
        (
            r"（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）",
            true,
            "",
            features![CardFeature::Damage],
        ),
        (
            r"（アタックによるダメージでライフクロスを２枚クラッシュする）",
            true,
            "*DOUBLE CRUSH*",
            features![],
        ),
        (
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える。【ダブルクラッシュ】を持つシグニがアタックによってダメージを与えた場合ライフクロスを１枚ではなく２枚クラッシュする）",
            true,
            "*DOUBLE CRUSH && ASSASSIN*",
            features![],
        ),
        (
            r"（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）",
            true,
            "",
            features![CardFeature::Lancer],
        ),
        (
            r"（このクラフトは効果以外によっては場に出せない）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (
            r"（このスペルはあなたのメインフェイズにルリグデッキから使用できる）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (
            r"（クラフトであるスペルは、使用後にゲームから除外される）",
            true,
            "",
            features![CardFeature::Craft],
        ),
        (r"アクセ", false, "*ACCE*", features![CardFeature::Acce]),
        (
            r"（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）",
            true,
            "",
            features![CardFeature::Acce],
        ),
        (
            any_num![
                "（あなたのルリグの下からカードを合計",
                "枚ルリグトラッシュに置く）"
            ],
            true,
            "*EXCEED*",
            features![CardFeature::Exceed],
        ),
        (
            any_num!["エクシード", ""],
            false,
            "*EXCEED*",
            features![CardFeature::Exceed],
        ),
        (
            r"（【チーム】または【ドリームチーム】を持つピースはルリグデッキに合計１枚までしか入れられない）",
            true,
            "*DREAM TEAM*",
            features![],
        ),
        (
            r"（あなたの場にいるルリグ３体がこの条件を満たす）",
            true,
            "*TEAM*",
            features![],
        ),
        (
            r"（シグニは覚醒すると場にあるかぎり覚醒状態になる）",
            true,
            "*AWAKE*",
            features![CardFeature::Awake],
        ),
        (
            r"（凍結されたシグニは次の自分のアップフェイズにアップしない）",
            true,
            "*FROZEN*",
            features![CardFeature::Freeze],
        ),
        (
            r"（フェゾーネマジックは５種類ある）",
            true,
            "*FESONE MAGIC*",
            features![],
        ),
        (
            r"（【出】能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）",
            true,
            "*CIP COST*",
            features![],
        ),
        (
            r"（ゲームを開始する際に、センタールリグでないルリグを表向きにしても《コインアイコン》を得られない）",
            true,
            "*GAIN NO COINS*",
            features![],
        ),
        (
            r"（コストの合計とは、カードの左上のエナコストの数字の合計である。例えばコストが《白×1》《無×1》の場合、コストの合計は２である）",
            true,
            "*TOTAL COST*",
            features![],
        ),
        (
            r"（【アサシン】を持つシグニがアタックすると正面のシグニとバトルをせず対戦相手にダメージを与える）",
            true,
            "*(ASSASSIN)*",
            features![],
        ),
        (
            r"《コインアイコン》を得る",
            false,
            "*GAIN COINS*",
            features![CardFeature::GainCoin],
        ),
        (
            r"ガードアイコン",
            true,
            "*GUARD*",
            features![CardFeature::Guard],
        ),
        (
            r"捨てさせる。",
            false,
            "*HAND DESTRUCTION*",
            features![CardFeature::DiscardOpponent],
        ),
        (
            r"見ないで選び、捨てさせる。",   // todo 確認
            false,
            "*RANDOM HAND DESTRUCTION*",
            features![CardFeature::RandomDiscard],
        ),
        (
            r"ダウンする。",
            false,
            "*DOWN*",
            features![CardFeature::Down],
        ),
        (
            r"エナチャージ",
            false,
            "*CHARGE*",
            features![CardFeature::Charge],
        ),
        (
            any_num!["カードを", "枚までエナゾーンに置"],
            false,
            "*CHARGE MANUALLY*",
            features![CardFeature::Charge],
        ),
        (
            r"残りを好きな順番でデッキの一番下に置く",
            false,
            "*BOTTOM CHECK*",
            features![CardFeature::BottomCheck],
        ),
        (
            r"(それ|シグニ)をトラッシュに置",
            false,
            "*TRASH*",
            features![CardFeature::Trash],
        ),
        (
            r"シグニバリア",
            false,
            "*BARRIER SIGNI*",
            features![CardFeature::Barrier],
        ),
        (
            r"ルリグバリア",
            false,
            "*BARRIER LRIG*",
            features![CardFeature::Barrier],
        ),
        // (r"がアタックしたとき", false, "*ON ATTACK*", features![CardFeature::OnAttack]),
        (
            r"アサシン",
            false,
            "*ASSASSIN*",
            features![CardFeature::Assassin],
        ),
        (
            r"シャドウ",
            false,
            "*SHADOW*",
            features![CardFeature::Shadow],
        ),
        (
            r"【マルチエナ】",
            false,
            "*MULTI ENER*",
            features![CardFeature::MultiEner],
        ),
        (
            r"（エナコストを支払う際、このカードは.+１つとして支払える）",
            true,
            "*DUAL COLOR ENER*",
            features![CardFeature::DualColorEner],
        ),
        (r"チャーム", false, "*CHARM*", features![CardFeature::Charm]),
        (
            r"ダブルクラッシュ",
            false,
            "*DOUBLE CRUSH*",
            features![CardFeature::DoubleCrush],
        ),
        (
            r"トリプルクラッシュ",
            false,
            "*TRIPLE CRUSH*",
            features![CardFeature::TripleCrush],
        ),
        (
            r"Sランサー",
            false,
            "*S LANCER*",
            features![CardFeature::SLancer],
        ),
        (
            r"Ｓランサー",
            false,
            "*S LANCER*",
            features![CardFeature::SLancer],
        ),
        (
            any_num!["対戦相手のシグニ", "体を対象とし、それをゲームから除外する"],
            false,
            "*REMOVE SIGNI*",
            features![CardFeature::RemoveSigni],
        ),
        (
            r"バニッシュ",
            false,
            "*BANISH*",
            features![CardFeature::Banish],
        ),
        (
            r"シグニ.+エナゾーンに置",
            false,
            "*ENER*",
            features![CardFeature::Ener],
        ),
        (
            r"凍結する",
            false,
            "*FREEZE*",
            features![CardFeature::Freeze],
        ),
        (
            any_num![
                "対戦相手のシグニ",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            false,
            "*BOUNCE*",
            features![CardFeature::Bounce],
        ),
        (
            any_num![
                "対戦相手のパワー",
                "体(まで|を)対象とし、(それら|それ)を手札に戻"
            ],
            false,
            "*BOUNCE*",
            features![CardFeature::Bounce],
        ),
        (
            any_num!["対戦相手のシグニ", "体を対象とし、それを手札に戻"],
            false,
            "BOUNCE",
            features![CardFeature::Bounce],
        ),
        // (r"手札に加え", false, "*SALVAGE*", features![CardFeature::Salvage]),
        (
            r"ライフクロス[（\u{FF10}-\u{FF19}）]+枚をトラッシュに置",
            false,
            "*LIFE TRASH*",
            features![CardFeature::LifeTrash],
        ),
        (
            any_num!["エナゾーンからカード", "枚(を|選び).+トラッシュに置"],
            false,
            "*ENER ATTACK*",
            features![CardFeature::EnerAttack],
        ),
        (
            r"ルリグトラッシュに置",
            false,
            "*LRIG TRASH*",
            features![CardFeature::LrigTrash],
        ),
        // (r"アタックフェイズ開始時", false, "*ON ATTACK START*", features![CardFeature::OnAttackStart]),
        (
            r"ライフクロスに加える",
            false,
            "*ADD LIFE*",
            features![CardFeature::AddLife],
        ),
        (
            r"ランサー",
            false,
            "*LANCER*",
            features![CardFeature::Lancer],
        ),
        (
            r"ライフクロスを１枚クラッシュする",
            false,
            "*CRUSH*",
            features![CardFeature::LifeCrush],
        ),
        (
            r"対戦相手のライフクロス１枚をクラッシュする。",
            false,
            "*CRUSH*",
            features![CardFeature::LifeCrush],
        ),
        (
            r"対戦相手にダメージを与える。",
            false,
            "*DAMAGE*",
            features![CardFeature::Damage],
        ),
        (
            r"リコレクトアイコン",
            false,
            "*RECOLLECT*",
            features![CardFeature::Recollect],
        ),
        (
            any_num![r"", "枚見"],
            false,
            "*SEEK*",
            features![CardFeature::SeekTop],
        ),
        (
            r"能力を失う",
            false,
            "*ERASE SKILL*",
            features![CardFeature::EraseSkill],
        ),
        (
            r"それを《サーバント　ＺＥＲＯ》にする",
            false,
            "*ERASE SKILL / SERVANT ZERO*",
            features![CardFeature::EraseSkill],
        ),
        (
            r"アタックできない",
            false,
            "*NON ATTACKABLE*",
            features![CardFeature::NonAttackable],
        ),
        (
            any_num!["カードを", "枚引"],
            false,
            "*DRAW*",
            features![CardFeature::Draw],
        ),
        (
            any_num!["デッキの上からカードを", "枚トラッシュに置"],
            false,
            "*DROP*",
            features![CardFeature::Drop],
        ),
        (
            any_num![
                "対戦相手のエナゾーンからカードを",
                "枚まで対象とし、それらを手札に戻"
            ],
            false,
            "*ENER ATTACK*",
            features![CardFeature::EnerAttack],
        ),
        (
            r"デッキの一番下に置",
            false,
            "*DECK BOUNCE*",
            features![CardFeature::DeckBounce],
        ),
        (
            r"シグニのパワーを＋",
            false,
            "*POWER UP*",
            features![CardFeature::PowerUp],
        ),
        (
            r"このシグニのパワーは＋",
            false,
            "*POWER UP*",
            features![CardFeature::PowerUp],
        ),
        (
            r"(シグニ|それ|それら)のパワーを＋",
            false,
            "*POWER UP*",
            features![CardFeature::PowerUp],
        ),
        (
            r"(シグニ|それ|それら)のパワーを－",
            false,
            "*POWER DOWN*",
            features![CardFeature::PowerDown],
        ),
        (
            r"(シグニ|それ)のパワーをこの方法で.+－",
            false,
            "*POWER DOWN*",
            features![CardFeature::PowerDown],
        ),
        (
            r"ダメージを受けない",
            false,
            "*CANCEL DAMAGE*",
            features![CardFeature::CancelDamage],
        ),
        (
            r"トラッシュからシグニ.+場に出",
            false,
            "*REANIMATE*",
            features![CardFeature::Reanimate],
        ),
        (
            any_num![
                // あなたのトラッシュから黒のシグニ１枚を対象とし、それを場に出す  // TODO
                "あなたのトラッシュから(シグニ|.+のシグニ)",
                "枚を対象とし、それを場に出"
            ],
            false,
            "*REANIMATE*",
            features![CardFeature::Reanimate],
        ),
        (
            r"このルリグをアップし",
            false,
            "*ADDITIONAL ATTACK*",
            features![CardFeature::AdditionalAttack],
        ),
        (
            r"対戦相手は【ガード】ができない",
            false,
            "*UNGUARDABLE*",
            features![CardFeature::UnGuardable],
        ),
        (
            any_num!["スペル", "枚を.+手札に加え"],
            false,
            "*SALVAGE SPELL*",
            features![CardFeature::SalvageSpell],
        ),
        (
            any_num![
                "(シグニ|シグニを|シグニをそれぞれ)",
                "枚(を|まで).+手札に加え"
            ],
            false,
            "*SALVAGE SIGNI*",
            features![CardFeature::Salvage],
        ),
        (
            any_num!["スペル", "枚をコストを支払わずに使用する"],
            false,
            "*FREE SPELL*",
            features![CardFeature::FreeSpell],
        ),
        (
            r"このシグニがアタックしたとき.+バニッシュする",
            false,
            "*BANISH ON ATTACK*",
            features![CardFeature::BanishOnAttack],
        ),
        (
            r"ルリグデッキに加える。（ゲーム終了時にそのレゾナがルリグデッキにあれば公開する）",
            false,
            "*CRAFT RESONA*",
            features![CardFeature::Craft],
        ),
        (
            any_num!["手札を", "枚捨ててもよい"],
            false,
            "*HAND COST*",
            features![CardFeature::HandCost],
        ),
        (
            r"アップ状態のルリグを好きな数ダウンする",
            false,
            "*ASSIST COST*",
            features![CardFeature::AssistCost],
        ),
        (
            r"このルリグはあなたのルリグトラッシュにあるレベル３の＜.+＞と同じカード名としても扱い、そのルリグの【自】能力を得る。",
            true,
            "*Inherit*",
            features![CardFeature::Inherit],
        ),
        (
            r"グロウするためのコスト",
            true,
            "*PREVENT GROW COST*",
            features![CardFeature::PreventGrowCost],
        ),
        (
            any_num!["シグニを", "枚まで対象とし、それを場に出す"],
            true,
            "*PUT BLOCKER*",
            features![CardFeature::PutSigniDefense, CardFeature::PutSigniOffense,],
        ),
    ]
}
