<template lang="pug">
  .drag_layer(
    @mouseup="mu"
    @mousemove="mm"
    @mouseleave="mu"
    v-if="is_dragging"
  )
</template>

<script setup lang="ts">
import {computed} from "vue";
import {useWindowStore} from "../stores/window";

const window_store = useWindowStore();

const mu = (): void => {
  window_store.release_dragging_target();
};

const mm = (e: MouseEvent): void => {
  window_store.move(
      {
        x: e.movementX,
        y: e.movementY
      });
};

const is_dragging = computed(() => {
  return window_store.dragging_target !== '';
});
</script>

<style scoped lang="less">
.drag_layer {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1200;

  background-color: transparent;

  width: 100vw;
  height: 100vh;
  overflow-x: hidden;
  overflow-y: hidden;
}
</style>