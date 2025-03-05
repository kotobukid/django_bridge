type Story = 'd' | '';
type Format = 1 | 2 | 3;


type CardDataCompact = {
    s: string,      // slug
    n: string,      // name
    p: string,      // pronounce
    i: string,      // img
    t: string,      // card_type
    lr: string[],   // lrig
    lv: string,     // level
    c: string[],    // color
    cl: string[],   // klass
    cs: string[],   // cost
    l: string,      // limit
    pw: string,     // power
    tm: string[],   // team
    tp: boolean,    // team_piece
    ti: string[],   // timing
    r: string,      // rarity
    b: boolean,     // has_lb
    bt: string,     // lb_text
    sk: string[],   // skills
    st: Story,      // story
    fm: Format,     // format
    co: string,     // coin
    so: number,     // sort
    pr: string      // product
};

type CardData = {
    slug: string,
    name: string,
    pronounce: string,
    img: string,
    card_type: string,
    lrig: string[],
    level: string,
    color: string[],
    klass: string[],
    cost: string[],
    limit: string,
    power: string,
    team: string[],
    team_piece: boolean,
    timing: string[],
    rarity: string,
    has_lb: boolean,
    lb_text: string,
    skills: string[],
    story: Story,
    format: Format,
    coin: string,
    sort: number,
    product: string
};

type CardDataClient = Omit<CardData, 'color' | 'skills' | 'lrig'> & {  // コンパクト/エキスパンドを作り込む前のフロント開発をこれで間に合わせる
    color: string,
    skills: string,
    lrig: string,
    prev: string,
    next: string
};

type EPS = {
    id: number,
    slug: string,
    json: string,
    method: string
}

type CommaSeparatedString = string;

type Deck = {
    id: number,
    name: string,
    source: string,
    is_deck: boolean,
    lrig: string,
    assists: CommaSeparatedString,
    is_public: boolean,
    ancestor?: number,
    owner: number,
    format: Format,
    tags: string,
    description: string,
    created_at: Date
}

export type {
    Story,
    Format,
    CardData,
    CardDataClient,
    CardDataCompact,
    EPS,
    Deck
};