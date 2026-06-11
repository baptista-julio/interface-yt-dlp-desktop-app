# yt-dlp interface

Interface gráfica desktop, passo a passo, para o [yt-dlp](https://github.com/yt-dlp/yt-dlp).
Construída com **Tauri 2 + Vue 3 + TypeScript**.

## O que faz

Um fluxo guiado, poucas opções por vez:

1. **URL** — cole o link de um vídeo ou de uma playlist do YouTube.
2. **Tipo** — Vídeo (`.mp4`) ou somente Áudio (`.mp3`).
3. **Qualidade** — (só vídeo) escolha a resolução; baixa a melhor faixa até esse limite.
4. **Corte** — (só vídeo único) timeline para baixar apenas um trecho; por padrão baixa tudo.
5. **Pasta** — escolha onde salvar.
6. **Download** — barra de progresso com %, velocidade, ETA e (em playlists) "item X de N".
7. **Concluído** — abra a pasta ou baixe outro.

Restrições: vídeos sempre em **.mp4**, áudios sempre em **.mp3** (garantido via ffmpeg).

## Pré-requisitos

O `yt-dlp` e o `ffmpeg` são empacotados como **sidecars** (o usuário final não instala nada).
Eles **não ficam no Git** (são grandes); baixe-os para [`src-tauri/binaries/`](src-tauri/binaries/)
com o script de setup — veja [src-tauri/binaries/README.md](src-tauri/binaries/README.md).

## Rodar em desenvolvimento

```powershell
pnpm install
pnpm run setup     # baixa yt-dlp e ffmpeg para src-tauri/binaries (1ª vez)
pnpm tauri dev
```

## Build de produção

```powershell
pnpm tauri build
```

Gera o instalador com os binários embutidos.

## Arquitetura

- **Frontend** (`src/`): máquina de estados linear em [App.vue](src/App.vue); telas em
  [src/components/](src/components/); chamadas ao backend em [src/lib/ytdlp.ts](src/lib/ytdlp.ts).
  Visual segue [docs/DESIGN_SYSTEM.md](docs/DESIGN_SYSTEM.md) (tokens em
  [src/styles/globals.css](src/styles/globals.css)).
- **Backend** (`src-tauri/src/lib.rs`): comandos `probe_url` (metadados via `yt-dlp -J`) e
  `start_download` (monta os argumentos e transmite progresso via eventos Tauri).
