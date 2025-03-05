<script setup lang="ts">
import {computed} from "vue";

const props = defineProps({
  skill: String
});

const skill_type = computed(() => {
  const skill = (props.skill || '');
  if (skill.indexOf('ライフバースト') === 0) {
    return 'lb';
  } else if (skill.indexOf('【出】') === 0) {
    return 'cip';
  } else if (skill.indexOf('【自】') === 0) {
    return 'passive';
  } else if (skill.indexOf('【常】') === 0) {
    return 'auto';
  } else if (skill.indexOf('【起】') === 0) {
    return 'trigger';
  } else if (skill.indexOf('gs[') === 0) {
    return 'gain';
  }
});

const skill_trimmed = computed(() => {
  return (props.skill || '').replace(/^(ライフバースト|【出】|【自】|【常】|【起】|gs\[)/, '');
})

</script>

<template lang="pug">
  .skill(:data-skill-type="skill_type") {{ skill_trimmed }}

</template>

<style scoped lang="less">
.skill {
  text-align: left;
  margin-bottom: 5px;
  line-height: 1.5rem;
  background-color: #d5d5d5;
  border-radius: 10px;
  padding: 10px;

  &:last-child {
    margin-bottom: 0;
  }

  &[data-skill-type="lb"] {
    background-color: #1a1a1a;
    color: white;

    &:before {
      display: inline-block;
      content: url('/lb_white_wrapped.svg');
      width: 1rem;
      height: 1rem;
      position: relative;
      top: 2px;
      margin-right: -2px;
    }
  }

  &[data-skill-type="cip"] {
    &:before {
      display: inline-block;
      content: url('/cip.svg');
      width: 1.4rem;
      height: 1.4rem;
      position: relative;
      top: 3px;
      margin-right: 0;
    }
  }

  &[data-skill-type="passive"] {
    &:before {
      display: inline-block;
      content: url('/passive.svg');
      width: 1.4rem;
      height: 1.4rem;
      position: relative;
      top: 3px;
      margin-right: 0;
    }
  }

  &[data-skill-type="auto"] {
    &:before {
      display: inline-block;
      content: url('/auto.svg');
      width: 1.4rem;
      height: 1.4rem;
      position: relative;
      top: 3px;
      margin-right: 0;
    }
  }

  &[data-skill-type="trigger"] {
    &:before {
      display: inline-block;
      content: url('/trigger.svg');
      width: 1.4rem;
      height: 1.4rem;
      position: relative;
      top: 3px;
      margin-right: 0;
    }
  }

  &[data-skill-type="gain"] {
    border: 2px solid black;
    border-radius: 0;
  }
}
</style>