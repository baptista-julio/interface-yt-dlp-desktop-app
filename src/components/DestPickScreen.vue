<script setup lang="ts">
// Passo 5 — escolher a pasta de destino (seletor nativo) e iniciar.

import { ref } from "vue";
import StepShell from "./StepShell.vue";
import { pickOutputDir } from "../lib/ytdlp";

const emit = defineEmits<{
  next: [dir: string];
  back: [];
}>();

const dir = ref("");

async function choose() {
  const picked = await pickOutputDir();
  if (picked) dir.value = picked;
}
</script>

<template>
  <StepShell
    title="Onde salvar?"
    subtitle="Escolha a pasta de destino para o download."
    step-label="Passo 5"
  >
    <button class="dest-picker" type="button" @click="choose">
      <span class="dest-icon" aria-hidden="true">
        <svg
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path
            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
          />
        </svg>
      </span>
      <span class="dest-text">
        <span class="dest-title">
          {{ dir ? "Pasta escolhida" : "Escolher pasta..." }}
        </span>
        <span v-if="dir" class="dest-path">{{ dir }}</span>
      </span>
    </button>

    <template #actions>
      <button class="btn-outline" type="button" @click="emit('back')">
        Voltar
      </button>
      <button
        class="btn-primary"
        type="button"
        :disabled="!dir"
        @click="emit('next', dir)"
      >
        Iniciar download
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.dest-picker {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  width: 100%;
  text-align: left;
  padding: 1rem 1.1rem;
  background-color: var(--surface);
  border: 2px dashed var(--border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  font-family: inherit;
  transition: all 0.2s ease;
}

.dest-picker:hover {
  border-color: var(--primary);
  background-color: var(--primary-light);
}

.dest-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2.75rem;
  height: 2.75rem;
  flex-shrink: 0;
  color: var(--primary);
  background-color: var(--primary-light);
  border-radius: var(--radius-md);
}

.dest-text {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  min-width: 0;
}

.dest-title {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text);
}

.dest-path {
  font-size: 0.8rem;
  color: var(--text-secondary);
  word-break: break-all;
}

.dest-picker:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}
</style>
