import {computed} from 'vue';

type StringMaybeContainComma = string;

const CachedGraduation: Record<string, string> = {};

export default function useGradientBg() {
    const bg_gradient_style = computed(() => {
        return (color: StringMaybeContainComma) => {
            if (CachedGraduation[color]) {
                console.log('cache hit')
                return CachedGraduation[color];
            }

            if (color.indexOf(',') > -1) {
                const colors = color.split(',');
                const offset: number = 10;  // グラデーションで塗る領域の両外側の幅
                const width_1 = Math.floor((100 - (offset * 2)) / (colors.length - 1));
                const gradient_code: string = colors.map((c: string, i: number) => {
                    const color_code: string = {
                        '白': '#fff1b4',
                        '青': '#b4ceff',
                        '黒': '#9263f9',
                        '赤': '#ffb4b4',
                        '緑': '#ccffb4',
                        '無': '#cfcfcf'
                    }[c] || '#ffffff';
                    return `${color_code} ${i * width_1 + offset}%`;
                }).join(',');

                const result: string = `background: linear-gradient(to right, ${gradient_code});`;
                CachedGraduation[color] = result;
                return result;
            } else {
                return '';
            }
        }
    });

    return {bg_gradient_style}
}