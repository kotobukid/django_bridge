<script setup lang="ts">
import {ref} from 'vue';

// 表示用のメッセージ
const message = ref('default');

// Wasm 実行
const runWasm = async () => {
  try {
    // Wasmパッケージを動的にインポート
    const { default: init, greet } = await import('/static/pkg/datapack.js');

    // 初期化を呼び出し (WasmファイルのURLを暗黙的に指定)
    await init('/pkg/datapack_bg.wasm');


    // Wasm関数を実行 (例: greet)
    message.value = greet('Nuxt');
  } catch (err) {
    console.error('Failed to load Wasm:', err);
    message.value = 'Error loading Wasm';
  }
};

onMounted(runWasm);

</script>

<template lang="pug">
  span {{ message }}
  .frame
    NavBar
</template>

<style scoped lang="less">
@import "../assets/style/basic.less";
</style>