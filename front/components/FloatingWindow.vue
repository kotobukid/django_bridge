<template lang="pug">
  .floating_window(:style="pos_text")
    .title.handler(@mousedown="md") {{ props.title }}
    slot
</template>

<script setup lang="ts">
import {useWindowStore, type WindowInfo} from "../stores/window";
import {computed} from "vue";

const window_store = useWindowStore();

const props = defineProps({
  id: {type: String, required: true},
  title: String
});

const info = computed((): WindowInfo => {
  return window_store.windows.get(props.id!);
});

const pos_text = computed(() => {
  return `top: ${info.value.y}px; left: ${info.value.x}px; z-index: ${info.value.z};`;
});

const md = (): void => {
  window_store.set_to_top(props.id!);
};
</script>

<style scoped lang="less">
.floating_window {
  border: 1px solid grey;
  border-radius: 5px;
  background-color: white;
  padding: 5px;

  position: fixed;
}

.title {
  font-weight: bolder;
}

.handler {
  user-select: none;
  background-color: lightblue;
  cursor: move;
}
</style>