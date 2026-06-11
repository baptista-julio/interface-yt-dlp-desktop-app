// yt-dlp interface — comandos Tauri.
//
// O yt-dlp e o ffmpeg sao empacotados como *sidecars* (ver tauri.conf.json
// -> bundle.externalBin e capabilities/default.json). Aqui montamos os
// argumentos conforme as escolhas do usuario e transmitimos o progresso ao
// frontend via eventos Tauri.

use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

/// Estado compartilhado: o processo de download em andamento (se houver),
/// para permitir o cancelamento.
#[derive(Default)]
struct DownloadState {
    child: Mutex<Option<CommandChild>>,
    /// Marca que o cancelamento foi pedido, para nao emitir "error" ao matar.
    canceled: Mutex<bool>,
}

/// Marcador usado no `--progress-template`. Cada campo e separado por '|'
/// para facilitar o parse linha a linha do stdout.
const PROGRESS_PREFIX: &str = "YTDLPROG|";
const PROGRESS_TEMPLATE: &str = "YTDLPROG|%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s|%(info.playlist_index)s|%(info.n_entries)s";

// ---------------------------------------------------------------------------
// Tipos compartilhados com o frontend
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub is_playlist: bool,
    pub playlist_count: u32,
    pub title: String,
    /// Duracao em segundos (0 quando desconhecida ou playlist).
    pub duration: f64,
    pub thumbnail: Option<String>,
    /// Alturas de video disponiveis, em ordem decrescente (ex: [1080, 720, 480]).
    pub video_qualities: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub url: String,
    /// "video" ou "audio".
    pub kind: String,
    /// Altura maxima do video (ex: 1080). Ignorado para audio.
    pub quality: Option<u32>,
    /// Inicio do corte em segundos. Aplicado apenas a video unico.
    pub start: Option<f64>,
    /// Fim do corte em segundos. Aplicado apenas a video unico.
    pub end: Option<f64>,
    /// Pasta de destino escolhida pelo usuario.
    pub output_dir: String,
    pub is_playlist: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressPayload {
    /// Progresso GERAL de 0 a 100, ja remapeado entre as faixas (video+audio)
    /// e monotonico (nunca diminui). Ver `DownloadProgress`.
    percent: f64,
    speed: String,
    eta: String,
    /// Fase atual: "preparing" (obtendo metadados, sem %), "downloading"
    /// (baixando faixas) ou "converting" (merge/extracao de audio).
    phase: String,
    /// Indice do item atual numa playlist (1-based); None para video unico.
    item_index: Option<u32>,
    /// Total de itens numa playlist; None para video unico.
    item_total: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DonePayload {
    /// Pasta onde os arquivos foram salvos (para o botao "Abrir pasta").
    output_dir: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorPayload {
    message: String,
}

// ---------------------------------------------------------------------------
// probe_url — coleta metadados (playlist?, duracao, qualidades) via `yt-dlp -J`
// ---------------------------------------------------------------------------

#[tauri::command]
async fn probe_url(
    app: AppHandle,
    url: String,
    no_playlist: Option<bool>,
) -> Result<ProbeResult, String> {
    // Estrategia para ser RAPIDO:
    // - Sempre sondamos o VIDEO sozinho com --no-playlist (~2s). Isso ja da
    //   duracao e qualidades reais, habilitando o corte.
    // - So tratamos como playlist quando a URL tem uma playlist DE VERDADE
    //   (list=PL/OL/FL/UU/LL...). Mix/Radio (list=RD..., RDMM..., start_radio)
    //   sao playlists infinitas auto-geradas: ignoramos e tratamos como video.
    let forced_single = no_playlist.unwrap_or(false);
    let has_real_playlist = !forced_single && url_has_real_playlist(&url);

    // 1) Probe rapido do video (sempre --no-playlist).
    let video_json = run_probe(&app, &url, true).await?;
    let mut result = parse_probe(&video_json);

    // 2) Se ha playlist real, conta os itens com um probe leve --flat-playlist.
    if has_real_playlist {
        if let Ok(list_json) = run_probe(&app, &url, false).await {
            let count = list_json
                .get("entries")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u32)
                .or_else(|| {
                    list_json
                        .get("playlist_count")
                        .and_then(|v| v.as_u64())
                        .map(|n| n as u32)
                })
                .unwrap_or(0);
            // So marca como playlist se realmente houver mais de 1 item.
            if count > 1 {
                result.is_playlist = true;
                result.playlist_count = count;
                if let Some(t) = list_json.get("title").and_then(|v| v.as_str()) {
                    result.title = t.to_string();
                }
            }
        }
    }

    Ok(result)
}

/// Roda o yt-dlp em modo probe (-J). `single=true` usa --no-playlist (rapido,
/// info completa do video); `single=false` usa --flat-playlist (lista os itens).
async fn run_probe(
    app: &AppHandle,
    url: &str,
    single: bool,
) -> Result<serde_json::Value, String> {
    let mut args: Vec<&str> = vec!["-J", "--no-warnings", "--no-playlist-reverse"];
    if single {
        args.push("--no-playlist");
    } else {
        args.push("--flat-playlist");
    }
    args.push(url);

    let output = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Falha ao localizar o yt-dlp: {e}"))?
        .args(args)
        .output()
        .await
        .map_err(|e| format!("Falha ao executar o yt-dlp: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(friendly_error(&stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(stdout.trim())
        .map_err(|e| format!("Resposta invalida do yt-dlp: {e}"))
}

/// Detecta, apenas pela URL, se ha uma playlist DE VERDADE (nao Mix/Radio).
/// Playlists reais: list=PL..., OL..., FL..., UU..., LL..., RDCLAK (album mix e
/// tratado como real? nao — RD* sao radios). Auto-geradas a ignorar: RD*, start_radio.
fn url_has_real_playlist(url: &str) -> bool {
    // Extrai o valor do parametro list= da query string.
    let list_id = url
        .split(['?', '&'])
        .find_map(|kv| kv.strip_prefix("list="))
        .map(|v| v.split('&').next().unwrap_or(v));

    let Some(id) = list_id else {
        return false;
    };

    // Mix/Radio (auto-geradas, "infinitas") comecam com RD. Ignoramos.
    if id.starts_with("RD") {
        return false;
    }
    // start_radio=1 tambem indica radio.
    if url.contains("start_radio=1") {
        return false;
    }

    // Demais prefixos de playlist real do YouTube.
    id.starts_with("PL")
        || id.starts_with("OL")
        || id.starts_with("FL")
        || id.starts_with("UU")
        || id.starts_with("LL")
}

fn parse_probe(json: &serde_json::Value) -> ProbeResult {
    let is_playlist = json
        .get("_type")
        .and_then(|v| v.as_str())
        .map(|t| t == "playlist" || t == "multi_video")
        .unwrap_or_else(|| json.get("entries").is_some());

    let title = json
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Sem titulo")
        .to_string();

    let thumbnail = json
        .get("thumbnail")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if is_playlist {
        let playlist_count = json
            .get("entries")
            .and_then(|v| v.as_array())
            .map(|a| a.len() as u32)
            .or_else(|| {
                json.get("playlist_count")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as u32)
            })
            .unwrap_or(0);

        return ProbeResult {
            is_playlist: true,
            playlist_count,
            title,
            duration: 0.0,
            thumbnail,
            // Qualidades nao sao resolvidas para playlist (cada item difere);
            // o frontend usa um fallback fixo.
            video_qualities: Vec::new(),
        };
    }

    let duration = json
        .get("duration")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let video_qualities = extract_qualities(json);

    ProbeResult {
        is_playlist: false,
        playlist_count: 1,
        title,
        duration,
        thumbnail,
        video_qualities,
    }
}

/// Extrai as alturas de video disponiveis do array `formats`, ordenadas
/// de forma decrescente e sem duplicatas.
fn extract_qualities(json: &serde_json::Value) -> Vec<u32> {
    let mut heights: Vec<u32> = json
        .get("formats")
        .and_then(|v| v.as_array())
        .map(|formats| {
            formats
                .iter()
                .filter(|f| {
                    // Apenas formatos com trilha de video.
                    f.get("vcodec")
                        .and_then(|v| v.as_str())
                        .map(|c| c != "none")
                        .unwrap_or(false)
                })
                .filter_map(|f| f.get("height").and_then(|h| h.as_u64()).map(|h| h as u32))
                .filter(|h| *h > 0)
                .collect()
        })
        .unwrap_or_default();

    heights.sort_unstable();
    heights.dedup();
    heights.reverse();
    heights
}

// ---------------------------------------------------------------------------
// start_download — monta os argumentos e transmite o progresso
// ---------------------------------------------------------------------------

#[tauri::command]
async fn start_download(
    app: AppHandle,
    state: State<'_, DownloadState>,
    options: DownloadOptions,
) -> Result<(), String> {
    let args = build_args(&app, &options)?;

    let (mut rx, child) = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Falha ao localizar o yt-dlp: {e}"))?
        .args(args)
        .spawn()
        .map_err(|e| format!("Falha ao iniciar o download: {e}"))?;

    // Reseta o flag de cancelamento e guarda o processo para permitir cancelar.
    *state.canceled.lock().unwrap() = false;
    *state.child.lock().unwrap() = Some(child);

    let output_dir = options.output_dir.clone();
    let mut last_error = String::new();
    // Rastreia as multiplas fases de um download (video, audio, conversao) e
    // remapeia o progresso por-faixa do yt-dlp num progresso geral monotonico.
    let mut progress = DownloadProgress::default();

    tauri::async_runtime::spawn(async move {
        let state = app.state::<DownloadState>();
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    let line = String::from_utf8_lossy(&bytes);
                    for raw in line.split('\n') {
                        let trimmed = raw.trim();
                        if let Some(payload) = progress.observe(trimmed) {
                            let _ = app.emit("download://progress", payload);
                        }
                    }
                }
                CommandEvent::Stderr(bytes) => {
                    let line = String::from_utf8_lossy(&bytes);
                    let trimmed = line.trim();
                    // yt-dlp escreve erros (ERROR:) no stderr.
                    if trimmed.contains("ERROR") {
                        last_error = trimmed.to_string();
                    }
                }
                CommandEvent::Terminated(payload) => {
                    // O processo terminou: limpa o estado.
                    *state.child.lock().unwrap() = None;
                    let was_canceled = *state.canceled.lock().unwrap();

                    if was_canceled {
                        // Cancelamento pedido pelo usuario: nao emite erro.
                        let _ = app.emit("download://canceled", ());
                    } else if payload.code == Some(0) {
                        let _ = app.emit(
                            "download://done",
                            DonePayload {
                                output_dir: output_dir.clone(),
                            },
                        );
                    } else {
                        let message = if last_error.is_empty() {
                            format!("O download falhou (codigo {:?}).", payload.code)
                        } else {
                            friendly_error(&last_error)
                        };
                        let _ = app.emit("download://error", ErrorPayload { message });
                    }
                }
                _ => {}
            }
        }
    });

    Ok(())
}

