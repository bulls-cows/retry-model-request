import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

export interface StatsSnapshot {
  total_requests: number
  successful_requests: number
  failed_requests: number
  total_retries: number
  success_rate: number
}

export const useStatsStore = defineStore('stats', () => {
  const stats = ref<StatsSnapshot>({
    total_requests: 0,
    successful_requests: 0,
    failed_requests: 0,
    total_retries: 0,
    success_rate: 0,
  })

  let unlistenStats: UnlistenFn | null = null

  async function setupListeners() {
    unlistenStats = await listen<StatsSnapshot>('proxy-stats', event => {
      stats.value = event.payload
    })
  }

  async function loadStats() {
    try {
      stats.value = await invoke<StatsSnapshot>('get_stats')
    } catch (e) {
      console.error('Failed to load stats:', e)
    }
  }

  async function resetStats() {
    try {
      await invoke('reset_stats')
      stats.value = {
        total_requests: 0,
        successful_requests: 0,
        failed_requests: 0,
        total_retries: 0,
        success_rate: 0,
      }
    } catch (e) {
      console.error('Failed to reset stats:', e)
    }
  }

  function cleanup() {
    unlistenStats?.()
  }

  return {
    stats,
    setupListeners,
    loadStats,
    resetStats,
    cleanup,
  }
})
