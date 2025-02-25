<script setup lang="ts">
import axios, {type AxiosResponse} from 'axios';
import {ref, type Ref} from 'vue';

const cards: Ref<{ name: string }[]> = ref([]);

axios.get("/card/api/card_list.json").then((data: AxiosResponse<{ cards: { name: string }[] }>) => {
  cards.value = data.data.cards;
});

</script>

<template lang="pug">
  div
    h1 card list
    ul
      li(v-for="card in cards") {{ card.name }}
</template>
