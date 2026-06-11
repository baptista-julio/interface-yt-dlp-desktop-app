<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import ThemeToggle from "./components/ThemeToggle.vue";
import UrlScreen from "./components/UrlScreen.vue";
import PlaylistScreen from "./components/PlaylistScreen.vue";
import TypeScreen from "./components/TypeScreen.vue";
import QualityScreen from "./components/QualityScreen.vue";
import TrimScreen from "./components/TrimScreen.vue";
import DestPickScreen from "./components/DestPickScreen.vue";
import ProcessingScreen from "./components/ProcessingScreen.vue";
import ResultScreen from "./components/ResultScreen.vue";
import ErrorScreen from "./components/ErrorScreen.vue";
import type { DownloadKind, ProbeResult } from "./lib/ytdlp";

type Step =
  | "url"
  | "playlist"
  | "type"
  | "quality"
  | "trim"
  | "dest"
  | "processing"
  | "result"
  | "error";

interface Choices {
  url: string;
  /** Probe do link: traz "e playlist?" + duracao/qualidades reais do video. */
  probe: ProbeResult | null;
  /** Escolha do usuario: baixar a playlist inteira (so relevante se a URL e playlist). */
  downloadPlaylist: boolean;
  kind: DownloadKind;
  quality: number | null;
  start: number | null;
  end: number | null;
  outputDir: string;
}

const initialChoices = (): Choices => ({
  url: "",
  probe: null,
  downloadPlaylist: true,
  kind: "video",
  quality: null,
  start: null,
  end: null,
  outputDir: "",
});

const step = ref<Step>("url");
const choices = reactive<Choices>(initialChoices());
const errorMessage = ref("");
const lastOutputDir = ref("");

// Direcao da transicao (1 = avancar, -1 = voltar) para a animacao.
const direction = ref(1);

// A URL aponta para uma playlist? (baseado no probe ORIGINAL do link)
const urlIsPlaylist = computed(() => choices.probe?.isPlaylist ?? false);

// Vamos efetivamente baixar como playlist? (playlist detectada E usuario escolheu inteira)
const downloadingPlaylist = computed(
  () => urlIsPlaylist.value && choices.downloadPlaylist,
);

// --- Navegacao ---

function goTo(next: Step, dir: 1 | -1 = 1) {
  direction.value = dir;
  step.value = next;
}

// Passo 1 -> URL validada (probe pronto)
function handleUrlDone(probe: ProbeResult, url: string) {
  choices.url = url;
  choices.probe = probe;
  // Reseta escolhas dependentes.
  choices.downloadPlaylist = true;
  choices.quality = probe.videoQualities[0] ?? null;
  choices.start = null;
  choices.end = null;
  // Se e playlist real, pergunta primeiro; senao, vai direto ao tipo.
  goTo(probe.isPlaylist ? "playlist" : "type", 1);
}

// Passo playlist -> baixar inteira ou so o video
function handlePlaylistNext(downloadPlaylist: boolean) {
  choices.downloadPlaylist = downloadPlaylist;
  goTo("type", 1);
}

// Passo 2 -> tipo (video/audio)
function handleTypeNext(kind: DownloadKind) {
  choices.kind = kind;
  if (kind === "video") {
    goTo("quality", 1);
  } else {
    // Audio: pula qualidade. Corte so quando NAO e playlist inteira.
    goTo(downloadingPlaylist.value ? "dest" : "trim", 1);
  }
}

// Passo 3 -> qualidade
function handleQualityNext(quality: number | null) {
  choices.quality = quality;
  goTo(downloadingPlaylist.value ? "dest" : "trim", 1);
}

// Passo 4 -> corte
function handleTrimNext(start: number | null, end: number | null) {
  choices.start = start;
  choices.end = end;
  goTo("dest", 1);
}

// Passo 5 -> pasta escolhida, inicia download
function handleDestNext(dir: string) {
  choices.outputDir = dir;
  goTo("processing", 1);
}

// --- Botoes "Voltar" ---

function backFromPlaylist() {
  goTo("url", -1);
}
function backFromType() {
  goTo(urlIsPlaylist.value ? "playlist" : "url", -1);
}
function backFromQuality() {
  goTo("type", -1);
}
function backFromTrim() {
  goTo(choices.kind === "video" ? "quality" : "type", -1);
}
function backFromDest() {
  if (downloadingPlaylist.value) {
    goTo(choices.kind === "video" ? "quality" : "type", -1);
  } else {
    goTo("trim", -1);
  }
}

// --- Fim do fluxo ---

function handleDownloadDone(dir: string) {
  lastOutputDir.value = dir;
  goTo("result", 1);
}

function handleDownloadError(message: string) {
  errorMessage.value = message;
  goTo("error", 1);
}

function restart() {
  Object.assign(choices, initialChoices());
  errorMessage.value = "";
  lastOutputDir.value = "";
  goTo("url", 1);
}

function retryFromError() {
  // Volta ao inicio mantendo a URL digitada para conveniencia.
  goTo("url", -1);
}
</script>

<template>
  <ThemeToggle />

  <main class="app-container">
    <Transition :name="direction === 1 ? 'screen' : 'screen-back'" mode="out-in">
      <UrlScreen
        v-if="step === 'url'"
        key="url"
        :initial-url="choices.url"
        @done="handleUrlDone"
      />

      <PlaylistScreen
        v-else-if="step === 'playlist'"
        key="playlist"
        :probe="choices.probe!"
        :selected="choices.downloadPlaylist"
        @next="handlePlaylistNext"
        @back="backFromPlaylist"
      />

      <TypeScreen
        v-else-if="step === 'type'"
        key="type"
        :model-value="choices.kind"
        :probe="choices.probe!"
        :downloading-playlist="downloadingPlaylist"
        @next="handleTypeNext"
        @back="backFromType"
      />

      <QualityScreen
        v-else-if="step === 'quality'"
        key="quality"
        :probe="choices.probe!"
        :selected="choices.quality"
        :downloading-playlist="downloadingPlaylist"
        @next="handleQualityNext"
        @back="backFromQuality"
      />

      <TrimScreen
        v-else-if="step === 'trim'"
        key="trim"
        :probe="choices.probe!"
        :kind="choices.kind"
        @next="handleTrimNext"
        @back="backFromTrim"
      />

      <DestPickScreen
        v-else-if="step === 'dest'"
        key="dest"
        @next="handleDestNext"
        @back="backFromDest"
      />

      <ProcessingScreen
        v-else-if="step === 'processing'"
        key="processing"
        :options="{
          url: choices.url,
          kind: choices.kind,
          quality: choices.quality,
          start: choices.start,
          end: choices.end,
          outputDir: choices.outputDir,
          isPlaylist: downloadingPlaylist,
        }"
        @done="handleDownloadDone"
        @error="handleDownloadError"
        @canceled="restart"
      />

      <ResultScreen
        v-else-if="step === 'result'"
        key="result"
        :output-dir="lastOutputDir"
        @restart="restart"
      />

      <ErrorScreen
        v-else
        key="error"
        :message="errorMessage"
        @retry="retryFromError"
      />
    </Transition>
  </main>
</template>

<style scoped>
/* Transicao "avancar" usa a animacao .screen padrao (definida em globals.css).
   A transicao "voltar" inverte o sentido do slide. */
.screen-back-enter-active,
.screen-back-leave-active {
  transition: opacity 0.3s ease-in-out, transform 0.3s ease-in-out;
}
.screen-back-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}
.screen-back-leave-to {
  opacity: 0;
  transform: translateY(20px);
}
</style>
