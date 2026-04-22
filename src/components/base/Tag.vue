<template>
  <span :class="['tag', `tag-${type}`]">
    <slot />
  </span>
</template>

<script setup lang="ts">
defineProps<{
  type?: 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'info'
}>()
</script>

<style lang="scss" scoped>
.tag {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  font-size: var(--font-xs);
  font-weight: 500;
  border-radius: var(--radius-sm);
}

.tag-default {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.tag-primary {
  background: rgba(64, 158, 255, 0.1);
  color: var(--color-primary);
}

.tag-success {
  background: rgba(103, 194, 58, 0.1);
  color: var(--color-success);
}

.tag-warning {
  background: rgba(230, 162, 60, 0.1);
  color: var(--color-warning);
}

.tag-danger {
  background: rgba(245, 108, 108, 0.1);
  color: var(--color-danger);
}

.tag-info {
  background: rgba(144, 147, 153, 0.1);
  color: var(--color-info);
}
</style>
