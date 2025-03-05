import type {CardDataClient} from '../types/card';
import {useCardStore} from "../stores/cards";

export default function useDetectCard() {
    const card_store = useCardStore();
    const by_whole_index = (index: number): void => {
        const next_card_detail: CardDataClient = card_store.cards[index];
        if (next_card_detail) {
            card_store.cache(next_card_detail);
            card_store.target = next_card_detail.slug;
        } else {
            card_store.target = '';
        }
    };

    const by_local_index = (delta: 1 | -1): void => {
        let next_card_index = card_store.page * card_store.cards_per_page + card_store.cursor + delta;
        if (next_card_index < 0) {
            next_card_index = card_store.cards.length - 1;
        } else if (next_card_index > card_store.cards.length - 1) {
            next_card_index = 0;
        }
        const next_card_detail: CardDataClient = card_store.cards[next_card_index];

        if (next_card_detail) {
            card_store.cache(next_card_detail);
            card_store.target = next_card_detail.slug;
        } else {
            card_store.target = '';
        }
    }
    return {by_whole_index, by_local_index}
}