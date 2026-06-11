<script setup lang="ts">
// Passo 2 — escolher entre baixar video (.mp4) ou somente audio (.mp3).

import { ref } from "vue";
import StepShell from "./StepShell.vue";
import SegmentedControl from "./SegmentedControl.vue";
import { formatTime, type DownloadKind, type ProbeResult } from "../lib/ytdlp";

const props = defineProps<{
  modelValue: DownloadKind;
  probe: ProbeResult;
  /** O download sera tratado como playlist inteira? */
  downloadingPlaylist: boolean;
}>();

const emit = defineEmits<{
  next: [kind: DownloadKind];
  back: [];
}>();

const kind = ref<DownloadKind>(props.modelValue);

const options: { value: DownloadKind; label: string }[] = [
  { value: "video", label: "Video (.mp4)" },
  { value: "audio", label: "Audio (.mp3)" },
];
</script>

<template>
  <StepShell
    title="O que voce quer baixar?"
    :subtitle="
      downloadingPlaylist
        ? `Playlist com ${probe.playlistCount} videos.`
        : probe.title
    "
    step-label="Passo 2"
  >
    <SegmentedControl v-model="kind" :options="options" />

    <div class="type-hint card">
      <template v-if="kind === 'video'">
        <strong>Video completo</strong> em formato <code>.mp4</code>.
        <span v-if="!probe.isPlaylist && probe.duration > 0">
          Duracao: {{ formatTime(probe.duration) }}.
        </span>
      </template>
      <template v-else>
        Extrai apenas o <strong>audio</strong> em formato <code>.mp3</code>
        com a melhor qualidade.
      </template>
    </div>

    <template #actions>
      <button class="btn-outline" type="button" @click="emit('back')">
        Voltar
      </button>
      <button class="btn-primary" type="button" @click="emit('next', kind)">
        Continuar
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.type-hint {
  text-align: left;
  font-size: 0.9rem;
  color: var(--text-secondary);
  line-height: 1.6;
}

.type-hint strong {
  color: var(--text);
  font-weight: 600;
}

.type-hint code {
  font-family: ui-monospace, "Cascadia Code", "Courier New", monospace;
  font-size: 0.85em;
  color: var(--primary);
  background-color: var(--primary-light);
  padding: 0.1rem 0.35rem;
  border-radius: var(--radius-sm);
}
</style>
