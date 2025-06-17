<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">WXDB Rule Editor</h1>
    
    <!-- 検索セクション -->
    <SearchInput 
      @search="handleSearch" 
      @update:selectedFeature="selectedFeature = $event"
      :loading="searching" 
    />
    
    <!-- 文リスト -->
    <SentenceList 
      v-if="sentences.length > 0"
      :sentences="sentences"
      v-model:classifications="classifications"
    />
    
    <!-- パターン生成セクション -->
    <div v-if="hasClassifications" class="mt-6">
      <button 
        @click="generatePattern"
        :disabled="generating"
        class="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-4 rounded-lg disabled:opacity-50"
      >
        {{ generating ? 'AIでパターンを生成中...' : 'AIでパターンを生成' }}
      </button>
    </div>
    
    <!-- パターンプレビュー -->
    <PatternPreview 
      v-if="suggestion"
      :suggestion="suggestion"
      :search_keyword="currentSearchKeyword"
      :selected_feature="selectedFeature"
      :positive_examples="positiveExamples"
      :negative_examples="negativeExamples"
      @save="savePattern"
      @cancel="suggestion = null"
    />
    
    <!-- 保存済みパターン一覧 -->
    <SavedPatterns ref="savedPatternsRef" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useApi } from '~/composables/useApi'

interface Classification {
  id: string
  type: 'positive' | 'negative' | 'ignore'
}

const { searchAndSplit, generatePattern: generatePatternApi, savePattern: savePatternApi } = useApi()

const searching = ref(false)
const generating = ref(false)
const sentences = ref<any[]>([])
const classifications = ref<Classification[]>([])
const suggestion = ref<any>(null)
const savedPatternsRef = ref()
const positiveExamples = ref<string[]>([])
const negativeExamples = ref<string[]>([])
const selectedFeature = ref<string>('')
const currentSearchKeyword = ref<string>('')

const hasClassifications = computed(() => {
  return classifications.value.some(c => c.type !== 'ignore')
})

const handleSearch = async (keyword: string) => {
  searching.value = true
  currentSearchKeyword.value = keyword
  try {
    sentences.value = await searchAndSplit(keyword)
    classifications.value = sentences.value.map(s => ({
      id: s.id,
      type: 'ignore' as const
    }))
    suggestion.value = null
  } catch (error) {
    console.error('Search error:', error)
    alert('検索エラー: ' + error)
  } finally {
    searching.value = false
  }
}

const generatePattern = async () => {
  positiveExamples.value = sentences.value
    .filter(s => classifications.value.find(c => c.id === s.id)?.type === 'positive')
    .map(s => s.text)
  
  negativeExamples.value = sentences.value
    .filter(s => classifications.value.find(c => c.id === s.id)?.type === 'negative')
    .map(s => s.text)
  
  if (positiveExamples.value.length === 0) {
    alert('マッチすべき例を少なくとも1つ選択してください')
    return
  }
  
  generating.value = true
  try {
    suggestion.value = await generatePatternApi({
      keyword: currentSearchKeyword.value,
      positive_examples: positiveExamples.value,
      negative_examples: negativeExamples.value,
      features: selectedFeature.value ? [selectedFeature.value] : []
    })
  } catch (error) {
    console.error('Pattern generation error:', error)
    alert('パターン生成エラー: ' + error)
  } finally {
    generating.value = false
  }
}

const savePattern = async (patternData: any) => {
  try {
    console.log('=== パターン保存リクエスト ===')
    console.log('Keyword:', patternData.keyword)
    console.log('Pattern:', patternData.pattern)
    console.log('Features:', patternData.features)
    console.log('Positive examples:', patternData.positive_examples.length, 'items')
    console.log('Negative examples:', patternData.negative_examples.length, 'items')
    
    const result = await savePatternApi(patternData)
    
    if (result.success) {
      alert(`パターンを保存しました！\n\nキーワード: ${patternData.keyword}\nパターン: ${patternData.pattern}`)
      suggestion.value = null
      sentences.value = []
      classifications.value = []
      currentSearchKeyword.value = ''
      selectedFeature.value = ''
      await savedPatternsRef.value?.loadPatterns()
    } else {
      alert('保存に失敗しました: ' + result.error)
    }
  } catch (error) {
    console.error('Save error:', error)
    alert('保存エラー: ' + error)
  }
}
</script>