/// Cancela o download em andamento, matando o processo do yt-dlp.
#[tauri::command]
fn cancel_download(state: State<'_, DownloadState>) -> Result<(), String> {
    *state.canceled.lock().unwrap() = true;
    if let Some(child) = state.child.lock().unwrap().take() {
        child
            .kill()
            .map_err(|e| format!("Falha ao cancelar: {e}"))?;
    }
    Ok(())
}

/// Monta a lista de argumentos do yt-dlp conforme as escolhas do usuario.
fn build_args(app: &AppHandle, opts: &DownloadOptions) -> Result<Vec<String>, String> {
    let ffmpeg = ffmpeg_path(app)?;
    let mut args: Vec<String> = Vec::new();

    // Saida: pasta escolhida + nome a partir do titulo.
    args.push("-P".into());
    args.push(opts.output_dir.clone());
    args.push("-o".into());
    args.push("%(title)s.%(ext)s".into());

    // Localizacao do ffmpeg sidecar (merge / remux / extracao de audio).
    args.push("--ffmpeg-location".into());
    args.push(ffmpeg);

    // Progresso parseavel + sem barra de progresso nativa colorida.
    args.push("--newline".into());
    args.push("--no-color".into());
    args.push("--progress".into());
    args.push("--progress-template".into());
    args.push(PROGRESS_TEMPLATE.into());

    // Acelera o download baixando fragmentos em paralelo.
    args.push("-N".into());
    args.push("4".into());

    // Playlist vs item unico.
    if opts.is_playlist {
        args.push("--yes-playlist".into());
    } else {
        args.push("--no-playlist".into());
    }

    match opts.kind.as_str() {
        "audio" => {
            // Sempre .mp3, melhor qualidade.
            args.push("-x".into());
            args.push("--audio-format".into());
            args.push("mp3".into());
            args.push("--audio-quality".into());
            args.push("0".into());
        }
        _ => {
            // Sempre .mp4. Prefere faixas que JA sao mp4/m4a para evitar
            // transcodificacao no merge (muito mais rapido). So cai para
            // outros codecs se nao houver mp4 na qualidade pedida.
            let height = opts.quality.unwrap_or(1080);
            args.push("-f".into());
            args.push(format!(
                "bv*[height<={h}][ext=mp4]+ba[ext=m4a]/bv*[height<={h}]+ba/b[height<={h}]/b",
                h = height
            ));
            args.push("--merge-output-format".into());
            args.push("mp4".into());
        }
    }

    // Corte: apenas para video unico e quando o intervalo nao cobre tudo.
    if !opts.is_playlist {
        if let (Some(start), Some(end)) = (opts.start, opts.end) {
            if end > start {
                args.push("--download-sections".into());
                args.push(format!("*{:.3}-{:.3}", start, end));
                args.push("--force-keyframes-at-cuts".into());
            }
        }
    }

    args.push(opts.url.clone());
    Ok(args)
}

