#!/usr/bin/env bash
# =========================================================================
# Script Penyiapan Cloudflare Tunnel untuk Widya-Lang Server (Linux)
# Membuka akses https://api.widya-lang.org tanpa IP Publik / Port Forward
# =========================================================================

set -e

echo "🚀 [Widya-Lang] Menyiapkan Cloudflare Tunnel di Linux..."

# 1. Unduh dan Pasang cloudflared
if ! command -v cloudflared &> /dev/null; then
    echo "📦 Mengunduh cloudflared binary..."
    curl -L --output cloudflared.deb https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
    sudo dpkg -i cloudflared.deb
    rm cloudflared.deb
    echo "✅ cloudflared berhasil dipasang."
else
    echo "✅ cloudflared sudah terpasang di sistem."
fi

echo "========================================================================="
echo "Untuk mengaktifkan tunnel, jalankan perintah berikut dengan token Anda:"
echo "sudo cloudflared service install <TUNNEL_TOKEN_DARI_CLOUDFLARE_ZERO_TRUST>"
echo "sudo systemctl start cloudflared"
echo "sudo systemctl status cloudflared"
echo "========================================================================="
