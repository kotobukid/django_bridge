import type {Format, Story, CardDataClient} from "./types/card";
import {FORMAT} from "./constants";

const create_default_card_data_client = () => {
    // @ts-ignore
    const cdc: CardDataClient = {
        slug: '',
        name: '',
        pronounce: '',
        img: '',
        card_type: '',
        lrig: '',
        level: '',
        color: '',
        klass: [],
        cost: [],
        limit: '',
        power: '',
        team: [],
        team_piece: false,
        timing: [],
        rarity: '',
        has_lb: false,
        lb_text: '',
        skills: '',
        story: '',
        format: FORMAT.all,
        coin: ''
    };
    return cdc;
}

export {create_default_card_data_client}