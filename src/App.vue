<template>
  <div class="app">
    <Sidebar />
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import Sidebar from '@/components/Sidebar.vue'
import { useProxyStore } from '@/stores/proxy'
import { useStatsStore } from '@/stores/stats'

const proxyStore = useProxyStore()
const statsStore = useStatsStore()

onMounted(async () => {
  // Setup stores
  await proxyStore.setupListeners()
  await statsStore.setupListeners()

  // Handle window close - minimize to tray instead
  const mainWindow = getCurrentWindow()
  await mainWindow.onCloseRequested(async event => {
    // Prevent the window from closing
    event.preventDefault()
    // Hide the window instead
    await mainWindow.hide()
  })
})

onUnmounted(() => {
  proxyStore.cleanup()
  statsStore.cleanup()
})
</script>

<style lang="scss" scoped>
.app {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
}

.main-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  background: var(--bg-secondary);
}
</style>
