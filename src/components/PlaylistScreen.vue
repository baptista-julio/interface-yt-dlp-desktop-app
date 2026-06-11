<script setup lang="ts">
// Passo intermediario (so quando a URL e uma playlist) — escolher entre
// baixar a playlist inteira ou apenas o video atual.

import { ref } from "vue";
import StepShell from "./StepShell.vue";
import type { ProbeResult } from "../lib/ytdlp";

const props = defineProps<{
  probe: ProbeResult;
  /** Escolha atual (true = playlist inteira). */
  selected: boolean;
}>();

const emit = defineEmits<{
  next: [downloadPlaylist: boolean];
  back: [];
}>();

const choice = ref<boolean>(props.selected);

// O probe ja traz duracao e qualidades reais do video (sondamos com
// --no-playlist), entao nao e preciso re-sondar ao escolher "so este video".
function submit() {
  emit("next", choice.value);
}
</script>

<template>
  <StepShell
    title="Isso e uma playlist"
    :subtitle="`Encontramos uma playlist com ${probe.playlistCount} videos. O que voce quer baixar?`"
    step-label="Playlist"
  >
    <div class="playlist-options" role="radiogroup">
      <button
        type="button"
        role="radio"
        class="playlist-item"
        :class="{ active: choice === true }"
        :aria-checked="choice === true"
        @click="choice = true"
      >
        <span class="playlist-radio" aria-hidden="true">
          <span v-if="choice === true" class="playlist-dot" />
        </span>
        <span class="playlist-text">
          <span class="playlist-title">Baixar a playlist inteira</span>
          <span class="playlist-desc">
            Todos os {{ probe.playlistCount }} videos, com o mesmo formato e
            qualidade.
          </span>
        </span>
      </button>

      <button
        type="button"
        role="radio"
        class="playlist-item"
        :class="{ active: choice === false }"
        :aria-checked="choice === false"
        @click="choice = false"
      >
        <span class="playlist-radio" aria-hidden="true">
          <span v-if="choice === false" class="playlist-dot" />
        </span>
        <span class="playlist-text">
          <span class="playlist-title">Baixar so este video</span>
          <span class="playlist-desc">
            Apenas o video do link, ignorando a playlist. Permite cortar um
            trecho.
          </span>
        </span>
      </button>
    </div>

    <template #actions>
      <button class="btn-outline" type="button" @click="emit('back')">
        Voltar
      </button>
      <button class="btn-primary" type="button" @click="submit">
        Continuar
      </button>
    </template>
  </StepShell>
</template>

<style scoped>
.playlist-options {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  width: 100%;
}

.playlist-item {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  text-align: left;
  padding: 0.9rem 1rem;
  background-color: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  cursor: pointer;
  font-family: inherit;
  transition: all 0.2s ease;
}

.playlist-item:hover {
  border-color: var(--border-dark);
}

.playlist-item.active {
  border-color: var(--primary);
  background-color: var(--primary-light);
}

.playlist-radio {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.1rem;
  height: 1.1rem;
  margin-top: 0.15rem;
  border-radius: 50%;
  border: 2px solid var(--border-dark);
  flex-shrink: 0;
}

.playlist-item.active .playlist-radio {
  border-color: var(--primary);
}

.playlist-dot {
  width: 0.55rem;
  height: 0.55rem;
  border-radius: 50%;
  background-color: var(--primary);
}

.playlist-text {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  min-width: 0;
}

.playlist-title {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--text);
}

.playlist-desc {
  font-size: 0.82rem;
  color: var(--text-secondary);
  line-height: 1.45;
}

.playlist-item:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}
</style>
