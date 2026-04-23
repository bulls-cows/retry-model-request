<template>
  <div class="select-wrapper">
    <label
      v-if="label"
      class="select-label"
    >{{ label }}</label>
    <div class="select-container">
      <select
        :value="modelValue"
        :disabled="disabled"
        class="select-field"
        @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
      >
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
        >
          {{ option.label }}
        </option>
      </select>
      <span class="select-arrow">▼</span>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Option {
  label: string
  value: string | number
}

defineProps<{
  modelValue: string | number
  label?: string
  options: Option[]
  disabled?: boolean
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<style lang="scss" scoped>
.select-wrapper {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.select-label {
  font-size: var(--font-md);
  color: var(--text-primary);
  font-weight: 500;
}

.select-container {
  position: relative;
  display: flex;
  align-items: center;
}

.select-field {
  width: 100%;
  padding: 10px 36px 10px 12px;
  font-size: var(--font-md);
  color: var(--text-primary);
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  cursor: pointer;
  appearance: none;
  transition: all var(--transition-fast);

  &:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
    outline: none;
  }

  &:disabled {
    cursor: not-allowed;
    color: var(--text-tertiary);
  }
}

.select-arrow {
  position: absolute;
  right: 12px;
  font-size: 10px;
  color: var(--text-tertiary);
  pointer-events: none;
}
</style>
