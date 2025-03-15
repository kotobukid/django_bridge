<template lang="pug">
  v-app
    NuxtPage
</template>

<script setup lang="ts">
import {useCardStore} from "~/stores/card_js";
const card_store = useCardStore();
card_store.fetch();

provide('card_store', card_store);
</script>
