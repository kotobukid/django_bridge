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
let fetch_cards = async (bit1: string, bit2: string) => {};
const fetch_cards_ = async () => {
  await fetch_cards(`${f1.value}`, `${f2.value}`);
};

let apply_bits = (a: [number, number]) => {
  // card_store.set_f1(bit1);
  //   card_store.set_f2(bit2);
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

    fetch_cards = async (bit1: string, bit2: string) => {
      let cards = fetch_by_f_bits(BigInt(bit1), BigInt(bit2));
      console.log(cards)
    };

    apply_bits = (shifts: [number, number]) => {
      let cards = fetch_by_f_shifts(shifts[0], shifts[1]);
      card_store.set_cards(cards);
    };

    gradient.value = (bits: number) => {
      const style = bits_to_gradient(bits);
      return style;
    };

    console.log(bits_to_gradient(34));
    conditions.value = feature_conditions();

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
}
</script>

<template lang="pug">
  .frame
    NavBar
    ColorSelector(
      :white="white"
      :blue="blue"
      :black="black"
      :red="red"
      :green="green"
      :colorless="colorless"
      @toggle-color="toggle_color"
    )
    FeatureConditions(:conditions="conditions" @emit-bits="apply_bits")
    span.count [ {{ card_store.cards_filtered.length }} items ]
    //span.color_bits {{ color_bits }}
    table
      colgroup
        col(style="width: 200px;")
        col(style="width: 400px;")
        col(style="width: 1400px;")
      thead
        tr
          th CODE
          th NAME
          th Skill
      tbody
        tr(v-for="card in card_store.cards_filtered" :key="card.id")
          td
            a(:href="`https://www.takaratomy.co.jp/products/wixoss/card_list.php?card=card_detail&card_no=${card.code}`" target="_blank") {{ card.code }}
          td.name(:style="`text-shadow: #fff 1px 1px 4px; ${gradient(card.color)}`") {{ card.name }}
          td.skill
            SoftWrap.normal(:text="card.skill_text")
            SoftWrap.burst(:text="card.burst_text")
</template>

<style scoped lang="less">
@import "../assets/style/basic.less";

table {
  table-layout: fixed;
  border-collapse: collapse;
}

th {
  padding: 3px;
  border: 1px solid #ffffff;
  background-color: #2b2b2b;
  color: white;
}

td {
  padding: 3px;
  border: 1px solid black;
}

td.name {
  font-weight: bolder;
}

td.skill {
  .burst {
    color: white;
    background-color: black;
  }
}

span.count {
  display: inline-block;
  width: 160px;
  margin-right: 10px;
}

input.feature {
  width: 140px;
  border: 1px solid black;
  padding: 2px;
  font-size: 1.2rem;
}
</style>