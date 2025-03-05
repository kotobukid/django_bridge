<script setup lang="ts">
import {computed, ref, inject} from "vue";
import type {CardDataClient, Format} from '../types/card'
import {FORMAT} from "../constants";
import axios, {type AxiosResponse} from "axios";
import CardTableColumn from "./CardTableColumn.vue";
import PageController from "./PageController.vue";
import useGradientBg from "../composable/multi_color_gradient_bg";
import {useCardStore} from "../stores/cards";
import {useKeepStore} from "../stores/keep";
import {useColumnStore} from "../stores/columns";

const card_store = useCardStore();
const keep_store = useKeepStore();
const column_store = useColumnStore();
let worker = <Worker>inject('worker');

const filter_word = computed({
  get: () => {
    return card_store.filter_word;
  },
  set: (value: string) => {
    card_store.set_filter_word(value);
  },
});

const card_type = computed({
  get: () => {
    return card_store.card_type;
  },
  set: (value: string) => {
    card_store.set_card_type(value);
  }
});

const format = computed({
  get: () => {
    return card_store.format;
  },
  set: (value: Format) => {
    localStorage.setItem('filter.format', '' + value);
    card_store.set_format(value);
  }
});

const color = computed({
  get: () => {
    return card_store.color;
  },
  set: (value: string) => {
    card_store.set_color(value);
  },
})

const emits = defineEmits<{
  (e: "set-target", slug: string): void
}>();

const burst = computed({
  get: () => {
    return card_store.has_lb;
    // return !['シグニ', 'スペル', ''].includes(card_type.value) ? 0 : card_store.has_lb;
  },
  set: (value: 0 | 1 | 2) => {
    card_store.set_has_lb(value);
  }
});

const set_target = ({card, index}: { card: CardDataClient, index: number }) => {
  if (keep_direct.value) {
    keep_store.append(card);
  } else if (card_store.target === card.slug) {
    keep_store.append(card);
  }

  card_store.cache(card);           // 明示的にストックさせる
  emits('set-target', card.slug);
  cursor.value = index;
};

const icon = computed(() => {
  return (c: CardDataClient): string => {
    if (c.has_lb) {
      return 'lb';
    } else if (c.team_piece) {
      return 'tp';
    } else {
      return '';
    }
  }
});

card_store.install_worker(worker).then(() => {
  const _f: number = parseInt(localStorage.getItem('filter.format'), 10);

  let format = FORMAT.all;
  if (!isNaN(_f)) {
    // @ts-ignore
    format = Math.max(Math.min(_f, 3), 1)
  }

  keep_direct.value = sessionStorage.getItem('behavior.keep_direct') === '1';

  axios.get('/generated/cards.json').then((res: AxiosResponse<{ cards: CardDataClient[] }>) => {
    card_store.initialize_cards(res.data.cards, format);
  });
});

const _keep_direct = ref<boolean>(false);
const keep_direct = computed({
  set: (value: boolean) => {
    sessionStorage.setItem('behavior.keep_direct', value ? '1' : '0');
    _keep_direct.value = value;
  },
  get: (): boolean => {
    return _keep_direct.value;
  }
});

const cards = computed((): CardDataClient[] => {
  return card_store.paged_cards;
});

const cursor = computed({
  get: (): number => {
    return card_store.cursor;
  },
  set: (value: number) => {
    card_store.cursor = value;
  }
});

const {bg_gradient_style} = useGradientBg();
</script>

<template lang="pug">
  .left_side(style="width: 781px;")
    .conditions
      select.format.filter_select(v-model.number="format")
        option(value="1") オールスター
        option(value="2") キー
        option(value="3") ディーヴァ
      select.card_type.filter_select(v-model="card_type")
        option(value="") カードタイプ
        option(value="シグニ") シグニ
        option(value="スペル") スペル
        option(value="ルリグ") ルリグ
        option(value="センター") センタールリグ
        option(value="アシスト") アシストルリグ
        option(value="ピース") ピース
        option(value="キー") キー
        option(value="アーツ") アーツ
        option(value="レゾナ") レゾナ
        option(value="リレー") ピース（リレー）
        option(value="クラフト") レゾナ（クラフト）/アーツ（クラフト）
      select.burst_type.filter_select(v-model.number="burst")
        option(value="0") LB有無
        option(value="1") バーストあり
        option(value="2") バーストなし
      select.color_type.filter_select(v-model="color")
        option(value="") 色
        option(value="白") 白
        option(value="青") 青
        option(value="黒") 黒
        option(value="赤") 赤
        option(value="緑") 緑
        option(value="無") 無
        option(value=",") 多色
      input.filter_word(type="text" name="filter_word" v-model.lazy="filter_word")
    .actions
      a.check(href="#" @click.prevent="keep_direct = !keep_direct" :data-keep-direct="keep_direct" title="カード名を1クリックでカードをキープリストに投入する") ダイレクトキープ
      PageController
    table
      colgroup
        col(v-for="c in column_store.active_columns"
          :key="c.key"
          :style="`width: ${c.width}px;`")
      thead
        tr
          th(v-for="column in column_store.active_columns" :key="column.key") {{ column.label }}
      tbody(v-if="card_store.cards.length !== 0")
        tr.card(v-for="(c, $index) in cards" :key="c.slug" :data-current="cursor === $index ? 'yes': ''" :data-color="c.color" :style="bg_gradient_style(c.color)")
          CardTableColumn(:columns ="column_store.active_columns" :index="$index" :card="c" @set-target="set_target")
      tbody.not_found(v-if="card_store.cards.length === 0")
        tr
          td(colspan="7" style="border: 1px solid black;") 検索条件に合致するカードはありません。

</template>

<style scoped lang="less">
@import "../composable/colored_table_row.less";

table {
  table-layout: fixed;
  background-color: white;
  color: black;
}

tr {
  .colored_table_row();
}

.left_side, .right_side {
  float: left;
}

.left_side {
  width: 1000px;
}

.right_side {
  width: 500px;

  // ad hoc
  position: fixed;
  left: 820px;
}

.conditions {
  margin-bottom: 12px;
}

select.filter_select, input[type="text"].filter_word {
  font-family: inherit;
  font-size: 100%;
  line-height: 1.15;
  margin: 0 10px 0 0;
  padding: 0.5em 1em;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-sizing: border-box;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
}

select.card_type {
  width: 8rem;
}

select {
  background-image: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 5'><path fill='none' stroke='black' stroke-linecap='round' d='M2 1L1 3h2zm0 0L3 3H1z'/></svg>");
  background-repeat: no-repeat;
  background-position: right 0.5em center;
  background-size: 0.65em auto;

  &.deck_type {
    padding-right: 2rem;
  }
}

.actions {
  margin-bottom: 10px;
}

a.check {
  cursor: pointer;
  padding: 3px 10px 3px 5px;
  border-radius: 3px;
  text-decoration: none;

  &:before {
    display: inline-block;
    width: 1rem;
    font-size: 1rem;
    line-height: 1rem;
  }

  &[data-keep-direct="true"] {
    color: #0c7251;
    background-color: lightgreen;
    border: 1px solid green;

    &:before {
      content: '✓';
    }
  }

  &[data-keep-direct="false"] {
    color: black;
    background-color: grey;
    border: 1px solid #232323;

    &:before {
      content: ' ';
    }
  }
}
</style>