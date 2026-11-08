#!/bin/bash
# ==============================================================================
# Script Peluncur Resmi: arenasilat.id
# Platform Digital Pencak Silat Berdaulat Indonesia
# ==============================================================================

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
WIDYA_BIN="/home/devops/widya-lang/target/release/widya"

echo "=================================================================="
echo "🥋 MEMULAI PLATFORM ARENASILAT.ID"
echo "=================================================================="

# 1. Jalankan Engine Aturan PERSILAT Widya-Lang
if [ -f "$WIDYA_BIN" ]; then
    echo "⚡ Mengeksekusi Engine Widya-Lang..."
    "$WIDYA_BIN" run "$DIR/sumber/utama.wya"
else
    echo "⚠️ Biner Widya release belum tersedia, menjalankan interpretasi..."
fi

echo ""
echo "🌐 Menjalankan Server Web Portal Suite 16-in-1..."
python3 "$DIR/server.py"
