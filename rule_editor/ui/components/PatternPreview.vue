<template>
  <div class="mt-6 bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold mb-4">生成されたパターン</h2>
    
    <div class="space-y-4">
      <div>
        <h3 class="font-medium text-gray-700 mb-2">正規表現パターン</h3>
        <code class="block bg-gray-100 p-3 rounded font-mono text-sm">{{ suggestion.pattern }}</code>
      </div>
      
      <div>
        <h3 class="font-medium text-gray-700 mb-2">説明</h3>
        <p class="text-gray-600">{{ suggestion.explanation }}</p>
      </div>
      
      <div v-if="suggestion.features.length > 0">
        <h3 class="font-medium text-gray-700 mb-2">検出されるCardFeature</h3>
        <div class="flex flex-wrap gap-2">
          <span 
            v-for="feature in suggestion.features" 
            :key="feature"
            class="px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm"
          >
            {{ feature }}
          </span>
        </div>
      </div>
      
      <div class="flex gap-3 mt-6">
        <button
          @click="handleSave"
          class="flex-1 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded"
        >
          パターンを保存
        </button>
        <button
          @click="$emit('cancel')"
          class="flex-1 bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded"
        >
          キャンセル
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { PatternSuggestion } from '~/composables/useApi'

interface Classification {
  id: string
  type: 'positive' | 'negative' | 'ignore'
}

const props = defineProps<{
  suggestion: PatternSuggestion
  positive_examples: string[]
  negative_examples: string[]
}>()

const emit = defineEmits<{
  save: [data: any]
  cancel: []
}>()

const handleSave = () => {
  emit('save', {
    keyword: props.suggestion.pattern.slice(0, 20), // パターンの一部をキーワードとして使用
    pattern: props.suggestion.pattern,
    features: props.suggestion.features,
    positive_examples: props.positive_examples,
    negative_examples: props.negative_examples
  })
}
</script>