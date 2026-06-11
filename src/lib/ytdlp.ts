// Wrapper TS para os comandos Rust e os eventos de progresso do download.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";

// --- Tipos compartilhados com o backend (ver src-tauri/src/lib.rs) ---

export interface ProbeResult {
  isPlaylist: boolean;
  playlistCount: number;
  title: string;
  /** Duracao em segundos (0 para playlist/desconhecida). */
  duration: number;
  thumbnail: string | null;
  /** Alturas de video disponiveis, em ordem decrescente. */
  videoQualities: number[];
}

export type DownloadKind = "video" | "audio";

export interface DownloadOptions {
  url: string;
  kind: DownloadKind;
  quality: number | null;
  start: number | null;
  end: number | null;
  outputDir: string;
  isPlaylist: boolean;
}

export interface ProgressPayload {
  percent: number;
  speed: string;
  eta: string;
  itemIndex: number | null;
  itemTotal: number | null;
}

export interface DonePayload {
  outputDir: string;
}

export interface ErrorPayload {
  message: string;
}

// --- Comandos ---

export function probeUrl(
  url: string,
  noPlaylist = false,
): Promise<ProbeResult> {
  return invoke<ProbeResult>("probe_url", { url, noPlaylist });
}

export function startDownload(options: DownloadOptions): Promise<void> {
  return invoke("start_download", { options });
}

export function cancelDownload(): Promise<void> {
  return invoke("cancel_download");
}

// --- Eventos ---

export function onProgress(
  cb: (p: ProgressPayload) => void,
): Promise<UnlistenFn> {
  return listen<ProgressPayload>("download://progress", (e) => cb(e.payload));
}

export function onDone(cb: (p: DonePayload) => void): Promise<UnlistenFn> {
  return listen<DonePayload>("download://done", (e) => cb(e.payload));
}

export function onError(cb: (p: ErrorPayload) => void): Promise<UnlistenFn> {
  return listen<ErrorPayload>("download://error", (e) => cb(e.payload));
}

export function onCanceled(cb: () => void): Promise<UnlistenFn> {
  return listen("download://canceled", () => cb());
}

// --- Helpers de sistema ---

/** Abre o seletor nativo de pasta. Retorna o caminho ou null se cancelado. */
export async function pickOutputDir(): Promise<string | null> {
  const result = await openDialog({
    directory: true,
    multiple: false,
    title: "Escolha a pasta de destino",
  });
  return typeof result === "string" ? result : null;
}

/** Abre o gerenciador de arquivos na pasta indicada. */
export async function openFolder(dir: string): Promise<void> {
  await openPath(dir);
}

// --- Utilidades ---

/** Formata segundos em HH:MM:SS (ou MM:SS quando < 1h). */
export function formatTime(totalSeconds: number): string {
  const s = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(s / 3600);
  const minutes = Math.floor((s % 3600) / 60);
  const seconds = s % 60;
  const pad = (n: number) => n.toString().padStart(2, "0");
  return hours > 0
    ? `${hours}:${pad(minutes)}:${pad(seconds)}`
    : `${minutes}:${pad(seconds)}`;
}

/** Qualidades de fallback quando o probe nao lista nenhuma (ex: playlist). */
export const FALLBACK_QUALITIES = [1080, 720, 480, 360];
