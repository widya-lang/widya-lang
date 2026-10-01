-- Skema Basis Data WidyaOS Sovereign Cloud
-- Tabel Registri Modul & Paket Sovereign
CREATE TABLE IF NOT EXISTS modul_sovereign (
    id_modul VARCHAR(50) PRIMARY KEY,
    nama_modul VARCHAR(100) NOT NULL,
    kategori VARCHAR(50) NOT NULL,
    versi VARCHAR(20) NOT NULL,
    status VARCHAR(30) DEFAULT 'AKTIF',
    hash_pqc_dilithium TEXT NOT NULL,
    tgl_rilis DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabel Audit Event & Kernel Security Log
CREATE TABLE IF NOT EXISTS kernel_audit_log (
    id_event INT AUTO_INCREMENT PRIMARY KEY,
    id_modul VARCHAR(50) NOT NULL,
    jenis_operasi VARCHAR(50) NOT NULL,
    level_keamanan VARCHAR(20) NOT NULL,
    signature_dilithium5 TEXT NOT NULL,
    verifikasi_kyber1024 VARCHAR(20) DEFAULT 'VALID',
    payload_sandbox TEXT,
    waktu_eksekusi DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Data Awal Registri Kernel
INSERT INTO modul_sovereign (id_modul, nama_modul, kategori, versi, status, hash_pqc_dilithium) VALUES
('MOD-01', 'Widya-Microkernel-Core', 'KERNEL', 'v2.4.0', 'RUNNING', 'dilithium5:8f9a2b4c6e...'),
('MOD-02', 'WidyaBase-Storage-Engine', 'STORAGE', 'v1.8.2', 'MOUNTED', 'dilithium5:3a7d9e1f5b...'),
('MOD-03', 'PQC-Crypto-Vault', 'SECURITY', 'v3.0.1', 'ENFORCED', 'dilithium5:5c2e8a4b7f...'),
('MOD-04', 'ZeroTrust-Policy-Daemon', 'NETWORK', 'v1.2.0', 'MONITORING', 'dilithium5:1e4b8d2a6c...');
