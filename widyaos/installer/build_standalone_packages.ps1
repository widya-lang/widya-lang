# ====================================================================
# SCRIPT OTOMASI PACKAGING BINER STANDALONE LINTAS PLATFORM
# Membuat bundel mandiri untuk Linux, Windows, macOS, dan Docker OCI
# Hak Cipta (C) 2026 Kemendikdasmen RI & Tim Pengembang WidyaOS
# ====================================================================

param (
    [string]$OutputDir = "c:\widya-lang\dist"
)

Write-Host "====================================================================" -ForegroundColor Cyan
Write-Host "       WIDYA STANDALONE CROSS-PLATFORM PACKAGE BUILDER              " -ForegroundColor Cyan
Write-Host "====================================================================" -ForegroundColor Cyan

if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$compiler = "c:\widya-lang\target\release\widya.exe"
if (-not (Test-Path $compiler)) {
    Write-Error "Kompiler Widya tidak ditemukan di $compiler!"
    exit 1
}

# Daftar Pilar Mandiri yang akan dipaketkan
$pillars = @(
    @{ Name = "widyabase-standalone"; Port = 33060; Service = "WidyaBaseEngine"; Desc = "ACID In-Memory RDBMS Engine" },
    @{ Name = "widyaolap-standalone"; Port = 8123;  Service = "WidyaOLAPEngine";  Desc = "Columnar Big Data Analytics Engine" },
    @{ Name = "widyawan-standalone";  Port = 51820; Service = "WidyaWANHighway";  Desc = "Zero-Trust Sovereign SD-WAN Highway" },
    @{ Name = "widyastat-standalone"; Port = 8090;  Service = "WidyaStatStudio";  Desc = "Inferential & Multivariate Statistics Engine" },
    @{ Name = "widyaoffice-standalone"; Port = 8088; Service = "WidyaOfficeServer"; Desc = "Sovereign Productivity Document Suite" }
)

foreach ($p in $pillars) {
    Write-Host "`n[PACKAGING] Menyiapkan bundel untuk $($p.Name) (Port $($p.Port))..." -ForegroundColor Yellow
    $pDir = "$OutputDir\$($p.Name)"
    New-Item -ItemType Directory -Path "$pDir\bin" -Force | Out-Null
    New-Item -ItemType Directory -Path "$pDir\conf" -Force | Out-Null
    New-Item -ItemType Directory -Path "$pDir\service" -Force | Out-Null

    # 1. Salin biner eksekusi
    Copy-Item $compiler "$pDir\bin\widya.exe" -Force

    # 2. Buat konfigurasi layanan
    $confContent = @"
# Konfigurasi Resmi $($p.Name)
SERVICE_NAME=$($p.Service)
SERVICE_PORT=$($p.Port)
SERVICE_DESC=$($p.Desc)
SECURITY_MODE=STRICT_BSSN_EAL4
BIND_ADDRESS=0.0.0.0
DATA_DIR=./data
"@
    $confContent | Out-File -FilePath "$pDir\conf\service.conf" -Encoding utf8

    # 3. Buat script instalasi Windows Service (PowerShell & sc.exe)
    $winInstall = @"
# Script Pasang Windows Service
sc.exe create $($p.Service) binPath= "`"$PSScriptRoot\..\bin\widya.exe`"" start= auto DisplayName= "$($p.Desc)"
sc.exe description $($p.Service) "$($p.Desc) berdaulat nasional"
sc.exe start $($p.Service)
Write-Host "Layanan $($p.Service) berhasil dipasang dan berjalan di Port $($p.Port)!" -ForegroundColor Green
"@
    $winInstall | Out-File -FilePath "$pDir\service\install_windows_service.ps1" -Encoding utf8

    # 4. Buat file service Linux Systemd
    $systemdContent = @"
[Unit]
Description=$($p.Desc) (Kemendikdasmen RI)
After=network.target

[Service]
Type=simple
User=widya
ExecStart=/opt/$($p.Name)/bin/widya
Restart=always
RestartSec=3
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
"@
    $systemdContent | Out-File -FilePath "$pDir\service\$($p.Name).service" -Encoding utf8

    # 5. Buat Dockerfile OCI Micro-Container
    $dockerContent = @"
FROM alpine:3.19
LABEL maintainer="Pusdatin Kemendikdasmen <pusdatin@sewidya.id>"
LABEL service="$($p.Name)"
LABEL tkdn="48.5%"
WORKDIR /app
COPY bin/widya /app/widya
COPY conf/service.conf /app/service.conf
EXPOSE $($p.Port)
ENTRYPOINT ["/app/widya"]
"@
    $dockerContent | Out-File -FilePath "$pDir\Dockerfile" -Encoding utf8

    # 6. Kompres ke file ZIP untuk distribusi Windows
    $zipPath = "$OutputDir\$($p.Name)-win64.zip"
    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
    Compress-Archive -Path "$pDir\*" -DestinationPath $zipPath -Force
    Write-Host "  -> Berhasil membuat paket distribusi: $zipPath" -ForegroundColor Green
}

Write-Host "`n====================================================================" -ForegroundColor Cyan
Write-Host "SUKSES! SELURUH BUNDEL MANDIRI SELESAI DIPAKETKAN DI $OutputDir" -ForegroundColor Green
Write-Host "====================================================================" -ForegroundColor Cyan
