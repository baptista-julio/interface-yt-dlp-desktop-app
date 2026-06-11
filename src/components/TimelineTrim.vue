<script setup lang="ts">
// Timeline de corte com dois handles arrastaveis (inicio e fim).
// Por padrao cobre o video inteiro (0 .. duracao). v-model em segundos.

import { computed, ref } from "vue";
import { formatTime } from "../lib/ytdlp";

const props = defineProps<{
  duration: number;
  start: number;
  end: number;
}>();

const emit = defineEmits<{
  "update:start": [value: number];
  "update:end": [value: number];
}>();

const trackRef = ref<HTMLElement | null>(null);
const dragging = ref<"start" | "end" | null>(null);

// Margem minima entre os dois handles (em segundos).
const MIN_GAP = 0.5;

const startPct = computed(() =>
  props.duration > 0 ? (props.start / props.duration) * 100 : 0,
);
const endPct = computed(() =>
  props.duration > 0 ? (props.end / props.duration) * 100 : 100,
);

function clientXToSeconds(clientX: number): number {
  const track = trackRef.value;
  if (!track || props.duration <= 0) return 0;
  const rect = track.getBoundingClientRect();
  const ratio = Math.min(1, Math.max(0, (clientX - rect.left) / rect.width));
  return ratio * props.duration;
}

function onPointerDown(handle: "start" | "end", event: PointerEvent) {
  dragging.value = handle;
  (event.target as HTMLElement).setPointerCapture(event.pointerId);
}

function onPointerMove(event: PointerEvent) {
  if (!dragging.value) return;
  const seconds = clientXToSeconds(event.clientX);
  if (dragging.value === "start") {
    emit("update:start", Math.min(seconds, props.end - MIN_GAP));
  } else {
    emit("update:end", Math.max(seconds, props.start + MIN_GAP));
  }
}

function onPointerUp() {
  dragging.value = null;
}

// Teclado: setas movem 1s, Shift+seta move 5s.
function onKey(handle: "start" | "end", event: KeyboardEvent) {
  const stepSize = event.shiftKey ? 5 : 1;
  let delta = 0;
  if (event.key === "ArrowLeft" || event.key === "ArrowDown") delta = -stepSize;
  else if (event.key === "ArrowRight" || event.key === "ArrowUp") delta = stepSize;
  else return;
  event.preventDefault();

  if (handle === "start") {
    const next = Math.max(0, Math.min(props.start + delta, props.end - MIN_GAP));
    emit("update:start", next);
  } else {
    const next = Math.min(
      props.duration,
      Math.max(props.end + delta, props.start + MIN_GAP),
    );
    emit("update:end", next);
  }
}
</script>

<template>
  <div class="timeline">
    <div
      ref="trackRef"
      class="timeline-track"
      @pointermove="onPointerMove"
      @pointerup="onPointerUp"
    >
      <!-- Trecho selecionado -->
      <div
        class="timeline-selection"
        :style="{ left: `${startPct}%`, right: `${100 - endPct}%` }"
      />

      <!-- Handle inicio -->
      <button
        type="button"
        class="timeline-handle"
        :style="{ left: `${startPct}%` }"
        :aria-label="`Inicio do corte: ${formatTime(start)}`"
        role="slider"
        :aria-valuemin="0"
        :aria-valuemax="duration"
        :aria-valuenow="start"
        @pointerdown="onPointerDown('start', $event)"
        @keydown="onKey('start', $event)"
      />

      <!-- Handle fim -->
      <button
        type="button"
        class="timeline-handle"
        :style="{ left: `${endPct}%` }"
        :aria-label="`Fim do corte: ${formatTime(end)}`"
        role="slider"
        :aria-valuemin="0"
        :aria-valuemax="duration"
        :aria-valuenow="end"
        @pointerdown="onPointerDown('end', $event)"
        @keydown="onKey('end', $event)"
      />
    </div>

    <div class="timeline-times">
      <span class="time-pill">{{ formatTime(start) }}</span>
      <span class="time-total">{{ formatTime(end - start) }} selecionados</span>
      <span class="time-pill">{{ formatTime(end) }}</span>
    </div>
  </div>
</template>

<style scoped>
.timeline {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  width: 100%;
  user-select: none;
}

.timeline-track {
  position: relative;
  height: 12px;
  background-color: var(--border);
  border-radius: 9999px;
  touch-action: none;
}

.timeline-selection {
  position: absolute;
  top: 0;
  bottom: 0;
  background-color: var(--primary);
  border-radius: 9999px;
}

.timeline-handle {
  position: absolute;
  top: 50%;
  width: 1.25rem;
  height: 1.25rem;
  transform: translate(-50%, -50%);
  background-color: var(--surface);
  border: 2px solid var(--primary);
  border-radius: 50%;
  box-shadow: var(--shadow-md);
  cursor: grab;
  padding: 0;
  transition: box-shadow 0.15s ease;
}

.timeline-handle:active {
  cursor: grabbing;
}

.timeline-handle:hover {
  box-shadow: var(--shadow-lg);
}

.timeline-handle:focus-visible {
  outline: none;
  box-shadow: var(--focus-ring), var(--shadow-md);
}

.timeline-times {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  font-variant-numeric: tabular-nums;
}

.time-pill {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text);
  background-color: var(--bg);
  border: 1px solid var(--border);
  padding: 0.25rem 0.6rem;
  border-radius: var(--radius-sm);
}

.time-total {
  font-size: 0.78rem;
  color: var(--text-secondary);
}
</style>
