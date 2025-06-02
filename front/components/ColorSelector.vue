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
  (e: "toggle-color", color: string): void,
  (e: "clear-color"): void
}>();

const toggle_color = (color: string): void => {
  emits("toggle-color", color);
}

const clear_color = (): void => {
  emits("clear-color");
}
</script>

<template lang="pug">
  .color_selector
    span.color.white(:data-active="props.white" @click="toggle_color('white')") 白
    span.color.blue(:data-active="props.blue" @click="toggle_color('blue')") 青
    span.color.black(:data-active="props.black" @click="toggle_color('black')") 黒
    span.color.red(:data-active="props.red" @click="toggle_color('red')") 赤
    span.color.green(:data-active="props.green" @click="toggle_color('green')") 緑
    span.color.rem2.colorless(:data-active="props.colorless" @click="toggle_color('colorless')") 無色
    span.color.rem3(@click="clear_color") クリア

</template>

<style scoped lang="less">
span.color {
  display: inline-block;
  width: 2rem;
  &.rem2 {
    width: 4rem;
  }
  &.rem3 {
    width: 6rem;
  }
  height: 2rem;
  line-height: 2rem;
  text-align: center;
  cursor: pointer;
  margin: 2px;
  border: 2px solid transparent;
  border-radius: 4px;
  transition: all 0.2s ease;
  background-color: #f5f5f5;
  
  &:hover {
    border-color: #ccc;
    background-color: #e0e0e0;
  }

  &[data-active="true"] {
    font-weight: bolder;
    border-color: #1976d2;
    background-color: #e3f2fd;
    color: #1976d2;
    box-shadow: 0 2px 4px rgba(25, 118, 210, 0.3);
  }

  // 各色のテーマカラー
  &.white {
    &[data-active="true"] {
      border-color: #e0e0e0;
      background-color: #ffffff;
      color: #333;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    }
  }

  &.blue {
    &[data-active="true"] {
      border-color: #1976d2;
      background-color: #e3f2fd;
      color: #1976d2;
      box-shadow: 0 2px 4px rgba(25, 118, 210, 0.3);
    }
  }

  &.black {
    &[data-active="true"] {
      border-color: #424242;
      background-color: #424242;
      color: #ffffff;
      box-shadow: 0 2px 4px rgba(66, 66, 66, 0.4);
    }
  }

  &.red {
    &[data-active="true"] {
      border-color: #d32f2f;
      background-color: #ffebee;
      color: #d32f2f;
      box-shadow: 0 2px 4px rgba(211, 47, 47, 0.3);
    }
  }

  &.green {
    &[data-active="true"] {
      border-color: #388e3c;
      background-color: #e8f5e8;
      color: #388e3c;
      box-shadow: 0 2px 4px rgba(56, 142, 60, 0.3);
    }
  }

  &.colorless {
    &[data-active="true"] {
      border-color: #9e9e9e;
      background-color: #f5f5f5;
      color: #616161;
      box-shadow: 0 2px 4px rgba(158, 158, 158, 0.3);
    }
  }
}
</style>