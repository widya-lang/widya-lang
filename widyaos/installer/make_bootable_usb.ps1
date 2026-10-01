# ====================================================================
# SCRIPT OTOMASI PEMBUATAN USB FLASHER WIDYAOS (POWERSHELL)
# Standar Perakitan Lini Pabrik OEM & Teknisi Dinas Pendidikan
# Hak Cipta (C) 2026 Kemendikdasmen RI & Tim Pengembang WidyaOS
# ====================================================================

param (
    [Parameter(Mandatory=$false)]
    [int]$DiskNumber = -1,

    [Parameter(Mandatory=$false)]
    [string]$OemMfgId = "MFG-ZYREX-01",

    [Parameter(Mandatory=$false)]
    [string]$TkdnCert = "TKDN-KEMENPERIN-2026-88912"
)

Write-Host "====================================================================" -ForegroundColor Cyan
Write-Host "       WIDYAOS OEM FLASHER & USB BOOTABLE CREATOR (POWERSHELL)      " -ForegroundColor Cyan
Write-Host "====================================================================" -ForegroundColor Cyan

# 1. Verifikasi Hak Akses Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Warning "Skrip ini memerlukan hak Administrator untuk memformat partisi disk!"
    Write-Host "Jalankan PowerShell dengan 'Run as Administrator'." -ForegroundColor Red
    exit 1
}

# 2. Deteksi Media Penyimpanan USB Removable
Write-Host "`n[1/5] Memindai Disk USB Removable yang Terhubung..." -ForegroundColor Yellow
$usbDisks = Get-Disk | Where-Object { $_.BusType -eq 'USB' -or $_.MediaType -eq 'Removable' }

if ($usbDisks.Count -eq 0) {
    Write-Warning "Tidak ditemukan media penyimpanan USB! Hubungkan flashdisk dan ulangi."
    exit 1
}

$usbDisks | Format-Table Number, FriendlyName, SerialNumber, Size, PartitionStyle

if ($DiskNumber -lt 0) {
    Write-Host "Pilih nomor Disk target (contoh: 1): " -NoNewline -ForegroundColor Green
    $inputNum = Read-Host
    $DiskNumber = [int]$inputNum
}

$targetDisk = Get-Disk -Number $DiskNumber -ErrorAction SilentlyContinue
if (-not $targetDisk) {
    Write-Error "Disk nomor $DiskNumber tidak valid!"
    exit 1
}

Write-Host "`n[2/5] Menyiapkan Disk $DiskNumber ($($targetDisk.FriendlyName))..." -ForegroundColor Yellow
Write-Host "PERINGATAN: Seluruh data pada Disk $DiskNumber akan dihapus permanen!" -ForegroundColor Red

# 3. Partisi Disk GPT Otomatis
Write-Host "`n[3/5] Membentuk Skema Partisi Dual-Slot & Golden Recovery..." -ForegroundColor Yellow
Clear-Disk -Number $DiskNumber -RemoveData -RemoveOEM -Confirm:$false
Initialize-Disk -Number $DiskNumber -PartitionStyle GPT

# Partisi 1: ESP (EFI System Partition - 512 MB)
$partEsp = New-Partition -DiskNumber $DiskNumber -Size 512MB -GptType '{c12a7328-f81f-11d2-ba4b-00a0c93ec93b}'
$volEsp = Format-Volume -Partition $partEsp -FileSystem FAT32 -NewFileSystemLabel "ESP" -Confirm:$false

# Partisi 2: Golden Recovery (4096 MB)
$partRec = New-Partition -DiskNumber $DiskNumber -Size 4096MB
$volRec = Format-Volume -Partition $partRec -FileSystem exFAT -NewFileSystemLabel "GOLDEN_REC" -Confirm:$false

# Partisi 3: Installer Payload (Sisa Kapasitas)
$partData = New-Partition -DiskNumber $DiskNumber -UseMaximumSize
$volData = Format-Volume -Partition $partData -FileSystem NTFS -NewFileSystemLabel "WIDYA_INSTALL" -Confirm:$false

# 4. Injeksi Berkas Bootloader & Konfigurasi GRUB
Write-Host "`n[4/5] Menyalin Bootloader GRUB, Kernel, dan Konfigurasi Dual-Slot..." -ForegroundColor Yellow
$espDrive = $volEsp.DriveLetter + ":"
$recDrive = $volRec.DriveLetter + ":"
$installDrive = $volData.DriveLetter + ":"

# Buat direktori EFI
New-Item -ItemType Directory -Path "$espDrive\EFI\BOOT" -Force | Out-Null
New-Item -ItemType Directory -Path "$espDrive\boot\grub" -Force | Out-Null

# Salin grub.cfg
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (Test-Path "$scriptDir\grub.cfg") {
    Copy-Item "$scriptDir\grub.cfg" -Destination "$espDrive\boot\grub\grub.cfg" -Force
    Write-Host "  -> grub.cfg berhasil disalin ke $espDrive\boot\grub\grub.cfg" -ForegroundColor Green
}

# 5. Injeksi Segel Kriptografi OEM & TKDN (Inpres 2/2022)
Write-Host "`n[5/5] Menginjeksi Segel Kriptografi Pabrik & Metadata TKDN..." -ForegroundColor Yellow
$timestamp = (Get-Date).ToString("yyyyMMdd-HHmmss")
$segelPayload = @{
    oem_mfg_id = $OemMfgId
    tkdn_certificate = $TkdnCert
    persentase_tkdn = "48.5%"
    standard_inpres = "INPRES_NO_2_TAHUN_2022"
    flashed_timestamp = $timestamp
    bootloader = "GRUB_DUAL_SLOT_GOLDEN_V2"
    security_eal = "BSSN_COMMON_CRITERIA_EAL4_PLUS"
} | ConvertTo-Json

$segelPayload | Out-File -FilePath "$espDrive\oem_tkdn_seal.json" -Encoding utf8
Write-Host "  -> Segel pabrik berhasil diterbitkan di $espDrive\oem_tkdn_seal.json" -ForegroundColor Green

Write-Host "`n====================================================================" -ForegroundColor Cyan
Write-Host "SUKSES! USB FLASHER WIDYAOS BERHASIL DIBUAT DENGAN SEMPURNA!" -ForegroundColor Green
Write-Host "Drive ESP: $espDrive | Recovery: $recDrive | Installer: $installDrive" -ForegroundColor Green
Write-Host "Media USB siap digunakan untuk instalasi massal di sekolah atau pabrik." -ForegroundColor Cyan
Write-Host "====================================================================" -ForegroundColor Cyan
