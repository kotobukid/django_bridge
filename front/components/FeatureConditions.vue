<template lang="pug">
  .feature-group(v-for="key_name in props.conditions.keys()" :key="tag")
    span.group-name {{ key_name }}
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
.feature-group {
  border: 1px solid grey;
  margin-bottom: 4px;
  padding: 2px 8px 1px 8px;
  background-color: white;
  border-radius: 4px;
}

.group-name {
  font-weight: bolder;
  margin-right: 1rem;
}

.condition {
  margin-right: 1rem;
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