<template>
  <div class="input-wrapper">
    <label v-if="label" class="input-label">{{ label }}</label>
    <div class="input-container" :class="{ 'input-error': error }">
      <input
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        class="input-field"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
    </div>
    <span v-if="error" class="input-error-text">{{ error }}</span>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: string | number
  label?: string
  type?: string
  placeholder?: string
  disabled?: boolean
  error?: string
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<style lang="scss" scoped>
.input-wrapper {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-label {
  font-size: var(--font-md);
  color: var(--text-primary);
  font-weight: 500;
}

.input-container {
  display: flex;
  align-items: center;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);

  &:focus-within {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
  }

  &.input-error {
    border-color: var(--color-danger);
  }
}

.input-field {
  flex: 1;
  padding: 10px 12px;
  font-size: var(--font-md);
  color: var(--text-primary);
  background: transparent;
  border: none;
  outline: none;

  &::placeholder {
    color: var(--text-placeholder);
  }

  &:disabled {
    cursor: not-allowed;
    color: var(--text-tertiary);
  }
}

.input-error-text {
  font-size: var(--font-xs);
  color: var(--color-danger);
}
</style>
