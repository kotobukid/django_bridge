<template>
  <div class="mt-8 bg-white rounded-lg shadow-md p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold">保存済みパターン</h2>
      <button
        @click="exportPatterns"
        class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded text-sm"
      >
        Rustコードをエクスポート
      </button>
    </div>
    
    <div v-if="loading" class="text-center py-4">
      読み込み中...
    </div>
    
    <div v-else-if="patterns.length === 0" class="text-center py-4 text-gray-500">
      保存されたパターンはありません
    </div>
    
    <div v-else class="space-y-3 max-h-96 overflow-y-auto">
      <div
        v-for="pattern in patterns"
        :key="pattern.id"
        class="border rounded-lg p-4 hover:bg-gray-50"
      >
        <div class="flex justify-between items-start">
          <div class="flex-1">
            <h3 class="font-medium">{{ pattern.keyword }}</h3>
            <code class="block text-sm bg-gray-100 p-2 mt-2 rounded font-mono">{{ pattern.pattern }}</code>
            <div class="flex flex-wrap gap-1 mt-2">
              <span 
                v-for="feature in pattern.features" 
                :key="feature"
                class="px-2 py-0.5 bg-blue-100 text-blue-700 rounded text-xs"
              >
                {{ feature }}
              </span>
            </div>
          </div>
          <span class="text-xs text-gray-500 ml-4">
            {{ new Date(pattern.created_at).toLocaleDateString() }}
          </span>
        </div>
      </div>
    </div>
    
    <!-- エクスポートプレビューモーダル -->
    <div v-if="exportedCode" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <div class="bg-white rounded-lg max-w-4xl w-full max-h-[80vh] overflow-hidden">
        <div class="p-4 border-b">
          <h3 class="text-lg font-semibold">エクスポートされたRustコード</h3>
        </div>
        <div class="p-4 overflow-auto max-h-[60vh]">
          <pre class="bg-gray-100 p-4 rounded font-mono text-sm">{{ exportedCode }}</pre>
        </div>
        <div class="p-4 border-t flex justify-end gap-2">
          <button
            @click="copyToClipboard"
            class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
          >
            クリップボードにコピー
          </button>
          <button
            @click="exportedCode = null"
            class="bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded"
          >
            閉じる
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTauri } from '~/composables/useTauri'
import type { RulePattern } from '~/composables/useTauri'

const { getSavedPatterns, exportPatterns: exportPatternsApi } = useTauri()

const loading = ref(false)
const patterns = ref<RulePattern[]>([])
const exportedCode = ref<string | null>(null)

const loadPatterns = async () => {
  loading.value = true
  try {
    patterns.value = await getSavedPatterns()
  } catch (error) {
    console.error('Failed to load patterns:', error)
  } finally {
    loading.value = false
  }
}

const exportPatterns = async () => {
  try {
    exportedCode.value = await exportPatternsApi()
  } catch (error) {
    console.error('Failed to export patterns:', error)
    alert('エクスポートエラー: ' + error)
  }
}

const copyToClipboard = async () => {
  if (exportedCode.value) {
    try {
      await navigator.clipboard.writeText(exportedCode.value)
      alert('クリップボードにコピーしました')
    } catch (error) {
      console.error('Failed to copy:', error)
    }
  }
}

onMounted(() => {
  loadPatterns()
})

defineExpose({
  loadPatterns
})
</script>