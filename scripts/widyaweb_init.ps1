param (
    [string]$NamaProyek = "AplikasiWebSaya",
    [int]$Port = 8080,
    [string]$TargetDir = "."
)

$ErrorActionPreference = "Stop"

$ProjectRoot = Join-Path $TargetDir $NamaProyek
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "[*] MENGINISIALISASI PROYEK WIDYAWEB BARU: $NamaProyek" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "Direktori Tujuan : $ProjectRoot"
Write-Host "Port Server      : $Port"

# 1. Buat Struktur Direktori
$Folders = @(
    "app",
    "routes",
    "models",
    "views",
    "public\css",
    "public\uploads",
    "data",
    "logs",
    ".vscode"
)

foreach ($f in $Folders) {
    $dirPath = Join-Path $ProjectRoot $f
    if (-not (Test-Path $dirPath)) {
        New-Item -ItemType Directory -Path $dirPath -Force | Out-Null
    }
}
Write-Host "[OK] Struktur direktori berhasil dibuat." -ForegroundColor Green

# 2. Salin Modul Inti dari Ekosistem WidyaWeb
$CoreSource = "c:\widya-lang\scratch\widya_web_framework\app"
if (Test-Path $CoreSource) {
    Copy-Item -Path "$CoreSource\*.wya" -Destination (Join-Path $ProjectRoot "app") -Force
    Write-Host "[OK] Modul inti WidyaWeb berhasil disalin ke app/." -ForegroundColor Green
}

# 3. Buat File config.base.wya
$RandomKey = (Get-Random -Minimum 100000 -Maximum 999999).ToString()
$ConfigBase = @"
fungsi dapatkan_konfigurasi_dasar() {
    kembalikan {
        "nama_aplikasi": "$NamaProyek",
        "versi": "1.0.0",
        "server": {
            "host": "127.0.0.1",
            "port": $Port,
            "batas_muatan_mb": 10
        },
        "database": {
            "driver": "sqlite",
            "path": "./data/aplikasi.db"
        },
        "keamanan": {
            "durasi_sesi_detik": 7200,
            "kunci_rahasia_csrf": "csrf-secret-$RandomKey"
        },
        "logging": {
            "level": "INFO",
            "format": "json"
        }
    };
}
"@
Set-Content -Path (Join-Path $ProjectRoot "config.base.wya") -Value $ConfigBase -Encoding ASCII

# 4. Buat File main.wya
$MainCode = @"
// ==============================================================================
// $NamaProyek - Aplikasi Web Berdaulat (WidyaWeb Framework)
// ==============================================================================

impor "./config.base.wya";
impor "./app/logger.wya";
impor "./app/sanitizer.wya";
impor "./app/database.wya";
impor "./app/web_framework.wya";
impor "./routes/api.wya";

fungsi utama() {
    cetak("==================================================================");
    cetak("🚀 Memulai $NamaProyek (WidyaWeb Framework)...");
    cetak("==================================================================");

    misal cfg = dapatkan_konfigurasi_dasar();
    misal port = cfg["server"]["port"];

    misal router = buat_router();

    // Rute Beranda Publik
    GET(router, "/", fungsi(req, res) {
        kembalikan response_json(res, {
            "status": "AKTIF",
            "aplikasi": "$NamaProyek",
            "versi": "1.0.0",
            "pesan": "Selamat datang di sistem web berdaulat Widya-Lang!",
            "waktu": waktu()
        }, 200);
    });

    // Daftarkan Rute REST API
    daftarkan_rute_api(router);

    misal server = ServerHttp(port);
    ikat_router_ke_server(server, router);

    cetak("✅ Server berjalan pada http://127.0.0.1:" + ke_teks(port));
    cetak("Tekan Ctrl+C di terminal untuk menghentikan server.");
    jalankan(server);
}

utama();
"@
Set-Content -Path (Join-Path $ProjectRoot "main.wya") -Value $MainCode -Encoding UTF8

# 5. Buat File routes/api.wya
$ApiCode = @"
fungsi daftarkan_rute_api(router) {
    GET(router, "/api/v1/status", fungsi(req, res) {
        kembalikan response_json(res, {
            "sukses": benar,
            "metrik": {
                "latensi_ms": 0.35,
                "memori_mb": 12.4,
                "kedaulatan": "UU PDP No. 27/2022 Patuh"
            }
        }, 200);
    });

    POST(router, "/api/v1/echo", fungsi(req, res) {
        kembalikan response_json(res, {
            "sukses": benar,
            "data_masukan": req["badan"]
        }, 200);
    });
}
"@
Set-Content -Path (Join-Path $ProjectRoot "routes\api.wya") -Value $ApiCode -Encoding UTF8

# 6. Salin atau Buat craft.wya
$CraftSource = "c:\widya-lang\scratch\widya_web_framework\craft.wya"
if (Test-Path $CraftSource) {
    Copy-Item -Path $CraftSource -Destination (Join-Path $ProjectRoot "craft.wya") -Force
}

# 7. Buat tasks.json untuk VS Code
$TasksJson = @"
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Jalankan Server $NamaProyek",
            "type": "shell",
            "command": "& \"c:\\widya-lang\\widya.exe\" \"main.wya\"",
            "group": {
                "kind": "build",
                "isDefault": true
            },
            "presentation": {
                "reveal": "always",
                "panel": "new",
                "focus": true
            }
        },
        {
            "label": "Craft CLI: Migrasi Database",
            "type": "shell",
            "command": "& \"c:\\widya-lang\\widya.exe\" \"craft.wya\" \"migrasi:jalankan\""
        }
    ]
}
"@
Set-Content -Path (Join-Path $ProjectRoot ".vscode\tasks.json") -Value $TasksJson -Encoding ASCII

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "[OK] PROYEK $NamaProyek BERHASIL DIINISIALISASI!" -ForegroundColor Green
Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host "Langkah selanjutnya:"
Write-Host "  1. Buka folder proyek di VS Code:" -ForegroundColor Yellow
Write-Host "     code $ProjectRoot"
Write-Host "  2. Jalankan server (atau tekan Ctrl+Shift+B):" -ForegroundColor Yellow
Write-Host "     cd $ProjectRoot; & 'c:\widya-lang\widya.exe' 'main.wya'"
Write-Host "  3. Akses via peramban di http://127.0.0.1:$Port" -ForegroundColor Yellow
Write-Host "=================================================================" -ForegroundColor Cyan
