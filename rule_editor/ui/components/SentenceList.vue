<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold">検索結果（{{ sentences.length }}件）</h2>
      <button
        @click="toggleQuickMode"
        :class="[
          'px-4 py-2 rounded text-sm font-medium transition-colors',
          quickMode 
            ? 'bg-purple-500 text-white hover:bg-purple-600' 
            : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
        ]"
      >
        {{ quickMode ? '連続マーク終了' : '連続マーク開始' }}
      </button>
    </div>
    
    <div v-if="quickMode" class="mb-4 p-3 bg-purple-50 rounded-lg">
      <p class="text-sm text-purple-700 font-medium">連続マークモード</p>
      <p class="text-xs text-purple-600 mt-1">
        ↑↓: 移動 | ←→: マッチ⇔無関係⇔除外 | Space: 適用＆次へ | Esc: 終了
      </p>
    </div>
    
    <div ref="scrollContainer" class="space-y-4 max-h-[600px] overflow-y-auto" @keydown="handleKeydown" tabindex="0">
      <div
        v-for="(sentence, index) in sentences"
        :key="sentence.id"
        :data-index="index"
        :class="[
          'border rounded-lg p-3 transition-colors',
          quickMode && index === currentIndex 
            ? 'border-purple-500 bg-purple-50' 
            : 'border-gray-200 hover:bg-gray-50'
        ]"
      >
        <div class="flex items-start gap-3">
          <div class="flex gap-3">
            <button
              @click="updateClassification(sentence.id, 'positive')"
              :class="[
                'px-3 py-1 rounded text-sm font-medium transition-colors border-2',
                getClassification(sentence.id) === 'positive'
                  ? 'bg-green-500 text-white border-green-500'
                  : quickMode && index === currentIndex && currentClassification === 'positive'
                  ? 'bg-green-100 text-green-700 border-green-400 ring-2 ring-green-300'
                  : 'bg-green-100 text-green-700 hover:bg-green-200 border-transparent'
              ]"
            >
              ✅ マッチ
            </button>
            
            <button
              @click="updateClassification(sentence.id, 'ignore')"
              :class="[
                'px-3 py-1 rounded text-sm font-medium transition-colors border-2',
                getClassification(sentence.id) === 'ignore'
                  ? 'bg-gray-500 text-white border-gray-500'
                  : quickMode && index === currentIndex && currentClassification === 'ignore'
                  ? 'bg-gray-100 text-gray-700 border-gray-400 ring-2 ring-gray-300'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200 border-transparent'
              ]"
            >
              ➖ 無関係
            </button>
            
            <button
              @click="updateClassification(sentence.id, 'negative')"
              :class="[
                'px-3 py-1 rounded text-sm font-medium transition-colors border-2',
                getClassification(sentence.id) === 'negative'
                  ? 'bg-red-500 text-white border-red-500'
                  : quickMode && index === currentIndex && currentClassification === 'negative'
                  ? 'bg-red-100 text-red-700 border-red-400 ring-2 ring-red-300'
                  : 'bg-red-100 text-red-700 hover:bg-red-200 border-transparent'
              ]"
            >
              ❌ 除外
            </button>
          </div>
          
          <div class="flex-1">
            <div class="mb-2">
              <p class="text-xs text-gray-600 font-medium">
                {{ sentence.card_number }} - {{ sentence.card_name }}
              </p>
            </div>
            <p class="text-sm whitespace-pre-wrap">{{ sentence.text }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue'

interface Sentence {
  id: string
  text: string
  card_number: string
  card_name: string
}

interface Classification {
  id: string
  type: 'positive' | 'negative' | 'ignore'
}

const props = defineProps<{
  sentences: Sentence[]
  classifications: Classification[]
}>()

const emit = defineEmits<{
  'update:classifications': [value: Classification[]]
}>()

const quickMode = ref(false)
const currentIndex = ref(0)
const currentClassification = ref<'positive' | 'negative' | 'ignore'>('ignore')
const scrollContainer = ref<HTMLElement>()

const getClassification = (id: string): string => {
  return props.classifications.find(c => c.id === id)?.type || 'ignore'
}

const updateClassification = (id: string, type: 'positive' | 'negative' | 'ignore') => {
  const newClassifications = [...props.classifications]
  const index = newClassifications.findIndex(c => c.id === id)
  
  if (index >= 0) {
    newClassifications[index] = { id, type }
  } else {
    newClassifications.push({ id, type })
  }
  
  emit('update:classifications', newClassifications)
}

const scrollToCurrentItem = async () => {
  if (!quickMode.value || !scrollContainer.value) return
  
  await nextTick()
  
  const activeElement = scrollContainer.value.querySelector(`[data-index="${currentIndex.value}"]`) as HTMLElement
  if (activeElement) {
    const containerRect = scrollContainer.value.getBoundingClientRect()
    const itemRect = activeElement.getBoundingClientRect()
    
    const isVisible = itemRect.top >= containerRect.top && itemRect.bottom <= containerRect.bottom
    
    if (!isVisible) {
      activeElement.scrollIntoView({
        behavior: 'smooth',
        block: 'center'
      })
    }
  }
}

const toggleQuickMode = () => {
  quickMode.value = !quickMode.value
  if (quickMode.value) {
    currentIndex.value = 0
    currentClassification.value = 'ignore' // デフォルトを中央の「無関係」に
    scrollToCurrentItem()
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (!quickMode.value) return
  
  event.preventDefault()
  
  switch (event.key) {
    case 'ArrowUp':
      if (currentIndex.value > 0) {
        currentIndex.value--
        currentClassification.value = 'ignore' // 上移動時も中央の「無関係」にリセット
        scrollToCurrentItem()
      }
      break
    case 'ArrowDown':
      if (currentIndex.value < props.sentences.length - 1) {
        currentIndex.value++
        currentClassification.value = 'ignore' // 下移動時も中央の「無関係」にリセット
        scrollToCurrentItem()
      }
      break
    case 'ArrowLeft':
      if (currentClassification.value === 'ignore') {
        currentClassification.value = 'positive'
      } else if (currentClassification.value === 'negative') {
        currentClassification.value = 'ignore'
      } else if (currentClassification.value === 'positive') {
        currentClassification.value = 'positive' // 左端なので変更なし
      }
      break
    case 'ArrowRight':
      if (currentClassification.value === 'positive') {
        currentClassification.value = 'ignore'
      } else if (currentClassification.value === 'ignore') {
        currentClassification.value = 'negative'
      } else if (currentClassification.value === 'negative') {
        currentClassification.value = 'negative' // 右端なので変更なし
      }
      break
    case ' ':
      if (props.sentences[currentIndex.value]) {
        updateClassification(
          props.sentences[currentIndex.value].id,
          currentClassification.value
        )
        if (currentIndex.value < props.sentences.length - 1) {
          currentIndex.value++
          currentClassification.value = 'ignore' // 次のアイテムでも中央の「無関係」にリセット
          scrollToCurrentItem()
        }
      }
      break
    case 'Escape':
      quickMode.value = false
      break
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>