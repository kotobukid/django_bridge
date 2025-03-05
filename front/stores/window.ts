import {defineStore} from "pinia";

type WindowInfo = {
    x: number,
    y: number,
    z: number,
    id: string
}


type State = {
    windows: Map<string, WindowInfo>,
    zIndexCursor: number,
    dragging_target: string
}

type Vector2D = {
    x: number,
    y: number
}

const useWindowStore = defineStore('window', {
    state(): State {
        return {
            windows: new Map(),
            zIndexCursor: 100,
            dragging_target: ''
        };
    },
    actions: {
        initialize(): void {
            this.windows.set('detail', {
                x: 780,
                y: 340,
                z: this.zIndexCursor + 10,
                id: 'detail'
            });
            this.windows.set('keep', {
                x: 1100,
                y: 43,
                z: this.zIndexCursor + 20,
                id: 'keep'
            });

            let max: number = this.zIndexCursor;
            const keys: IterableIterator<string> = this.windows.keys();
            for (let key of keys) {
                max = Math.max(max, this.windows.get(key)!.z);
            }
            this.zIndexCursor = max;
        },
        move(movement: Vector2D): void {
            const w: WindowInfo | undefined = this.windows.get(this.dragging_target);
            if (w) {
                w.x = w.x + movement.x;
                w.y = w.y + movement.y;
            }
        },
        release_dragging_target(): void {
            this.dragging_target = '';
        },
        set_to_top(id: string): void {
            this.dragging_target = id;
            const w: WindowInfo | undefined = this.windows.get(id);
            if (w) {
                this.zIndexCursor = this.zIndexCursor + 15;
                w.z = this.zIndexCursor;

                if (this.zIndexCursor > 1000) {
                    const keys: IterableIterator<string> = this.windows.keys();

                    for (let key of keys) {
                        this.windows.get(key)!.z = this.windows.get(key)!.z - 800;
                    }
                    this.zIndexCursor = 100;
                }
            }
        }
    },
    getters: {
        keep(): WindowInfo {
            return this.windows.get('keep')!;
        }
    }
});

export {useWindowStore, type WindowInfo};