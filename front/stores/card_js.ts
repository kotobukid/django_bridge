import {defineStore, type StoreDefinition} from "pinia";
import axios, {type AxiosResponse} from "axios";
import type {CardData, CardData2} from "~/types/card";


type State = {
    cards: CardData2[],
    filter_word: string,
    color: string,
    card_type: string,
    format: 1 | 2 | 3,
    has_lb: 0 | 1 | 2,  // 0指定なし　1なし　2あり

    cached_cards: Map<string, CardData2>,
    target: string,
    page: number,
    cursor: number,
    f_bits1: number,
    f_bits2: number,
};

type Getter = {
    detail_by_slug: () => Function,
    cards_per_page: () => number
}

type Actions = {
    set_cards(payload: CardData2[]): void,
    fetch(): Promise<void>,
    set_f1(f: number): void,
    set_f2(f: number): void,
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
            cached_cards: new Map(),
            target: '',
            page: 0,
            cursor: 0,
            f_bits1: 0,
            f_bits2: 0,
        };
    },
    actions: {
        set_cards(payload: CardData2[]): void {
            this.cards = payload;
        },
        fetch(): Promise<void> {
            return new Promise(resolve => {
                axios.get("/api/card/list.json").then((res: AxiosResponse<{ cards: CardData2[] }>) => {
                    console.log(res.data.cards)
                    this.cards = res.data.cards;
                    resolve();
                })
            })
        },
        set_f1(f: number) {
            this.f_bits1 = f;
        },
        set_f2(f: number) {
            this.f_bits2 = f;
        }
    },
    getters: {
        cards_filtered(state: State): CardData2[] {
            // const color_bits_value = color_bits.value;
            // const colorMatcher = color_bits_value === 0
            //     ? () => true
            //     : (card: CardData2) => (card.color & color_bits_value) === color_bits_value;

            // フィーチャーのチェック関数を事前に確定
            const bits1 = state.f_bits1;
            const bits2 = state.f_bits2;

            const featureMatcher = (() => {
                if (bits1 === 0 && bits2 === 0) {
                    return () => true;
                }
                if (bits2 === 0 || bits2 === 1) {
                    return (card: CardData2) => {
                        return (card.feature_bits1 & bits1) !== 0
                        // return (card.feature_bits1 & bits1) === bits1
                    };

                }
                if (bits1 === 0 || bits1 === 1) {
                    return (card: CardData2) => {
                        // return (card.feature_bits2 & bits2) === bits2
                        return (card.feature_bits2 & bits2) !== 0
                    };
                }
                return (card: CardData2) =>
                    (card.feature_bits1 & bits1) === bits1 &&
                    (card.feature_bits2 & bits2) === bits2;
            })();

            // 確定した条件関数でフィルタリング
            return state.cards.filter(card =>
                    featureMatcher(card)
                // colorMatcher(card) && featureMatcher(card)
            );
        },
        detail_by_slug(state: State): Function {
            return async (slug: string): Promise<CardData2> => {
                return new Promise<CardData2>((resolve) => {
                    const detail: CardData2 | undefined = state.cached_cards.get(slug);
                    if (detail) {
                        resolve(detail);
                    } else {
                        let found: boolean = false;
                        for (let i = 0; i < state.cards.length; i++) {
                            // if (state.cards[i].slug === slug) {
                            //     // @ts-ignore
                            //     this.cache(state.cards[i]);
                            //     found = true;
                            //     return resolve(state.cards[i]);
                            // }
                        }
                        if (!found) {
                            axios.get(`/api/card_detail/${slug}`).then((res: AxiosResponse<{
                                success: boolean,
                                card: CardData2 | null
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
        paged_cards(state: State): CardData2[] {
            const start: number = state.page * this.cards_per_page;
            const end: number = start + this.cards_per_page;
            return state.cards.slice(start, end);
        }
    }
});

export {useCardStore};