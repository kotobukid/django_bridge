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
    console.log(data.data);
    if (data.data.success) {
      admin_alive.value = true;
      if (data.data.entry) {
        admin_entry.value = "/" + data.data.entry;
      }
    }
  });
}

const stopAdmin = () => {
  axios.post("/admin_operation/api/stop_admin.json").then((data: AxiosResponse<DjangoStartResult>) => {
    if (data.data.success) {
      admin_alive.value = false;
      admin_entry.value = "";
    }
    console.log(
        data.data
    )
  });
}

</script>

<template lang="pug">
  .admin-handler
    h1 Admin Handler
    v-btn(@click="startAdmin" color="primary") Start Admin
    v-btn(@click="stopAdmin" color="brown") Stop Admin
    br
    a(:href="admin_entry" target="_blank" v-if="admin_alive") {{ admin_entry }}
</template>

<style scoped lang="less">

</style>