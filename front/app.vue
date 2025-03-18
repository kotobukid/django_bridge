<template>
  <v-app>
    <Suspense>
      <template #default>
        <NuxtPage />
      </template>
      <template #fallback>
        <span>loading...</span>
      </template>
    </Suspense>
  </v-app>

</template>
<script setup lang="ts">
</script>