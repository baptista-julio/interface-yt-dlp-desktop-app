<script setup lang="ts">
// Passo 7 — sucesso. Abrir pasta e baixar outro.

import { ref } from "vue";
import StepShell from "./StepShell.vue";
import { openFolder } from "../lib/ytdlp";

const props = defineProps<{
  outputDir: string;
}>();

const emit = defineEmits<{
  restart: [];
}>();

const opening = ref(false);

async function open() {
  if (opening.value) return;
  opening.value = true;
  try {
    await openFolder(props.outputDir);
  } catch {
    /* silencioso — abrir a pasta e best-effort */
  } finally {
    opening.value = false;
  }
}
</script>

<template>
  <StepShell
    title="Download concluido!"
    subtitle="Seu arquivo foi salvo na pasta escolhida."
  >
    <div class="success-icon" aria-hidden="true">
      <svg
        width="32"
        height="32"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polyline points="20 6 9 17 4 12" />
      </svg>
    </div>

    <p v-if="outputDir" class="success-path">{{ outputDir }}</p>

    <template #actions>
      <button class="btn-secondary" type="button" @click="open">
        Abrir pasta
      </button>
      <button class="btn-primary" type="button" @click="emit('restart')">
        Baixar outro
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.success-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  margin: 0 auto;
  color: white;
  background-color: var(--success);
  border-radius: 50%;
  animation: popIn 0.3s ease-out both;
}

.success-path {
  font-size: 0.8rem;
  color: var(--text-secondary);
  word-break: break-all;
  background-color: var(--bg);
  border: 1px solid var(--border);
  padding: 0.5rem 0.75rem;
  border-radius: var(--radius-md);
}

@keyframes popIn {
  from {
    opacity: 0;
    transform: scale(0.8);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
