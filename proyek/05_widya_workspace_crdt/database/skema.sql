-- ==============================================================================
-- skema.sql - Skema Basis Data Widya Workspace CRDT
-- Engine: WidyaBase v1.0.0
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA dokumen_workspace (
    id_dokumen TEKS KUNCI_UTAMA,
    judul TEKS,
    konten_markdown TEKS,
    penulis_terakhir TEKS,
    versi INT,
    waktu_update TEKS
);

SISIP_KE dokumen_workspace NILAI ('DOC-PROPOSAL-01', 'Proposal Kerja Sama Digitalisasi KDMP', '# Proposal...', 'agen_orkestrator', 1, '2026-09-30 17:00 WIB');
