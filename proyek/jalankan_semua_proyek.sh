#!/bin/bash
# ==============================================================================
# SCRIPT ORCHESTRATOR PROYEK MANDIRI WIDYA-LANG
# Setiap proyek berjalan mandiri di port unik menggunakan runtime resmi Widya
# ==============================================================================

WIDYA_BIN="/home/devops/widya-lang/target/release/widya"
PROYEK_DIR="/home/devops/widya-lang/proyek"

echo "================================================================================"
echo "🚀 MEMULAI EKSEKUSI PROYEK MANDIRI WIDYA-LANG (ISOLATED WEB ENGINES)"
echo "================================================================================"

start_project() {
    local name=$1
    local dir=$2
    local port=$3
    echo "[+] Menjalankan $name pada http://localhost:$port (Port Mandiri)..."
    nohup "$WIDYA_BIN" web dev "$PROYEK_DIR/$dir/web" -p "$port" > "$PROYEK_DIR/$dir/server.log" 2>&1 &
    echo "    PID: $! | Direktori: $PROYEK_DIR/$dir"
}

start_project "01. Koperasi Desa Merah Putih (KDMP)" "01_koperasi_kdmp" 8001
start_project "02. WidyaPay Core Switcher 50k TPS" "02_widyapay_switcher" 8002
start_project "03. SmartCity ISPU & ATCS Telemetri" "03_smartcity_telemetri" 8003
start_project "04. WidyaMedika SIMRS SatuSehat" "04_widyamedika_simrs" 8004
start_project "05. Widya Workspace CRDT Collab" "05_widya_workspace_crdt" 8005
start_project "06. WidyaOS Sovereign Cloud System" "06_widyaos_sovereign" 8006

echo "================================================================================"
echo "✅ Seluruh 6 Proyek Mandiri Telah Aktif di Port 8001 s/d 8006"
echo "Setiap proyek memiliki folder docs/ (PRD, ERD, Security, QA, DevOps), web/, dan database/"
echo "================================================================================"
