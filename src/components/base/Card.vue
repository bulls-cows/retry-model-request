<template>
  <div class="card">
    <div v-if="title || $slots.header" class="card-header">
      <h3 class="card-title">
        {{ title }}
      </h3>
      <slot name="header" />
    </div>
    <div class="card-body">
      <slot />
    </div>
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>

<style lang="scss" scoped>
.card {
  background: var(--bg-primary);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  border: 1px solid var(--border-primary);
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-primary);
}

.card-title {
  font-size: var(--font-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.card-body {
  padding: var(--spacing-lg);
}

.card-footer {
  padding: var(--spacing-md) var(--spacing-lg);
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}
</style>
