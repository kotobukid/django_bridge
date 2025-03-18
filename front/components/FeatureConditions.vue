<template lang="pug">
  div(v-for="key_name in props.conditions.keys()" :key="tag")
    span {{ key_name }}
    button.small-button.condition(v-for="feature in props.conditions.get(key_name)" :key="feature.name" @click="emits('emit-bits', feature.bit_shift)") {{ feature.name }}
</template>

<script lang="ts" setup>

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
.condition {
  margin-right: 1rem;
  margin-bottom: 0.5rem;
}
.small-button {
  cursor: pointer;
  &:hover {
    background-color: #aaaaff;
  }
  &:before {
    content: "[";
  }
  &:after {
    content: "]";
  }
}
</style>