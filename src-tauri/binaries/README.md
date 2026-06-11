# Binários sidecar (yt-dlp e ffmpeg)

Este app empacota o **yt-dlp** e o **ffmpeg** como *sidecars* do Tauri. Eles
precisam estar nesta pasta com o sufixo do *target triple* da máquina, senão o
build/dev falha.

> **Os binários NÃO ficam no Git** (são grandes, ~114 MB). Esta pasta é
> versionada apenas com este README. Baixe os binários com o script abaixo.

## Forma recomendada: script automático

Da raiz do projeto:

```powershell
pnpm run setup
```

O script [scripts/setup-binaries.ps1](../../scripts/setup-binaries.ps1) detecta o
target triple, baixa o yt-dlp mais recente e um build estático do ffmpeg, e os
coloca aqui com o nome certo. Para atualizar depois: `pnpm run setup` com `-Force`
(ou `powershell -File scripts/setup-binaries.ps1 -Force`).

## Arquivos esperados (Windows x64)

```
yt-dlp-x86_64-pc-windows-msvc.exe
ffmpeg-x86_64-pc-windows-msvc.exe
```

> O sufixo é o `target triple`. Descubra o seu com:
> ```powershell
> rustc --print host-tuple
> ```
> Em outra plataforma, renomeie os binários com o sufixo correto
> (ex: `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`).

## Forma manual (se preferir baixar à mão)

- **yt-dlp:** https://github.com/yt-dlp/yt-dlp/releases (baixe `yt-dlp.exe`).
- **ffmpeg:** build estático para Windows, ex: https://www.gyan.dev/ffmpeg/builds/
  (use o `ffmpeg.exe` de dentro do pacote).

Depois renomeie com o sufixo do triple (PowerShell):

```powershell
$triple = (rustc --print host-tuple)
Copy-Item caminho\para\yt-dlp.exe ".\yt-dlp-$triple.exe"
Copy-Item caminho\para\ffmpeg.exe ".\ffmpeg-$triple.exe"
```
