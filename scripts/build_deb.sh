#!/usr/bin/env bash
set -euo pipefail

VERSION="2.5.0"
ARCH="amd64"
PKG_DIR="target/deb/widya_${VERSION}_${ARCH}"
DIST_DIR="dist"

echo "=== Membangun Paket Debian: widya_${VERSION}_${ARCH}.deb ==="

mkdir -p "${PKG_DIR}/DEBIAN"
mkdir -p "${PKG_DIR}/usr/local/bin"
mkdir -p "${PKG_DIR}/usr/share/man/man1"
mkdir -p "${PKG_DIR}/usr/share/doc/widya"
mkdir -p "${PKG_DIR}/usr/share/widya/compiler_self_hosted"
mkdir -p "${PKG_DIR}/lib/systemd/system"
mkdir -p "${DIST_DIR}"

# Copy binary
if [ ! -f "target/release/widya" ]; then
    echo "Mengompilasi binary release widya..."
    cargo build --release
fi
cp target/release/widya "${PKG_DIR}/usr/local/bin/"
chmod 755 "${PKG_DIR}/usr/local/bin/widya"

# Copy self-hosted compiler files
if [ -d "compiler_self_hosted" ]; then
    cp -r compiler_self_hosted/* "${PKG_DIR}/usr/share/widya/compiler_self_hosted/"
fi

# Copy documentation & license
cp LICENSE "${PKG_DIR}/usr/share/doc/widya/copyright"
cp README.md "${PKG_DIR}/usr/share/doc/widya/"
cp CHANGELOG.md "${PKG_DIR}/usr/share/doc/widya/"

# Systemd service
if [ -f "scripts/widya.service" ]; then
    cp scripts/widya.service "${PKG_DIR}/lib/systemd/system/"
fi

# Generate control file
cat <<EOF > "${PKG_DIR}/DEBIAN/control"
Package: widya
Version: ${VERSION}
Section: devel
Priority: optional
Architecture: ${ARCH}
Maintainer: Tim Pengembang Widya-Lang <dev@widya-lang.org>
Description: Bahasa Pemrograman Asli Indonesia Berperforma Tinggi
 Widya-Lang adalah bahasa pemrograman multi-paradigma modern yang dirancang
 untuk keandalan tinggi, performa native (LLVM / WASM / C), sistem inferensi
 tipe Hindley-Milner, WidyaBase database engine, dan ekosistem terpadu.
Homepage: https://widya-lang.org
EOF

# Generate postinst
cat <<'EOF' > "${PKG_DIR}/DEBIAN/postinst"
#!/bin/sh
set -e
echo "Widya-Lang v2.5.0 berhasil dipasang!"
echo "Jalankan 'widya --help' atau 'widya learn' untuk memulai."
exit 0
EOF
chmod 755 "${PKG_DIR}/DEBIAN/postinst"

# Build .deb package
dpkg-deb --build "${PKG_DIR}" "${DIST_DIR}/widya_${VERSION}_${ARCH}.deb"
sha256sum "${DIST_DIR}/widya_${VERSION}_${ARCH}.deb" > "${DIST_DIR}/widya_${VERSION}_${ARCH}.deb.sha256"

echo "=== Paket Debian Berhasil Dibuat: ${DIST_DIR}/widya_${VERSION}_${ARCH}.deb ==="
cat "${DIST_DIR}/widya_${VERSION}_${ARCH}.deb.sha256"
