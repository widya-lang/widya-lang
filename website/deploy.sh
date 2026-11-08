#!/usr/bin/env bash
# ==============================================================================
# Script Otomatis Deploy Portal widya-lang.org ke Server VPS
# Penggunaan: ./deploy.sh [user@server_ip]
# Contoh: ./deploy.sh root@103.123.45.67
# ==============================================================================

set -e

TARGET="${1:-root@widya-lang.org}"
REMOTE_DIR="/var/www/widya-lang.org"

echo "🚀 Memulai Deployment Portal widya-lang.org ke: ${TARGET}:${REMOTE_DIR}"

# 1. Pastikan direktori remote ada
ssh "${TARGET}" "mkdir -p ${REMOTE_DIR} ${REMOTE_DIR}/downloads ${REMOTE_DIR}/.well-known"

# 2. Sinkronkan berkas menggunakan rsync
echo "📦 Mengunggah berkas web statis..."
rsync -avz --delete \
    --exclude="deploy.sh" \
    --exclude=".git" \
    ./ "${TARGET}:${REMOTE_DIR}/"

# 3. Set izin akses yang benar (read-only untuk web server)
echo "🔒 Memperbarui izin akses di server remote..."
ssh "${TARGET}" "chown -R www-data:www-data ${REMOTE_DIR} && chmod -R 755 ${REMOTE_DIR}"

# 4. Reload Nginx atau Caddy
echo "🔄 Me-reload web server..."
ssh "${TARGET}" "systemctl reload nginx || systemctl reload caddy || true"

echo "✨ Deployment Selesai! Portal widya-lang.org aktif."