/// Resolve o caminho absoluto do ffmpeg sidecar empacotado, ao lado do
/// executavel principal.
///
/// Em DEV o Tauri mantem o sufixo de target triple no nome do arquivo
/// (ex: `ffmpeg-x86_64-pc-windows-msvc.exe`); em producao o sufixo e
/// removido (`ffmpeg.exe`). Tentamos os dois.
fn ffmpeg_path(_app: &AppHandle) -> Result<String, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("Nao foi possivel localizar o executavel: {e}"))?;
    let dir = exe
        .parent()
        .ok_or("Pasta do executavel desconhecida")?;

    let triple = env!("YTDLP_TARGET_TRIPLE");
    #[cfg(windows)]
    let ext = ".exe";
    #[cfg(not(windows))]
    let ext = "";

    let candidates = [
        format!("ffmpeg-{triple}{ext}"), // dev
        format!("ffmpeg{ext}"),          // producao
    ];

    for candidate in candidates.iter() {
        let path = dir.join(candidate);
        if path.exists() {
            return Ok(path.to_string_lossy().into_owned());
        }
    }

    // Ultimo recurso: deixa o yt-dlp procurar o ffmpeg no PATH.
    Ok("ffmpeg".to_string())
}

/// Maquina de estados que remapeia o progresso do yt-dlp.
///
/// Um download de video sai do yt-dlp como 2 downloads separados (a faixa de
/// video e a de audio), cada um indo de 0% a 100%, seguidos de um merge. Audio
/// (mp3) e 1 download seguido de uma extracao. Se repassassemos o `_percent_str`
/// cru, a barra "voltaria para o inicio" entre as faixas. Aqui mapeamos cada
/// faixa numa fatia do total (com 2 faixas: video 0-50%, audio 50-100%) e
/// garantimos que o valor nunca diminua (o download paralelo `-N` reporta
/// fragmentos fora de ordem, gerando saltos para tras).
#[derive(Default)]
struct DownloadProgress {
    /// Quantas faixas (formatos) o download tem. 0 = ainda nao sabido.
    total_streams: u32,
    /// Quantas faixas ja comecaram a baixar (via "[download] Destination:").
    streams_started: u32,
    /// True quando a etapa de pos-processamento (merge/extracao) comecou.
    converting: bool,
    /// Maior percentual geral ja reportado, para manter a barra monotonica.
    overall_max: f64,
    /// Ultimo indice/total de playlist visto, para rotular a fase de conversao.
    last_item: Option<u32>,
    last_item_total: Option<u32>,
}

