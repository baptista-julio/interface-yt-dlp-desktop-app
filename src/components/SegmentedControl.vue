<script setup lang="ts" generic="T extends string | number">
// Controle segmentado com indicador deslizante (secao 12).
// Usado para escolher entre Video / Audio.

import { computed } from "vue";

interface Option {
  value: T;
  label: string;
}

const props = defineProps<{
  modelValue: T;
  options: Option[];
}>();

const emit = defineEmits<{
  "update:modelValue": [value: T];
}>();

const activeIndex = computed(() =>
  Math.max(
    0,
    props.options.findIndex((o) => o.value === props.modelValue),
  ),
);

function select(value: T) {
  emit("update:modelValue", value);
}
</script>

<template>
  <div
    class="segmented"
    role="tablist"
    :style="{ '--count': options.length, '--active': activeIndex }"
  >
    <div class="segmented-indicator" aria-hidden="true" />
    <button
      v-for="opt in options"
      :key="String(opt.value)"
      type="button"
      role="tab"
      class="segmented-option"
      :class="{ active: opt.value === modelValue }"
      :aria-selected="opt.value === modelValue"
      @click="select(opt.value)"
    >
      {{ opt.label }}
    </button>
  </div>
</template>

<style scoped>
.segmented {
  position: relative;
  display: grid;
  grid-template-columns: repeat(var(--count), 1fr);
  gap: 0;
  padding: 0.25rem;
  background-color: var(--bg);
  border: 1px solid var(--border);
  border-radius: 9999px;
  width: 100%;
}

.segmented-indicator {
  position: absolute;
  top: 0.25rem;
  bottom: 0.25rem;
  left: 0.25rem;
  width: calc((100% - 0.5rem) / var(--count));
  background-color: var(--surface);
  border-radius: 9999px;
  box-shadow: var(--shadow-sm);
  transform: translateX(calc(var(--active) * 100%));
  /* Spring-like (stiffness 400, damping 30 -> aprox.) */
  transition: transform 0.35s cubic-bezier(0.34, 1.4, 0.64, 1);
}

.segmented-option {
  position: relative;
  z-index: 1;
  appearance: none;
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 0.6rem 0.75rem;
  font-family: inherit;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-secondary);
  border-radius: 9999px;
  transition: color 0.2s ease;
  white-space: nowrap;
}

.segmented-option.active {
  color: var(--primary);
}

.segmented-option:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}
</style>
