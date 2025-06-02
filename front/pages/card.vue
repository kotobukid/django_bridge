<script setup lang="ts">
import {ref} from 'vue';
import {useCardStore} from "~/stores/card_js";

const card_store = useCardStore();

const color_bits = computed(() => {
  let flag = 0;
  if (white.value) flag |= 2;
  if (blue.value) flag |= 4;
  if (black.value) flag |= 16;
  if (red.value) flag |= 8;
  if (green.value) flag |= 32;
  if (colorless.value) flag |= 64;

  return flag;
});

// 表示用のメッセージ
const message = ref('');

let print_detail = (id: number) => {
};
let fetch_cards = async (bit1: string, bit2: string, color_filter?: number) => {
};
let fetch_by_f_bits_fn: any = null;
let fetch_by_f_shifts_fn: any = null;
const fetch_cards_ = async () => {
  await fetch_cards(`${f1.value}`, `${f2.value}`, color_bits.value);
};

const selectedFeatures = ref<Map<string, any>>(new Map());

let apply_bits = (a: [number, number]) => {
  // card_store.set_f1(bit1);
  //   card_store.set_f2(bit2);
};

const applyFeatureFilter = (feature: any, isAdding: boolean) => {
  const featureId = `${feature.name}_${feature.bit_shift[0]}_${feature.bit_shift[1]}`;
  
  console.log(`=== Feature Filter Applied ===`);
  console.log(`Feature: ${feature.name}`);
  console.log(`Action: ${isAdding ? 'ADDING' : 'REMOVING'}`);
  console.log(`Current selected count BEFORE: ${selectedFeatures.value.size}`);
  
  if (isAdding) {
    selectedFeatures.value.set(featureId, feature);
  } else {
    selectedFeatures.value.delete(featureId);
  }
  
  console.log(`Current selected count AFTER: ${selectedFeatures.value.size}`);
  console.log(`Selected features:`, Array.from(selectedFeatures.value.keys()));
  
  // 必ず現在の選択状態に基づいてフィルタリングを実行
  if (selectedFeatures.value.size > 0) {
    console.log(`Applying multiple feature filter`);
    applyMultipleFeatureFilter();
  } else {
    console.log(`No features selected, showing all cards with color filter only`);
    fetch_cards_();
  }
  console.log(`=== End Feature Filter ===`);
};

const applyMultipleFeatureFilter = () => {
  if (!fetch_by_f_shifts_fn) {
    console.warn('WASM not initialized yet');
    return;
  }
  
  console.log(`--- Multiple Feature Filter (AND condition with ID intersection) ---`);
  console.log(`Selected features count: ${selectedFeatures.value.size}`);
  
  if (selectedFeatures.value.size === 0) {
    console.log(`No features selected, calling fetch_cards_() instead`);
    fetch_cards_();
    return;
  }
  
  // 各フィーチャーの結果をIDベースで交差させる
  let cardIdSets: Set<number>[] = [];
  
  for (const feature of selectedFeatures.value.values()) {
    const [shift1, shift2] = feature.bit_shift;
    console.log(`Getting cards for feature: ${feature.name}, shifts: [${shift1}, ${shift2}]`);
    
    // この特定のフィーチャーを持つカードを取得
    const featureCards = fetch_by_f_shifts_fn(shift1, shift2);
    console.log(`Feature ${feature.name} has ${featureCards.length} cards`);
    
    // IDのSetを作成
    const idSet = new Set<number>();
    featureCards.forEach((card: any) => {
      idSet.add(card.id);
    });
    
    cardIdSets.push(idSet);
  }
  
  // 全てのSetの交差を計算（AND条件）
  let intersectionIds = cardIdSets[0];
  for (let i = 1; i < cardIdSets.length; i++) {
    intersectionIds = new Set([...intersectionIds].filter(id => cardIdSets[i].has(id)));
    console.log(`After intersection with feature ${i}: ${intersectionIds.size} cards`);
  }
  
  console.log(`Final intersection: ${intersectionIds.size} card IDs`);
  
  // 交差するIDを持つカードのみを取得
  const allCards = fetch_by_f_bits_fn(0n, 0n);
  let filteredCards = allCards.filter((card: any) => intersectionIds.has(card.id));
  
  // カラーフィルターも適用
  if (color_bits.value > 0) {
    const beforeCount = filteredCards.length;
    filteredCards = filteredCards.filter((card: any) => (card.color & color_bits.value) === color_bits.value);
    console.log(`After color filter: ${filteredCards.length} (was ${beforeCount})`);
  }
  
  console.log(`Final card count (AND condition): ${filteredCards.length}`);
  card_store.set_cards(filteredCards);
  console.log(`--- End Multiple Feature Filter ---`);
};

// let gradient = (bits: number) => {return "";};
const gradient = ref<(bits: number) => string>(() => '');

