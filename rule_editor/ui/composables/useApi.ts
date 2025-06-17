export interface SentenceResult {
  id: string
  text: string
  card_number: string
  card_name: string
  full_skill_text: string
}

export interface PatternRequest {
  keyword: string
  positive_examples: string[]
  negative_examples: string[]
  features: string[]
}

export interface PatternSuggestion {
  pattern: string
  explanation: string
  features: string[]
}

export interface SavePatternRequest {
  keyword: string
  pattern: string
  features: string[]
  positive_examples: string[]
  negative_examples: string[]
}

export interface RulePattern {
  id: number
  keyword: string
  pattern: string
  features: string[]
  positive_examples: string[]
  negative_examples: string[]
  created_at: string
  updated_at: string
  is_active: boolean
}

export interface CardFeatureData {
  name: string
  bit_shift: [number, number]
  tag: string
}

export interface FeaturesByTag {
  [tag: string]: CardFeatureData[]
}

// Development時はNuxtのプロキシ経由でアクセス
const getApiUrl = (path: string) => {
  return path
}

export const useApi = () => {
  const searchAndSplit = async (keyword: string): Promise<SentenceResult[]> => {
    const response = await fetch(getApiUrl(`/api/search?keyword=${encodeURIComponent(keyword)}`))
    if (!response.ok) {
      throw new Error(`Search failed: ${response.statusText}`)
    }
    return await response.json()
  }
  
  const generatePattern = async (request: PatternRequest): Promise<PatternSuggestion> => {
    const response = await fetch(getApiUrl('/api/generate-pattern'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    })
    if (!response.ok) {
      throw new Error(`Pattern generation failed: ${response.statusText}`)
    }
    return await response.json()
  }
  
  const savePattern = async (request: SavePatternRequest): Promise<number> => {
    const response = await fetch(getApiUrl('/api/patterns'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    })
    if (!response.ok) {
      throw new Error(`Save pattern failed: ${response.statusText}`)
    }
    const result = await response.json()
    if (!result.success) {
      throw new Error(result.error || 'Save failed')
    }
    return result.id
  }
  
  const getSavedPatterns = async (): Promise<RulePattern[]> => {
    const response = await fetch(getApiUrl('/api/patterns'))
    if (!response.ok) {
      throw new Error(`Get patterns failed: ${response.statusText}`)
    }
    return await response.json()
  }
  
  const exportPatterns = async (): Promise<string> => {
    const response = await fetch(getApiUrl('/api/export'), {
      method: 'POST',
    })
    if (!response.ok) {
      throw new Error(`Export failed: ${response.statusText}`)
    }
    const result = await response.json()
    if (!result.success) {
      throw new Error(result.error || 'Export failed')
    }
    return result.code
  }
  
  const getFeatures = async (): Promise<{ features_by_tag: FeaturesByTag }> => {
    const response = await fetch(getApiUrl('/api/features'))
    if (!response.ok) {
      throw new Error(`Get features failed: ${response.statusText}`)
    }
    return await response.json()
  }
  
  return {
    searchAndSplit,
    generatePattern,
    savePattern,
    getSavedPatterns,
    exportPatterns,
    getFeatures
  }
}