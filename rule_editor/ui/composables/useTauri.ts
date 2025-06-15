import { invoke } from '@tauri-apps/api/core'

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

export const useTauri = () => {
  const searchAndSplit = async (keyword: string): Promise<SentenceResult[]> => {
    return await invoke('search_and_split', { keyword })
  }
  
  const generatePattern = async (request: PatternRequest): Promise<PatternSuggestion> => {
    return await invoke('generate_pattern', { request })
  }
  
  const savePattern = async (request: SavePatternRequest): Promise<number> => {
    return await invoke('save_pattern', { request })
  }
  
  const getSavedPatterns = async (): Promise<RulePattern[]> => {
    return await invoke('get_saved_patterns')
  }
  
  const exportPatterns = async (): Promise<string> => {
    return await invoke('export_patterns')
  }
  
  return {
    searchAndSplit,
    generatePattern,
    savePattern,
    getSavedPatterns,
    exportPatterns
  }
}