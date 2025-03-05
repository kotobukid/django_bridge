<template lang="pug">
  .page_controller
    a.page_link(href="#" @click.prevent="set_page(0)") ⇤
    a.page_link(v-for="page in pages" @click.prevent="set_page(page)" :data-current="page === card_store.page ? 'yes': ''") {{ page + 1 }}
    a.page_link(href="#" @click.prevent="set_max_page") ⇥
    span.amount(v-if="card_store.cards" v-text="`${card_store.cards.length} items`")
</template>

<script setup lang="ts">
import {useCardStore} from "../stores/cards";
import useDetectCard from "../composable/detect_card";
import {computed, type ComputedRef} from "vue";

const card_store = useCardStore();

const total: ComputedRef<number> = computed((): number => {
  return card_store.cards.length;
});

const max_page = computed(() => {
  return Math.ceil(total.value / card_store.cards_per_page);
});

const set_max_page = () => {
  card_store.set_page(max_page.value - 1)
};

const pages = computed(() => {
  let start = Math.max(card_store.page - 2, 0); //カレントページから2を減算し、その結果が0より小さければ0を採用
  let end = Math.min(start + 5, max_page.value); // startから5ページ分だけ取得するが、max_pageを超えないように調整
  start = Math.max(end - 5, 0); // max_pageに達してしまい、5ページ取得できなかった場合の調整
  return Array.from({length: end - start}, (_, i) => i + start);
});

const {by_whole_index} = useDetectCard();

const set_page = (page: number): void => {
  const next_card_index = page * card_store.cards_per_page;
  by_whole_index(next_card_index);
  card_store.set_page(page);
};

</script>

<style scoped lang="less">
.page_controller {
  float: right;

  height: 1rem;
  line-height: 1rem;
}

a.page_link {
  user-select: none;
  cursor: pointer;
  display: inline-block;
  padding: 2px 0.5rem;

  &:hover {
    text-decoration: underline;
    background-color: lightblue;
  }

  &[data-current="yes"] {
    color: red;
    background-color: pink;
  }
}

span.amount {
  display: inline-block;
  line-height: 1rem;
  font-size: 1rem;
  width: 100px;
}
</style>