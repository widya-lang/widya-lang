# =========================================================================
# Script Penyiapan Cloudflare Tunnel untuk Widya-Lang Server (Windows)
# Membuka akses https://api.widya-lang.org tanpa IP Publik / Port Forward
# =========================================================================

param (
    [string]$TunnelToken = ""
)

Write-Host "🚀 [Widya-Lang] Menyiapkan Cloudflare Tunnel di Windows..." -ForegroundColor Cyan

# 1. Unduh cloudflared jika belum ada
$CloudflaredPath = "$env:ProgramFiles\cloudflared\cloudflared.exe"
if (-not (Test-Path $CloudflaredPath)) {
    Write-Host "📦 Mengunduh cloudflared executable..." -ForegroundColor Yellow
    $DownloadUrl = "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe"
    $TargetDir = "$env:ProgramFiles\cloudflared"
    New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $CloudflaredPath
    Write-Host "✅ cloudflared berhasil diunduh ke $CloudflaredPath" -ForegroundColor Green
} else {
    Write-Host "✅ cloudflared sudah terpasang di sistem." -ForegroundColor Green
}

# 2. Pasang Service jika Token diberikan
if ($TunnelToken -ne "") {
    Write-Host "⚙️ Memasang layanan Windows Service untuk Cloudflare Tunnel..." -ForegroundColor Yellow
    & "$CloudflaredPath" service install $TunnelToken
    Start-Service "cloudflared"
    Write-Host "🎉 Layanan Cloudflare Tunnel aktif dan berjalan di latar belakang!" -ForegroundColor Green
} else {
    Write-Host "💡 Jalankan script ini dengan parameter Token Anda:" -ForegroundColor Cyan
    Write-Host "   .\setup_tunnel.ps1 -TunnelToken '<TOKEN_DARI_CLOUDFLARE_ZERO_TRUST>'" -ForegroundColor White
}
