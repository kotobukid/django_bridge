<template>
  <div class="bg-white rounded-lg shadow-md p-4">
    <h2 class="text-xl font-semibold mb-4">検索結果（{{ sentences.length }}件）</h2>
    
    <div class="space-y-3 max-h-96 overflow-y-auto">
      <div
        v-for="sentence in sentences"
        :key="sentence.id"
        class="border rounded-lg p-3 hover:bg-gray-50"
      >
        <div class="flex items-start gap-3">
          <div class="flex flex-col gap-2">
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                :name="`classification-${sentence.id}`"
                :value="'positive'"
                :checked="getClassification(sentence.id) === 'positive'"
                @change="updateClassification(sentence.id, 'positive')"
                class="mr-2 text-green-500"
              />
              <span class="text-green-600">✅ マッチすべき</span>
            </label>
            
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                :name="`classification-${sentence.id}`"
                :value="'negative'"
                :checked="getClassification(sentence.id) === 'negative'"
                @change="updateClassification(sentence.id, 'negative')"
                class="mr-2 text-red-500"
              />
              <span class="text-red-600">❌ マッチすべきでない</span>
            </label>
            
            <label class="flex items-center cursor-pointer">
              <input
                type="radio"
                :name="`classification-${sentence.id}`"
                :value="'ignore'"
                :checked="getClassification(sentence.id) === 'ignore'"
                @change="updateClassification(sentence.id, 'ignore')"
                class="mr-2 text-gray-500"
              />
              <span class="text-gray-600">➖ 無関係</span>
            </label>
          </div>
          
          <div class="flex-1">
            <p class="text-sm font-medium mb-1">{{ sentence.text }}</p>
            <p class="text-xs text-gray-500">
              {{ sentence.card_number }} - {{ sentence.card_name }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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
</script>