<script setup lang="ts">
// Passo 1 — input da URL. Ao continuar, chama probe_url e mostra loader.

import { ref } from "vue";
import StepShell from "./StepShell.vue";
import { probeUrl, type ProbeResult } from "../lib/ytdlp";

const props = defineProps<{
  initialUrl?: string;
}>();

const emit = defineEmits<{
  done: [probe: ProbeResult, url: string];
}>();

const url = ref(props.initialUrl ?? "");
const loading = ref(false);
const error = ref("");

async function submit() {
  const value = url.value.trim();
  if (!value || loading.value) return;

  loading.value = true;
  error.value = "";
  try {
    const probe = await probeUrl(value);
    emit("done", probe, value);
  } catch (e) {
    error.value = typeof e === "string" ? e : "Nao foi possivel ler a URL.";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <StepShell
    title="Baixar do YouTube"
    subtitle="Cole o link de um video ou de uma playlist para comecar."
    step-label="Passo 1"
  >
    <form class="url-form" @submit.prevent="submit">
      <input
        v-model="url"
        class="text-input"
        type="text"
        inputmode="url"
        placeholder="https://www.youtube.com/watch?v=..."
        autofocus
        :disabled="loading"
        @input="error = ''"
      />

      <p v-if="error" class="field-error">{{ error }}</p>

      <button class="btn-primary" type="submit" :disabled="!url.trim() || loading">
        <span v-if="loading" class="spinner btn-spinner" aria-hidden="true" />
        {{ loading ? "Lendo o link..." : "Continuar" }}
      </button>
    </form>
  </StepShell>
</template>

<style scoped>
.url-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
}

.field-error {
  font-size: 0.85rem;
  color: var(--error);
  text-align: left;
}

.btn-spinner {
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255, 255, 255, 0.4);
  border-top-color: white;
  border-radius: 50%;
}
</style>
