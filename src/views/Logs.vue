<template>
  <div class="logs-page">
    <div class="page-header">
      <h2 class="page-title">
        实时日志
      </h2>
      <div class="page-actions">
        <Select
          v-model="levelFilter"
          :options="levelOptions"
          label="日志级别"
        />
        <Button
          type="default"
          @click="handleClear"
        >
          清空日志
        </Button>
      </div>
    </div>

    <Card class="logs-card">
      <div
        ref="logContainer"
        class="log-container"
      >
        <div
          v-for="(log, index) in filteredLogs"
          :key="index"
          :class="['log-entry', `log-${log.level.toLowerCase()}`]"
        >
          <span class="log-time">{{ formatTime(log.timestamp) }}</span>
          <Tag :type="getTagType(log.level)">
            {{ log.level }}
          </Tag>
          <span class="log-message">{{ log.message }}</span>
        </div>
        <div
          v-if="filteredLogs.length === 0"
          class="log-empty"
        >
          暂无日志
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import Card from '@/components/base/Card.vue'
import Button from '@/components/base/Button.vue'
import Select from '@/components/base/Select.vue'
import Tag from '@/components/base/Tag.vue'
import { useProxyStore } from '@/stores/proxy'

const proxyStore = useProxyStore()
const logContainer = ref<HTMLElement | null>(null)
const levelFilter = ref('ALL')

const levelOptions = [
  { label: '全部', value: 'ALL' },
  { label: 'INFO', value: 'INFO' },
  { label: 'WARN', value: 'WARN' },
  { label: 'ERROR', value: 'ERROR' },
]

const filteredLogs = computed(() => {
  if (levelFilter.value === 'ALL') {
    return proxyStore.logs
  }
  return proxyStore.logs.filter(log => log.level === levelFilter.value)
})

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}

function getTagType(
  level: string
): 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'info' {
  const map: Record<string, 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'info'> = {
    INFO: 'info',
    WARN: 'warning',
    ERROR: 'danger',
  }
  return map[level] || 'default'
}

function handleClear() {
  proxyStore.clearLogs()
}

function scrollToBottom() {
  nextTick(() => {
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  })
}

watch(
  () => proxyStore.logs.length,
  () => scrollToBottom()
)

onMounted(() => {
  scrollToBottom()
})
</script>

<style lang="scss" scoped>
.logs-page {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 48px);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--spacing-lg);
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.page-actions {
  display: flex;
  gap: var(--spacing-md);
  align-items: flex-end;
}

.logs-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;

  :deep(.card-body) {
    flex: 1;
    min-height: 0;
    padding: 0;
  }
}

.log-container {
  height: 100%;
  max-height: calc(100vh - 200px);
  overflow-y: auto;
  font-family: 'SF Mono', 'Monaco', 'Consolas', monospace;
  font-size: var(--font-sm);
  padding: var(--spacing-md);
}

.log-entry {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  margin-bottom: 4px;
  background: var(--bg-secondary);

  &:hover {
    background: var(--bg-tertiary);
  }
}

.log-time {
  color: var(--text-tertiary);
  font-size: var(--font-xs);
  white-space: nowrap;
}

.log-message {
  flex: 1;
  color: var(--text-primary);
  word-break: break-all;
}

.log-info {
  border-left: 3px solid var(--color-info);
}

.log-warn {
  border-left: 3px solid var(--color-warning);
  background: rgba(230, 162, 60, 0.05);
}

.log-error {
  border-left: 3px solid var(--color-danger);
  background: rgba(245, 108, 108, 0.05);
}

.log-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--text-tertiary);
}
</style>
