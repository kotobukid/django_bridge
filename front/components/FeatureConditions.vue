<template lang="pug">
  div(v-for="(features, index) in props.conditions" :key="tag")
    span {{ index }}
    div.a(v-for="feature in features" :key="feature.name")
      v-btn.b(v-for="f in feature" @click="emits('emit-bits', f.bit_shift)")
        label {{ f.tag }}
        span /
        label {{ f.name }}
        span /
        span.c(v-text="f.bit_shift")
</template>

<script lang="ts" setup>
import {ref} from "vue";

const props = defineProps({
  conditions: {
    type: Object,
    required: true,
  }
});

const emits = defineEmits<{
  (e: "emit-bits", bits: [number, number]): void
}>();

</script>

<style scoped lang="less">
.a {
  color: black;
}

.b {
  color: blue;
}

.c {
  color: red
}
</style>