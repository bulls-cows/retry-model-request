import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

export interface LogEntry {
  timestamp: string
  level: string
  message: string
  details: Record<string, unknown> | null
}

export const useProxyStore = defineStore('proxy', () => {
  const status = ref<'running' | 'stopped'>('stopped')
  const port = ref<number | null>(null)
  const logs = ref<LogEntry[]>([])
  const error = ref<string | null>(null)

  let unlistenLog: UnlistenFn | null = null
  let unlistenStatus: UnlistenFn | null = null

  async function setupListeners() {
    // Listen for log events
    unlistenLog = await listen<LogEntry>('proxy-log', event => {
      logs.value.push(event.payload)
      // Keep only last 1000 logs
      if (logs.value.length > 1000) {
        logs.value = logs.value.slice(-1000)
      }
    })

    // Listen for status events
    unlistenStatus = await listen<string>('proxy-status', event => {
      status.value = event.payload as 'running' | 'stopped'
    })
  }

  async function start() {
    error.value = null
    try {
      port.value = await invoke<number>('start_proxy')
      status.value = 'running'
    } catch (e) {
      error.value = String(e)
    }
  }

  async function stop() {
    error.value = null
    try {
      await invoke('stop_proxy')
      status.value = 'stopped'
      port.value = null
    } catch (e) {
      error.value = String(e)
    }
  }

  async function checkStatus() {
    try {
      const result = await invoke<string>('get_proxy_status')
      status.value = result as 'running' | 'stopped'
    } catch (e) {
      error.value = String(e)
    }
  }

  function clearLogs() {
    logs.value = []
  }

  function cleanup() {
    unlistenLog?.()
    unlistenStatus?.()
  }

  return {
    status,
    port,
    logs,
    error,
    setupListeners,
    start,
    stop,
    checkStatus,
    clearLogs,
    cleanup,
  }
})
