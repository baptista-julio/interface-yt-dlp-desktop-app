<script setup lang="ts">
// Passo 2.1 — escolher a qualidade do video.
// Usa as alturas detectadas no probe; cai para um fallback fixo quando
// nenhuma foi listada (ex: playlist).

import { computed, ref } from "vue";
import StepShell from "./StepShell.vue";
import { FALLBACK_QUALITIES, type ProbeResult } from "../lib/ytdlp";

const props = defineProps<{
  probe: ProbeResult;
  selected: number | null;
  /** O download sera tratado como playlist inteira? */
  downloadingPlaylist: boolean;
}>();

const emit = defineEmits<{
  next: [quality: number | null];
  back: [];
}>();

const qualities = computed<number[]>(() =>
  props.probe.videoQualities.length > 0
    ? props.probe.videoQualities
    : FALLBACK_QUALITIES,
);

const choice = ref<number | null>(props.selected ?? qualities.value[0] ?? null);

function label(height: number): string {
  if (height >= 2160) return `${height}p (4K)`;
  if (height >= 1440) return `${height}p (2K)`;
  if (height >= 1080) return `${height}p (Full HD)`;
  if (height >= 720) return `${height}p (HD)`;
  return `${height}p`;
}
</script>

<template>
  <StepShell
    title="Qualidade do video"
    subtitle="Escolha a resolucao. O app baixa a melhor faixa disponivel ate esse limite."
    step-label="Passo 3"
  >
    <p v-if="downloadingPlaylist" class="playlist-note">
      Aplicado a todos os {{ probe.playlistCount }} videos da playlist.
    </p>

    <div class="quality-list" role="radiogroup">
      <button
        v-for="q in qualities"
        :key="q"
        type="button"
        role="radio"
        class="quality-item"
        :class="{ active: choice === q }"
        :aria-checked="choice === q"
        @click="choice = q"
      >
        <span class="quality-label">{{ label(q) }}</span>
        <span class="quality-radio" aria-hidden="true">
          <span v-if="choice === q" class="quality-dot" />
        </span>
      </button>
    </div>

    <template #actions>
      <button class="btn-outline" type="button" @click="emit('back')">
        Voltar
      </button>
      <button
        class="btn-primary"
        type="button"
        :disabled="choice === null"
        @click="emit('next', choice)"
      >
        Continuar
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.playlist-note {
  font-size: 0.85rem;
  color: var(--primary);
  background-color: var(--primary-light);
  padding: 0.5rem 0.75rem;
  border-radius: var(--radius-md);
  text-align: center;
}

.quality-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
}

.quality-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.8rem 1rem;
  background-color: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-family: inherit;
  font-size: 0.95rem;
  color: var(--text);
  transition: all 0.2s ease;
}

.quality-item:hover {
  border-color: var(--border-dark);
}

.quality-item.active {
  border-color: var(--primary);
  background-color: var(--primary-light);
}

.quality-label {
  font-weight: 500;
}

.quality-radio {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.1rem;
  height: 1.1rem;
  border-radius: 50%;
  border: 2px solid var(--border-dark);
  flex-shrink: 0;
}

.quality-item.active .quality-radio {
  border-color: var(--primary);
}

.quality-dot {
  width: 0.55rem;
  height: 0.55rem;
  border-radius: 50%;
  background-color: var(--primary);
}

.quality-item:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}
</style>
