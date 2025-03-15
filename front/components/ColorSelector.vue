<script setup lang="ts">
import {ref} from 'vue';

const props = defineProps({
  white: Boolean,
  blue: Boolean,
  black: Boolean,
  red: Boolean,
  green: Boolean,
  colorless: Boolean,
});

const white = 2;
const blue = 4;
const black = 16;
const red = 8;
const green = 32;
const colorless = 64;

const emits = defineEmits<{
  (e: "toggle-color", color: string): void
}>();

const toggle_color = (color: string): void => {
  emits("toggle-color", color);
}
</script>

<template lang="pug">
  .color_selector
    span.color(:data-active="props.white" @click="toggle_color('white')") 白
    span.color(:data-active="props.blue" @click="toggle_color('blue')") 青
    span.color(:data-active="props.black" @click="toggle_color('black')") 黒
    span.color(:data-active="props.red" @click="toggle_color('red')") 赤
    span.color(:data-active="props.green" @click="toggle_color('green')") 緑
    span.color(:data-active="props.colorless" @click="toggle_color('colorless')") 無色

</template>

<style scoped lang="less">
span.color {
  display: inline-block;
  width: 2rem;
  text-align: center;
  cursor: pointer;

  &[data-active="true"] {
    font-weight: bolder;
  }
}
</style>