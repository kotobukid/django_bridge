<template>
  <div class="mb-6">
    <form @submit.prevent="handleSubmit" class="flex gap-2">
      <input
        v-model="keyword"
        type="text"
        placeholder="検索キーワードを入力（例：手札に加える）"
        class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        :disabled="loading"
      />
      <button
        type="submit"
        :disabled="!keyword || loading"
        class="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {{ loading ? '検索中...' : '検索' }}
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  loading: boolean
}>()

const emit = defineEmits<{
  search: [keyword: string]
}>()

const keyword = ref('')

const handleSubmit = () => {
  if (keyword.value && !props.loading) {
    emit('search', keyword.value)
  }
}
</script>