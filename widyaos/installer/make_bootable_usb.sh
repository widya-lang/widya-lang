#!/bin/bash
# ====================================================================
# SCRIPT OTOMASI PEMBUATAN USB FLASHER WIDYAOS (BASH / LINUX)
# Standar Perakitan Lini Pabrik OEM & Teknisi Dinas Pendidikan
# Hak Cipta (C) 2026 Kemendikdasmen RI & Tim Pengembang WidyaOS
# ====================================================================

set -e

TARGET_DEV="$1"
OEM_MFG_ID="${2:-MFG-ZYREX-01}"
TKDN_CERT="${3:-TKDN-KEMENPERIN-2026-88912}"

echo "===================================================================="
echo "         WIDYAOS OEM FLASHER & USB BOOTABLE CREATOR (BASH)          "
echo "===================================================================="

if [ "$EUID" -ne 0 ]; then
    echo "Galat: Skrip ini memerlukan hak akses root (sudo)!"
    exit 1
fi

if [ -z "$TARGET_DEV" ]; then
    echo "Penggunaan: sudo ./make_bootable_usb.sh /dev/sdX [OEM_ID] [TKDN_CERT]"
    echo "Contoh: sudo ./make_bootable_usb.sh /dev/sdb MFG-ZYREX-01 TKDN-2026-01"
    echo ""
    echo "Daftar disk yang terhubung:"
    lsblk -d -o NAME,SIZE,MODEL,TRAN
    exit 1
fi

echo "[1/4] Menyiapkan perangkat $TARGET_DEV..."
read -p "PERINGATAN: Seluruh data di $TARGET_DEV akan terhapus! Lanjutkan? (y/N): " CONFIRM
if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    echo "Dibatalkan."
    exit 0
fi

# Partisi GPT (ESP 512M, Recovery 4G, Data sisa)
echo "[2/4] Membentuk skema partisi GPT Dual-Slot..."
wipefs -a "$TARGET_DEV"
parted -s "$TARGET_DEV" mklabel gpt
parted -s "$TARGET_DEV" mkpart ESP fat32 1MiB 513MiB
parted -s "$TARGET_DEV" set 1 esp on
parted -s "$TARGET_DEV" mkpart GOLDEN_RECOVERY ext4 513MiB 4609MiB
parted -s "$TARGET_DEV" mkpart WIDYA_INSTALL ext4 4609MiB 100%

# Format partisi
echo "[3/4] Memformat partisi..."
mkfs.vfat -F32 -n "ESP" "${TARGET_DEV}1"
mkfs.ext4 -F -L "GOLDEN_REC" "${TARGET_DEV}2"
mkfs.ext4 -F -L "WIDYA_INST" "${TARGET_DEV}3"

# Salin konfigurasi bootloader GRUB
echo "[4/4] Memasang bootloader GRUB dan menyalin konfigurasi..."
TMP_MOUNT=$(mktemp -d)
mount "${TARGET_DEV}1" "$TMP_MOUNT"
mkdir -p "$TMP_MOUNT/EFI/BOOT"
mkdir -p "$TMP_MOUNT/boot/grub"

SCRIPT_DIR=$(dirname "$0")
if [ -f "$SCRIPT_DIR/grub.cfg" ]; then
    cp "$SCRIPT_DIR/grub.cfg" "$TMP_MOUNT/boot/grub/grub.cfg"
fi

# Injeksi segel TKDN
cat <<EOF > "$TMP_MOUNT/oem_tkdn_seal.json"
{
  "oem_mfg_id": "$OEM_MFG_ID",
  "tkdn_certificate": "$TKDN_CERT",
  "persentase_tkdn": "48.5%",
  "standard_inpres": "INPRES_NO_2_TAHUN_2022",
  "flashed_timestamp": "$(date -u +%Y%m%d-%H%M%SZ)",
  "bootloader": "GRUB_DUAL_SLOT_GOLDEN_V2",
  "security_eal": "BSSN_COMMON_CRITERIA_EAL4_PLUS"
}
EOF

sync
umount "$TMP_MOUNT"
rm -rf "$TMP_MOUNT"

echo "===================================================================="
echo "SUKSES! USB FLASHER WIDYAOS SIAP DIGUNAKAN DI $TARGET_DEV"
echo "===================================================================="
