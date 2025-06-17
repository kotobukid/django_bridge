<template>
  <div class="mt-6 bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold mb-4">生成されたパターン</h2>
    
    <div class="space-y-4">
      <div>
        <h3 class="font-medium text-gray-700 mb-2">検索キーワード</h3>
        <div class="bg-blue-50 p-3 rounded">
          <code class="text-blue-800 font-mono text-sm">{{ search_keyword }}</code>
        </div>
      </div>

      <div>
        <h3 class="font-medium text-gray-700 mb-2">生成された正規表現パターン</h3>
        <code class="block bg-gray-100 p-3 rounded font-mono text-sm">{{ suggestion.pattern }}</code>
      </div>
      
      <div>
        <h3 class="font-medium text-gray-700 mb-2">AIの説明</h3>
        <p class="text-gray-600">{{ suggestion.explanation }}</p>
      </div>
      
      <div v-if="selected_feature">
        <h3 class="font-medium text-gray-700 mb-2">対象CardFeature</h3>
        <span class="px-3 py-1 bg-green-100 text-green-700 rounded-full text-sm">
          {{ selected_feature }}
        </span>
      </div>
      
      <div v-if="positive_examples.length > 0">
        <h3 class="font-medium text-gray-700 mb-2">マッチした例文 ({{ positive_examples.length }}件)</h3>
        <div class="bg-green-50 p-3 rounded max-h-40 overflow-y-auto">
          <div v-for="(example, index) in positive_examples" :key="index" class="mb-2 text-sm">
            <span class="text-green-600 font-mono">{{ index + 1 }}.</span>
            <span class="text-gray-700 ml-2">{{ example }}</span>
          </div>
        </div>
      </div>
      
      <div class="border-t pt-4 mt-6">
        <div class="bg-green-50 border border-green-200 rounded-lg p-4 mb-4">
          <div class="flex items-center">
            <div class="text-green-600 text-2xl mr-3">✅</div>
            <div>
              <h4 class="text-green-800 font-medium">パターン生成完了</h4>
              <p class="text-green-700 text-sm">上記のパターンをルールとしてデータベースに保存しますか？</p>
            </div>
          </div>
        </div>
        
        <div class="flex gap-3">
          <button
            @click="handleSave"
            class="flex-1 bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-6 rounded-lg transition-colors duration-200"
          >
            💾 ルールとして保存する
          </button>
          <button
            @click="$emit('cancel')"
            class="flex-1 bg-gray-500 hover:bg-gray-600 text-white font-bold py-3 px-6 rounded-lg transition-colors duration-200"
          >
            ❌ キャンセル
          </button>
        </div>
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
  search_keyword: string
  selected_feature: string
  positive_examples: string[]
  negative_examples: string[]
}>()

const emit = defineEmits<{
  save: [data: any]
  cancel: []
}>()

const handleSave = () => {
  emit('save', {
    keyword: props.search_keyword,
    pattern: props.suggestion.pattern,
    features: props.selected_feature ? [props.selected_feature] : props.suggestion.features,
    positive_examples: props.positive_examples,
    negative_examples: props.negative_examples
  })
}
</script>