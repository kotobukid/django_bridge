<template lang="pug">
  .eps_editor
    table(v-if="card")
      colgroup
        col(style="width: 200px;")
        col(style="width: 400px;")
      thead
        tr
          th 項目
          th パース値
      tbody
        tr
          th slug
          td
            router-link(:to="`/card/${card.slug}`") {{ card.slug }}
        tr
          th name
          td(v-html="card.name")
        tr
          th card_type
          td {{ card.card_type }}
        tr.skills(v-for="(skill, $index) in skill_list")
          th skill {{ $index + 1 }}
          td {{ skill }}

    .actions
      a.button(href="#" @click.prevent="new_eps") 新規
      a.button(href="#" @click.prevent="set_value('skills')") ↓効果テキスト

    table(v-if="epss.length > 0" style="width: 600px;")
      colgroup
        col(style="width: 30px;")
        col(style="width: 500px;")
        col(style="width: 70px;")
      thead
        tr
          th ID
          th Json
          th
      tbody
        tr(v-for="(eps, $index) in epss")
          td.center {{ eps.id }}
          td.center
            textarea.eps_skill(@blur="update_eps($index, $event)") {{ eps.json }}
          td.center
            a.button(href="#" @click.prevent="submit_eps($index)") 保存
</template>

<script setup lang="ts">
import axios, {type AxiosResponse} from "axios";
import {onMounted, ref, type Ref} from "vue";
import type {CardDataClient} from '../../../../ex/types/card';
import type {EPS} from "../../../../ex/types/card";

type Props = {
  slug: { type: string, required: true, default: '' }
}

const props = defineProps<Props>();

const card!: Ref<CardDataClient | null> = ref(null);
const epss: Ref<EPS[]> = ref([]);
const skill_list: Ref<string[]> = ref([]);

onMounted(() => {
  if (props.slug) {
    axios.get(`/api/admin/card_detail/${props.slug}`).then((res: AxiosResponse<{
      card: CardDataClient,
      epss: EPS[]
    }>) => {
      card.value = res.data.card;
      epss.value = res.data.epss;
      skill_list.value = res.data.card.skills.split('@@');
    });
  } else {
    alert('slug not found');
  }
});

const set_value = (key: keyof CardDataClient): void => {
  const content: Partial<CardDataClient> = {[key]: card.value![key]};
  epss.value = [...epss.value, {
    id: -1,
    json: JSON.stringify(content),
    slug: card.value!.slug,
    method: 'extend'
  }];
}

const submit_eps = (index: number): void => {
  const post_data: EPS = epss.value[index];
  axios.post('/api/admin/update_eps', {eps: post_data}).then((res: AxiosResponse<{ epss: EPS[] }>) => {
    epss.value = res.data.epss;
    alert('保存されました');
  });
};

const new_eps = () => {
  epss.value = [...epss.value, {
    id: -1,
    json: "{}",
    slug: card.value!.slug,
    method: 'extend'
  }];
};

const update_eps = ($index: number, event: { target: { value: string } }) => {
  epss.value = epss.value.map((eps: EPS, index: number) => {
    if (index === $index) {
      eps.json = event.target.value;
      return eps;
    } else {
      return eps;
    }
  });
}

</script>

<style scoped lang="less">
@import "../../composable/button.less";

.eps_editor {

}

table {
  table-layout: fixed;
  border-collapse: collapse;
}

th, td {
  border: 1px solid black;
  padding: 5px;
}

th {
  background-color: #272727;
  color: white;
}

.skills textarea {
  width: 380px;
  height: 80px;
  line-height: 1.2rem;
}

textarea.eps_skill {
  width: 480px;
  height: 90px;
}

a.button {
  .small_button();
}
</style>