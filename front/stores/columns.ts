import {defineStore} from "pinia";
import type {CardDataClient} from '../../ex/types/card'

type Column = {
    label: string,
    key: keyof CardDataClient | 'operation',
    order: number,
    display: boolean,
    align: '' | 'center' | 'right',
    width: number
}

type State = {
    columns: Column[]
};

const useColumnStore = defineStore('column', {
    state(): State {
        return {
            columns: [
                {
                    key: 'slug',
                    label: 'No.',
                    order: 1,
                    display: true,
                    align: '',
                    width: 160
                },
                {
                    key: 'name',
                    label: '名前',
                    order: 2,
                    display: true,
                    align: '',
                    width: 240
                },
                {
                    key: 'color',
                    label: '色',
                    order: 3,
                    display: true,
                    align: 'center',
                    width: 60
                },
                {
                    key: 'level',
                    label: 'Lv',
                    order: 4,
                    display: true,
                    align: 'center',
                    width: 30
                },
                {
                    key: 'klass',
                    label: '種類',
                    order: 5,
                    display: true,
                    align: 'center',
                    width: 120
                },
                {
                    key: 'power',
                    label: 'パワー',
                    order: 6,
                    display: true,
                    align: 'right',
                    width: 50
                },
                {
                    key: 'operation',
                    label: '操作',
                    order: 7,
                    display: true,
                    align: 'center',
                    width: 100
                },
            ]
        }
    },
    getters: {
        active_columns(state: State) {
            return state.columns.sort((a, b) => a.order - b.order).filter(c => c.display)
        }
    }
});


export {useColumnStore};