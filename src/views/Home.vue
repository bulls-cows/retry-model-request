<template>
  <div class="home-page">
    <div class="page-header">
      <h2 class="page-title">仪表盘</h2>
      <p class="page-desc">配置代理服务并启动</p>
    </div>

    <!-- Status Card -->
    <Card class="status-card">
      <template #header>
        <span>服务状态</span>
      </template>
      <div class="status-content">
        <div class="status-main">
          <div :class="['status-badge', proxyStore.status]">
            <span class="status-icon" />
            {{ proxyStore.status === 'running' ? '运行中' : '已停止' }}
          </div>
          <div v-if="proxyStore.status === 'running'" class="status-info">
            <span class="status-label">监听端口:</span>
            <span class="status-value">{{ proxyStore.port }}</span>
          </div>
        </div>
        <div class="status-actions">
          <Button
            v-if="proxyStore.status === 'stopped'"
            type="primary"
            size="large"
            @click="handleStart"
          >
            启动服务
          </Button>
          <Button v-else type="danger" size="large" @click="handleStop"> 停止服务 </Button>
        </div>
      </div>
    </Card>

    <!-- Profile Selection -->
    <Card class="profile-card">
      <template #header>
        <span>配置方案</span>
      </template>
      <div class="profile-selector">
        <Select
          v-model="activeProfileId"
          label="选择配置方案"
          :options="profileOptions"
          @update:model-value="handleProfileChange"
        />
        <Button type="default" @click="showCreateModal = true"> 新建方案 </Button>
      </div>
    </Card>

    <!-- Configuration Form -->
    <Card v-if="activeProfile" class="config-card">
      <template #header>
        <span>配置详情</span>
      </template>
      <div class="config-form">
        <Input v-model="activeProfile.name" label="方案名称" placeholder="输入方案名称" />
        <Input
          v-model.number="activeProfile.local_port"
          label="本地端口"
          type="number"
          placeholder="3000"
        />
        <Input
          v-model="activeProfile.target_base_url"
          label="目标接口地址"
          placeholder="https://api.example.com/v1"
        />
        <div class="form-row">
          <Input
            v-model.number="activeProfile.max_retries"
            label="最大重试次数"
            type="number"
            placeholder="3"
          />
          <Input
            v-model.number="activeProfile.retry_delay_ms"
            label="重试延迟 (ms)"
            type="number"
            placeholder="1000"
          />
        </div>
        <Input
          v-model="retryCodesText"
          label="重试状态码 (逗号分隔)"
          placeholder="429,500,502,503,504"
        />
        <div class="form-actions">
          <Button type="primary" @click="handleSaveConfig"> 保存配置 </Button>
          <Button type="danger" @click="handleDeleteProfile"> 删除方案 </Button>
        </div>
      </div>
    </Card>

    <!-- Quick Stats -->
    <div class="quick-stats">
      <Card>
        <div class="stat-item">
          <span class="stat-value">{{ statsStore.stats.total_requests }}</span>
          <span class="stat-label">总请求数</span>
        </div>
      </Card>
      <Card>
        <div class="stat-item">
          <span class="stat-value text-success">{{ statsStore.stats.successful_requests }}</span>
          <span class="stat-label">成功请求</span>
        </div>
      </Card>
      <Card>
        <div class="stat-item">
          <span class="stat-value text-danger">{{ statsStore.stats.failed_requests }}</span>
          <span class="stat-label">失败请求</span>
        </div>
      </Card>
      <Card>
        <div class="stat-item">
          <span class="stat-value text-warning">{{ statsStore.stats.total_retries }}</span>
          <span class="stat-label">重试次数</span>
        </div>
      </Card>
    </div>

    <!-- Create Profile Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h3 class="modal-title">新建配置方案</h3>
        <Input v-model="newProfileName" label="方案名称" placeholder="输入方案名称" />
        <div class="modal-actions">
          <Button type="default" @click="showCreateModal = false"> 取消 </Button>
          <Button type="primary" @click="handleCreateProfile"> 创建 </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import Card from '@/components/base/Card.vue'
