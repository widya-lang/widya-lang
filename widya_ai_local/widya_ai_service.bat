@echo off
REM ============================================================================
REM WIDYA SOVEREIGN AI CORE — SERVICE LAUNCHER (WINDOWS)
REM Hardware: AMD Ryzen 7 8840HS | AMD Radeon 780M | AMD Ryzen AI XDNA NPU
REM ============================================================================

set WIDYA_EXE=..\target\release\widya.exe
if not exist "%WIDYA_EXE%" (
    set WIDYA_EXE=widya.exe
)

if "%1"=="" goto help
if "%1"=="server" goto run_server
if "%1"=="cli" goto run_cli
if "%1"=="test" goto run_test
if "%1"=="health" goto run_health

:run_server
echo [WIDYA-AI] Menjalankan Widya Sovereign Enterprise Server (Port 8088)...
"%WIDYA_EXE%" server_widya_ai.wya
goto end

:run_cli
echo [WIDYA-AI] Menjalankan Antarmuka CLI Interaktif...
"%WIDYA_EXE%" cli_widya_ai.wya
goto end

:run_test
echo [WIDYA-AI] Menjalankan Seluruh Pengujian Otomatis...
echo 1/3: Pengujian Evolusi Mandiri & Skills...
"%WIDYA_EXE%" simulasi_evolusi_mandiri.wya
echo 2/3: Pengujian Standar Industri Enterprise...
"%WIDYA_EXE%" simulasi_industri_widya_ai.wya
echo 3/3: Pengujian Saraf Tiruan SLM, HNSW, Swarm, Ed25519...
"%WIDYA_EXE%" simulasi_widya_ai_lanjutan.wya
goto end

:run_health
echo [WIDYA-AI] Memeriksa status kesehatan server di http://localhost:8088/v1/health...
curl -s http://localhost:8088/v1/health
echo.
goto end

:help
echo ============================================================================
echo   WIDYA LOCAL SOVEREIGN AI CORE — SERVICE MANAGER
echo ============================================================================
echo   Penggunaan:
echo     widya_ai_service.bat server   - Jalankan Enterprise Server (Port 8088)
echo     widya_ai_service.bat cli      - Buka CLI Interaktif
echo     widya_ai_service.bat test     - Eksekusi 3 Suite Pengujian Otomatis
echo     widya_ai_service.bat health   - Periksa kesehatan server (HTTP GET /v1/health)
echo ============================================================================

:end
