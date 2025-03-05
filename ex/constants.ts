import {Story, Format} from "./types/card.js";

const FORMAT: Record<'all' | 'key' | 'diva', Format> = {
    all: 1,
    key: 2,
    diva: 3
};

const CARD_TYPE = {
    LRIG: 'ルリグ',
    ARTS: 'アーツ',
    SIGNI: 'シグニ',
    SPELL: 'スペル',
    KEY: 'キー',
    RESONA: 'レゾナ',
    PIECE: 'ピース',
    ASSIST: 'アシストルリグ',
    PIECE_RELAY: 'ピース(リレー)',
    ARTS_CRAFT: 'アーツ(クラフト)',
    RESONA_CRAFT: 'レゾナ(クラフト)',
    UNKNOWN_CARD_TYPE: 'UNKNOWN CARD TYPE',

    FLAG: {
        CRAFT: 'クラフト'
    }
};

const TEAM_TYPE = {
    ANCIENT_SURPRISE: '＜アンシエント･サプライズ＞',
    CARD_JOCKEY: '＜Card Jockey＞',
    NO_LIMIT: '＜No Limit＞',
    DIAGRAM: '＜DIAGRAM＞',
    DXM: '＜デウス・エクス・マキナ＞',
    UCHU_NO_HAJIMARI: '＜うちゅうのはじまり＞',
    SANBAKA: '＜さんばか＞',
    KYURUKYURUN: '＜きゅるきゅるーん☆＞',

    DREAM_TEAM: '＜ドリームチーム＞',
    NO_TEAM: 'チーム制限なし'
};
// ['＜アンシエント･サプライズ＞', 'ansp'],
// ['＜Card Jockey＞', 'cjky'],
// ['＜No Limit＞', 'nlmt'],
// ['＜DIAGRAM＞', 'dgrm'],
// ['＜デウス・エクス・マキナ＞', 'dxma'],
// ['＜うちゅうのはじまり＞', 'uhjm'],
// ['＜さんばか＞', 'snbk'],
// ['＜きゅるきゅるーん☆＞', 'crcr'],

const LRIG_TYPE = {
    TAMA: 'タマ',
    TAWIL: 'タウィル',
    REMEMBA: 'リメンバ',
    SASHE: 'サシェ',
    DONA: 'ドーナ',
    EMMA: 'エマ',
    RIZE: 'リゼ',
    ANJU: 'アンジュ',
    AKINO: 'アキノ',
    LION: 'LION',
    NOVA: 'ノヴァ',
    YUKAYUKA: 'ゆかゆか',
    HANAYO: '花代',
    YUZUKI: 'ユヅキ',
    RIL: 'リル',
    CARNIVAL: 'カーニバル',
    REILA: 'レイラ',
    LOV: 'ＬｏＶ',
    HIRANA: 'ヒラナ',
    LOVIT: 'LOVIT',
    EX: 'エクス',
    PIRURUKU: 'ピルルク',
    ELDORA: 'エルドラ',
    MIRURUN: 'ミルルン',
    AYA: 'あや',
    REI: 'レイ',
    TAMAGO: 'タマゴ',
    MADOKA: 'マドカ',
    MIKOMIKO: 'みこみこ',
    MIDORIKO: '緑子',
    ANN: 'アン',
    AIYAI: 'アイヤイ',
    MEL: 'メル',
    MAMA: 'ママ',
    AT: 'アト',
    WOLF: 'WOLF',
    BANG: 'バン',
    SANGA: 'サンガ',
    URITH: 'ウリス',
    IONA: 'イオナ',
    UMR: 'ウムル',
    MYU: 'ミュウ',
    ALFOU: 'アルフォウ',
    HANARE: 'ハナレ',
    NANASHI: 'ナナシ',
    GUZUKO: 'グズ子',
    TOKO: 'とこ',
    MUZIKA: 'ムジカ',
    DEUX: 'デウス',
    MACHINA: 'マキナ',
    MAHOMAHO: 'まほまほ',
    MITO: '美兎',
    MUGEN: '夢限',
    MAYU: '？',
    NIJISANJI: 'にじさんじ',
};

const COMMON_WORD = {
    LIFE_BURST: 'ライフバースト',
    UNKNOWN_LRIG_TEAM: 'UNKNOWN LRIG/TEAM',
    UNKNOWN_COLOR: 'UNKNOWN COLOR'
};

const STORY: Record<string, Story> = {
    DISSONA: 'd',
    ANY: ''
};

export {
    STORY,
    FORMAT,
    CARD_TYPE,
    TEAM_TYPE,
    LRIG_TYPE,
    COMMON_WORD
};