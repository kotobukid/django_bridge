<template lang="pug">
  .feature-group(v-for="key in keys_with_removed_prefix" :key="key.original")
    span.group-name {{ key.display }}
    button.small-button.condition(v-for="feature in props.conditions.get(key.original)" :key="feature.name" @click="emits('emit-bits', feature.bit_shift)") {{ feature.name }}
</template>

<script lang="ts" setup>

const props = defineProps({
  conditions: {
    type: Map,
    required: true,
  }
});

const keys_with_removed_prefix = computed(() => {
  const keys = Array.from(props.conditions.keys()) as Array<string>;

  // ソートする（キーの順序に基づく）
  keys.sort((a, b) => a.localeCompare(b));

  // キーごとに、元のキーと表示用のキーを保持した新しい構造を作成
  return keys.map((key) => ({
    original: key,        // 元のキー（Mapから値を検索する用）
    display: key.substring(2) // 表示用に先頭2文字を削除
  }));
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