import Button from '@/components/base/Button.vue'
import Input from '@/components/base/Input.vue'
import Select from '@/components/base/Select.vue'
import { useConfigStore } from '@/stores/config'
import { useProxyStore } from '@/stores/proxy'
import { useStatsStore } from '@/stores/stats'

const configStore = useConfigStore()
const proxyStore = useProxyStore()
const statsStore = useStatsStore()

const showCreateModal = ref(false)
const newProfileName = ref('')
const activeProfileId = ref<string>('')

const activeProfile = computed(() => configStore.activeProfile)

const profileOptions = computed(() =>
  (configStore.config?.profiles || []).map(p => ({
    label: p.name,
    value: p.id,
  }))
)

const retryCodesText = computed({
  get: () => activeProfile.value?.retry_status_codes.join(',') || '',
  set: (val: string) => {
    if (activeProfile.value) {
      activeProfile.value.retry_status_codes = val
        .split(',')
        .map(s => parseInt(s.trim(), 10))
        .filter(n => !isNaN(n))
    }
  },
})

watch(
  () => configStore.config?.active_profile_id,
  id => {
    if (id) activeProfileId.value = id
  },
  { immediate: true }
)

async function handleStart() {
  await proxyStore.start()
}

async function handleStop() {
  await proxyStore.stop()
}

async function handleProfileChange(id: string) {
  await configStore.setActiveProfile(id)
}

async function handleSaveConfig() {
  if (activeProfile.value) {
    await configStore.updateProfile(activeProfile.value)
  }
}

async function handleDeleteProfile() {
  if (activeProfile.value && (configStore.config?.profiles?.length ?? 0) > 1) {
    await configStore.deleteProfile(activeProfile.value.id)
  }
}

async function handleCreateProfile() {
  if (newProfileName.value.trim()) {
    await configStore.createProfile(newProfileName.value.trim())
    newProfileName.value = ''
    showCreateModal.value = false
  }
}

onMounted(async () => {
  await configStore.loadConfig()
  await proxyStore.checkStatus()
  await statsStore.loadStats()
})
</script>

<style lang="scss" scoped>
.home-page {
  max-width: 900px;
}

.page-header {
  margin-bottom: var(--spacing-lg);
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.page-desc {
  font-size: var(--font-md);
  color: var(--text-tertiary);
  margin: 0;
}

.status-card {
  margin-bottom: var(--spacing-lg);
}

.status-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.status-main {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-weight: 500;

  &.running {
    background: rgba(103, 194, 58, 0.1);
    color: var(--color-success);
  }

  &.stopped {
    background: var(--bg-tertiary);
    color: var(--text-tertiary);
  }
}

.status-icon {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: currentColor;
}

.status-info {
  display: flex;
  gap: 8px;
  font-size: var(--font-md);
}

.status-label {
  color: var(--text-tertiary);
}

.status-value {
  color: var(--text-primary);
  font-weight: 500;
}

.profile-card {
  margin-bottom: var(--spacing-lg);
}

.profile-selector {
  display: flex;
  gap: var(--spacing-md);
  align-items: flex-end;
}

.config-card {
  margin-bottom: var(--spacing-lg);
}

.config-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

.form-actions {
  display: flex;
  gap: var(--spacing-md);
  margin-top: var(--spacing-md);
}

.quick-stats {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--spacing-md);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.stat-value {
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.stat-label {
  font-size: var(--font-sm);
  color: var(--text-tertiary);
  margin-top: 4px;
}

.text-success {
  color: var(--color-success);
}

.text-danger {
  color: var(--color-danger);
}

.text-warning {
  color: var(--color-warning);
}

// Modal
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  padding: var(--spacing-lg);
  width: 400px;
  max-width: 90%;
}

.modal-title {
  font-size: var(--font-xl);
  font-weight: 600;
  margin: 0 0 var(--spacing-md) 0;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-md);
  margin-top: var(--spacing-lg);
}
</style>
