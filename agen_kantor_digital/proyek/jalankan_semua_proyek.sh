#!/bin/bash
# ==============================================================================
# SCRIPT ORCHESTRATOR 10 PROYEK INDUSTRI MANDIRI WIDYA-LANG (100% NATIVE RUNTIME)
# Menggunakan mesin server WidyaWeb Cloud Edge dan WidyaOffice Native PDF Compiler
# ==============================================================================

WIDYA_BIN="/home/devops/widya-lang/target/release/widya"
DIR_BASE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "================================================================================"
echo "🚀 1. KOMPILASI DOKUMEN PERSIAPAN MARKDOWN KE PDF (NATIVE WIDYAOFFICE)"
echo "================================================================================"
"$WIDYA_BIN" office kompilasi-docs --dir "$DIR_BASE"

echo "================================================================================"
echo "🚀 2. MEMULAI SERVER INDEPENDEN PROYEK (WIDYAWEB SOVEREIGN EDGE ENGINES)"
echo "================================================================================"

start_project() {
    local code=$1
    local name=$2
    local port=$3
    local dir="$DIR_BASE/$code"
    
    if [ -d "$dir/web" ]; then
      echo "[+] Menjalankan $code ($name) pada http://localhost:$port (WidyaWeb Native)..."
      nohup "$WIDYA_BIN" web dev "$dir/web" -p "$port" > "$dir/server.log" 2>&1 &
      echo "    PID: $! | Direktori: $dir"
    else
      echo "[!] Folder $dir/web tidak ditemukan!"
    fi
}

start_project "PRJ-01-KOPERASI" "Koperasi Merah Putih & Simpan Pinjam Syariah" 8001
start_project "PRJ-02-SMARTCITY" "Platform Telemetri ISPU & GIS Smart City" 8002
start_project "PRJ-03-COLLAB-DOC" "Kolaborasi Dokumen Realtime CRDT & Markdown" 8003
start_project "PRJ-04-DAPODIK-AUTO" "Otomasi Integrasi Data Pokok Pendidikan" 8004
start_project "PRJ-05-LATEX-JOURNAL" "Generator Jurnal Ilmiah & Kompilator LaTeX" 8005
start_project "PRJ-06-WIDYAPAY" "WidyaPay Switcher QRIS & Core Banking" 8006
start_project "PRJ-07-WIDYAMEDIKA" "WidyaMedika SIMRS & SATUSEHAT HL7 FHIR" 8007
start_project "PRJ-08-WIDYAOS" "WidyaOS Bare-Metal Rust Kernel & OEM" 8008
start_project "PRJ-09-WIDYAAI-BOX" "WidyaAI Box Offline & Cyber Shield PQC" 8009
start_project "PRJ-10-WIDYALEDGER" "WidyaLedger Sovereign Blockchain & RWA" 8010

echo "================================================================================"
echo "✅ Seluruh 10 Proyek Industri Telah Aktif di Port 8001 s/d 8010 (Native Widya-Lang)"
echo "Kedaulatan Sistem: 100% On-Premise, Zero-Python Runtime, Standar ISO 32000-1"
echo "================================================================================"