impl DownloadProgress {
    /// Processa uma linha de stdout do yt-dlp. Retorna um payload de progresso
    /// quando a linha gera uma atualizacao para o frontend.
    fn observe(&mut self, line: &str) -> Option<ProgressPayload> {
        // 1) Numero de faixas: "[info] ...: Downloading 2 format(s): 398+140".
        //    Contamos os formatos pelo '+' na parte apos "format(s):". Esta
        //    linha tambem marca o inicio de cada item de uma playlist, entao
        //    reiniciamos o tracking da faixa aqui (a barra representa o item
        //    atual; o "Video X de Y" no rotulo cuida do total).
        if line.starts_with("[info]") && line.contains("Downloading") && line.contains("format(s):")
        {
            if let Some(spec) = line.rsplit("format(s):").next() {
                let count = spec.split('+').filter(|s| !s.trim().is_empty()).count() as u32;
                self.total_streams = count.max(1);
            }
            self.streams_started = 0;
            self.converting = false;
            self.overall_max = 0.0;
            return None;
        }

        // 2) Inicio de uma nova faixa de download.
        if line.starts_with("[download] Destination:") {
            self.streams_started = self.streams_started.saturating_add(1);
            return None;
        }

        // 3) Pos-processamento: merge (video) ou extracao de audio (mp3).
        if line.starts_with("[Merger]") || line.starts_with("[ExtractAudio]") {
            if !self.converting {
                self.converting = true;
                self.overall_max = 100.0;
                return Some(ProgressPayload {
                    percent: 100.0,
                    speed: String::new(),
                    eta: String::new(),
                    phase: "converting".into(),
                    item_index: self.last_item,
                    item_total: self.last_item_total,
                });
            }
            return None;
        }

        // 4) Linha de progresso propriamente dita.
        let rest = line.strip_prefix(PROGRESS_PREFIX)?;
        self.parse_progress(rest)
    }