// Wasm 実行
const runWasm = async () => {
  try {
    // Wasmパッケージを動的にインポート
    const {
      default: init, greet, say_goodbye, get_by_id,
      fetch_by_f_bits,
      fetch_by_f_shifts,
      feature_conditions,
      bits_to_gradient,
    } = await import('/static/pkg/datapack.js');

    // 初期化を呼び出し (WasmファイルのURLを暗黙的に指定)
    await init('/pkg/datapack_bg.wasm');

    console.log(say_goodbye())

    print_detail = (id) => {
      console.log(get_by_id(id));
    };

    // WASM関数への参照を保存
    fetch_by_f_bits_fn = fetch_by_f_bits;
    fetch_by_f_shifts_fn = fetch_by_f_shifts;

    fetch_cards = async (bit1: string, bit2: string, color_filter?: number) => {
      let cards = fetch_by_f_bits(BigInt(bit1), BigInt(bit2));
      if (color_filter && color_filter > 0) {
        cards = cards.filter((card: any) => (card.color & color_filter) === color_filter);
      }
      card_store.set_cards(cards);
      console.log(cards);
    };

    apply_bits = (shifts: [number, number]) => {
      let cards = fetch_by_f_shifts(shifts[0], shifts[1]);
      if (color_bits.value > 0) {
        cards = cards.filter((card: any) => (card.color & color_bits.value) === color_bits.value);
      }
      card_store.set_cards(cards);
    };

    gradient.value = (bits: number) => {
      const style = bits_to_gradient(bits);
      return style;
    };

    console.log(bits_to_gradient(34));
    conditions.value = feature_conditions();

    // 初期状態で全カードを取得
    await fetch_cards('0', '0');

    // Wasm関数を実行 (例: greet)
    message.value = greet('Nuxt');
  } catch (err) {
    console.error('Failed to load Wasm:', err);
    message.value = 'Error loading Wasm';
  }
};

onMounted(runWasm);

const f1 = computed(() => {
  return card_store.f_bits1;
})
const f2 = computed(() => {
  return card_store.f_bits2;
})

const conditions = ref(new Map());

type ColorName = 'white' | 'blue' | 'black' | 'red' | 'green' | 'colorless';
const white = ref(false);
const blue = ref(false);
const black = ref(false);
const red = ref(false);
const green = ref(false);
const colorless = ref(false);

const toggle_color = (color: ColorName) => {
  switch (color) {
    case 'white':
      white.value = !white.value;
      break;
    case 'blue':
      blue.value = !blue.value;
      break;
    case 'black':
      black.value = !black.value;
      break;
    case 'red':
      red.value = !red.value;
      break;
    case 'green':
      green.value = !green.value;
      break;
    case 'colorless':
      colorless.value = !colorless.value;
      break;
    default:
      break;
  }
  
  // カラー選択変更時に自動でフィルタリングを更新
  if (f1.value > 0 || f2.value > 0) {
    apply_bits([f1.value, f2.value]);
  } else {
    fetch_cards_();
  }
}

const clear_color = () => {
  white.value = false;
  blue.value = false;
  black.value = false;
  red.value = false;
  green.value = false;
  colorless.value = false;

  fetch_cards_();
}

const navBarRef = ref();

const clearAllFilters = () => {
  clear_color();
  selectedFeatures.value.clear();
  // 全カードを表示
  fetch_cards_();
}
</script>

<template lang="pug">
  .frame
    NavBar(ref="navBarRef" :conditions="conditions" :selectedFeatures="selectedFeatures" @emit-bits="apply_bits" @feature-toggle="applyFeatureFilter" @clear-filters="clearAllFilters")
    .filters-section
      .filters-header
        h3.section-title Color Filters
        button.clear-all-btn(@click="clearAllFilters") Clear All
      .color-filters
        ColorSelector(
          :white="white"
          :blue="blue"
          :black="black"
          :red="red"
          :green="green"
          :colorless="colorless"
          @toggle-color="toggle_color"
          @clear-color="clear_color"
        )
    .results-section
      .results-header
        span.count [ {{ card_store.cards_filtered.length }} items ]
      table
      colgroup
        col(style="width: 150px;")
        col(style="width: 400px;")
        col(style="width: 1450px;")
      thead
        tr
          th CODE
          th NAME
          th Skill
      tbody
        tr(v-for="card in card_store.cards_filtered" :key="card.id")
          td
            a(:href="`https://www.takaratomy.co.jp/products/wixoss/card_list.php?card=card_detail&card_no=${card.code}`" target="_blank") {{ card.code }}
          td.name(
            :style="`text-shadow: #fff 1px 1px 4px; ${gradient(card.color)}`"
            @click="print_detail(card.id)"
          ) {{ card.name }}
          td.skill
            SoftWrap.normal(:text="card.skill_text")
            SoftWrap.burst(:text="card.burst_text" v-if="card.has_burst == 2")
</template>

<style scoped lang="less">
@import "../assets/style/basic.less";

.frame {
  max-width: 100%;
  margin: 0 auto;
}

.filters-section {
  background-color: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
}

.filters-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-title {
  margin: 0;
  color: #495057;
  font-size: 18px;
  font-weight: 600;
}

.clear-all-btn {
  padding: 6px 12px;
  background-color: #dc3545;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 500;
  transition: background-color 0.2s ease;
  
  &:hover {
    background-color: #c82333;
  }
}

.color-filters {
  display: flex;
  align-items: center;
  justify-content: center;
}

.results-section {
  background-color: white;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.results-header {
  background-color: #f8f9fa;
  padding: 12px 16px;
  border-bottom: 1px solid #dee2e6;
}

.count {
  font-weight: 500;
  color: #495057;
  font-size: 14px;
}

table {
  width: 100%;
  table-layout: fixed;
  border-collapse: collapse;
  margin: 0;
}

th {
  padding: 12px 8px;
  border: 1px solid #dee2e6;
  background-color: #343a40;
  color: white;
  font-weight: 600;
  font-size: 14px;
  text-align: left;
}

td {
  padding: 8px;
  border: 1px solid #dee2e6;
  font-size: 13px;
  vertical-align: top;
}

td.name {
  font-weight: bolder;
}

td.skill {
  .burst {
    color: white;
    background-color: black;
    padding: 2px 4px;
    border-radius: 2px;
    font-size: 12px;
  }
  
  .normal {
    line-height: 1.4;
  }
}

a {
  color: #007bff;
  text-decoration: none;
  
  &:hover {
    text-decoration: underline;
  }
}
</style>