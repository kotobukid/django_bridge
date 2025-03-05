import {defineStore} from "pinia";
import type {CardDataClient, Deck} from '../../ex/types/card';
import axios, {type AxiosResponse} from 'axios';

type KeptCard = {
    amount: number,
} & CardDataClient;

type Group = 'main_lb' | 'main_no_lb' | 'white' | 'others';

type State = {
    deck_id: number,
    name: string
} & Record<Group, KeptCard[]>;

const judge_card_group = (card: CardDataClient): Group => {
    if (['シグニ', 'スペル'].includes(card.card_type)) {
        return card.has_lb ? 'main_lb' : 'main_no_lb';
    } else if (['ルリグ', 'センタールリグ', 'アシストルリグ', 'レゾナ', 'ピース', 'キー'].includes(card.card_type)) {
        return 'white';
    } else {
        return 'others';
    }
};

const useKeepStore = defineStore('keep', {
    state(): State {
        return {
            deck_id: -1,
            name: '',
            main_lb: [],
            main_no_lb: [],
            white: [],
            others: []
        };
    },
    actions: {
        append(card: CardDataClient): void {
            const group: Group = judge_card_group(card);
            const target_group = this[group];
            let found: boolean = false;
            const max_amount: number = ['white', 'others'].includes(group) ? 1 : 4;
            for (let i: number = 0; i < target_group.length; i++) {
                if (target_group[i].pronounce === card.pronounce) {
                    target_group[i].amount = Math.min(max_amount, Math.max(-1, target_group[i].amount + 1));
                    found = true;
                    break;
                }
            }

            if (!found) {
                target_group.push({amount: 1, ...card});
            }

            this[group] = [...target_group];
        },
        append_to_others(card: CardDataClient): void {
            const target_group = this.others;
            let found: boolean = false;
            for (let i = 0; i < target_group.length; i++) {
                if (target_group[i].pronounce === card.pronounce) {
                    target_group[i].amount = Math.min(1, Math.max(0, target_group[i].amount));
                    found = true;
                    break;
                }
            }

            if (!found) {
                target_group.push({amount: 1, ...card});
            }

            this.others = [...target_group];

        },
        increase(pronounce: string, group: Group, delta: 1 | -1): void {
            const target_group = this[group];
            const max_amount: number = ['white', 'others'].includes(group) ? 1 : 4;

            for (let i: number = 0; i < target_group.length; i++) {
                if (target_group[i].pronounce === pronounce) {
                    target_group[i].amount = Math.min(max_amount, Math.max(-1, target_group[i].amount + delta));
                    break;
                }
            }

            this[group] = [...target_group];
        },
        remove(pronounce: string, group: Group): void {
            const trimmed: KeptCard[] = [];

            this[group].forEach((c: KeptCard): void => {
                if (c.pronounce !== pronounce) {
                    trimmed.push(c);
                }
            });

            this[group] = trimmed;
        },
        trim(): void {
            const groups: Group[] = ['main_lb', 'main_no_lb', 'white', 'others'];

            const others_new_members: KeptCard[] = [];

            const trim_in_group = (group: Group): KeptCard[] => {
                const trimmed: KeptCard[] = [];
                const max_amount: number = ['white', 'others'].includes(group) ? 1 : 4;

                this[group].forEach((c: KeptCard): void => {
                    if (c.amount > -1) {
                        c.amount = Math.min(max_amount, Math.max(0, c.amount));

                        if (c.amount === 0) {
                            if (group !== 'others') {
                                others_new_members.push(c);
                            }
                        } else {
                            trimmed.push(c);
                        }
                    }
                });
                return trimmed;
            };

            for (const group of groups) {
                this[group] = trim_in_group(group);
            }

            others_new_members.forEach((c: KeptCard): void => {
                this.append_to_others(c);
            });
        },
        save_deck(info: any): Promise<void> {
            console.log(info);
            return new Promise<void>((resolve): void => {
                const now = new Date();
                const deck: Deck = {
                    id: this.deck_id || -1,
                    name: this.name || 'NO NAME',
                    source: '',
                    is_deck: true,
                    lrig: '',
                    assists: '',
                    is_public: true,
                    ancestor: -1,
                    owner: -1,
                    format: 3,
                    tags: '',
                    description: '',
                    created_at: now
                };

                axios.post('/api/save_deck', {
                    deck
                }).then((res: AxiosResponse<{ success: boolean }>): void => {
                    console.log(res.data.success);
                    resolve();
                });
            });
        }
    },
    getters: {}
});

export {useKeepStore, type KeptCard};