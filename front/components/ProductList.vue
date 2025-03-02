<script setup lang="ts">
import {type Product} from "~/types";
import {ref, onMounted} from "vue";
import axios, {type AxiosResponse} from "axios";

const products = ref<Product[]>([]);

onMounted(() => {
  axios.get("/product/api/list.json").then((data: AxiosResponse<{ products: Product[] }>) => {
    products.value = data.data.products.sort((a, b) => a.sort_asc > b.sort_asc ? 1 : -1);
  });
});


</script>

<template lang="pug">
  v-table
    //colgroup
      col(style="width: 100px;")
      col(style="width: 200px;")
    thead
      tr
        th Product Code
        th Name
        th Product page
    tbody
      tr(v-for="product in products" :key="product.id")
        td {{ product.product_code }}
        td {{ product.name }}
        td
          a(:href="product.url" target="_blank" v-if="product.url") {{ product.url }}

</template>

<style scoped lang="less">

</style>