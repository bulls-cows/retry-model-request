<template>
  <button
    :class="['btn', `btn-${type}`, `btn-${size}`, { 'btn-loading': loading }]"
    :disabled="disabled || loading"
    @click="$emit('click', $event)"
  >
    <span
      v-if="loading"
      class="btn-spinner"
    />
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps<{
  type?: 'primary' | 'success' | 'warning' | 'danger' | 'default'
  size?: 'small' | 'medium' | 'large'
  loading?: boolean
  disabled?: boolean
}>()

defineEmits<{
  click: [event: MouseEvent]
}>()
</script>

<style lang="scss" scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}

// Sizes
.btn-small {
  padding: 6px 12px;
  font-size: var(--font-sm);
}

.btn-medium {
  padding: 10px 20px;
  font-size: var(--font-md);
}

.btn-large {
  padding: 14px 28px;
  font-size: var(--font-lg);
}

// Types
.btn-default {
  background: var(--bg-primary);
  border-color: var(--border-primary);
  color: var(--text-primary);

  &:hover:not(:disabled) {
    border-color: var(--color-primary);
    color: var(--color-primary);
  }
}

.btn-primary {
  background: var(--color-primary);
  color: white;

  &:hover:not(:disabled) {
    background: #66b1ff;
  }
}

.btn-success {
  background: var(--color-success);
  color: white;

  &:hover:not(:disabled) {
    background: #85ce61;
  }
}

.btn-warning {
  background: var(--color-warning);
  color: white;

  &:hover:not(:disabled) {
    background: #ebb563;
  }
}

.btn-danger {
  background: var(--color-danger);
  color: white;

  &:hover:not(:disabled) {
    background: #f78989;
  }
}

// Loading spinner
.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid currentColor;
  border-right-color: transparent;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
