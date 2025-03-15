<script setup lang="ts">
import axios, {type AxiosResponse} from 'axios';
import {ref, type Ref} from 'vue';

const cards: Ref<{ name: string }[]> = ref([]);

axios.get("/card/api/list.json").then((data: AxiosResponse<{ cards: { name: string }[] }>) => {
  cards.value = data.data.cards;
});

</script>

<template lang="pug">
  div
    h1 card list
    table
      colgroup
        col(width="160px")
        col(width="400px")
        col(width="100px")
      thead
        tr
          th CODE
          th NAME
          th COST
      tbody
        tr(v-for="card in cards" :data-color="card.color" :key="card.code")
          td {{ card.code }}
          td {{ card.name }}
          td.center {{ card.cost }}
</template>


<style scoped lang="less">
table {
  table-layout: fixed;
  border-collapse: collapse;
}

th {
  background-color: #2b2b2b;
  color: white;
}

th, td {
  &.center {
    text-align: center;
  }

  padding: 1px;
  border: 1px solid black;
}

.gradient-background(@color1, @color2) {
  background: @color1; // フォールバック用の単色
  background: linear-gradient(45deg, @color1 30%, @color2 70%); // グラデーション
}

@white: #fff6e3;
@red: #ffa3a3;
@blue: #9d9dff;
@green: #aeffb8;
@black: #be86ff;
@colorless: #d6d6d6;
@no_color: #fff;

tr {
  &[data-color="128"] {
    background-color: @no_color;
  }
  &[data-color="64"] {
    background-color: @colorless;
  }
  &[data-color="2"] {
    background-color: @white;
  }
  &[data-color="4"] {
    background-color: @blue;
  }
  &[data-color="8"] {
    background-color: @red;
  }
  &[data-color="16"] {
    background-color: @black;
  }
  &[data-color="32"] {
    background-color: @green;
  }

  &[data-color="6"] {
    .gradient-background(@white, @blue);
  }

  &[data-color="18"] {
    .gradient-background(@black, @white);
  }

  &[data-color="48"] {
    .gradient-background(@green, @black);
  }
}
</style>