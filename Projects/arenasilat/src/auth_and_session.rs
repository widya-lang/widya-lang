// Projects/arenasilat/src/auth_and_session.rs
// ==============================================================================
// ARENA SILAT SAAS — SISTEM AUTENTIKASI, MULTI-ROLE SSO & REGISTRASI PENGGUNA
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definisi Peran Akses Sistem (RBAC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRole {
    SuperAdmin,
    FederationIpsi,
    EoOrganizer,
    TechnicalDelegate,
    KetuaPertandingan,
    Jury,
    DoctorMedic,
    MarshallingOfficer,
    CoachOfficial,
    AthleteUser,
    PublicGuest,
}

impl UserRole {
    pub fn label(&self) -> &'static str {
        match self {
            UserRole::SuperAdmin => "Super Admin Platform",
            UserRole::FederationIpsi => "Pengurus Federasi IPSI",
            UserRole::EoOrganizer => "Event Organizer / Panitia",
            UserRole::TechnicalDelegate => "Technical Delegate / Dewan Hakim",
            UserRole::KetuaPertandingan => "Ketua Pertandingan (KP)",
            UserRole::Jury => "Wasit Juri Penilai",
            UserRole::DoctorMedic => "Dokter Pertandingan & Medis",
            UserRole::MarshallingOfficer => "Petugas Marshalling & Timbangan",
            UserRole::CoachOfficial => "Pelatih / Manajer Kontingen",
            UserRole::AthleteUser => "Pesilat / Atlet Mandiri",
            UserRole::PublicGuest => "Publik / Penonton Umum",
        }
    }
}

/// Akun Pengguna Terdaftar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkunPengguna {
    pub user_id: String,
    pub email: String,
    pub nomor_telepon: String,
    pub nama_lengkap: String,
    pub password_hash: String,
    pub roles: Vec<UserRole>,
    pub active_role: UserRole,
    pub organisasi_kontingen: Option<String>,
    pub terverifikasi: bool,
}

/// Mesin Autentikasi & Sesi Pengguna
pub struct AuthEngine {
    pub users: HashMap<String, AkunPengguna>, // Key: user_id
    pub email_to_id: HashMap<String, String>,
}

impl AuthEngine {
    pub fn baru() -> Self {
        let mut engine = Self {
            users: HashMap::new(),
            email_to_id: HashMap::new(),
        };

        // Inisialisasi Akun Demo Bawaan untuk Tiap Peran
        let _ = engine.daftarkan_user("admin@arenasilat.id", "08110000001", "Super Admin Master", "admin123", UserRole::SuperAdmin, Some("Arena Silat Pusat"));
        let _ = engine.daftarkan_user("eo@silat.id", "08110000002", "Panitia Kejurnas 2026", "eo123", UserRole::EoOrganizer, Some("Panitia Pelaksana"));
        let _ = engine.daftarkan_user("td@ipsi.or.id", "08110000003", "Dr. Sugiyanto (TD)", "td123", UserRole::TechnicalDelegate, Some("PB IPSI"));
        let _ = engine.daftarkan_user("kp@gelanggang.id", "08110000004", "Drs. H. Mulyono (KP Gelanggang A)", "kp123", UserRole::KetuaPertandingan, Some("Gelanggang A"));
        let _ = engine.daftarkan_user("juri1@ipsi.id", "08110000005", "Budi Santoso (Juri 1)", "juri123", UserRole::Jury, Some("Wasit Juri Nasional"));
        let _ = engine.daftarkan_user("medis@rsud.id", "08110000006", "dr. Anita Rahayu, Sp.KO", "medis123", UserRole::DoctorMedic, Some("Tim Medis RSUD"));
        let _ = engine.daftarkan_user("marshall@arena.id", "08110000007", "Petugas Dimas (Call Room)", "marshall123", UserRole::MarshallingOfficer, Some("Ruang Marshalling"));
        let _ = engine.daftarkan_user("coach@jateng.id", "08110000008", "Pelatih Bambang Supriyanto", "coach123", UserRole::CoachOfficial, Some("Kontingen Jawa Tengah"));
        let _ = engine.daftarkan_user("atlet@eko.id", "08110000009", "Eko Prasetyo (Pesilat Dewasa)", "atlet123", UserRole::AthleteUser, Some("Kontingen Jawa Tengah"));

        engine
    }

    /// Hash password menggunakan SHA-256 murni
    pub fn hash_password(password: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Daftarkan Pengguna Baru
    pub fn daftarkan_user(
        &mut self,
        email: &str,
        no_telp: &str,
        nama: &str,
        password: &str,
        role: UserRole,
        organisasi: Option<&str>,
    ) -> Result<String, &'static str> {
        if self.email_to_id.contains_key(email) {
            return Err("Email sudah terdaftar!");
        }

        let user_id = format!("USR-{}", self.users.len() + 1);
        let pwd_hash = Self::hash_password(password);

        let user = AkunPengguna {
            user_id: user_id.clone(),
            email: email.to_lowercase(),
            nomor_telepon: no_telp.to_string(),
            nama_lengkap: nama.to_string(),
            password_hash: pwd_hash,
            roles: vec![role],
            active_role: role,
            organisasi_kontingen: organisasi.map(|s| s.to_string()),
            terverifikasi: true,
        };

        self.email_to_id.insert(email.to_lowercase(), user_id.clone());
        self.users.insert(user_id.clone(), user);

        Ok(user_id)
    }

    /// Login Pengguna (Email/No Telp + Password)
    pub fn login(&self, identitas: &str, password: &str) -> Option<&AkunPengguna> {
        let identitas_lower = identitas.to_lowercase();
        let pwd_hash = Self::hash_password(password);

        // Cari berdasarkan Email
        if let Some(uid) = self.email_to_id.get(&identitas_lower) {
            if let Some(user) = self.users.get(uid) {
                if user.password_hash == pwd_hash {
                    return Some(user);
                }
            }
        }

        // Cari berdasarkan Nomor Telepon
        for user in self.users.values() {
            if (user.nomor_telepon == identitas || user.email == identitas_lower) && user.password_hash == pwd_hash {
                return Some(user);
            }
        }

        None
    }

    /// Switch Context Peran (1-Klik Tanpa Logout)
    pub fn switch_context(&mut self, user_id: &str, target_role: UserRole) -> bool {
        if let Some(user) = self.users.get_mut(user_id) {
            if user.roles.contains(&target_role) {
                user.active_role = target_role;
                return true;
            }
        }
        false
    }
}
