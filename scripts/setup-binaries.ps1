# Baixa os sidecars (yt-dlp.exe e ffmpeg.exe) para src-tauri/binaries/ com o
# sufixo de target triple que o Tauri exige. Rode uma vez apos clonar o repo:
#
#   pnpm run setup            (ou)   powershell -ExecutionPolicy Bypass -File scripts/setup-binaries.ps1
#
# Para atualizar o yt-dlp depois, rode de novo com -Force.

param(
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Pasta de destino: <repo>/src-tauri/binaries
$repoRoot = Split-Path -Parent $PSScriptRoot
$binDir = Join-Path $repoRoot "src-tauri\binaries"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# Descobre o target triple desta maquina (ex: x86_64-pc-windows-msvc).
try {
    $triple = (& rustc --print host-tuple).Trim()
} catch {
    Write-Error "rustc nao encontrado. Instale o Rust (https://rustup.rs) e tente de novo."
    exit 1
}
if ([string]::IsNullOrWhiteSpace($triple)) {
    # Fallback para a plataforma mais comum.
    $triple = "x86_64-pc-windows-msvc"
}
Write-Host "Target triple: $triple" -ForegroundColor Cyan

$ytdlpPath = Join-Path $binDir "yt-dlp-$triple.exe"
$ffmpegPath = Join-Path $binDir "ffmpeg-$triple.exe"

# ---------------------------------------------------------------------------
# yt-dlp — baixa o .exe mais recente direto das releases do GitHub.
# ---------------------------------------------------------------------------
if ((Test-Path $ytdlpPath) -and -not $Force) {
    Write-Host "yt-dlp ja existe (use -Force para atualizar): $ytdlpPath" -ForegroundColor DarkGray
} else {
    Write-Host "Baixando yt-dlp..." -ForegroundColor Yellow
    $ytdlpUrl = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
    Invoke-WebRequest -Uri $ytdlpUrl -OutFile $ytdlpPath
    Write-Host "  -> $ytdlpPath" -ForegroundColor Green
}

# ---------------------------------------------------------------------------
# ffmpeg — baixa o build essentials (gyan.dev) e extrai so o ffmpeg.exe.
# ---------------------------------------------------------------------------
if ((Test-Path $ffmpegPath) -and -not $Force) {
    Write-Host "ffmpeg ja existe (use -Force para atualizar): $ffmpegPath" -ForegroundColor DarkGray
} else {
    Write-Host "Baixando ffmpeg (build essentials, ~30 MB compactado)..." -ForegroundColor Yellow
    $ffmpegUrl = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
    $tmpZip = Join-Path $env:TEMP "ffmpeg-release-essentials.zip"
    $tmpExtract = Join-Path $env:TEMP "ffmpeg-extract"

    Invoke-WebRequest -Uri $ffmpegUrl -OutFile $tmpZip

    if (Test-Path $tmpExtract) { Remove-Item -Recurse -Force $tmpExtract }
    Expand-Archive -Path $tmpZip -DestinationPath $tmpExtract -Force

    # O zip extrai para uma pasta tipo ffmpeg-<versao>-essentials_build\bin\ffmpeg.exe
    $found = Get-ChildItem -Path $tmpExtract -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
    if ($null -eq $found) {
        Write-Error "ffmpeg.exe nao encontrado no zip baixado."
        exit 1
    }
    Copy-Item $found.FullName -Destination $ffmpegPath -Force
    Write-Host "  -> $ffmpegPath" -ForegroundColor Green

    Remove-Item -Force $tmpZip -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force $tmpExtract -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Host "Pronto! Sidecars em src-tauri/binaries/:" -ForegroundColor Cyan
Get-ChildItem $binDir -Filter "*.exe" | ForEach-Object {
    "  {0}  ({1:N1} MB)" -f $_.Name, ($_.Length / 1MB)
}
