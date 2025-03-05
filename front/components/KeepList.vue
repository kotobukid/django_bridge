<script setup lang="ts">
import {useKeepStore, type KeptCard} from "../stores/keep";
import {computed} from "vue";
import useGradientBg from "../composable/multi_color_gradient_bg";

const keep_store = useKeepStore();

const increase = (pronounce: string, group: 'main_lb' | 'main_no_lb' | 'white' | 'others', delta: 1 | -1): void => {
  keep_store.increase(pronounce, group, delta);
};

const remove = (pronounce: string, group: 'main_lb' | 'main_no_lb' | 'white' | 'others'): void => {
  keep_store.remove(pronounce, group);
};

const trim = () => {
  const do_trim = confirm('枚数に応じてカードリストを整理してもよろしいですか？');
  if (do_trim) {
    keep_store.trim();
  }
};

const amount = computed(() => {
  return (amount: number): string => {
    if (amount === -1) {
      return '×';
    } else if (amount === 0) {
      return '-';
    } else {
      return '' + amount;
    }
  }
});

const total = computed(() => {
  return (cards: KeptCard[]): number => {
    return (cards || []).reduce((t: number, c: KeptCard) => {
      return Math.max(0, c.amount) + t;
    }, 0);
  };
});

const lrig_deck_over = computed((): 'over' | '' => {
  let lv0_amount = 0;
  const _total = (keep_store.white || []).reduce((t: number, c: KeptCard) => {
    if (c.level === '0') {
      lv0_amount = lv0_amount + 1;
    }
    return Math.max(0, c.amount) + t;
  }, 0);
  if (lv0_amount === 3) {
    return _total > 12 ? 'over' : '';
  } else {
    return _total > 10 ? 'over' : '';
  }
});

const name = computed({
  get: () => {
    return keep_store.name;
  },
  set: (value: string) => {
    keep_store.name = value;
  }
});

const save = () => {
  keep_store.save_deck(100);
};

const {bg_gradient_style} = useGradientBg();
</script>

