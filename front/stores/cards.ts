import {defineStore, type StoreDefinition} from "pinia";
import type {CardDataClient} from '../types/card';
import axios, {type AxiosResponse} from "axios";

type State = {
    cards: CardDataClient[],
    filter_word: string,
    color: string,
    card_type: string,
    format: 1 | 2 | 3,
    has_lb: 0 | 1 | 2,  // 0指定なし　1なし　2あり

    worker: Worker | null,
    cached_cards: Map<string, CardDataClient>,
    target: string,
    page: number,
    cursor: number
};

type Getter = {
    detail_by_slug: () => Function,
    cards_per_page: () => number
}

type Actions = {
    install_worker(worker: Worker): Promise<void>,
    initialize_cards(payload: CardDataClient[], format: 1 | 2 | 3): void,
    set_cards(payload: CardDataClient[]): void,
    set_filter_word(payload: string): void,
    set_color(payload: string): void,
    set_card_type(payload: string): void,
    set_format(payload: 1 | 2 | 3): void,
    set_has_lb(payload: 0 | 1 | 2): void,
    cache(card: CardDataClient): void,
    cursor_incr(): void,
    cursor_decr(): void,
    set_page(page: number): void,
};

const useCardStore: StoreDefinition<"card", State, Getter, Actions> = defineStore('card', {
    state(): State {
        return {
            cards: [],
            filter_word: '',
            color: '',
            card_type: '',
            format: 3,
            has_lb: 0,
            worker: null,
            cached_cards: new Map(),
            target: '',
            page: 0,
            cursor: 0
        };
    },
    actions: {
        install_worker(worker: Worker): Promise<void> {
            return new Promise((resolve): void => {
                worker.onmessage = (event: MessageEvent<{ type: string, payload: CardDataClient[] }>): void => {
                    this.set_cards(event.data.payload);
                };
                this.worker = worker;
                resolve();
            });
        },
        initialize_cards(payload: CardDataClient[], format: 1 | 2 | 3): void {
            this.worker?.postMessage({type: 'initialize-cards', payload, format});
        },
        set_cards(payload: CardDataClient[]): void {
            this.cards = payload;
            this.page = 0;
            this.cursor = 0;
            if (this.cards.length > 0) {
                this.target = this.cards[0].slug;
            } else {
                this.target = '';
            }
        },
        set_filter_word(payload: string): void {
            this.filter_word = payload;
            this.worker?.postMessage({type: 'filter_word', payload});
        },
        set_color(payload: string): void {
            this.color = payload;
            this.worker?.postMessage({type: 'color', payload});
        },
        set_card_type(payload: string): void {
            this.card_type = payload;
            this.worker?.postMessage({type: 'card_type', payload});
        },
        set_format(payload: 1 | 2 | 3): void {
            this.format = payload;
            this.worker?.postMessage({type: 'format', payload});
        },
        set_has_lb(payload: 0 | 1 | 2): void {
            this.has_lb = payload;
            this.worker?.postMessage({type: 'has_lb', payload});
        },
        cache(card: CardDataClient): void {
            this.cached_cards.set(card.slug, card);
        },
        cursor_incr(): void {
            let next: number = this.cursor + 1;
            if (next + 1 > this.paged_cards.length) {
                next = 0;
                let total_page: number = Math.ceil(this.cards.length / this.cards_per_page);
                let next_page = this.page + 1;
                if (next_page > total_page - 1) {
                    next_page = 0;
                }
                this.page = next_page;
            }
            this.cursor = next;
        },
        cursor_decr(): void {
            let next: number = this.cursor - 1;
            if (next < 0) {
                next = this.cards_per_page - 1;
                let next_page = this.page - 1;
                if (next_page < 0) {
                    next_page = Math.ceil(this.cards.length / this.cards_per_page) - 1;
                    next = this.cards.length % this.cards_per_page - 1;
                }
                this.page = next_page;
            }
            this.cursor = next;
        },
        set_page(page: number): void {
            this.cursor = 0;
            this.page = page;
        }
    },
    getters: {
        detail_by_slug(state: State): Function {
            return async (slug: string): Promise<CardDataClient> => {
                return new Promise<CardDataClient>((resolve) => {
                    const detail: CardDataClient | undefined = state.cached_cards.get(slug);
                    if (detail) {
                        resolve(detail);
                    } else {
                        let found: boolean = false;
                        for (let i = 0; i < state.cards.length; i++) {
                            if (state.cards[i].slug === slug) {
                                // @ts-ignore
                                this.cache(state.cards[i]);
                                found = true;
                                return resolve(state.cards[i]);
                            }
                        }
                        if (!found) {
                            axios.get(`/api/card_detail/${slug}`).then((res: AxiosResponse<{
                                success: boolean,
                                card: CardDataClient | null
                            }>): void => {
                                if (res.data.success) {
                                    if (res.data.card) {
                                        // @ts-ignore
                                        this.cache(res.data.card);
                                        resolve(res.data.card);
                                    }
                                }
                            });
                        }
                    }
                });
            }
        },
        cards_per_page(): number {
            return 25;
        },
        paged_cards(state: State): CardDataClient[] {
            const start: number = state.page * this.cards_per_page;
            const end: number = start + this.cards_per_page;
            return state.cards.slice(start, end);
        }
    }
});

export {useCardStore};