    /// Parse de uma linha de progresso (sem o prefixo) e remapeamento para o
    /// progresso geral. Formato: percent|speed|eta|playlist_index|n_entries
    fn parse_progress(&mut self, rest: &str) -> Option<ProgressPayload> {
        let parts: Vec<&str> = rest.split('|').collect();
        if parts.is_empty() {
            return None;
        }

        let stream_percent = parts.first().map(|s| parse_percent(s)).unwrap_or(0.0);
        let speed = clean_na(parts.get(1).copied().unwrap_or(""));
        let eta = clean_na(parts.get(2).copied().unwrap_or(""));
        let item_index = parts.get(3).and_then(|s| parse_uint(s));
        let item_total = parts.get(4).and_then(|s| parse_uint(s));
        self.last_item = item_index;
        self.last_item_total = item_total;

        // Se a primeira faixa ainda nao foi contada (a linha "Destination:"
        // pode nao ter aparecido em alguns formatos), assume que estamos nela.
        if self.streams_started == 0 {
            self.streams_started = 1;
        }

        // Remapeia o percentual da faixa atual para a fatia dela no total.
        // streams_started e 1-based; a faixa atual e a de indice (started - 1).
        let total = self.total_streams.max(1);
        let current = self.streams_started.saturating_sub(1).min(total - 1);
        let slice = 100.0 / total as f64;
        let overall = current as f64 * slice + stream_percent.clamp(0.0, 100.0) / 100.0 * slice;

        // Mantem monotonico: ignora regressoes do download paralelo.
        if overall > self.overall_max {
            self.overall_max = overall;
        }

        Some(ProgressPayload {
            percent: self.overall_max,
            speed,
            eta,
            phase: "downloading".into(),
            item_index,
            item_total,
        })
    }
}

fn parse_percent(s: &str) -> f64 {
    s.trim()
        .trim_end_matches('%')
        .trim()
        .parse::<f64>()
        .unwrap_or(0.0)
}

fn parse_uint(s: &str) -> Option<u32> {
    let t = s.trim();
    if t.is_empty() || t == "NA" || t == "N/A" {
        None
    } else {
        t.parse::<u32>().ok()
    }
}

fn clean_na(s: &str) -> String {
    let t = s.trim();
    if t == "NA" || t == "N/A" {
        String::new()
    } else {
        t.to_string()
    }
}

/// Traduz mensagens de erro comuns do yt-dlp em algo legivel.
fn friendly_error(stderr: &str) -> String {
    let lower = stderr.to_lowercase();
    if lower.contains("is not a valid url") || lower.contains("unsupported url") {
        "URL invalida ou nao suportada.".into()
    } else if lower.contains("video unavailable") || lower.contains("private video") {
        "Video indisponivel ou privado.".into()
    } else if lower.contains("unable to download") || lower.contains("network") {
        "Erro de rede ao acessar o video. Verifique sua conexao.".into()
    } else if lower.contains("sign in") || lower.contains("age") {
        "Este video exige login ou tem restricao de idade.".into()
    } else {
        // Pega a primeira linha de ERROR para nao despejar o stderr inteiro.
        stderr
            .lines()
            .find(|l| l.contains("ERROR"))
            .map(|l| l.trim().to_string())
            .unwrap_or_else(|| "Nao foi possivel processar a URL.".into())
    }
}

