<script setup lang="ts">
// Passo 6 — download em andamento. Inicia o download e escuta os eventos
// de progresso, emitindo "done"/"error"/"canceled" ao final.

import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import StepShell from "./StepShell.vue";
import {
  cancelDownload,
  onCanceled,
  onDone,
  onError,
  onProgress,
  startDownload,
  type DownloadOptions,
} from "../lib/ytdlp";
import type { UnlistenFn } from "@tauri-apps/api/event";

const props = defineProps<{
  options: DownloadOptions;
}>();

const emit = defineEmits<{
  done: [outputDir: string];
  error: [message: string];
  canceled: [];
}>();

const percent = ref(0);
const speed = ref("");
const eta = ref("");
const itemIndex = ref<number | null>(null);
const itemTotal = ref<number | null>(null);
const canceling = ref(false);

const unlisteners: UnlistenFn[] = [];

const isPlaylistRun = computed(
  () => props.options.isPlaylist && (itemTotal.value ?? 0) > 1,
);

const statusText = computed(() => {
  if (canceling.value) return "Cancelando...";
  if (percent.value >= 100) return "Finalizando (convertendo)...";
  if (percent.value > 0) return "Baixando...";
  return "Preparando...";
});

async function cancel() {
  if (canceling.value) return;
  canceling.value = true;
  try {
    await cancelDownload();
  } catch {
    /* se falhar, o evento de termino ainda resolve o fluxo */
  }
}

onMounted(async () => {
  unlisteners.push(
    await onProgress((p) => {
      percent.value = p.percent;
      speed.value = p.speed;
      eta.value = p.eta;
      itemIndex.value = p.itemIndex;
      itemTotal.value = p.itemTotal;
    }),
  );
  unlisteners.push(await onDone((p) => emit("done", p.outputDir)));
  unlisteners.push(await onError((p) => emit("error", p.message)));
  unlisteners.push(await onCanceled(() => emit("canceled")));

  try {
    await startDownload(props.options);
  } catch (e) {
    emit("error", typeof e === "string" ? e : "Falha ao iniciar o download.");
  }
});

onBeforeUnmount(() => {
  unlisteners.forEach((fn) => fn());
});
</script>

<template>
  <StepShell
    title="Baixando"
    :subtitle="
      isPlaylistRun
        ? `Video ${itemIndex ?? 1} de ${itemTotal} da playlist`
        : statusText
    "
  >
    <div class="progress-panel">
      <div class="progress-head">
        <span class="progress-percent">{{ Math.round(percent) }}%</span>
        <span class="progress-status">{{ statusText }}</span>
      </div>

      <div class="progress-bar" role="progressbar" :aria-valuenow="Math.round(percent)">
        <div class="progress-fill" :style="{ width: `${Math.min(percent, 100)}%` }" />
      </div>

      <div v-if="speed || eta || isPlaylistRun" class="progress-meta">
        <span v-if="speed">{{ speed }}</span>
        <span v-if="eta">ETA {{ eta }}</span>
        <span v-if="isPlaylistRun" class="progress-item">
          {{ itemIndex ?? 1 }}/{{ itemTotal }}
        </span>
      </div>
    </div>

    <p class="processing-note">
      Mantenha o app aberto ate concluir. A conversao final pode levar alguns
      segundos.
    </p>

    <template #actions>
      <button
        class="btn-outline"
        type="button"
        :disabled="canceling"
        @click="cancel"
      >
        {{ canceling ? "Cancelando..." : "Cancelar" }}
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.progress-panel {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
  background-color: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 1.25rem;
  box-shadow: var(--shadow-sm);
}

.progress-head {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.15rem;
}

.progress-status {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.progress-percent {
  font-size: 2rem;
  font-weight: 700;
  color: var(--primary);
  font-variant-numeric: tabular-nums;
  line-height: 1.1;
}

.progress-bar {
  width: 100%;
  height: 10px;
  background-color: var(--border);
  border-radius: 5px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: var(--primary);
  border-radius: 5px;
  transition: width 0.3s ease-out;
}

.progress-meta {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  font-size: 0.8rem;
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
}

.processing-note {
  font-size: 0.8rem;
  color: var(--text-secondary);
  text-align: center;
  max-width: 22rem;
  margin: 0 auto;
}
</style>