<template lang="pug">
  .keep_list(v-if="keep_store")
    .info
      input(type="text" v-model.lazy="name")
    .actions
      a.small(href="#" @click.prevent="save") 保存
      a.small(href="#" @click.prevent="trim" title="枚数が-になっているものを「その他」カテゴリに移動し、×になっているものをリストから削除します") トリム
    .top
      table.keep_list
        colgroup
          col(style="width: 20px;")
          col(style="width: 230px;")
        tbody
          tr
            th.center
              span.amount(:data-over="total(keep_store.main_lb) > 20 ? 'over': ''" v-text="total(keep_store.main_lb)")
            th メインデッキ(LBあり)
          tr(v-if="keep_store" v-for="card of keep_store.main_lb" :key="card.slug" :data-color="card.color" :style="bg_gradient_style(card.color)")
            td.right
              span.amount(v-text="amount(card.amount)")
            td.label_action
              span [LB] {{ card.name }}
              .actions_layered
                a.button.increase(href="#" @click.prevent="increase(card.pronounce, 'main_lb', 1)" title="1枚増やす")
                a.button.decrease(href="#" @click.prevent="increase(card.pronounce, 'main_lb', -1)" title="1枚減らす")
                a.button.remove(href="#" @click.prevent="remove(card.pronounce, 'main_lb')" title="取り除く")
      table.keep_list
        colgroup
          col(style="width: 20px;")
          col(style="width: 230px;")
        tbody
          tr
            th.center
              span.amount(v-text="total(keep_store.main_no_lb)")
            th メインデッキ(LBなし)
          tr(v-if="keep_store" v-for="card of keep_store.main_no_lb" :key="card.slug" :data-color="card.color" :style="bg_gradient_style(card.color)")
            td.right
              span.amount(v-text="amount(card.amount)")
            td.label_action
              span {{ card.name }}
              .actions_layered
                a.button.increase(href="#" @click.prevent="increase(card.pronounce, 'main_no_lb', 1)" title="1枚増やす")
                a.button.decrease(href="#" @click.prevent="increase(card.pronounce, 'main_no_lb', -1)" title="1枚減らす")
                a.button.remove(href="#" @click.prevent="remove(card.pronounce, 'main_no_lb')" title="取り除く")
      br.clearfix
    .bottom
      table.keep_list
        colgroup
          col(style="width: 20px;")
          col(style="width: 230px;")
        tbody
          tr
            th.center
              span.amount(:data-over="lrig_deck_over" v-text="total(keep_store.white)")
            th ルリグデッキ
          tr(v-if="keep_store" v-for="card of keep_store.white" :key="card.slug" :data-color="card.color" :style="bg_gradient_style(card.color)")
            td.right
              span.amount(v-text="amount(card.amount)")
            td.label_action
              span [白枠] {{ card.name }}
              .actions_layered
                a.button.increase(href="#" @click.prevent="increase(card.pronounce, 'white', 1)" title="1枚増やす")
                a.button.decrease(href="#" @click.prevent="increase(card.pronounce, 'white', -1)" title="1枚減らす")
                a.button.remove(href="#" @click.prevent="remove(card.pronounce, 'white')" title="取り除く")
      table.keep_list
        colgroup
          col(style="width: 20px;")
          col(style="width: 230px;")
        tbody(v-if="keep_store.others.length > 0")
          tr
            th
            th その他
          tr(v-if="keep_store" v-for="card of keep_store.others" :key="card.slug" :data-color="card.color" :style="bg_gradient_style(card.color)")
            td.right
              span.amount(v-text="amount(card.amount)")
            td.label_action
              span [他] {{ card.name }}
              .actions_layered
                a.button.increase(href="#" @click.prevent="increase(card.pronounce, 'others', 1)" title="1枚増やす")
                a.button.decrease(href="#" @click.prevent="increase(card.pronounce, 'others', -1)" title="1枚減らす")
                a.button.remove(href="#" @click.prevent="remove(card.pronounce, 'others')" title="取り除く")
      br.clearfix
</template>

<style scoped lang="less">
@import "../composable/colored_table_row.less";
@import "../composable/button.less";

.keep_list {
  width: 520px;
  background-color: white;

  table {
    width: 250px;
    float: left;
  }
}

.top {
  margin-bottom: 5px;
  min-height: 30px;
}

.actions {
  margin-bottom: 4px;
}

table {
  table-layout: fixed;
  border-collapse: collapse;

  &:first-child {
    margin-right: 4px;
  }
}

tr {
  color: black;
  height: 28px;
  .colored_table_row();
}

td.label_action {
  span {
    font-size: 1rem;
    height: 21px;
    display: inline-block;
    line-height: 21px;
  }

  .actions_layered {
    opacity: 0.8;
    display: none;
  }

  &:hover {
    span {
      display: none;
    }

    .actions_layered {
      display: block;
    }
  }
}

span.amount {
  margin-right: 0.3rem;

  &[data-over="over"] {
    color: #ff4949;
  }
}

a.small {
  .small_button();
}

a.button {
  min-width: 24px;
  height: 21px;
  text-align: center;
  display: inline-block;
  border: 1px solid grey;
  border-radius: 4px;
  background-color: white;
  margin: 0 2px 0 0;

  &:last-child {
    margin-right: 0;
  }

  &:active {
    position: relative;
    top: 1px;
  }

  &:before {
    display: inline-block;
    width: 1rem;
    height: 1rem;
    position: relative;
    top: 2px;
  }

  &.increase {
    &:hover {
      background-color: #d6ffd6;
    }

    &:before {
      content: url('/plus.svg');
    }
  }

  &.decrease {
    &:hover {
      background-color: pink;
    }

    &:before {
      content: url('/minus.svg');
    }
  }

  &.remove {
    &:hover {
      background-color: #6d6d6d;
    }

    &:before {
      content: url('/remove.svg');
    }
  }
}
</style>