// ---------------------------------------------------------------------------
// run
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(DownloadState::default())
        .invoke_handler(tauri::generate_handler![
            probe_url,
            start_download,
            cancel_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ---------------------------------------------------------------------------
// Testes do remapeamento de progresso
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Roda uma sequencia de linhas de stdout e devolve os percentuais/fases
    /// emitidos ao frontend.
    fn run(lines: &[&str]) -> Vec<(f64, String)> {
        let mut p = DownloadProgress::default();
        lines
            .iter()
            .filter_map(|l| p.observe(l))
            .map(|payload| (payload.percent, payload.phase))
            .collect()
    }

    #[test]
    fn video_two_streams_never_rewinds_and_maps_to_halves() {
        // Saida real (resumida) de um download de video: 2 faixas + merge.
        let out = run(&[
            "[info] uedJx6hkKS0: Downloading 1 format(s): 398+140",
            "[download] Destination: ...f398.mp4",
            "YTDLPROG| 28.1%|  19.23MiB/s|00:00|NA|NA",
            "YTDLPROG| 56.2%|  17.16MiB/s|00:00|NA|NA",
            "YTDLPROG|100.0%|  14.14MiB/s|00:00|NA|NA",
            "[download] Destination: ...f140.m4a",
            "YTDLPROG|  0.0%| 186.44KiB/s|00:13|NA|NA",
            "YTDLPROG| 39.6%|  15.02MiB/s|00:00|NA|NA",
            "YTDLPROG|100.0%|  12.95MiB/s|00:00|NA|NA",
            "[Merger] Merging formats into \"...mp4\"",
        ]);

        let percents: Vec<f64> = out.iter().map(|(p, _)| *p).collect();
        // Faixa de video (50% do total): 28% -> 14, 56% -> 28, 100% -> 50.
        // Faixa de audio (50% restante): 0% -> 50, 40% -> ~70, 100% -> 100.
        assert!((percents[0] - 14.05).abs() < 0.5, "{:?}", percents);
        assert!((percents[2] - 50.0).abs() < 0.01, "{:?}", percents);
        assert!((percents[3] - 50.0).abs() < 0.01, "audio comeca em 50% {:?}", percents);
        assert!((percents[5] - 100.0).abs() < 0.01, "{:?}", percents);

        // Monotonico: nunca diminui ao longo de todo o download.
        for w in percents.windows(2) {
            assert!(w[1] >= w[0], "regrediu: {:?}", percents);
        }
        // Termina na fase de conversao em 100%.
        assert_eq!(out.last().unwrap().1, "converting");
    }

    #[test]
    fn parallel_fragment_jitter_is_clamped_monotonic() {
        // O download paralelo (-N 4) reporta percentuais fora de ordem.
        let out = run(&[
            "[info] x: Downloading 1 format(s): 398+140",
            "[download] Destination: ...f398.mp4",
            "YTDLPROG| 69.3%|  16.64MiB/s|00:00|NA|NA",
            "YTDLPROG| 69.3%| Unknown B/s|Unknown|NA|NA",
            "YTDLPROG| 70.2%|  46.50MiB/s|00:00|NA|NA",
            "YTDLPROG| 71.1%|   4.39MiB/s|00:00|NA|NA",
        ]);
        let percents: Vec<f64> = out.iter().map(|(p, _)| *p).collect();
        for w in percents.windows(2) {
            assert!(w[1] >= w[0], "regrediu: {:?}", percents);
        }
    }

    #[test]
    fn audio_single_stream_uses_full_bar() {
        let out = run(&[
            "[info] x: Downloading 1 format(s): 251",
            "[download] Destination: ...webm",
            "YTDLPROG| 51.6%|   8.47MiB/s|00:00|NA|NA",
            "YTDLPROG|100.0%|  11.62MiB/s|00:00|NA|NA",
            "[ExtractAudio] Destination: ...mp3",
        ]);
        // Uma so faixa => o percentual da faixa e o percentual geral.
        assert!((out[0].0 - 51.6).abs() < 0.01, "{:?}", out);
        assert!((out[1].0 - 100.0).abs() < 0.01, "{:?}", out);
        assert_eq!(out.last().unwrap().1, "converting");
    }

    #[test]
    fn playlist_resets_per_item() {
        let out = run(&[
            "[info] a: Downloading 1 format(s): 398+140",
            "[download] Destination: ...a.f398.mp4",
            "YTDLPROG|100.0%|  9MiB/s|00:00|1|3",
            "[download] Destination: ...a.f140.m4a",
            "YTDLPROG|100.0%|  9MiB/s|00:00|1|3",
            "[Merger] Merging into a.mp4",
            // Item 2: novo [info] reinicia o tracking; a barra volta a subir.
            "[info] b: Downloading 1 format(s): 398+140",
            "[download] Destination: ...b.f398.mp4",
            "YTDLPROG| 10.0%|  9MiB/s|00:00|2|3",
        ]);
        // O ultimo evento e o item 2 com a barra de volta a um valor baixo.
        let (last_pct, _) = out.last().unwrap();
        assert!(*last_pct < 20.0, "item 2 deveria reiniciar baixo: {:?}", out);
    }
}
