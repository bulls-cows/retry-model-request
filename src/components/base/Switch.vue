<template>
  <label class="switch-wrapper">
    <span v-if="label" class="switch-label">{{ label }}</span>
    <button
      type="button"
      role="switch"
      :aria-checked="modelValue"
      :class="['switch', { 'switch-on': modelValue, 'switch-disabled': disabled }]"
      :disabled="disabled"
      @click="toggle"
    >
      <span class="switch-thumb" />
    </button>
  </label>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean
  label?: string
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

function toggle() {
  if (!props.disabled) {
    emit('update:modelValue', !props.modelValue)
  }
}
</script>

<style lang="scss" scoped>
.switch-wrapper {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.switch-label {
  font-size: var(--font-md);
  color: var(--text-primary);
}

.switch {
  position: relative;
  width: 44px;
  height: 22px;
  background: var(--border-primary);
  border: none;
  border-radius: 11px;
  cursor: pointer;
  transition: background var(--transition-fast);
  padding: 0;

  &:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }
}

.switch-on {
  background: var(--color-primary);
}

.switch-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.switch-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  background: white;
  border-radius: 50%;
  transition: transform var(--transition-fast);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.switch-on .switch-thumb {
  transform: translateX(22px);
}
</style>
