# ============================================================================
# WIDYA SOVEREIGN AI CORE — POWERSHELL SERVICE CONTROLLER
# Hardware Target: AMD Ryzen 7 8840HS | AMD Radeon 780M | AMD Ryzen AI XDNA
# Standar: OpenAI API v1, MCP JSON-RPC 2.0, Prometheus, Ed25519 W3C VC
# ============================================================================

param (
    [Parameter(Position = 0)]
    [ValidateSet("start", "stop", "status", "health", "test", "cli", "chat", "mcp-call", "hnsw-query", "swarm-exec")]
    [string]$Action = "status",

    [Parameter(Position = 1)]
    [string]$Query = ""
)

$WidyaExe = "c:\widya-lang\target\release\widya.exe"
$ServerScript = "c:\widya-lang\widya_ai_local\server_widya_ai.wya"
$Port = 8088

function Show-Banner {
    Write-Host "======================================================================" -ForegroundColor Cyan
    Write-Host "   WIDYA SOVEREIGN AI CORE — SERVICE CONTROLLER (POWERSHELL)" -ForegroundColor Yellow
    Write-Host "   Hardware: AMD Ryzen 7 8840HS (Zen 4) | XDNA NPU (16 TOPS)" -ForegroundColor White
    Write-Host "======================================================================" -ForegroundColor Cyan
}

switch ($Action) {
    "start" {
        Show-Banner
        Write-Host "[WIDYA-AI] Memulai Server HTTP Enterprise di latar belakang..." -ForegroundColor Green
        $proc = Start-Process -FilePath $WidyaExe -ArgumentList $ServerScript -WorkingDirectory "c:\widya-lang" -PassThru -WindowStyle Hidden
        Write-Host "[WIDYA-AI] Server berjalan dengan PID: $($proc.Id)" -ForegroundColor Green
        Start-Sleep -Seconds 1
        Invoke-RestMethod -Uri "http://localhost:$Port/v1/health" -Method Get | ConvertTo-Json
    }

    "status" {
        Show-Banner
        $conn = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
        if ($conn) {
            Write-Host "[STATUS] Server AKTIF pada port $Port (PID: $($conn.OwningProcess))" -ForegroundColor Green
            try {
                $h = Invoke-RestMethod -Uri "http://localhost:$Port/v1/health" -Method Get
                Write-Host "  Model Status : $($h.status)" -ForegroundColor White
                Write-Host "  Versi Genom  : $($h.versi)" -ForegroundColor White
                Write-Host "  Kedaulatan   : $($h.kedaulatan)" -ForegroundColor White
            } catch {
                Write-Host "  Peringatan: Port terbuka namun HTTP belum merespon." -ForegroundColor Yellow
            }
        } else {
            Write-Host "[STATUS] Server TIDAK BERJALAN pada port $Port." -ForegroundColor Red
        }
    }

    "health" {
        try {
            $resp = Invoke-RestMethod -Uri "http://localhost:$Port/v1/health" -Method Get
            $resp | Format-List
        } catch {
            Write-Host "[ERROR] Gagal menghubungi http://localhost:$Port/v1/health: $_" -ForegroundColor Red
        }
    }

    "chat" {
        $promptText = if ($Query -ne "") { $Query } else { "Berapa jarak Jakarta ke IKN Nusantara?" }
        Write-Host "[CHAT] Mengirim permintaan ke /v1/chat/completions..." -ForegroundColor Cyan
        $body = @{
            model = "widya-sovereign-rsi-01"
            messages = @(
                @{ role = "user"; content = $promptText }
            )
        } | ConvertTo-Json

        $resp = Invoke-RestMethod -Uri "http://localhost:$Port/v1/chat/completions" -Method Post -Body $body -ContentType "application/json"
        Write-Host "`nJawaban AI:" -ForegroundColor Green
        Write-Host $resp.choices[0].message.content -ForegroundColor White
    }

    "hnsw-query" {
        $kueriText = if ($Query -ne "") { $Query } else { "ikn sepaku smart city" }
        Write-Host "[HNSW] Melakukan pencarian graf vektor multi-layer untuk '$kueriText'..." -ForegroundColor Cyan
        $body = @{
            kueri = $kueriText
            k = 3
        } | ConvertTo-Json

        $resp = Invoke-RestMethod -Uri "http://localhost:$Port/v1/hnsw/query" -Method Post -Body $body -ContentType "application/json"
        $resp | ConvertTo-Json -Depth 4
    }

    "swarm-exec" {
        $instruksiText = if ($Query -ne "") { $Query } else { "Analisis alokasi demografi desa Sepaku IKN" }
        Write-Host "[SWARM] Menjalankan 4-Agent Swarm Orchestration dengan 4/4 Consensus..." -ForegroundColor Cyan
        $body = @{
            instruksi = $instruksiText
        } | ConvertTo-Json

        $resp = Invoke-RestMethod -Uri "http://localhost:$Port/v1/swarm/execute" -Method Post -Body $body -ContentType "application/json"
        $resp | ConvertTo-Json -Depth 4
    }

    "test" {
        Show-Banner
        Write-Host "[TEST 1/3] Menjalankan Simulasi Evolusi Mandiri (16 Tahap)..." -ForegroundColor Cyan
        & $WidyaExe "c:\widya-lang\widya_ai_local\simulasi_evolusi_mandiri.wya"

        Write-Host "`n[TEST 2/3] Menjalankan Simulasi Industri Enterprise (10 Tahap)..." -ForegroundColor Cyan
        & $WidyaExe "c:\widya-lang\widya_ai_local\simulasi_industri_widya_ai.wya"

        Write-Host "`n[TEST 3/3] Menjalankan Simulasi Lanjutan SLM, HNSW, Swarm, Ed25519 (12 Tahap)..." -ForegroundColor Cyan
        & $WidyaExe "c:\widya-lang\widya_ai_local\simulasi_widya_ai_lanjutan.wya"
    }

    "cli" {
        Show-Banner
        & $WidyaExe "c:\widya-lang\widya_ai_local\cli_widya_ai.wya"
    }
}
