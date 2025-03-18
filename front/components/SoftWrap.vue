<script setup lang="ts">
import {computed} from "vue";

const props = defineProps<{text: string}>();

const broken = computed(() => {
  return props.text.replace(/\n/g, '<br />');
})
</script>

<template lang="pug">
div(v-html="broken")
</template>