<template>
  <div class="mb-6 space-y-4">
    <div class="flex gap-2">
      <select
        v-model="selectedTag"
        @change="handleTagChange"
        class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        <option value="">FeatureTagを選択</option>
        <option v-for="tag in featureTags" :key="tag" :value="tag">
          {{ tag }}
        </option>
      </select>
      
      <select
        v-model="selectedFeature"
        :disabled="!selectedTag"
        class="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
      >
        <option value="">CardFeatureを選択</option>
        <option v-for="feature in availableFeatures" :key="feature.name" :value="feature.name">
          {{ feature.name }}
        </option>
      </select>
    </div>
    
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
import { ref, computed, onMounted } from 'vue'
import { useApi, type CardFeatureData, type FeaturesByTag } from '~/composables/useApi'

const props = defineProps<{
  loading: boolean
}>()

const emit = defineEmits<{
  search: [keyword: string]
  'update:selectedFeature': [feature: string]
}>()

const { getFeatures } = useApi()

const keyword = ref('')
const selectedTag = ref('')
const selectedFeature = ref('')
const featuresData = ref<FeaturesByTag>({})

const featureTags = computed(() => Object.keys(featuresData.value).sort())
const availableFeatures = computed<CardFeatureData[]>(() => {
  if (!selectedTag.value || !featuresData.value[selectedTag.value]) {
    return []
  }
  return featuresData.value[selectedTag.value]
})

const handleTagChange = () => {
  selectedFeature.value = ''
  emit('update:selectedFeature', '')
}

const handleSubmit = () => {
  if (keyword.value && !props.loading) {
    emit('search', keyword.value)
    emit('update:selectedFeature', selectedFeature.value)
  }
}

onMounted(async () => {
  try {
    const response = await getFeatures()
    featuresData.value = response.features_by_tag
  } catch (error) {
    console.error('Failed to load features:', error)
  }
})
</script>