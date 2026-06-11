<script setup lang="ts">
// Tela de erro — mensagem + tentar novamente.

import StepShell from "./StepShell.vue";

defineProps<{
  message: string;
}>();

const emit = defineEmits<{
  retry: [];
}>();
</script>

<template>
  <StepShell
    title="Algo deu errado"
    subtitle="Nao foi possivel concluir o download."
  >
    <div class="error-icon" aria-hidden="true">
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
        <circle cx="12" cy="12" r="10" />
        <line x1="15" y1="9" x2="9" y2="15" />
        <line x1="9" y1="9" x2="15" y2="15" />
      </svg>
    </div>

    <p v-if="message" class="error-box">{{ message }}</p>

    <template #actions>
      <button class="btn-primary" type="button" @click="emit('retry')">
        Tentar novamente
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.error-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  margin: 0 auto;
  color: white;
  background-color: var(--error);
  border-radius: 50%;
  animation: popIn 0.3s ease-out both;
}

.error-box {
  font-size: 0.9rem;
  color: var(--error);
  background-color: var(--error-light);
  border: 1px solid var(--error);
  padding: 0.75rem 1rem;
  border-radius: var(--radius-md);
  text-align: left;
  line-height: 1.5;
  max-width: 100%;
  word-break: break-word;
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
