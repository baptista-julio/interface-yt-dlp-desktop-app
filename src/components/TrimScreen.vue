<script setup lang="ts">
// Passo 2.1.1 — corte por timeline. Por padrao cobre o video inteiro,
// entao o usuario pode simplesmente continuar para baixar tudo.
// Aparece apenas para URL de video unico (video ou audio).

import { computed, ref } from "vue";
import StepShell from "./StepShell.vue";
import TimelineTrim from "./TimelineTrim.vue";
import { formatTime, type DownloadKind, type ProbeResult } from "../lib/ytdlp";

const props = defineProps<{
  probe: ProbeResult;
  kind: DownloadKind;
}>();

const emit = defineEmits<{
  next: [start: number | null, end: number | null];
  back: [];
}>();

// Se a duracao for desconhecida (0), nao da pra cortar — segue direto.
const hasDuration = computed(() => props.probe.duration > 0);

const start = ref(0);
const end = ref(props.probe.duration);

// "Cortou" = o intervalo difere do video completo (com tolerancia).
const isTrimmed = computed(
  () => start.value > 0.5 || end.value < props.probe.duration - 0.5,
);

function reset() {
  start.value = 0;
  end.value = props.probe.duration;
}

function submit() {
  if (!hasDuration.value || !isTrimmed.value) {
    emit("next", null, null); // baixa completo
  } else {
    emit("next", start.value, end.value);
  }
}
</script>

<template>
  <StepShell
    title="Cortar um trecho?"
    :subtitle="
      kind === 'audio'
        ? 'Arraste para extrair so um trecho do audio, ou continue para baixar tudo.'
        : 'Arraste as alcas para escolher um trecho, ou continue para baixar o video inteiro.'
    "
    step-label="Passo 4"
  >
    <template v-if="hasDuration">
      <TimelineTrim
        v-model:start="start"
        v-model:end="end"
        :duration="probe.duration"
      />

      <div class="trim-summary">
        <span v-if="isTrimmed">
          Trecho de <strong>{{ formatTime(start) }}</strong> a
          <strong>{{ formatTime(end) }}</strong>
        </span>
        <span v-else class="trim-full">
          {{ kind === "audio" ? "Audio" : "Video" }} completo
          ({{ formatTime(probe.duration) }})
        </span>
        <button
          v-if="isTrimmed"
          type="button"
          class="trim-reset"
          @click="reset"
        >
          Voltar ao completo
        </button>
      </div>
    </template>

    <p v-else class="trim-unknown card">
      A duracao deste video nao pode ser lida, entao o corte nao esta
      disponivel. O arquivo sera baixado por completo.
    </p>

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
.trim-summary {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.4rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.trim-summary strong {
  color: var(--primary);
  font-variant-numeric: tabular-nums;
}

.trim-full {
  font-weight: 500;
}

.trim-reset {
  appearance: none;
  background: none;
  border: none;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.8rem;
  color: var(--primary);
  text-decoration: underline;
  padding: 0;
}

.trim-unknown {
  text-align: left;
  font-size: 0.9rem;
  color: var(--text-secondary);
  line-height: 1.6;
}
</style>
