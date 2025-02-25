<script setup lang="ts">
import axios, {type AxiosResponse} from 'axios';
import {ref} from 'vue';

type DjangoStartResult = {
  success: boolean,
  entry?: string,
}

const admin_alive = ref(false);
const admin_entry = ref("");

const startAdmin = () => {
  axios.post("/admin_operation/api/start_admin.json").then((data: AxiosResponse<DjangoStartResult>) => {
    if (data.data.success) {
      // @ts-ignore
      timer.value = setInterval(health_check, 2000) as number;
      if (data.data.entry) {
        admin_entry.value = "/" + data.data.entry;
      }
    }
  });
}

const stopAdmin = () => {
  axios.post("/admin_operation/api/stop_admin.json").then((data: AxiosResponse<DjangoStartResult>) => {
    if (data.data.success) {
      admin_entry.value = "";
    }
    console.log(
        data.data
    )
  });
}

const health_check = () => {
  axios.get("/admin_proxy/health-check", {
    timeout: 1000,
  }).then((data: AxiosResponse<string>) => {
    admin_alive.value = (data.data == 'OK');
  }).catch((_err) => {
    admin_alive.value = false;
  })
}

health_check();
const timer: Ref<number> = ref(-1);
// @ts-ignore
timer.value = setInterval(health_check, 2000) as number;

const toggle_health_check = () => {
  if (timer.value != -1) {
    clearInterval(timer.value);
    timer.value = -1;
  } else {
    // @ts-ignore
    timer.value = setInterval(health_check, 2000) as number;
  }
}
</script>

<template lang="pug">
  .admin-handler
    h1 Admin Handler
    v-btn(@click="startAdmin" v-if="!admin_alive" color="primary") Start Admin
    v-btn(@click="stopAdmin" v-if="admin_alive" color="brown") Stop Admin
    br
    a(href="#" @click.prevent="toggle_health_check")
      span(v-if="timer === -1") [health check inactive]
      span(v-else) [health check active]
    br
    a(:href="admin_entry" target="_blank" v-if="admin_alive && admin_entry") {{ admin_entry }}
</template>

<style scoped lang="less">
.admin-handler {
  border: 1px solid grey;
  padding: 10px;
  width: 500px;
}
</style>