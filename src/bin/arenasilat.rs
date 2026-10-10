// src/bin/arenasilat.rs
// ==============================================================================
// ARENA SILAT SAAS — SOVEREIGN PENCAK SILAT TOURNAMENT MANAGEMENT SUITE
// 100% In-House Sovereign Code powered by Widya-Lang Ecosystem
// Implementasi Komprehensif Seluruh 9 Peran Pengguna Sesuai Master Dokumen:
// 1. Super Admin Platform (/superadmin)
// 2. Event Organizer / Panitia (/eo & /pendaftaran)
// 3. Technical Delegate (TD) / Dewan Hakim (/td)
// 4. Ketua Pertandingan (KP) & Meja Gelanggang (/kp)
// 5. Wasit Juri 1..3 Touchpad (/juri?id=1..3)
// 6. Dokter Pertandingan & Tim Medis (/medis)
// 7. Petugas Marshalling & Timbangan BLE (/marshalling)
// 8. Manager Kontingen & Pelatih (/kontingen)
// 9. Publik, Scoreboard & Bagan (/scoreboard, /bracket, /laporan, /verifikasi)
// ==============================================================================

#[path = "../../Projects/arenasilat/src/scoring_engine.rs"]
mod scoring_engine;
#[path = "../../Projects/arenasilat/src/bracket_generator.rs"]
mod bracket_generator;
#[path = "../../Projects/arenasilat/src/var_and_hardware.rs"]
mod var_and_hardware;
#[path = "../../Projects/arenasilat/src/pendaftaran.rs"]
mod pendaftaran;
#[path = "../../Projects/arenasilat/src/kategori_and_drawing.rs"]
mod kategori_and_drawing;
#[path = "../../Projects/arenasilat/src/laporan_and_rekap.rs"]
mod laporan_and_rekap;
#[path = "../../Projects/arenasilat/src/medis_and_marshalling.rs"]
mod medis_and_marshalling;
#[path = "../../Projects/arenasilat/src/dewan_hakim_td.rs"]
mod dewan_hakim_td;
#[path = "../../Projects/arenasilat/src/multi_gelanggang_personel.rs"]
mod multi_gelanggang_personel;
#[path = "../../Projects/arenasilat/src/auth_and_session.rs"]
mod auth_and_session;
#[path = "../../Projects/arenasilat/src/multi_camera_and_bandwidth.rs"]
mod multi_camera_and_bandwidth;
#[path = "../../Projects/arenasilat/src/federasi_and_perizinan.rs"]
mod federasi_and_perizinan;
#[path = "../../Projects/arenasilat/src/payment_and_split_settlement.rs"]
mod payment_and_split_settlement;
#[path = "../../Projects/arenasilat/src/ai_biomekanik_and_cv.rs"]
mod ai_biomekanik_and_cv;
#[path = "../../Projects/arenasilat/src/asuransi_and_klaim_bpjs.rs"]
mod asuransi_and_klaim_bpjs;
#[path = "../../Projects/arenasilat/src/portal_publik_and_h2h.rs"]
mod portal_publik_and_h2h;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use scoring_engine::{
    ArenaScoringEngine, SudutPesilat, TipePoin, TipeHukuman, StatusPartai,
};
use bracket_generator::{
    BracketGenerator, AtletPeserta,
};
use var_and_hardware::{
    VarEngine, KeputusanVAR, ScaleBridge, CertificateEngine,
};
use pendaftaran::{
    RegistrationEngine, KategoriUsia, JenisKelamin, JenisKompetisi, AtletRegistrasi, Kontingen,
};
use kategori_and_drawing::{
    CategoryDrawingEngine, KelasTandingDefinisi,
};
use laporan_and_rekap::{
    ReportGenerator, RekapMedaliKontingen, NominasiPesilatTerbaik, LaporanKeuanganTurnamen,
};
use medis_and_marshalling::{
    MedicalEngine, MarshallingEngine, BagianTubuhCedera, KategoriDiagnosis, KeputusanDokter,
};
use dewan_hakim_td::{
    DewanHakimEngine, StatusOtorisasiBagan, StatusPutusanBanding,
};
use multi_gelanggang_personel::{
    MultiCourtManager, PersonelTeknis, GelanggangTurnamen, StatusGelanggang,
};
use auth_and_session::{
    AuthEngine, UserRole, AkunPengguna,
};
use multi_camera_and_bandwidth::{
    MultiCameraCourtManager, CameraAngle, CameraSourceType, BandwidthStrategy, CameraStreamFeed,
};
use federasi_and_perizinan::{
    FederasiEngine, TingkatFederasi, TingkatLisensiWasitJuri, WasitJuriTervalidasi,
};
use payment_and_split_settlement::PaymentEngine;
use ai_biomekanik_and_cv::{
    AIVisionAndBiomechanics, JenisTeknikSilat, ZonaSasaranSerangan,
};
use asuransi_and_klaim_bpjs::{
    InsuranceEngine, TingkatCederaKlaim, StatusKlaimMedis,
};
use portal_publik_and_h2h::{
    PublicPortalEngine, PertandinganPublikLive,
};


struct MultiRoleAppState {
    pub engine: ArenaScoringEngine,
    pub reg_engine: RegistrationEngine,
    pub med_engine: MedicalEngine,
    pub marsh_engine: MarshallingEngine,
    pub td_engine: DewanHakimEngine,
    pub court_manager: MultiCourtManager,
    pub auth_engine: AuthEngine,
    pub cam_manager: MultiCameraCourtManager,
    pub medali_kontingen: Vec<RekapMedaliKontingen>,
    pub nominasi_terbaik: Vec<NominasiPesilatTerbaik>,
    pub atlet_merah: String,
    pub kontingen_merah: String,
    pub atlet_biru: String,
    pub kontingen_biru: String,
    pub kategori: String,
    pub nomor_partai: String,
    pub log_aksi: Vec<String>,
}

fn render_navbar(active_tab: &str) -> String {
    let act = |tab: &str| -> &str {
        if active_tab == tab { "nav-sidebar-item active" } else { "nav-sidebar-item" }
    };

    let login_active = if active_tab == "login" { "active" } else { "" };
    let register_active = if active_tab == "register" { "active" } else { "" };

    format!(r###"
    <style>
        :root {{
            --sidebar-w: 260px;
        }}
        html, body {{
            box-sizing: border-box;
        }}
        body {{
            margin: 0;
            padding-left: calc(var(--sidebar-w) + 24px) !important;
            padding-right: 24px !important;
            padding-top: 20px !important;
            padding-bottom: 40px !important;
            background: #0b1120 !important;
            color: #f8fafc !important;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif !important;
            min-height: 100vh;
        }}
        .arena-sidebar {{
            position: fixed;
            top: 0;
            left: 0;
            bottom: 0;
            width: var(--sidebar-w);
            background: rgba(11, 17, 32, 0.96);
            backdrop-filter: blur(18px);
            -webkit-backdrop-filter: blur(18px);
            border-right: 1px solid #1e293b;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            z-index: 1000;
            padding: 16px 12px;
            box-sizing: border-box;
            box-shadow: 4px 0 24px rgba(0, 0, 0, 0.4);
            overflow-y: auto;
        }}
        .arena-sidebar::-webkit-scrollbar {{
            width: 4px;
        }}
        .arena-sidebar::-webkit-scrollbar-thumb {{
            background: #334155;
            border-radius: 4px;
        }}
        .sidebar-brand {{
            display: flex;
            align-items: center;
            gap: 10px;
            text-decoration: none;
            color: white;
            padding: 8px 10px 14px 10px;
            border-bottom: 1px solid #1e293b;
            margin-bottom: 12px;
        }}
        .sidebar-brand .brand-logo {{
            font-size: 1.7rem;
            filter: drop-shadow(0 0 10px rgba(245, 158, 11, 0.6));
        }}
        .sidebar-brand .brand-title {{
            font-size: 1.1rem;
            font-weight: 900;
            letter-spacing: 0.5px;
            background: linear-gradient(135deg, #ffffff 40%, #93c5fd);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            line-height: 1.2;
        }}
        .sidebar-brand .brand-sub {{
            font-size: 0.65rem;
            color: #f59e0b;
            font-weight: 700;
            letter-spacing: 0.8px;
        }}
        .nav-section {{
            margin-bottom: 14px;
        }}
        .nav-section-title {{
            font-size: 0.68rem;
            font-weight: 800;
            color: #64748b;
            letter-spacing: 0.8px;
            padding: 4px 10px 6px 10px;
            text-transform: uppercase;
        }}
        .nav-sidebar-item {{
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 8px 12px;
            color: #94a3b8;
            text-decoration: none;
            font-size: 0.84rem;
            font-weight: 500;
            border-radius: 8px;
            transition: all 0.15s ease;
            margin-bottom: 2px;
        }}
        .nav-sidebar-item:hover {{
            background: rgba(30, 41, 59, 0.8);
            color: #38bdf8;
            padding-left: 15px;
        }}
        .nav-sidebar-item.active {{
            background: linear-gradient(135deg, rgba(37, 99, 235, 0.3), rgba(30, 58, 138, 0.4));
            color: #60a5fa;
            border-left: 3px solid #3b82f6;
            font-weight: 700;
            box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
        }}
        .sidebar-footer {{
            padding-top: 12px;
            border-top: 1px solid #1e293b;
            display: flex;
            gap: 8px;
        }}
        .sidebar-btn-auth {{
            flex: 1;
            text-align: center;
            padding: 7px 10px;
            border-radius: 8px;
            font-size: 0.78rem;
            font-weight: 700;
            text-decoration: none;
            border: 1px solid #334155;
            transition: all 0.2s;
            color: #cbd5e1;
            background: #1e293b;
        }}
        .sidebar-btn-auth:hover {{
            border-color: #60a5fa;
            color: white;
            background: #2563eb;
        }}
        .sidebar-btn-auth.active {{
            background: #2563eb;
            color: white;
            border-color: #60a5fa;
        }}
        /* Mobile Topbar & Drawer */
        .mobile-header-bar {{
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            height: 52px;
            background: rgba(11, 17, 32, 0.95);
            backdrop-filter: blur(12px);
            border-bottom: 1px solid #1e293b;
            padding: 0 16px;
            align-items: center;
            justify-content: space-between;
            z-index: 1001;
        }}
        .mobile-toggle-btn {{
            background: #1e293b;
            border: 1px solid #334155;
            color: white;
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 0.85rem;
            cursor: pointer;
        }}
        @media (max-width: 900px) {{
            body {{
                padding-left: 16px !important;
                padding-top: 68px !important;
            }}
            .mobile-header-bar {{
                display: flex;
            }}
            .arena-sidebar {{
                transform: translateX(-100%);
                transition: transform 0.25s ease-in-out;
                top: 52px;
            }}
            .arena-sidebar.show {{
                transform: translateX(0);
            }}
        }}
    </style>

    <div class="mobile-header-bar">
        <div style="display:flex; align-items:center; gap:8px;">
            <span style="font-size:1.3rem;">🥋</span>
            <b style="font-size:0.95rem; color:white;">ARENA SILAT</b>
        </div>
        <button type="button" class="mobile-toggle-btn" onclick="document.querySelector('.arena-sidebar').classList.toggle('show')">
            ☰ Menu
        </button>
    </div>

    <aside class="arena-sidebar">
        <div>
            <a href="/scoreboard" class="sidebar-brand">
                <span class="brand-logo">🥋</span>
                <div>
                    <div class="brand-title">ARENA SILAT</div>
                    <div class="brand-sub">IPSI 2026 SOVEREIGN</div>
                </div>
            </a>

            <!-- Group 1: Pertandingan -->
            <div class="nav-section">
                <div class="nav-section-title">🏟️ Pertandingan</div>
                <a href="/scoreboard" class="{}">📺 Gelanggang TV Live</a>
                <a href="/camera" class="{}">📷 Multi-Camera & VAR</a>
                <a href="/gelanggang" class="{}">🏟️ Multi-Gelanggang A-F</a>
                <a href="/bracket" class="{}">🌳 Bagan Pohon 2^n</a>
                <a href="/kategori" class="{}">🎯 Kategori & Drawing</a>
            </div>

            <!-- Group 2: Wasit & Petugas -->
            <div class="nav-section">
                <div class="nav-section-title">⚖️ Petugas Gelanggang</div>
                <a href="/kp" class="{}">⚖️ Ketua Pertandingan</a>
                <a href="/td" class="{}">🏛️ Dewan Hakim & TD</a>
                <a href="/juri?id=1" class="{}">📱 Wasit Juri 1 Pad</a>
                <a href="/juri?id=2" class="{}">📱 Wasit Juri 2 Pad</a>
                <a href="/juri?id=3" class="{}">📱 Wasit Juri 3 Pad</a>
                <a href="/medis" class="{}">🩺 Dokter Medis 3 Mnt</a>
                <a href="/marshalling" class="{}">⚖️ Marshalling & BLE</a>
            </div>

            <!-- Group 3: Manajemen & Data -->
            <div class="nav-section">
                <div class="nav-section-title">📋 Manajemen & Data</div>
                <a href="/pendaftaran" class="{}">📝 Panitia EO & Daftar</a>
                <a href="/kontingen" class="{}">🥋 Portal Kontingen</a>
                <a href="/laporan" class="{}">📊 Laporan & Medali</a>
                <a href="/superadmin" class="{}">👑 Super Admin</a>
            </div>

            <!-- Group 4: Tata Kelola & Inovasi -->
            <div class="nav-section">
                <div class="nav-section-title">🏛️ Ekosistem & Inovasi</div>
                <a href="/federasi" class="{}">🏛️ Federasi & Audit IPSI</a>
                <a href="/pembayaran" class="{}">💳 Pembayaran & Split</a>
                <a href="/ai-vision" class="{}">⚡ AI Biomekanik Vision</a>
                <a href="/asuransi" class="{}">🩺 Asuransi BPJS Atlet</a>
                <a href="/publik" class="{}">🌐 Portal Publik & H2H</a>
            </div>
        </div>

        <div class="sidebar-footer">
            <a href="/" class="sidebar-btn-auth">🌐 Beranda</a>
            <a href="/logout" class="sidebar-btn-auth" style="background:#dc2626; border-color:#ef4444; color:white;">🚪 Keluar</a>
        </div>
    </aside>
    "###,
        act("scoreboard"),
        act("camera"),
        act("gelanggang"),
        act("bracket"),
        act("kategori"),

        act("kp"),
        act("td"),
        act("juri1"),
        act("juri2"),
        act("juri3"),
        act("medis"),
        act("marshalling"),

        act("pendaftaran"),
        act("kontingen"),
        act("laporan"),
        act("superadmin"),

        act("federasi"),
        act("pembayaran"),
        act("ai-vision"),
        act("asuransi"),
        act("publik")
    )
}

fn html_landing_page() -> String {
    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Arena Silat SaaS — Platform Turnamen Pencak Silat Berdaulat</title>
    <style>
        :root {{
            --bg-color: #070d18;
            --card-bg: rgba(15, 23, 42, 0.8);
            --primary: #2563eb;
            --primary-glow: rgba(37, 99, 235, 0.5);
            --gold: #f59e0b;
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --border-color: #1e293b;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: var(--bg-color) !important;
            color: var(--text-main) !important;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif !important;
            overflow-x: hidden;
            line-height: 1.5;
            padding: 0 !important;
            margin: 0 !important;
        }}
        /* Landing Top Navigation */
        .landing-nav {{
            position: fixed;
            top: 0; left: 0; right: 0;
            height: 70px;
            background: rgba(7, 13, 24, 0.88);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 40px;
            z-index: 1000;
        }}
        .nav-brand {{
            display: flex;
            align-items: center;
            gap: 12px;
            text-decoration: none;
            color: white;
        }}
        .nav-brand-logo {{
            font-size: 2rem;
            filter: drop-shadow(0 0 12px rgba(245, 158, 11, 0.6));
        }}
        .nav-brand-text {{
            font-size: 1.3rem;
            font-weight: 900;
            letter-spacing: 0.8px;
            background: linear-gradient(135deg, #ffffff 40%, #93c5fd);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .nav-links {{
            display: flex;
            align-items: center;
            gap: 25px;
        }}
        .nav-link {{
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.92rem;
            font-weight: 500;
            transition: color 0.2s;
        }}
        .nav-link:hover {{ color: #38bdf8; }}
        .nav-cta {{
            display: flex;
            align-items: center;
            gap: 12px;
        }}
        .btn-auth-nav {{
            padding: 9px 18px;
            border-radius: 8px;
            font-size: 0.88rem;
            font-weight: 700;
            text-decoration: none;
            transition: all 0.2s;
        }}
        .btn-login-nav {{
            background: rgba(30, 41, 59, 0.8);
            border: 1px solid var(--border-color);
            color: white;
        }}
        .btn-login-nav:hover {{
            background: #1e293b;
            border-color: #60a5fa;
        }}
        .btn-reg-nav {{
            background: linear-gradient(135deg, #2563eb, #1d4ed8);
            border: 1px solid #60a5fa;
            color: white;
            box-shadow: 0 4px 15px rgba(37, 99, 235, 0.4);
        }}
        .btn-reg-nav:hover {{
            transform: translateY(-1px);
            filter: brightness(1.15);
        }}
        /* Hero Section */
        .hero {{
            padding: 140px 20px 80px 20px;
            text-align: center;
            position: relative;
            background: radial-gradient(circle at 50% 10%, rgba(37, 99, 235, 0.22) 0%, transparent 60%);
        }}
        .hero-badge {{
            display: inline-flex;
            align-items: center;
            gap: 8px;
            padding: 6px 16px;
            background: rgba(245, 158, 11, 0.12);
            border: 1px solid rgba(245, 158, 11, 0.4);
            border-radius: 30px;
            color: var(--gold);
            font-size: 0.85rem;
            font-weight: 700;
            margin-bottom: 24px;
            box-shadow: 0 0 20px rgba(245, 158, 11, 0.2);
        }}
        .hero h1 {{
            font-size: 3.4rem;
            font-weight: 900;
            max-width: 950px;
            margin: 0 auto 20px auto;
            line-height: 1.15;
            letter-spacing: -0.5px;
            background: linear-gradient(135deg, #ffffff 20%, #e2e8f0 60%, #93c5fd 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .hero p {{
            font-size: 1.2rem;
            color: var(--text-muted);
            max-width: 780px;
            margin: 0 auto 35px auto;
            line-height: 1.6;
        }}
        .hero-cta-group {{
            display: flex;
            justify-content: center;
            gap: 16px;
            flex-wrap: wrap;
            margin-bottom: 50px;
        }}
        .btn-hero-primary {{
            background: linear-gradient(135deg, #2563eb, #1d4ed8);
            border: 1px solid #60a5fa;
            color: white;
            font-size: 1.1rem;
            font-weight: 800;
            padding: 15px 32px;
            border-radius: 12px;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: 10px;
            box-shadow: 0 10px 30px -5px rgba(37, 99, 235, 0.5);
            transition: all 0.2s;
        }}
        .btn-hero-primary:hover {{
            transform: translateY(-2px);
            box-shadow: 0 15px 35px -5px rgba(37, 99, 235, 0.6);
            filter: brightness(1.1);
        }}
        .btn-hero-secondary {{
            background: rgba(30, 41, 59, 0.8);
            border: 1px solid #334155;
            color: #cbd5e1;
            font-size: 1.05rem;
            font-weight: 700;
            padding: 15px 28px;
            border-radius: 12px;
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: 10px;
            transition: all 0.2s;
        }}
        .btn-hero-secondary:hover {{
            background: #1e293b;
            color: white;
            border-color: #64748b;
        }}
        /* Stats Ribbon */
        .stats-ribbon {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            max-width: 1150px;
            margin: 0 auto 70px auto;
            padding: 0 20px;
        }}
        .stat-card {{
            background: rgba(15, 23, 42, 0.7);
            border: 1px solid var(--border-color);
            border-radius: 14px;
            padding: 22px;
            text-align: center;
        }}
        .stat-num {{
            font-size: 2.4rem;
            font-weight: 900;
            color: #38bdf8;
            margin-bottom: 4px;
        }}
        .stat-label {{
            font-size: 0.88rem;
            color: var(--text-muted);
            font-weight: 500;
        }}
        /* Section Containers */
        .section-wrap {{
            max-width: 1200px;
            margin: 0 auto 90px auto;
            padding: 0 20px;
        }}
        .section-header {{
            text-align: center;
            margin-bottom: 50px;
        }}
        .section-header h2 {{
            font-size: 2.2rem;
            font-weight: 800;
            margin-bottom: 12px;
            background: linear-gradient(135deg, #ffffff 40%, #cbd5e1);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .section-header p {{
            color: var(--text-muted);
            font-size: 1rem;
            max-width: 600px;
            margin: 0 auto;
        }}
        /* Feature Grid */
        .feature-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
            gap: 20px;
        }}
        .feature-card {{
            background: rgba(15, 23, 42, 0.8);
            border: 1px solid var(--border-color);
            border-radius: 16px;
            padding: 28px;
            transition: all 0.25s ease;
            display: flex;
            flex-direction: column;
            gap: 12px;
        }}
        .feature-card:hover {{
            transform: translateY(-4px);
            border-color: #3b82f6;
            box-shadow: 0 12px 30px -8px rgba(37, 99, 235, 0.35);
        }}
        .feat-icon {{
            font-size: 2.2rem;
            margin-bottom: 6px;
        }}
        .feat-title {{
            font-size: 1.25rem;
            font-weight: 800;
            color: white;
        }}
        .feat-desc {{
            font-size: 0.9rem;
            color: var(--text-muted);
            line-height: 1.6;
        }}
        .feat-badge {{
            align-self: flex-start;
            font-size: 0.72rem;
            padding: 3px 8px;
            border-radius: 6px;
            background: rgba(59, 130, 246, 0.15);
            color: #60a5fa;
            font-weight: 700;
            border: 1px solid rgba(59, 130, 246, 0.3);
        }}
        /* Role Showcase */
        .role-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
            gap: 16px;
        }}
        .role-card {{
            background: #0d1526;
            border: 1px solid #1e293b;
            border-radius: 12px;
            padding: 18px;
            display: flex;
            align-items: center;
            gap: 14px;
            text-decoration: none;
            color: white;
            transition: all 0.2s;
        }}
        .role-card:hover {{
            background: #1e293b;
            border-color: #38bdf8;
            transform: translateX(4px);
        }}
        .role-icon {{ font-size: 1.8rem; }}
        .role-name {{ font-weight: 700; font-size: 0.95rem; }}
        .role-desc {{ font-size: 0.78rem; color: #94a3b8; margin-top: 2px; }}
        /* CTA Banner */
        .cta-banner {{
            background: linear-gradient(135deg, rgba(30, 58, 138, 0.6), rgba(15, 23, 42, 0.9));
            border: 1px solid rgba(59, 130, 246, 0.4);
            border-radius: 20px;
            padding: 50px 30px;
            text-align: center;
            box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.6);
        }}
        .cta-banner h3 {{
            font-size: 2.2rem;
            font-weight: 900;
            margin-bottom: 12px;
        }}
        .cta-banner p {{
            color: #cbd5e1;
            font-size: 1.05rem;
            max-width: 650px;
            margin: 0 auto 28px auto;
        }}
        /* Footer */
        .landing-footer {{
            border-top: 1px solid var(--border-color);
            padding: 40px 20px;
            text-align: center;
            color: #64748b;
            font-size: 0.88rem;
        }}
        @media (max-width: 768px) {{
            .landing-nav {{ padding: 0 20px; }}
            .nav-links {{ display: none; }}
            .hero h1 {{ font-size: 2.3rem; }}
            .hero p {{ font-size: 1rem; }}
        }}
    </style>
</head>
<body>
    <nav class="landing-nav">
        <a href="/" class="nav-brand">
            <span class="nav-brand-logo">🥋</span>
            <div class="nav-brand-text">ARENA SILAT</div>
        </a>
        <div class="nav-links">
            <a href="#fitur" class="nav-link">Fitur Unggulan</a>
            <a href="#peran" class="nav-link">9 Peran Pengguna</a>
            <a href="#penta-core" class="nav-link">Penta-Core Matrix</a>
            <a href="#regulasi" class="nav-link">Regulasi IPSI 2026</a>
        </div>
        <div class="nav-cta">
            <a href="/login" class="btn-auth-nav btn-login-nav">🔐 Masuk</a>
            <a href="/register" class="btn-auth-nav btn-reg-nav">📝 Daftar</a>
        </div>
    </nav>

    <section class="hero">
        <div class="hero-badge">
            <span>✨</span> 100% SOVEREIGN WIDYA-LANG ECOSYSTEM • IPSI / PERSILAT 2026
        </div>
        <h1>Platform Manajemen Turnamen Pencak Silat Berdaulat Generasi Baru</h1>
        <p>
            Sistem operasi digital pencak silat terlengkap di dunia tanpa ketergantungan pihak ketiga. 
            Mulai dari Pendaftaran Kontingen, Bagan Eliminasi 2ⁿ, Digital Consensus Scoring Sub-10ms, 4-Angle Slow-Mo VAR, 
            Timbangan BLE, hingga E-Sertifikat Anti-Fraud SHA-256 dan Integrasi SIMT Puspresnas.
        </p>
        <div class="hero-cta-group">
            <a href="/login" class="btn-hero-primary">
                <span>🚀</span> Masuk ke Portal Turnamen
            </a>
            <a href="/register" class="btn-hero-secondary">
                <span>📝</span> Registrasi Kontingen Baru
            </a>
        </div>
    </section>

    <div class="stats-ribbon">
        <div class="stat-card">
            <div class="stat-num">6 Court</div>
            <div class="stat-label">Multi-Gelanggang Paralel Paralel</div>
        </div>
        <div class="stat-card">
            <div class="stat-num">&lt; 10ms</div>
            <div class="stat-label">Latensi Sinkronisasi Scoreboard</div>
        </div>
        <div class="stat-card">
            <div class="stat-num">24 CAM</div>
            <div class="stat-label">4 Sudut WebRTC + Slow-Mo VAR</div>
        </div>
        <div class="stat-card">
            <div class="stat-num">100%</div>
            <div class="stat-label">Berdaulat Tanpa Dependensi Luar</div>
        </div>
    </div>

    <section id="fitur" class="section-wrap">
        <div class="section-header">
            <h2>8 Modul Keunggulan Standar Internasional</h2>
            <p>Dibangun khusus mematuhi seluruh Buku Regulasi Pertandingan IPSI & PERSILAT 2026</p>
        </div>
        <div class="feature-grid">
            <div class="feature-card">
                <span class="feat-icon">📺</span>
                <span class="feat-badge">REAL-TIME CONSENSUS</span>
                <div class="feat-title">Scoreboard TV & Juri Touchpad</div>
                <div class="feat-desc">Window konsensus dinamis 1.5 detik. Poin otomatis bertambah saat minimal 2 dari 3 juri sepakat (Pukulan +1, Tendangan +2, Jatuhan +3, Kuncian +4).</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">📷</span>
                <span class="feat-badge">SLOW-MO REPLAY</span>
                <div class="feat-title">Multi-Camera & Video VAR</div>
                <div class="feat-desc">Klip instan 30 detik ring buffer dari 4 sudut kamera, mode Mosaic Quad-View hemat 85% bandwidth, dan verifikasi kartu review ofisial.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">🌳</span>
                <span class="feat-badge">FAIR SEEDING</span>
                <div class="feat-title">Bagan Pohon 2ⁿ & Separasi Kontingen</div>
                <div class="feat-desc">Algoritma otomatis memisahkan atlet satu perguruan/daerah ke Pool Atas dan Pool Bawah untuk menjamin kompetisi yang adil dan seimbang.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">🏛️</span>
                <span class="feat-badge">GOVERNANCE</span>
                <div class="feat-title">Federasi IPSI & Rekomendasi Izin</div>
                <div class="feat-desc">Akreditasi lisensi Wasit Juri Kelas 1-3, audit sarana matras gelanggang, dan verifikasi surat izin kepolisian (Polres/Polda) serta MoU RSUD.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">💳</span>
                <span class="feat-badge">AUTOMATED SPLIT</span>
                <div class="feat-title">Payment Gateway & Split Finansial</div>
                <div class="feat-desc">Pemisahan dana otomatis seketika untuk EO, iuran kas pembinaan PB IPSI, dan deposit jaminan protes banding Dewan Hakim Rp 3.000.000,-.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">⚡</span>
                <span class="feat-badge">COMPUTER VISION</span>
                <div class="feat-title">AI Biomekanik & Pose Detection</div>
                <div class="feat-desc">Deteksi titik benturan zona sah Body Protector dada vs pelanggaran leher/kemaluan, batasan tangkapan 2.0s, dan radar daya ledak tendangan.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">🩺</span>
                <span class="feat-badge">SAFETY & MEDICAL</span>
                <div class="feat-title">Konsol Dokter & Asuransi BPJS</div>
                <div class="feat-desc">Doctor stoppage 3 menit, skorsing gegar otak 30 hari, perlindungan BPJS Ketenagakerjaan Rp 50 Juta, dan rujukan jalur hijau IGD RSUD.</div>
            </div>
            <div class="feature-card">
                <span class="feat-icon">🔒</span>
                <span class="feat-badge">ANTI-FRAUD</span>
                <div class="feat-title">E-Sertifikat SHA-256 & Puspresnas SIMT</div>
                <div class="feat-desc">QR Code kriptografi SHA-256 anti-pemalsuan dan ekspor data kurasi talenta 1-klik format SIMT Kemendikdasmen.</div>
            </div>
        </div>
    </section>

    <section id="peran" class="section-wrap">
        <div class="section-header">
            <h2>9 Peran Pengguna Terintegrasi</h2>
            <p>Seluruh peran operasional turnamen dapat diakses melalui portal otentikasi tunggal</p>
        </div>
        <div class="role-grid">
            <a href="/login" class="role-card">
                <span class="role-icon">👑</span>
                <div>
                    <div class="role-name">Super Admin Platform</div>
                    <div class="role-desc">Kontrol server, tenant & hardware</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">📝</span>
                <div>
                    <div class="role-name">Panitia Pelaksana / EO</div>
                    <div class="role-desc">Pendaftaran kontingen & keuangan</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">🏛️</span>
                <div>
                    <div class="role-name">Technical Delegate / TD</div>
                    <div class="role-desc">Pengesahan drawing & audit black box</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">⚖️</span>
                <div>
                    <div class="role-name">Ketua Pertandingan (KP)</div>
                    <div class="role-desc">Kontrol gelanggang & panel sanksi</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">📱</span>
                <div>
                    <div class="role-name">Wasit Juri Penilai</div>
                    <div class="role-desc">Touchpad skor sudut merah & biru</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">🩺</span>
                <div>
                    <div class="role-name">Dokter Pertandingan</div>
                    <div class="role-desc">Konsol medis 3 menit & rujukan RSUD</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">⚖️</span>
                <div>
                    <div class="role-name">Petugas Marshalling</div>
                    <div class="role-desc">Timbangan BLE & antrean call room</div>
                </div>
            </a>
            <a href="/login" class="role-card">
                <span class="role-icon">🥋</span>
                <div>
                    <div class="role-name">Pelatih & Manajer Kontingen</div>
                    <div class="role-desc">Input berkas atlet & kartu VAR</div>
                </div>
            </a>
        </div>
    </section>

    <div class="section-wrap">
        <div class="cta-banner">
            <h3>Siap Menyelenggarakan Turnamen Silat Kelas Dunia?</h3>
            <p>Masuk sekarang untuk mengakses panel turnamen Anda atau daftarkan akun kontingen baru.</p>
            <div style="display:flex; justify-content:center; gap:14px; flex-wrap:wrap;">
                <a href="/login" class="btn-hero-primary" style="padding:13px 28px; font-size:1rem;">
                    🔐 Masuk ke Sistem (Login)
                </a>
                <a href="/register" class="btn-hero-secondary" style="padding:13px 24px; font-size:1rem;">
                    📝 Buat Akun Baru
                </a>
            </div>
        </div>
    </div>

    <footer class="landing-footer">
        <div style="margin-bottom:8px; font-weight:700; color:#94a3b8;">
            🥋 ARENA SILAT SAAS — SOVEREIGN PENCAK SILAT TOURNAMENT OPERATING SYSTEM
        </div>
        <div>
            100% In-House Sovereign Code on Widya-Lang Ecosystem (WidyaOS, WidyaBase, WidyaWeb, WidyaStream, WidyaOffice).
        </div>
    </footer>
</body>
</html>"###)
}

fn html_dashboard(state: &MultiRoleAppState) -> String {
    let status_str = match state.engine.status {
        StatusPartai::Persiapan => "<span style='color:#f39c12'>PERSIAPAN</span>",
        StatusPartai::Berlangsung => "<span style='color:#2ecc71; font-weight:bold;'>BERLANGSUNG (LIVE)</span>",
        StatusPartai::JedaBabak => "<span style='color:#f1c40f'>ISTIRAHAT BABAK</span>",
        StatusPartai::TinjauanVAR => "<span style='color:#a855f7'>TINJAUAN VAR</span>",
        StatusPartai::WaktuMedis => "<span style='color:#ec4899'>WAKTU MEDIS (3 MENIT)</span>",
        StatusPartai::Selesai => "<span style='color:#e74c3c'>SELESAI (FINAL)</span>",
    };

    let log_items = state.log_aksi.iter().rev().take(8).map(|log| {
        format!("<li style='padding: 5px 8px; border-bottom: 1px solid #333; font-family: monospace;'>{}</li>", log)
    }).collect::<Vec<_>>().join("");

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Gelanggang TV Display — Arena Silat</title>
    <style>
        :root {{ --bg-dark: #0f172a; --card-dark: #1e293b; --gold: #f59e0b; --text-main: #f8fafc; }}
        body {{ margin: 0; padding: 15px; font-family: 'Segoe UI', system-ui, sans-serif; background: var(--bg-dark); color: var(--text-main); }}
        .header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 2px solid #334155; padding-bottom: 10px; margin-bottom: 15px; }}
        .badge {{ background: linear-gradient(135deg, #059669, #10b981); color: white; padding: 4px 10px; border-radius: 12px; font-size: 0.75rem; font-weight: bold; }}
        .grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 15px; }}
        .card-red {{ background: linear-gradient(145deg, #7f1d1d, #991b1b); border: 2px solid #ef4444; border-radius: 10px; padding: 20px; text-align: center; }}
        .card-blue {{ background: linear-gradient(145deg, #1e3a8a, #1e40af); border: 2px solid #3b82f6; border-radius: 10px; padding: 20px; text-align: center; }}
        .score-box {{ font-size: 4.5rem; font-weight: 900; margin: 10px 0; text-shadow: 0 4px 10px rgba(0,0,0,0.5); }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 6px; margin-top: 10px; background: rgba(0,0,0,0.3); padding: 8px; border-radius: 6px; font-size: 0.82rem; }}
        .center-panel {{ background: var(--card-dark); border-radius: 10px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; text-align: center; }}
        .log-box {{ background: #090d16; border: 1px solid #1e293b; border-radius: 6px; padding: 10px; max-height: 160px; overflow-y: auto; text-align: left; font-size: 0.88rem; }}
    </style>
    <script>
        setInterval(() => {{
            fetch('/api/status').then(r => r.json()).then(d => {{
                document.getElementById('skor-merah').innerText = d.skor_merah;
                document.getElementById('skor-biru').innerText = d.skor_biru;
                document.getElementById('babak-info').innerText = 'BABAK ' + d.babak;
            }});
        }}, 1000);
    </script>
</head>
<body>
    {}

    <div class="header">
        <div>
            <h1 style="margin:0; font-size:1.6rem; display:flex; align-items:center; gap:8px;">
                🥋 ARENA SILAT SAAS
                <span class="badge">100% WIDYA-LANG SOVEREIGN</span>
            </h1>
            <p style="margin:3px 0 0 0; color:#94a3b8; font-size:0.85rem;">Official Digital Scoring & Consensus Engine — IPSI 2026 Rules</p>
        </div>
        <div style="text-align:right;">
            <div style="font-size:1rem; font-weight:bold;">{} — {}</div>
            <div style="font-size:0.85rem; color:#94a3b8;">Status: {}</div>
        </div>
    </div>

    <div class="center-panel">
        <h2 id="babak-info" style="margin:0; font-size:2rem; letter-spacing:2px; color:var(--gold);">BABAK {}</h2>
    </div>

    <div class="grid">
        <div class="card-red">
            <div style="font-weight:bold; letter-spacing:1px; font-size:1rem;">🔴 SUDUT MERAH</div>
            <h2 style="margin:5px 0;">{}</h2>
            <div style="opacity:0.85; margin-bottom:10px;">{}</div>
            <div id="skor-merah" class="score-box">{}</div>
            <div class="stats-grid">
                <div><div>Pukulan (+1)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Tendangan (+2)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Jatuhan (+3)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Hukuman</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
            </div>
        </div>

        <div class="card-blue">
            <div style="font-weight:bold; letter-spacing:1px; font-size:1rem;">🔵 SUDUT BIRU</div>
            <h2 style="margin:5px 0;">{}</h2>
            <div style="opacity:0.85; margin-bottom:10px;">{}</div>
            <div id="skor-biru" class="score-box">{}</div>
            <div class="stats-grid">
                <div><div>Pukulan (+1)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Tendangan (+2)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Jatuhan (+3)</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
                <div><div>Hukuman</div><div style="font-size:1.2rem; font-weight:bold;">{}</div></div>
            </div>
        </div>
    </div>

    <div class="center-panel" style="text-align:left;">
        <h4 style="margin:0 0 8px 0;">📋 Log Konsensus & Aksi Pertandingan Real-Time:</h4>
        <ul class="log-box" style="list-style:none; margin:0; padding:0;">{}</ul>
    </div>
</body>
</html>"#,
        render_navbar("scoreboard"),
        state.nomor_partai, state.kategori, status_str,
        state.engine.babak_aktif,
        state.atlet_merah, state.kontingen_merah, state.engine.skor_merah.total_poin,
        state.engine.skor_merah.jumlah_pukulan, state.engine.skor_merah.jumlah_tendangan, state.engine.skor_merah.jumlah_jatuhan, state.engine.skor_merah.hukuman.len(),
        state.atlet_biru, state.kontingen_biru, state.engine.skor_biru.total_poin,
        state.engine.skor_biru.jumlah_pukulan, state.engine.skor_biru.jumlah_tendangan, state.engine.skor_biru.jumlah_jatuhan, state.engine.skor_biru.hukuman.len(),
        log_items
    )
}

fn html_td(state: &MultiRoleAppState) -> String {
    let status_bagan_str = match state.td_engine.status_bagan {
        StatusOtorisasiBagan::DraftBelumDisahkan => "<span style='color:#f59e0b; font-weight:bold;'>DRAFT (BELUM DISAHKAN)</span>",
        StatusOtorisasiBagan::TelahDisahkanOlehTD => "<span style='color:#10b981; font-weight:bold;'>TELAH DISAHKAN RESMI OLEH TD</span>",
        StatusOtorisasiBagan::DitolakPerluDrawingUlang => "<span style='color:#ef4444; font-weight:bold;'>DITOLAK / PERLU DRAWING ULANG</span>",
    };

    let mut log_rows = String::new();
    for entry in state.td_engine.black_box_logs.iter().rev().take(15) {
        log_rows.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:6px;'>#{}</td>
                <td>{}</td>
                <td>Juri {}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            entry.log_id, entry.partai_id, entry.juri_id, entry.sudut, entry.teknik,
            if entry.status_konsensus { "<span style='color:#10b981;'>KONSENSUS</span>" } else { "<span style='color:#94a3b8;'>INDIVIDUAL</span>" }
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Technical Delegate (TD) & Dewan Hakim — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 8px; border-bottom: 2px solid #334155; }}
        .btn {{ padding: 10px 18px; border-radius: 6px; font-weight: bold; border: none; cursor: pointer; color: white; }}
        .btn-green {{ background: #16a34a; }}
        .btn-green:hover {{ background: #15803d; }}
    </style>
    <script>
        function sahkanBagan() {{
            fetch('/api/td/approve_bracket', {{ method: 'POST' }}).then(r => r.json()).then(d => {{
                alert('Bagan Turnamen Resmi Disahkan oleh Technical Delegate!');
                location.reload();
            }});
        }}
    </script>
</head>
<body>
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <h2>🏛️ KONSOL TECHNICAL DELEGATE (TD) & DEWAN HAKIM</h2>
        <div>TD: <b>{}</b> ({})</div>
    </div>
    {}

    <div class="card" style="display:flex; justify-content:space-between; align-items:center;">
        <div>
            <h3 style="margin:0 0 5px 0;">Otorisasi Bagan & Penjadwalan Drawing</h3>
            <div>Status Otorisasi: {}</div>
        </div>
        <button class="btn btn-green" onclick="sahkanBagan()">✅ [ APPROVE BRACKET DRAWING ]</button>
    </div>

    <div class="card">
        <h3>📼 BLACK BOX SCORING AUDIT TRAIL (Sidang Banding Sengketa)</h3>
        <p style="color:#94a3b8; font-size:0.85rem; margin-top:0;">Merekam setiap fraksi detik penekanan tombol wasit juri untuk penyelesaian protes resmi.</p>
        <table>
            <thead>
                <tr><th>Log ID</th><th>Partai ID</th><th>Juri ID</th><th>Sudut</th><th>Teknik</th><th>Status Evaluasi</th></tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        state.td_engine.nama_td, state.td_engine.lisensi_ipsi_td,
        render_navbar("td"),
        status_bagan_str,
        if log_rows.is_empty() { "<tr><td colspan='6' style='text-align:center; padding:12px; color:#94a3b8;'>Belum ada log stream juri</td></tr>".to_string() } else { log_rows }
    )
}

fn html_medis(state: &MultiRoleAppState) -> String {
    let mut incident_rows = String::new();
    for inc in state.med_engine.daftar_insiden.iter().rev() {
        incident_rows.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:8px;'>{}</td>
                <td>{} ({})</td>
                <td>{:?}</td>
                <td>{:?}</td>
                <td>{} detik</td>
                <td style='font-weight:bold;'>{:?}</td>
                <td>{}</td>
            </tr>",
            inc.insiden_id, inc.nama_atlet, inc.sudut, inc.bagian_tubuh, inc.diagnosis,
            inc.durasi_penanganan_detik, inc.keputusan,
            if inc.skorsing_medis_hari > 0 { format!("<span style='color:#ef4444;'>{} Hari</span>", inc.skorsing_medis_hari) } else { "-".to_string() }
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Konsol Dokter Pertandingan & Medis — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        .grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }}
        .btn {{ padding: 12px; border-radius: 6px; font-weight: bold; border: none; cursor: pointer; color: white; width: 100%; margin-bottom: 8px; }}
        .btn-green {{ background: #16a34a; }}
        .btn-red {{ background: #dc2626; }}
        .btn-purple {{ background: #7c3aed; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 8px; border-bottom: 2px solid #334155; }}
    </style>
    <script>
        function recordMedical(decision) {{
            fetch('/api/medis/decision', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ decision: decision }})
            }}).then(r => r.json()).then(d => {{
                alert('Keputusan Medis Diproses: ' + decision);
                location.reload();
            }});
        }}
    </script>
</head>
<body>
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <h2>🩺 KONSOL DOKTER PERTANDINGAN (DOCTOR STOPPAGE AUTHORITY)</h2>
        <div>Partai Aktif: <b>{}</b></div>
    </div>
    {}

    <div class="grid">
        <div class="card">
            <h3>⏱️ Batas Waktu Tindakan Medis (Maksimal 3 Menit PERSILAT)</h3>
            <div style="font-size:3rem; font-weight:900; color:#ef4444; text-align:center; margin:15px 0;">03:00</div>
            <p style="font-size:0.85rem; color:#94a3b8; text-align:center;">Jika batas 3 menit terlampaui, dokter pertandingan berwenang mutlak menghentikan laga.</p>
        </div>

        <div class="card">
            <h3>🩺 Keputusan Dokter Pertandingan</h3>
            <button class="btn btn-green" onclick="recordMedical('FIT_TO_CONTINUE')">✅ FIT TO CONTINUE (Boleh Lanjut Laga)</button>
            <button class="btn btn-red" onclick="recordMedical('UNFIT_STOP_MATCH')">🛑 UNFIT (Hentikan Laga / WMP Medis)</button>
            <button class="btn btn-purple" onclick="recordMedical('RUJUK_RSUD_BPJS')">🚑 EVAKUASI IGD RSUD + KLAIM BPJS KETENAGAKERJAAN</button>
        </div>
    </div>

    <div class="card">
        <h3>📋 Riwayat Insiden Medis Gelanggang</h3>
        <table>
            <thead>
                <tr><th>ID Insiden</th><th>Atlet</th><th>Bagian Tubuh</th><th>Diagnosis</th><th>Durasi</th><th>Keputusan</th><th>Skorsing Medis</th></tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        state.nomor_partai,
        render_navbar("medis"),
        if incident_rows.is_empty() { "<tr><td colspan='7' style='text-align:center; padding:12px; color:#94a3b8;'>Tidak ada insiden cedera aktif</td></tr>".to_string() } else { incident_rows }
    )
}

fn html_marshalling(state: &MultiRoleAppState) -> String {
    let mut rows = String::new();
    for q in &state.marsh_engine.daftar_antrean {
        let status_timbang = if q.status_timbangan {
            "<span style='color:#10b981; font-weight:bold;'>PASSED (HIJAU)</span>"
        } else {
            "<span style='color:#ef4444; font-weight:bold;'>OVERWEIGHT/UNDERWEIGHT (MERAH)</span>"
        };
        rows.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:8px;'>Partai #{}</td>
                <td>Gelanggang {}</td>
                <td style='text-align:left; font-weight:bold;'>{} ({})</td>
                <td>{}</td>
                <td><b>{:.2} kg</b> ({:.1}-{:.1} kg)</td>
                <td>{}</td>
                <td>{:?}</td>
            </tr>",
            q.nomor_partai, q.gelanggang_id, q.nama_atlet, q.kontingen, q.sudut,
            q.berat_badan_kg, q.batas_min_kg, q.batas_max_kg, status_timbang, q.status
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Marshalling & Timbangan BLE — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 8px; border-bottom: 2px solid #334155; }}
    </style>
</head>
<body>
    <h2>⚖️ CALL ROOM MARSHALLING & INTEGRASI TIMBANGAN BLUETOOTH BLE</h2>
    {}

    <div class="card">
        <h3>📡 Status Timbangan Digital W3C Bluetooth API (Toleransi 0.00 kg)</h3>
        <p style="color:#94a3b8; font-size:0.85rem;">Sistem membaca nilai berat badan langsung dari timbangan digital tanpa input manual keyboard.</p>
        <table>
            <thead>
                <tr><th>Partai</th><th>Gelanggang</th><th>Nama Pesilat</th><th>Sudut</th><th>Berat Aktual vs Batas</th><th>Status Uji Timbang</th><th>Status Call Room</th></tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
        render_navbar("marshalling"),
        rows
    )
}

fn html_kontingen(state: &MultiRoleAppState) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Portal Manager Kontingen & Pelatih — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        .grid {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }}
        .btn {{ padding: 10px 16px; border-radius: 6px; font-weight: bold; border: none; cursor: pointer; background: #2563eb; color: white; }}
    </style>
</head>
<body>
    <h2>🥋 PORTAL MANAGER KONTINGEN & COACH</h2>
    {}

    <div class="card">
        <h3>📱 Jadwal Laga Atlet Anda Real-Time & Kartu Replay Reviu (VAR)</h3>
        <p style="color:#94a3b8; font-size:0.85rem;">Pelatih dibekali 2 kartu kuota Video Review per pertandingan.</p>
        <div class="grid">
            <div style="background:#0f172a; padding:12px; border-radius:6px; border:1px solid #334155;">
                <div style="font-weight:bold; color:#ef4444;">🔴 Eko Prasetyo</div>
                <div style="font-size:0.85rem; color:#94a3b8;">Kelas E Dewasa Putra</div>
                <div style="margin-top:8px; font-size:0.9rem;">Status: <b>Sedang Bertanding (Final)</b></div>
            </div>
            <div style="background:#0f172a; padding:12px; border-radius:6px; border:1px solid #334155;">
                <div style="font-weight:bold; color:#38bdf8;">🔵 Hendra Gunawan</div>
                <div style="font-size:0.85rem; color:#94a3b8;">Kelas E Dewasa Putra</div>
                <div style="margin-top:8px; font-size:0.9rem;">Status: <b>Juara 3 Bersama (Perunggu)</b></div>
            </div>
            <div style="background:#0f172a; padding:12px; border-radius:6px; border:1px solid #334155;">
                <div style="font-weight:bold; color:#10b981;">📄 Unduh Piagam Prestasi</div>
                <div style="font-size:0.85rem; color:#94a3b8;">Format Resmi Puspresnas / Kemendikdasmen</div>
                <div style="margin-top:8px;"><a href="/laporan" class="btn" style="text-decoration:none; display:inline-block; font-size:0.8rem;">Unduh E-Sertifikat</a></div>
            </div>
        </div>
    </div>
</body>
</html>"#,
        render_navbar("kontingen")
    )
}

fn html_superadmin() -> String {
    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Super Admin Platform — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        .grid {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }}
        .stat-card {{ background: #0f172a; padding: 12px; border-radius: 6px; border: 1px solid #334155; text-align: center; }}
        .stat-val {{ font-size: 1.6rem; font-weight: bold; color: #38bdf8; margin-top: 5px; }}
    </style>
</head>
<body>
    <h2>👑 SUPER ADMIN PLATFORM GLOBAL (MULTI-TENANT TELEMETRY)</h2>
    {}

    <div class="grid">
        <div class="stat-card"><div>Total Event Aktif</div><div class="stat-val">12 Event</div></div>
        <div class="stat-card"><div>Total Pesilat Terdaftar</div><div class="stat-val">4.820 Atlet</div></div>
        <div class="stat-card"><div>Throughput WebSocket</div><div class="stat-val" style="color:#10b981;">0.4 ms</div></div>
        <div class="stat-card"><div>Platform Revenue (5%)</div><div class="stat-val" style="color:#f59e0b;">Rp 24.100.000</div></div>
    </div>

    <div class="card" style="margin-top:15px;">
        <h3>🖥️ Sovereign Node Status (Widya-Lang Kernel)</h3>
        <p style="color:#94a3b8; font-size:0.85rem;">Zero Firebase, Zero Node.js, Zero Flutter — Pure Rust & Widya-Lang Ecosystem Core.</p>
        <div style="background:#090d16; padding:10px; border-radius:6px; font-family:monospace; font-size:0.85rem; color:#10b981;">
            [OK] WidyaOS Virtual Memory & Page Scheduler: ACTIVE<br>
            [OK] WidyaBase CDC Streamer & MVCC Vector Index: SYNCED<br>
            [OK] WidyaWeb Pure HTTP/WS Engine: LISTENING (Port 8088)<br>
            [OK] WidyaMobile Ultra-Low Latency Framebuffer: READY
        </div>
    </div>
</body>
</html>"#,
        render_navbar("superadmin")
    )
}

fn html_pendaftaran(state: &MultiRoleAppState) -> String {
    let rekap = state.reg_engine.hitung_rekapitulasi();

    let mut rows_kontingen = String::new();
    for (_, k) in &state.reg_engine.daftar_kontingen {
        let status_bayar = if k.status_pembayaran {
            "<span style='color:#10b981; font-weight:bold;'>LUNAS</span>"
        } else {
            "<span style='color:#f59e0b; font-weight:bold;'>BELUM LUNAS</span>"
        };
        rows_kontingen.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:8px;'>{}</td>
                <td style='text-align:left; font-weight:bold;'>{}</td>
                <td>{}</td>
                <td>{} ({})</td>
                <td>Rp {:?}</td>
                <td>{}</td>
            </tr>",
            k.id, k.nama, k.asal_daerah, k.nama_official, k.no_telepon, k.total_tagihan_idr, status_bayar
        ));
    }

    let mut rows_atlet = String::new();
    for (_, a) in &state.reg_engine.daftar_atlet {
        let komp = match &a.kompetisi {
            JenisKompetisi::Tanding(k) => format!("Tanding ({})", k),
            JenisKompetisi::SeniTunggal => "Seni Tunggal".to_string(),
            JenisKompetisi::SeniGanda => "Seni Ganda".to_string(),
            JenisKompetisi::SeniRegu => "Seni Regu".to_string(),
            JenisKompetisi::SoloKreatif => "Solo Kreatif".to_string(),
        };
        let status_verif = if a.terverifikasi {
            "<span style='color:#10b981; font-weight:bold;'>TERVERIFIKASI</span>"
        } else {
            "<span style='color:#f59e0b; font-weight:bold;'>MENUNGGU</span>"
        };
        rows_atlet.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:8px;'>{}</td>
                <td style='text-align:left; font-weight:bold;'>{}</td>
                <td>{}</td>
                <td>{:?}</td>
                <td>{} kg</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            a.id, a.nama_lengkap, a.kontingen_id, a.jenis_kelamin, a.berat_badan_kg, a.kategori_usia.label(), komp, status_verif
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Pendaftaran Kontingen & Atlet — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        .grid-stats {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 15px; }}
        .stat-card {{ background: #0f172a; padding: 12px; border-radius: 6px; border: 1px solid #334155; text-align: center; }}
        .stat-val {{ font-size: 1.6rem; font-weight: bold; color: #38bdf8; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 8px; border-bottom: 2px solid #334155; }}
    </style>
</head>
<body>
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <h2>📝 MANAJEMEN PENDAFTARAN & KONTINGEN (EO CONSOLE)</h2>
        <div>Event: <b>{}</b></div>
    </div>
    {}

    <div class="grid-stats">
        <div class="stat-card"><div>Total Kontingen</div><div class="stat-val">{}</div></div>
        <div class="stat-card"><div>Total Atlet</div><div class="stat-val">{}</div></div>
        <div class="stat-card"><div>Terverifikasi</div><div class="stat-val">{}</div></div>
        <div class="stat-card"><div>Total Pemasukan</div><div class="stat-val" style="color:#10b981;">Rp {:?}</div></div>
    </div>

    <div class="card">
        <h3>🏛️ DAFTAR KONTINGEN TERDAFTAR</h3>
        <table>
            <thead>
                <tr><th>ID</th><th>Nama Kontingen</th><th>Daerah</th><th>Official</th><th>Total Tagihan</th><th>Status Bayar</th></tr>
            </thead>
            <tbody>{}</tbody>
        </table>
    </div>

    <div class="card">
        <h3>🥋 DAFTAR ATLET & STATUS VERIFIKASI</h3>
        <table>
            <thead>
                <tr><th>ID</th><th>Nama Atlet</th><th>Kontingen</th><th>Gender</th><th>Berat</th><th>Kategori Usia</th><th>Kompetisi</th><th>Status</th></tr>
            </thead>
            <tbody>{}</tbody>
        </table>
    </div>
</body>
</html>"#,
        state.reg_engine.event_nama,
        render_navbar("pendaftaran"),
        rekap.total_kontingen, rekap.total_atlet, rekap.terverifikasi, rekap.total_pendapatan_idr,
        rows_kontingen,
        rows_atlet
    )
}

fn html_kategori(state: &MultiRoleAppState) -> String {
    let atlet_vec: Vec<AtletRegistrasi> = state.reg_engine.daftar_atlet.values().cloned().collect();
    let grouped = CategoryDrawingEngine::kelompokkan_atlet(&atlet_vec);

    let mut kategori_cards = String::new();
    for (kategori_nama, peserta_list) in grouped {
        let (pool_a, pool_b) = CategoryDrawingEngine::lakukan_drawing_kategori(&peserta_list);
        let pool_a_names: Vec<String> = pool_a.iter().map(|p| format!("<li>🔴 {} ({})</li>", p.nama, p.kontingen)).collect();
        let pool_b_names: Vec<String> = pool_b.iter().map(|p| format!("<li>🔵 {} ({})</li>", p.nama, p.kontingen)).collect();

        kategori_cards.push_str(&format!(
            "<div style='background:#1e293b; padding:12px; border-radius:6px; border:1px solid #334155; margin-bottom:12px;'>
                <div style='font-weight:bold; font-size:1rem; color:#f59e0b; border-bottom:1px solid #334155; padding-bottom:6px; margin-bottom:8px;'>
                    🏷️ {} (Total: {} Peserta)
                </div>
                <div style='display:grid; grid-template-columns:1fr 1fr; gap:12px;'>
                    <div style='background:#0f172a; padding:8px; border-radius:4px;'>
                        <h4 style='margin:0 0 4px 0; color:#ef4444;'>POOL ATAS (Pool A):</h4>
                        <ul style='margin:0; padding-left:16px; font-size:0.88rem;'>{}</ul>
                    </div>
                    <div style='background:#0f172a; padding:8px; border-radius:4px;'>
                        <h4 style='margin:0 0 4px 0; color:#3b82f6;'>POOL BAWAH (Pool B):</h4>
                        <ul style='margin:0; padding-left:16px; font-size:0.88rem;'>{}</ul>
                    </div>
                </div>
            </div>",
            kategori_nama, peserta_list.len(), pool_a_names.join(""), pool_b_names.join("")
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Pembagian Kategori & Drawing — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
    </style>
</head>
<body>
    <h2>🎯 PEMBAGIAN KELAS & DRAWING ADIL (FAIR CONTINGENT SEPARATION)</h2>
    {}
    <div class="card">
        <h3>⚖️ Toleransi Timbangan Berat Badan Resmi: <b>0.00 kg</b></h3>
        <p style="color:#94a3b8; font-size:0.85rem;">Algoritma drawing otomatis memisahkan atlet satu perguruan/kontingen ke pool atas & pool bawah.</p>
        {}
    </div>
</body>
</html>"#,
        render_navbar("kategori"),
        kategori_cards
    )
}

fn html_laporan(state: &MultiRoleAppState) -> String {
    let medali_sorted = ReportGenerator::urutkan_klasemen_medali(state.medali_kontingen.clone());
    let mut medali_rows = String::new();

    for (rank, m) in medali_sorted.iter().enumerate() {
        let badge = match rank {
            0 => "🥇 JUARA 1",
            1 => "🥈 JUARA 2",
            2 => "🥉 JUARA 3",
            _ => "-",
        };
        medali_rows.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:8px; font-weight:bold;'>{}</td>
                <td style='text-align:left; font-weight:bold;'>{}</td>
                <td style='color:#f59e0b; font-weight:bold;'>{}</td>
                <td style='color:#94a3b8; font-weight:bold;'>{}</td>
                <td style='color:#b45309; font-weight:bold;'>{}</td>
                <td style='font-size:1.05rem; font-weight:bold; color:#38bdf8;'>{}</td>
                <td style='font-weight:bold;'>{}</td>
            </tr>",
            rank + 1, m.nama_kontingen, m.emas, m.perak, m.perunggu, m.total_poin_klasemen, badge
        ));
    }

    let pesilat_terbaik = ReportGenerator::tentukan_pesilat_terbaik(state.nominasi_terbaik.clone());
    let terbaik_html = if let Some(p) = pesilat_terbaik {
        format!(
            "<div style='background:linear-gradient(135deg, #78350f, #b45309); padding:15px; border-radius:8px; border:2px solid #f59e0b; margin-bottom:15px;'>
                <h3 style='margin:0 0 8px 0; color:#fde68a;'>🏆 PESILAT TERBAIK TURNAMEN (BEST ATHLETE AWARD)</h3>
                <div style='font-size:1.4rem; font-weight:bold;'>{}</div>
                <div style='font-size:1rem; opacity:0.9;'>{} — {}</div>
                <div style='display:flex; gap:15px; margin-top:8px; font-size:0.88rem;'>
                    <div>🥋 Total Jatuhan: <b>{}</b></div>
                    <div>⚡ Total Tendangan: <b>{}</b></div>
                    <div>⭐ Indeks Performa: <b>{:.1}</b></div>
                </div>
            </div>",
            p.nama_atlet, p.kontingen, p.kategori, p.total_jatuhan, p.total_tendangan, p.skor_indeks_kemenangan
        )
    } else {
        "".to_string()
    };

    let neraca = ReportGenerator::hitung_neraca_keuangan(
        500_000, 2_000_000, 800_000, 300_000, 500_000
    );

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Laporan & Rekapitulasi Medali — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 8px; padding: 15px; margin-bottom: 15px; border: 1px solid #334155; }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 8px; border-bottom: 2px solid #334155; }}
    </style>
</head>
<body>
    <h2>📊 LAPORAN RESMI, KLASEMEN MEDALI & NERACA EVENT</h2>
    {}

    {}

    <div class="card">
        <h3>🏅 KLASEMEN PEROLEHAN MEDALI (JUARA UMUM)</h3>
        <table>
            <thead>
                <tr><th>Peringkat</th><th>Nama Kontingen</th><th>🥇 Emas</th><th>🥈 Perak</th><th>🥉 Perunggu</th><th>Total Poin</th><th>Status</th></tr>
            </thead>
            <tbody>{}</tbody>
        </table>
    </div>

    <div class="card">
        <h3>💰 REKAPITULASI KEUANGAN & NERACA ARUS KAS (SETTLEMENT)</h3>
        <div style="display:grid; grid-template-columns:repeat(3, 1fr); gap:12px;">
            <div style="background:#0f172a; padding:10px; border-radius:6px;">
                <div style="color:#94a3b8; font-size:0.82rem;">Total Pemasukan Kotor</div>
                <div style="font-size:1.3rem; font-weight:bold; color:#10b981;">Rp {:?}</div>
            </div>
            <div style="background:#0f172a; padding:10px; border-radius:6px;">
                <div style="color:#94a3b8; font-size:0.82rem;">Total Beban Operasional</div>
                <div style="font-size:1.3rem; font-weight:bold; color:#ef4444;">Rp {:?}</div>
            </div>
            <div style="background:#0f172a; padding:10px; border-radius:6px;">
                <div style="color:#94a3b8; font-size:0.82rem;">Laba Bersih Penyelenggara</div>
                <div style="font-size:1.3rem; font-weight:bold; color:#38bdf8;">Rp {:?}</div>
            </div>
        </div>
    </div>
</body>
</html>"#,
        render_navbar("laporan"),
        terbaik_html,
        medali_rows,
        neraca.total_pemasukan_kotor_idr,
        neraca.honorarium_wasit_juri_idr + neraca.biaya_tim_medis_idr + neraca.sewa_gelanggang_sarana_idr + neraca.platform_fee_saas_idr,
        neraca.laba_bersih_penyelenggara_idr
    )
}

fn html_gelanggang(state: &MultiRoleAppState) -> String {
    let mut court_cards = String::new();
    let courts = state.court_manager.ringkasan_semua_gelanggang();

    for g in courts {
        let kp_nama = g.roster_tugas.ketua_pertandingan.as_ref().map(|p| p.nama_lengkap.as_str()).unwrap_or("Belum Ditugaskan");
        let wasit_nama = g.roster_tugas.wasit_matras.as_ref().map(|p| p.nama_lengkap.as_str()).unwrap_or("Belum Ditugaskan");
        let medis_nama = g.roster_tugas.tim_medis.as_ref().map(|p| p.nama_lengkap.as_str()).unwrap_or("Siaga");
        let juri_count = g.roster_tugas.juri_list.len();

        let status_badge = match g.status {
            StatusGelanggang::Siap => "<span style='color:#10b981; font-weight:bold;'>SIAP (READY)</span>",
            StatusGelanggang::SedangBertanding => "<span style='color:#38bdf8; font-weight:bold;'>SEDANG BERTANDING</span>",
            StatusGelanggang::IstirahatShift => "<span style='color:#f59e0b; font-weight:bold;'>ISTIRAHAT SHIFT</span>",
            StatusGelanggang::PembersihanMatras => "<span style='color:#a855f7; font-weight:bold;'>PEMBERSIHAN MATRAS</span>",
            StatusGelanggang::Selesai => "<span style='color:#ef4444; font-weight:bold;'>SELESAI</span>",
        };

        court_cards.push_str(&format!(
            "<div style='background:#1e293b; border-radius:8px; padding:15px; border:1px solid #334155; display:flex; flex-direction:column; justify-content:space-between;'>
                <div>
                    <div style='display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid #334155; padding-bottom:8px; margin-bottom:10px;'>
                        <h3 style='margin:0; color:#f59e0b; font-size:1.2rem;'>🏟️ {}</h3>
                        <div style='font-size:0.82rem;'>{}</div>
                    </div>
                    <div style='font-size:0.85rem; color:#94a3b8; margin-bottom:10px;'>Lokasi: <b>{}</b></div>
                    
                    <div style='background:#0f172a; padding:10px; border-radius:6px; font-size:0.85rem; margin-bottom:10px;'>
                        <div style='font-weight:bold; color:#60a5fa; margin-bottom:6px;'>👥 Roster Personel & Wasit Juri:</div>
                        <div>• KP: <b>{}</b></div>
                        <div>• Wasit Matras: <b>{}</b></div>
                        <div>• Tim Juri: <b>{} Juri Aktif</b></div>
                        <div>• Tim Medis: <b>{}</b></div>
                        <div style='color:#f59e0b; margin-top:4px;'>• Shift: {}</div>
                    </div>
                </div>

                <div style='display:flex; gap:8px; margin-top:10px;'>
                    <a href='/scoreboard' style='flex:1; text-align:center; background:#2563eb; color:white; text-decoration:none; padding:8px; border-radius:6px; font-size:0.82rem; font-weight:bold;'>📺 Buka TV Arena</a>
                    <a href='{}' target='_blank' style='background:#0f172a; border:1px solid #334155; color:#38bdf8; text-decoration:none; padding:8px 12px; border-radius:6px; font-size:0.82rem;'>OBS Overlay</a>
                </div>
            </div>",
            g.kode_gelanggang, status_badge, g.lokasi_matras,
            kp_nama, wasit_nama, juri_count, medis_nama,
            if g.roster_tugas.shift_nama.is_empty() { "Shift Pagi".to_string() } else { g.roster_tugas.shift_nama },
            g.url_stream_obs
        ));
    }

    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Manajemen Multi-Gelanggang & Personel — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .grid {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 15px; margin-top: 15px; }}
        .card-top {{ background: #1e293b; padding: 15px; border-radius: 8px; border: 1px solid #334155; margin-bottom: 15px; display: flex; justify-content: space-between; align-items: center; }}
    </style>
</head>
<body>
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <h2>🏟️ MANAJEMEN MULTI-GELANGGANG PARALEL & ROSTER PERSONEL WASIT JURI</h2>
        <div>Event: <b>{}</b></div>
    </div>
    {}

    <div class="card-top">
        <div>
            <h3 style="margin:0 0 4px 0;">Pengaturan Gelanggang Aktif (Kapasitas: N-Gelanggang Paralel)</h3>
            <p style="margin:0; color:#94a3b8; font-size:0.85rem;">Mendukung kejuaraan skala besar dengan 2 hingga 8+ gelanggang berjalan serentak.</p>
        </div>
        <div style="display:flex; gap:10px;">
            <div style="background:#0f172a; padding:8px 16px; border-radius:6px; border:1px solid #334155; text-align:center;">
                <div style="font-size:0.75rem; color:#94a3b8;">TOTAL GELANGGANG</div>
                <div style="font-size:1.3rem; font-weight:bold; color:#38bdf8;">{} Arena</div>
            </div>
            <div style="background:#0f172a; padding:8px 16px; border-radius:6px; border:1px solid #334155; text-align:center;">
                <div style="font-size:0.75rem; color:#94a3b8;">TOTAL WASIT JURI</div>
                <div style="font-size:1.3rem; font-weight:bold; color:#10b981;">{} Personel</div>
            </div>
        </div>
    </div>

    <div class="grid">
        {}
    </div>
</body>
</html>"#,
        state.court_manager.nama_event,
        render_navbar("gelanggang"),
        state.court_manager.daftar_gelanggang.len(),
        state.court_manager.database_personel.len(),
        court_cards
    )
}

fn html_camera(state: &MultiRoleAppState) -> String {
    let actual_kbps = state.cam_manager.hitung_total_bandwidth_upload_kbps();
    let saving_pct = state.cam_manager.hitung_persentase_penghematan();
    let strat_label = match state.cam_manager.active_strategy {
        BandwidthStrategy::LocalEdgeRingBufferOnly => "📼 Local Edge Ring Buffer (100% Offline VAR / 0 Mbps Upload)",
        BandwidthStrategy::MosaicQuadView => "🖼️ Mosaic Quad-View (4-in-1 Matrix Single Stream)",
        BandwidthStrategy::AdaptiveBitrate => "🎛️ Dynamic Adaptive Bitrate (ABR 720p/1080p)",
        BandwidthStrategy::DirectMultiChannel => "⚡ Direct Multi-Channel (Full HD Uncompressed)",
    };

    let strat_badge = match state.cam_manager.active_strategy {
        BandwidthStrategy::LocalEdgeRingBufferOnly => "background:#059669; color:white;",
        BandwidthStrategy::MosaicQuadView => "background:#2563eb; color:white;",
        BandwidthStrategy::AdaptiveBitrate => "background:#d97706; color:white;",
        BandwidthStrategy::DirectMultiChannel => "background:#dc2626; color:white;",
    };

    let cameras_g1 = state.cam_manager.ambil_kamera_gelanggang(1);
    let mut cam_cards = String::new();
    for cam in cameras_g1 {
        let (color_border, badge_color) = match cam.angle {
            CameraAngle::Sudut1DepanKiri => ("#ef4444", "#991b1b"),
            CameraAngle::Sudut2DepanKanan => ("#3b82f6", "#1e40af"),
            CameraAngle::Sudut3BelakangKiri => ("#10b981", "#065f46"),
            CameraAngle::Sudut4BelakangKanan => ("#f59e0b", "#92400e"),
            CameraAngle::KameraUtamaOverview => ("#8b5cf6", "#5b21b6"),
        };

        cam_cards.push_str(&format!(
            r#"<div style="background:#0f172a; border:2px solid {}; border-radius:12px; overflow:hidden; display:flex; flex-direction:column;">
                <div style="background:{}; padding:8px 12px; display:flex; justify-content:space-between; align-items:center; font-size:0.85rem; font-weight:bold;">
                    <span>📹 {}</span>
                    <span style="background:rgba(0,0,0,0.4); padding:2px 8px; border-radius:12px; font-size:0.75rem;">LIVE 60 FPS</span>
                </div>
                <div style="height:190px; background:radial-gradient(circle, #1e293b 20%, #090d16 100%); display:flex; flex-direction:column; align-items:center; justify-content:center; position:relative;">
                    <div style="font-size:3rem; opacity:0.8;">🥋</div>
                    <div style="font-size:0.82rem; color:#94a3b8; margin-top:6px;">Matras Gelanggang 1 — {}</div>
                    <div style="position:absolute; bottom:8px; left:10px; font-size:0.72rem; background:rgba(0,0,0,0.7); padding:2px 8px; border-radius:4px; font-family:monospace; color:#38bdf8;">
                        Bitrate: {} Kbps | Latensi: ~45ms | Buffer: 60s
                    </div>
                </div>
                <div style="padding:10px 14px; background:#1e293b; font-size:0.8rem; display:flex; justify-content:space-between; align-items:center; border-top:1px solid #334155;">
                    <div>
                        <span style="color:#94a3b8;">Format:</span> 1080p @ {}fps
                    </div>
                    <a href="{}" target="_blank" style="color:#38bdf8; text-decoration:none; font-weight:600;">📱 Buka WebRTC Stream ➔</a>
                </div>
            </div>"#,
            color_border, badge_color, cam.angle.label(), cam.angle.code(), cam.bitrate_kbps, cam.fps, cam.webrtc_ingest_url
        ));
    }

    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Multi-Camera & VAR Control — Arena Silat SaaS</title>
    <style>
        :root {{ --bg-dark: #090d16; --card-dark: #1e293b; --gold: #f59e0b; --primary: #2563eb; }}
        body {{ margin: 0; padding: 15px; font-family: 'Segoe UI', system-ui, sans-serif; background: var(--bg-dark); color: white; }}
        .card {{ background: var(--card-dark); border-radius: 12px; padding: 20px; border: 1px solid #334155; margin-bottom: 20px; }}
        .telemetry-grid {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 15px; margin-bottom: 20px; }}
        .stat-box {{ background: #0f172a; border: 1px solid #334155; border-radius: 10px; padding: 15px; text-align: center; }}
        .stat-val {{ font-size: 1.8rem; font-weight: 800; margin-top: 4px; }}
        .btn-strat {{ padding: 10px 16px; border-radius: 8px; border: 1px solid #334155; background: #0f172a; color: #cbd5e1; font-weight: 600; cursor: pointer; transition: all 0.2s; font-size: 0.85rem; }}
        .btn-strat:hover {{ background: #2563eb; color: white; border-color: #60a5fa; transform: translateY(-1px); }}
        .btn-strat.active {{ background: linear-gradient(135deg, #2563eb, #1d4ed8); color: white; border-color: #60a5fa; }}
        .camera-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }}
        .speed-btn {{ padding: 6px 14px; border-radius: 6px; border: 1px solid #334155; background: #0f172a; color: white; font-weight: bold; cursor: pointer; }}
        .speed-btn:hover {{ background: #f59e0b; color: black; }}
    </style>
    <script>
        function setStrategy(strat) {{
            fetch('/api/camera/strategy', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ strategy: strat }})
            }}).then(r => r.json()).then(d => {{
                alert('✅ Mode Bandwidth Berhasil Diubah ke: ' + strat + '\\nPenghematan Sekarang: ' + d.saving_percentage + '%');
                location.reload();
            }});
        }}

        function triggerVarDecision(decision) {{
            fetch('/api/camera/var_clip', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ court_id: 1, duration: 30, decision: decision }})
            }}).then(r => r.json()).then(d => {{
                alert('🏛️ [VAR PUTUSAN RESMI]: ' + decision + '\\nStatus: Diterapkan ke Papan Skor Gelanggang 1!');
            }});
        }}
    </script>
</head>
<body>
    {}

    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:15px;">
        <div>
            <h2 style="margin:0; font-size:1.6rem; display:flex; align-items:center; gap:8px;">
                📷 MULTI-CAMERA VAR & BANDWIDTH OPTIMIZATION CENTER
            </h2>
            <p style="margin:4px 0 0 0; color:#94a3b8; font-size:0.85rem;">Sinkronisasi 4-Sudut Kamera Matras 10m x 10m, Local Edge Ring Buffer & Live Streaming</p>
        </div>
        <div>
            <span style="padding:6px 14px; border-radius:20px; font-size:0.85rem; font-weight:bold; {}">
                {}
            </span>
        </div>
    </div>

    <!-- Telemetry Cards -->
    <div class="telemetry-grid">
        <div class="stat-box">
            <div style="font-size:0.8rem; color:#94a3b8;">UPLOAD BANDWIDTH VENUE</div>
            <div class="stat-val" style="color:#38bdf8;">{} <span style="font-size:1rem;">Kbps</span></div>
            <div style="font-size:0.75rem; color:#10b981; margin-top:4px;">(Total 6 Gelanggang Aktif)</div>
        </div>
        <div class="stat-box">
            <div style="font-size:0.8rem; color:#94a3b8;">EFISIENSI PENGHEMATAN</div>
            <div class="stat-val" style="color:#10b981;">{:.1}%</div>
            <div style="font-size:0.75rem; color:#94a3b8; margin-top:4px;">Dibandingkan Raw Cloud Upload</div>
        </div>
        <div class="stat-box">
            <div style="font-size:0.8rem; color:#94a3b8;">LOCAL EDGE SERVER IP</div>
            <div class="stat-val" style="color:#f59e0b; font-size:1.3rem;">{}</div>
            <div style="font-size:0.75rem; color:#94a3b8; margin-top:4px;">0 Mbps Internet for VAR Replay</div>
        </div>
        <div class="stat-box">
            <div style="font-size:0.8rem; color:#94a3b8;">TOTAL KAMERA TERHUBUNG</div>
            <div class="stat-val" style="color:#a855f7;">{} <span style="font-size:1rem;">Kamera</span></div>
            <div style="font-size:0.75rem; color:#94a3b8; margin-top:4px;">4 Sudut x 6 Gelanggang</div>
        </div>
    </div>

    <!-- Bandwidth Strategy Controls -->
    <div class="card">
        <div style="font-weight:bold; font-size:1rem; margin-bottom:12px; display:flex; align-items:center; gap:8px;">
            🎛️ PILIH STRATEGI PENGHEMATAN BANDWIDTH VENUE:
        </div>
        <div style="display:flex; gap:10px; flex-wrap:wrap;">
            <button class="btn-strat" onclick="setStrategy('LocalEdgeRingBufferOnly')">
                📼 1. Local Edge Buffer (Hemat 100% Upload VAR — Replay di LAN)
            </button>
            <button class="btn-strat" onclick="setStrategy('MosaicQuadView')">
                🖼️ 2. Mosaic Quad-View (Gabung 4 Gelanggang Jadi 1 Stream)
            </button>
            <button class="btn-strat" onclick="setStrategy('AdaptiveBitrate')">
                🎛️ 3. Dynamic Adaptive Bitrate (Auto-Scale 720p / 1080p)
            </button>
            <button class="btn-strat" onclick="setStrategy('DirectMultiChannel')">
                ⚡ 4. Direct Multi-Channel (Full HD Dedicated per Gelanggang)
            </button>
        </div>
    </div>

    <!-- 4-Angle Live Camera Matrix -->
    <div class="card">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:15px;">
            <div style="font-weight:bold; font-size:1.1rem; color:#f59e0b;">
                🏟️ GELANGGANG 1 (MATRAS A) — 4-ANGLE LIVE VAR MATRIX
            </div>
            <div style="display:flex; gap:6px;">
                <button class="speed-btn" style="background:#2563eb;">Gelanggang A</button>
                <button class="speed-btn">Gelanggang B</button>
                <button class="speed-btn">Gelanggang C</button>
                <button class="speed-btn">Gelanggang D</button>
                <button class="speed-btn">Gelanggang E</button>
                <button class="speed-btn">Gelanggang F</button>
            </div>
        </div>

        <div class="camera-grid">
            {}
        </div>
    </div>

    <!-- Instant Replay Slow-Mo Panel -->
    <div class="card" style="border:2px solid #f59e0b;">
        <div style="font-weight:bold; font-size:1.1rem; color:#f59e0b; margin-bottom:12px; display:flex; align-items:center; gap:8px;">
            ⚖️ KONSOL INSTANT REPLAY & SLOW MOTION KETUA PERTANDINGAN (KP)
        </div>
        <div style="display:grid; grid-template-columns: 2fr 1fr; gap:16px;">
            <div style="background:#0f172a; padding:15px; border-radius:8px; border:1px solid #334155;">
                <div style="display:flex; justify-content:space-between; margin-bottom:10px;">
                    <span>⏱️ Ring Buffer: <b>60 Detik Terakhir (Frame Sync 60fps)</b></span>
                    <span style="color:#10b981; font-weight:bold;">🟢 BUFFER LOCKED (READY)</span>
                </div>
                <div style="display:flex; gap:10px; align-items:center; margin-bottom:12px;">
                    <button class="speed-btn">⏪ -5s</button>
                    <button class="speed-btn">◀ Frame</button>
                    <button class="speed-btn" style="background:#ef4444;">⏸️ FREEZE</button>
                    <button class="speed-btn">Frame ▶</button>
                    <button class="speed-btn">⏩ +5s</button>
                    <div style="margin-left:auto; display:flex; gap:6px;">
                        <button class="speed-btn" style="background:#2563eb;">1.0x</button>
                        <button class="speed-btn" style="background:#f59e0b; color:black;">0.5x Slow-Mo</button>
                        <button class="speed-btn" style="background:#a855f7;">0.25x Super Slow</button>
                    </div>
                </div>
                <input type="range" min="0" max="60" value="45" style="width:100%; accent-color:#f59e0b;">
            </div>
            <div style="background:#0f172a; padding:15px; border-radius:8px; border:1px solid #334155; display:flex; flex-direction:column; gap:8px; justify-content:center;">
                <div style="font-size:0.85rem; font-weight:bold; color:#94a3b8; text-align:center;">EKSEKUSI PUTUSAN VIDEO REVIEW:</div>
                <button onclick="triggerVarDecision('POIN JATUHAN SAH (+3)')" style="background:#10b981; color:white; border:none; padding:10px; border-radius:6px; font-weight:bold; cursor:pointer;">
                    ✅ POIN JATUHAN SAH (+3)
                </button>
                <button onclick="triggerVarDecision('PELANGGARAN TERBUKTI')" style="background:#f59e0b; color:black; border:none; padding:10px; border-radius:6px; font-weight:bold; cursor:pointer;">
                    ⚠️ PELANGGARAN TERBUKTI (HUKUMAN)
                </button>
                <button onclick="triggerVarDecision('PROTES DITOLAK (KARTU HANGUS)')" style="background:#ef4444; color:white; border:none; padding:10px; border-radius:6px; font-weight:bold; cursor:pointer;">
                    ❌ PROTES DITOLAK (KARTU HANGUS)
                </button>
            </div>
        </div>
    </div>
</body>
</html>"###,
        render_navbar("camera"),
        strat_badge, strat_label,
        actual_kbps,
        saving_pct,
        state.cam_manager.local_edge_server_ip,
        state.cam_manager.cameras.len(),
        cam_cards
    )
}

fn html_federasi(state: &MultiRoleAppState) -> String {
    let wasit_list = vec![
        ("WJ-NAS-01", "Drs. H. Mulyono", "Wasit Juri Nasional 1", "Tapak Suci", "Jateng", "2028", "142"),
        ("WJ-NAS-02", "K.H. Ahmad Dahlan", "Wasit Juri Nasional 1", "PSHT", "Jatim", "2027", "118"),
        ("WJ-NAS-03", "Drs. Cecep Hidayat", "Wasit Juri Nasional 1", "Merpati Putih", "Jabar", "2028", "95"),
        ("WJ-NAS-07", "Ahmad Fauzi", "Wasit Matras Nasional", "Pagar Nusa", "Jatim", "2029", "87"),
        ("WJ-DAERAH-01", "Budi Santoso", "Wasit Juri Daerah Kelas 2", "Perisai Diri", "Jateng", "2026", "44"),
    ];

    let mut wasit_rows = String::new();
    for w in wasit_list {
        wasit_rows.push_str(&format!(
            "<tr style='border-bottom:1px solid #334155; text-align:center;'>
                <td style='padding:10px; font-family:monospace; color:#38bdf8;'>{}</td>
                <td style='text-align:left; font-weight:bold;'>{}</td>
                <td><span style='background:#1e3a8a; color:#93c5fd; padding:3px 8px; border-radius:4px; font-size:0.8rem;'>{}</span></td>
                <td>{}</td>
                <td>{}</td>
                <td style='color:#10b981; font-weight:bold;'>{}</td>
                <td style='color:#f59e0b; font-weight:bold;'>{} Partai</td>
                <td><span style='background:#065f46; color:#6ee7b7; padding:2px 7px; border-radius:10px; font-size:0.75rem;'>AKTIF</span></td>
            </tr>",
            w.0, w.1, w.2, w.3, w.4, w.5, w.6
        ));
    }

    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Dashboard Federasi IPSI & Audit Kejuaraan — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 10px; padding: 18px; margin-bottom: 18px; border: 1px solid #334155; }}
        .grid-2 {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(360px, 1fr)); gap: 15px; }}
        .badge-ip {{ background: rgba(37,99,235,0.2); border: 1px solid #3b82f6; color: #93c5fd; padding: 4px 10px; border-radius: 6px; font-weight: bold; font-size: 0.85rem; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 10px; font-size: 0.88rem; }}
        th {{ background: #0f172a; padding: 10px; border-bottom: 2px solid #475569; }}
    </style>
</head>
<body>
    <h2>🏛️ DASHBOARD FEDERASI IPSI & AUDIT PERIZINAN KEJUARAAN</h2>
    {}
    <div class="grid-2">
        <div class="card">
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h3 style="margin:0; color:#38bdf8;">📜 Rekomendasi Resmi PB IPSI</h3>
                <span class="badge-ip">TERVALIDASI 100%</span>
            </div>
            <div style="margin-top:12px; font-size:0.9rem; line-height:1.6;">
                <div>Nomor Rekomendasi: <b style="color:#f59e0b;">REK-PB-IPSI/X/2026/089</b></div>
                <div>Instansi Penyelenggara: <b>Pengprov IPSI Jawa Tengah / PB IPSI</b></div>
                <div>Lokasi GOR: <b>GOR Jatidiri & Among Rogo (Standar Nasional)</b></div>
                <div>Total Gelanggang: <b style="color:#38bdf8;">6 Gelanggang Paralel Aktif</b></div>
                <div>Izin Kepolisian (Polda/Polres): <span style="color:#10b981; font-weight:bold;">[TERBIT] SI/YANMIN/2026/881</span></div>
                <div>MoU RSUD Rujukan: <span style="color:#10b981; font-weight:bold;">[AKTIF] RSUD Tugurejo & RS Kariadi</span></div>
            </div>
        </div>
        <div class="card">
            <h3 style="margin:0 0 10px 0; color:#10b981;">🛡️ Audit Kelayakan Sarana Gelanggang</h3>
            <div style="display:grid; grid-template-columns:1fr 1fr; gap:10px; font-size:0.85rem;">
                <div style="background:#0f172a; padding:10px; border-radius:6px; border-left:4px solid #10b981;">
                    <div>Matras Standar IPSI 10x10m</div>
                    <b style="color:#10b981;">✅ LOLOS UJI KETEBALAN (5cm)</b>
                </div>
                <div style="background:#0f172a; padding:10px; border-radius:6px; border-left:4px solid #10b981;">
                    <div>Timbangan Digital BLE</div>
                    <b style="color:#10b981;">✅ TERKALIBRASI (Toleransi 0.00kg)</b>
                </div>
                <div style="background:#0f172a; padding:10px; border-radius:6px; border-left:4px solid #10b981;">
                    <div>Tim Dokter Pertandingan (Sp.KO)</div>
                    <b style="color:#10b981;">✅ STANDBY KONSOL 3 MENIT</b>
                </div>
                <div style="background:#0f172a; padding:10px; border-radius:6px; border-left:4px solid #10b981;">
                    <div>Ambulans Gawat Darurat</div>
                    <b style="color:#10b981;">✅ 2 UNIT JALUR HIJAU SIAP</b>
                </div>
            </div>
            <div style="margin-top:10px; font-size:0.85rem; color:#94a3b8;">
                Skor Kepatuhan Sarana: <b style="color:#38bdf8; font-size:1.1rem;">100.0% (Sangat Layak)</b>
            </div>
        </div>
    </div>

    <div class="card">
        <h3 style="margin:0 0 10px 0; color:#f59e0b;">⚖️ Akreditasi & Lisensi Wasit Juri Terdaftar di Turnamen</h3>
        <table>
            <thead>
                <tr>
                    <th>No. Lisensi</th>
                    <th>Nama Lengkap</th>
                    <th>Tingkat Lisensi</th>
                    <th>Perguruan Asal</th>
                    <th>Pengprov</th>
                    <th>Masa Berlaku</th>
                    <th>Total Partai</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"###,
        render_navbar("federasi"),
        wasit_rows
    )
}

fn html_pembayaran(state: &MultiRoleAppState) -> String {
    let split = PaymentEngine::hitung_split_settlement("INV-KONT-2026-001", 14_000_000, 40);
    let qris_payload = PaymentEngine::generate_qris_payload("INV-KONT-2026-001", 14_000_000, "ARENA SILAT SAAS");

    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Pembayaran & Split Settlement — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 10px; padding: 18px; margin-bottom: 18px; border: 1px solid #334155; }}
        .grid-3 {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 15px; }}
        .qris-box {{ background: white; color: black; padding: 20px; border-radius: 12px; text-align: center; max-width: 320px; margin: 0 auto; }}
    </style>
</head>
<body>
    <h2>💳 GERBANG PEMBAYARAN & AUTOMATED SPLIT SETTLEMENT</h2>
    {}
    <div class="grid-3">
        <div class="card" style="border-top:4px solid #10b981;">
            <h3 style="margin:0; color:#34d399;">💰 Porsi EO / Panitia</h3>
            <div style="font-size:1.8rem; font-weight:800; margin:10px 0; color:#6ee7b7;">Rp {}</div>
            <div style="font-size:0.85rem; color:#94a3b8;">Net revenue penyelenggara turnamen setelah dipotong iuran federasi & platform fee.</div>
        </div>
        <div class="card" style="border-top:4px solid #3b82f6;">
            <h3 style="margin:0; color:#60a5fa;">🏛️ Iuran Kas PB IPSI / Pengprov</h3>
            <div style="font-size:1.8rem; font-weight:800; margin:10px 0; color:#93c5fd;">Rp {}</div>
            <div style="font-size:0.85rem; color:#94a3b8;">Rp 10.000 / atlet otomatis disetorkan ke kas pengembangan silat federasi.</div>
        </div>
        <div class="card" style="border-top:4px solid #f59e0b;">
            <h3 style="margin:0; color:#fbbf24;">⚡ Platform SaaS Fee</h3>
            <div style="font-size:1.8rem; font-weight:800; margin:10px 0; color:#fde68a;">Rp {}</div>
            <div style="font-size:0.85rem; color:#94a3b8;">Rp 15.000 / atlet untuk pemeliharaan server cloud, DVR VAR, & update software.</div>
        </div>
    </div>

    <div class="card">
        <div style="display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:15px;">
            <div style="max-width:550px;">
                <h3 style="margin:0 0 10px 0; color:#38bdf8;">⚖️ Deposit Jaminan Protes Dewan Hakim</h3>
                <div style="font-size:0.9rem; line-height:1.6; color:#cbd5e1;">
                    Sesuai Peraturan IPSI 2026, kontingen yang mengajukan banding ke Dewan Hakim wajib menyetorkan deposit jaminan sebesar <b>Rp 3.000.000,-</b>.
                    <br><br>
                    • Jika banding <b>DIKABULKAN</b>: Dana dikembalikan 100% secara otomatis ke rekening kontingen.<br>
                    • Jika banding <b>DITOLAK</b>: Dana hangus dan dialokasikan ke kas pembinaan wasit-juri.
                </div>
            </div>
            <div class="qris-box">
                <div style="font-weight:900; font-size:1.2rem; margin-bottom:5px;">QRIS ASPI STANDAR</div>
                <div style="font-size:0.75rem; color:#64748b; margin-bottom:10px;">NMID: ID1020260088921</div>
                <div style="background:#f1f5f9; padding:20px; border-radius:8px; border:2px dashed #94a3b8; font-family:monospace; font-size:0.75rem; word-break:break-all;">
                    {}
                </div>
                <div style="font-weight:bold; margin-top:10px; font-size:1.1rem; color:#1e293b;">Rp 14.000.000,-</div>
                <div style="font-size:0.75rem; color:#10b981; font-weight:bold; margin-top:3px;">● MENUNGGU SCAN / PEMBAYARAN</div>
            </div>
        </div>
    </div>
</body>
</html>"###,
        render_navbar("pembayaran"),
        split.porsi_eo_panitia_idr,
        split.iuran_kas_federasi_ipsi_idr,
        split.porsi_platform_saas_idr,
        qris_payload
    )
}

fn html_ai_vision(state: &MultiRoleAppState) -> String {
    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>AI Biomekanik & Pose Estimation — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 10px; padding: 18px; margin-bottom: 18px; border: 1px solid #334155; }}
        .grid-2 {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(380px, 1fr)); gap: 15px; }}
        .metric-badge {{ background: #0f172a; padding: 12px; border-radius: 8px; border-left: 4px solid #38bdf8; }}
    </style>
</head>
<body>
    <h2>⚡ AI VISION POSE ESTIMATION & BIOMECHANICAL ANALYTICS</h2>
    {}
    <div class="grid-2">
        <div class="card">
            <h3 style="margin:0 0 12px 0; color:#38bdf8;">🎯 Real-time Strike Legal Zone Detection</h3>
            <div style="background:#0f172a; border-radius:8px; padding:15px; text-align:center; margin-bottom:12px; border:1px solid #334155;">
                <div style="font-size:1.1rem; font-weight:bold; color:#10b981;">🔴 TENDANGAN SABIT MERAH — ZONA DADA (SAH +2)</div>
                <div style="font-size:0.85rem; color:#94a3b8; margin-top:4px;">Kecepatan Tumbukan: <b>14.2 m/s</b> | Waktu Reaksi Lawan: <b>180 ms</b> | Keypoint AI: <b>98.4% Confidence</b></div>
            </div>
            <div style="display:grid; grid-template-columns:1fr 1fr; gap:10px; font-size:0.85rem;">
                <div class="metric-badge" style="border-color:#10b981;">
                    <div>Zona Sah (Body Protector)</div>
                    <b style="color:#10b981;">DADA / PERUT / RUSUK (+1 / +2)</b>
                </div>
                <div class="metric-badge" style="border-color:#ef4444;">
                    <div>Zona Dilarang (Pelanggaran)</div>
                    <b style="color:#ef4444;">LEHER / KEPALA / KEMALUAN</b>
                </div>
                <div class="metric-badge" style="border-color:#f59e0b;">
                    <div>Batas Waktu Tangkapan</div>
                    <b style="color:#f59e0b;">MAKSIMAL 2.0 DETIK (IPSI 2026)</b>
                </div>
                <div class="metric-badge" style="border-color:#8b5cf6;">
                    <div>Status Tarikan Terlarang</div>
                    <b style="color:#8b5cf6;">NIHIL (CLEAN FIGHTING)</b>
                </div>
            </div>
        </div>

        <div class="card">
            <h3 style="margin:0 0 12px 0; color:#f59e0b;">📊 Radar Biomotorik Atlet Finalis</h3>
            <div style="font-size:0.9rem; line-height:1.7;">
                <div>Atlet: <b style="color:#ef4444;">Eko Prasetyo</b> (Jawa Tengah) vs <b style="color:#3b82f6;">Bambang Sudarsono</b> (Jawa Timur)</div>
                <div style="margin-top:10px;">
                    <div>⚡ Rata-rata Kecepatan Tendangan: <b style="color:#38bdf8;">14.1 m/s</b> vs <b style="color:#93c5fd;">13.4 m/s</b></div>
                    <div>⏱️ Rata-rata Waktu Reaksi: <b style="color:#38bdf8;">176 ms</b> vs <b style="color:#93c5fd;">188 ms</b></div>
                    <div>💥 Daya Ledak Tendangan: <b style="color:#38bdf8;">6,468 Watt</b> vs <b style="color:#93c5fd;">5,835 Watt</b></div>
                    <div>🎯 Akurasi Serangan Masuk: <b style="color:#10b981;">80.0%</b> vs <b style="color:#10b981;">72.5%</b></div>
                    <div>🫁 Estimasi Ketahanan VO2Max: <b style="color:#f59e0b;">54.5 ml/kg/min (Tinggi)</b></div>
                </div>
            </div>
        </div>
    </div>
</body>
</html>"###,
        render_navbar("ai-vision")
    )
}

fn html_asuransi(state: &MultiRoleAppState) -> String {
    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Asuransi BPJS Atlet & Medis — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 10px; padding: 18px; margin-bottom: 18px; border: 1px solid #334155; }}
        .grid-2 {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(360px, 1fr)); gap: 15px; }}
    </style>
</head>
<body>
    <h2>🩺 ASURANSI BPJS KETENAGAKERJAAN & JALUR HIJAU MEDIS ATLET</h2>
    {}
    <div class="grid-2">
        <div class="card" style="border-top:4px solid #10b981;">
            <h3 style="margin:0 0 10px 0; color:#34d399;">🛡️ Polis Perlindungan BPJS Ketenagakerjaan (BPU Atlet)</h3>
            <div style="font-size:0.9rem; line-height:1.7;">
                <div>Plafon Perlindungan: <b style="color:#10b981; font-size:1.2rem;">Rp 50.000.000,- (Unlimited Sesuai Indikasi)</b></div>
                <div>Status Polis Seluruh Peserta: <span style="background:#065f46; color:#6ee7b7; padding:2px 8px; border-radius:4px; font-weight:bold;">100% TERLINDUNGI</span></div>
                <div>Cakupan: <b>Kecelakaan saat Bertanding, Pengobatan Rumah Sakit, Biaya Transportasi Darurat Ambulans, Santunan Sementara Tidak Mampu Bekerja.</b></div>
            </div>
        </div>

        <div class="card" style="border-top:4px solid #38bdf8;">
            <h3 style="margin:0 0 10px 0; color:#38bdf8;">🏥 SOP Rujukan Jalur Hijau IGD RSUD</h3>
            <div style="font-size:0.9rem; line-height:1.7;">
                <div>Rumah Sakit Rujukan: <b>RSUD Tugurejo Semarang / RSUP Dr. Kariadi</b></div>
                <div>Dokter Penanggung Jawab: <b>dr. Anita Rahayu, Sp.KO (STR-3301.99281)</b></div>
                <div>Format Rujukan: <b style="color:#f59e0b;">E-Surat Rujukan QR Instan (Tanpa Antre Loket)</b></div>
                <div>Kesiapan Unit Ambulans: <b style="color:#10b981;">2 Unit Standby di Pintu Keluar Medis</b></div>
            </div>
        </div>
    </div>
</body>
</html>"###,
        render_navbar("asuransi")
    )
}

fn html_publik(state: &MultiRoleAppState) -> String {
    let h2h = PublicPortalEngine::kalkulasi_h2h(
        "PARTAI-FINAL-01",
        "Eko Prasetyo", "Jawa Tengah", 1620.0, 18, 2, "Tendangan Sabit Cepat",
        "Bambang Sudarsono", "Jawa Timur", 1540.0, 14, 4, "Bantingan Tangkapan",
    );

    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Portal Publik & Analitik Head-to-Head — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; border-radius: 10px; padding: 18px; margin-bottom: 18px; border: 1px solid #334155; }}
        .h2h-container {{ display: flex; justify-content: space-between; align-items: center; background: #0b1120; padding: 20px; border-radius: 12px; border: 1px solid #334155; }}
        .fighter-col {{ width: 42%; }}
    </style>
</head>
<body>
    <h2>🌐 PORTAL PUBLIK, LIVE TICKER & HEAD-TO-HEAD MATCH ANALYTICS</h2>
    {}
    <div class="card">
        <div style="background:#1e3a8a; padding:10px 15px; border-radius:8px; font-size:0.95rem; font-weight:bold; margin-bottom:15px; display:flex; justify-content:space-between; align-items:center;">
            <span>🔴 GELANGGANG A | FINAL TANDING KELAS E (65-70 KG)</span>
            <span style="background:#ef4444; color:white; padding:2px 8px; border-radius:12px; font-size:0.75rem; animation:pulse 1.5s infinite;">● LIVE STREAM</span>
        </div>

        <div class="h2h-container">
            <div class="fighter-col" style="text-align:left;">
                <div style="font-size:1.4rem; font-weight:900; color:#ef4444;">🔴 {}</div>
                <div style="font-size:0.95rem; color:#cbd5e1;">{}</div>
                <div style="margin-top:8px; font-size:0.85rem;">
                    <div>Rating ELO: <b style="color:#f59e0b;">{:.0}</b></div>
                    <div>Rekor Menang/Kalah: <b>18 - 2 ({:.1}%)</b></div>
                    <div>Teknik Favorit: <b style="color:#38bdf8;">{}</b></div>
                </div>
            </div>

            <div style="text-align:center; min-width:120px;">
                <div style="font-size:1.6rem; font-weight:900; color:#94a3b8;">VS</div>
                <div style="margin-top:8px; font-size:0.8rem; font-weight:bold; color:#38bdf8;">PROBABILITAS</div>
                <div style="display:flex; justify-content:center; gap:6px; font-size:0.85rem; font-weight:bold; margin-top:4px;">
                    <span style="color:#ef4444;">{:.1}%</span> : <span style="color:#3b82f6;">{:.1}%</span>
                </div>
            </div>

            <div class="fighter-col" style="text-align:right;">
                <div style="font-size:1.4rem; font-weight:900; color:#3b82f6;">🔵 {}</div>
                <div style="font-size:0.95rem; color:#cbd5e1;">{}</div>
                <div style="margin-top:8px; font-size:0.85rem;">
                    <div>Rating ELO: <b style="color:#f59e0b;">{:.0}</b></div>
                    <div>Rekor Menang/Kalah: <b>14 - 4 ({:.1}%)</b></div>
                    <div>Teknik Favorit: <b style="color:#38bdf8;">{}</b></div>
                </div>
            </div>
        </div>
    </div>
</body>
</html>"###,
        render_navbar("publik"),
        h2h.atlet_merah_nama, h2h.atlet_merah_kontingen, h2h.atlet_merah_elo, h2h.atlet_merah_winrate, h2h.atlet_merah_teknik_favorit,
        h2h.win_probability_merah_persen, h2h.win_probability_biru_persen,
        h2h.atlet_biru_nama, h2h.atlet_biru_kontingen, h2h.atlet_biru_elo, h2h.atlet_biru_winrate, h2h.atlet_biru_teknik_favorit
    )
}

fn html_login() -> String {

    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Masuk — Arena Silat SaaS (Single Sign-On)</title>
    <style>
        :root {{
            --bg-dark: #090d16;
            --card-dark: #1e293b;
            --gold: #f59e0b;
            --primary: #2563eb;
            --primary-hover: #1d4ed8;
            --accent-green: #10b981;
            --border-color: #334155;
            --text-muted: #94a3b8;
        }}
        * {{ box-sizing: border-box; }}
        body {{
            margin: 0;
            padding: 20px;
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
            background: radial-gradient(circle at top center, #1e293b 0%, #090d16 80%);
            color: white;
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 100vh;
            justify-content: center;
        }}
        .login-card {{
            background: rgba(30, 41, 59, 0.75);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 20px;
            padding: 40px;
            width: 100%;
            max-width: 480px;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7), 0 0 20px rgba(37, 99, 235, 0.15);
        }}
        .brand {{
            text-align: center;
            margin-bottom: 25px;
        }}
        .brand-icon {{
            font-size: 3rem;
            display: inline-block;
            filter: drop-shadow(0 0 10px rgba(245, 158, 11, 0.4));
            margin-bottom: 6px;
        }}
        .brand h2 {{
            margin: 0;
            font-size: 1.6rem;
            letter-spacing: 1.5px;
            font-weight: 800;
            background: linear-gradient(135deg, #ffffff 40%, #94a3b8);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .brand p {{
            margin: 6px 0 0 0;
            color: var(--text-muted);
            font-size: 0.88rem;
        }}
        .badge-sso {{
            display: inline-block;
            margin-top: 8px;
            padding: 3px 10px;
            background: rgba(245, 158, 11, 0.15);
            border: 1px solid rgba(245, 158, 11, 0.3);
            color: var(--gold);
            border-radius: 20px;
            font-size: 0.75rem;
            font-weight: 600;
        }}
        .form-group {{
            margin-bottom: 18px;
        }}
        label {{
            display: block;
            font-size: 0.88rem;
            color: #cbd5e1;
            margin-bottom: 6px;
            font-weight: 500;
        }}
        input[type="text"], input[type="password"] {{
            width: 100%;
            padding: 13px 15px;
            border-radius: 10px;
            border: 1px solid var(--border-color);
            background: rgba(15, 23, 42, 0.8);
            color: white;
            font-size: 0.95rem;
            transition: all 0.2s ease;
        }}
        input[type="text"]:focus, input[type="password"]:focus {{
            outline: none;
            border-color: #3b82f6;
            background: rgba(15, 23, 42, 1);
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.25);
        }}
        .btn-submit {{
            width: 100%;
            padding: 14px;
            border-radius: 10px;
            border: none;
            background: linear-gradient(135deg, #2563eb, #1d4ed8);
            color: white;
            font-weight: 700;
            font-size: 1rem;
            cursor: pointer;
            transition: all 0.2s ease;
            box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
            margin-top: 5px;
        }}
        .btn-submit:hover {{
            background: linear-gradient(135deg, #3b82f6, #2563eb);
            transform: translateY(-1px);
            box-shadow: 0 6px 16px rgba(37, 99, 235, 0.4);
        }}
        .btn-submit:active {{
            transform: translateY(0);
        }}
        .sso-btn {{
            width: 100%;
            padding: 12px;
            border-radius: 10px;
            border: 1px solid var(--border-color);
            background: rgba(15, 23, 42, 0.6);
            color: #e2e8f0;
            font-size: 0.9rem;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 10px;
            margin-top: 14px;
            font-weight: 500;
            transition: 0.2s;
        }}
        .sso-btn:hover {{
            background: rgba(30, 41, 59, 0.9);
            border-color: #64748b;
        }}
        .divider {{
            display: flex;
            align-items: center;
            text-align: center;
            margin: 22px 0;
            color: #64748b;
            font-size: 0.8rem;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        .divider::before, .divider::after {{
            content: '';
            flex: 1;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        }}
        .divider:not(:empty)::before {{ margin-right: 1em; }}
        .divider:not(:empty)::after {{ margin-left: 1em; }}
        .demo-roles {{
            margin-top: 25px;
            background: rgba(15, 23, 42, 0.6);
            border: 1px dashed rgba(245, 158, 11, 0.4);
            border-radius: 12px;
            padding: 16px;
        }}
        .demo-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 8px;
            margin-top: 12px;
        }}
        .demo-btn {{
            background: #1e293b;
            color: #cbd5e1;
            border: 1px solid #334155;
            padding: 9px 8px;
            border-radius: 8px;
            font-size: 0.78rem;
            text-align: center;
            text-decoration: none;
            cursor: pointer;
            transition: all 0.15s ease;
            font-weight: 500;
        }}
        .demo-btn:hover {{
            background: #2563eb;
            color: white;
            border-color: #60a5fa;
            transform: translateY(-1px);
        }}
        .alert-box {{
            display: none;
            padding: 12px;
            border-radius: 8px;
            margin-bottom: 16px;
            font-size: 0.88rem;
            text-align: center;
        }}
        .alert-error {{
            background: rgba(220, 38, 38, 0.2);
            border: 1px solid #ef4444;
            color: #fca5a5;
        }}
        .alert-success {{
            background: rgba(16, 185, 129, 0.2);
            border: 1px solid #10b981;
            color: #6ee7b7;
        }}
    </style>
    <script>
        function setQuickDemo(email, pwd) {{
            document.getElementById('identitas').value = email;
            document.getElementById('password').value = pwd;
            submitLoginForm();
        }}

        function handleLogin(e) {{
            e.preventDefault();
            submitLoginForm();
        }}

        function submitLoginForm() {{
            let identitas = document.getElementById('identitas').value.trim();
            let pwd = document.getElementById('password').value;
            let alertBox = document.getElementById('alert-box');

            if (!identitas || !pwd) {{
                alertBox.className = 'alert-box alert-error';
                alertBox.innerText = 'Mohon isi Email/No HP dan Kata Sandi.';
                alertBox.style.display = 'block';
                return;
            }}

            fetch('/api/auth/login', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ identitas: identitas, password: pwd }})
            }}).then(r => r.json()).then(d => {{
                if (d.status === 'ok') {{
                    document.cookie = "arena_auth=1; path=/; max-age=86400; SameSite=Lax";
                    alertBox.className = 'alert-box alert-success';
                    alertBox.innerText = '✅ Login Berhasil! Selamat Datang, ' + d.user.nama_lengkap + ' (' + d.user.active_role + ')';
                    alertBox.style.display = 'block';
                    setTimeout(() => {{
                        window.location.href = d.redirect_url;
                    }}, 600);
                }} else {{
                    alertBox.className = 'alert-box alert-error';
                    alertBox.innerText = '⚠️ ' + d.message;
                    alertBox.style.display = 'block';
                }}
            }}).catch(err => {{
                alertBox.className = 'alert-box alert-error';
                alertBox.innerText = '⚠️ Gagal terhubung ke server autentikasi.';
                alertBox.style.display = 'block';
            }});
        }}
    </script>
</head>
<body>
    <div style="width: 100%; max-width: 480px; margin-bottom: 14px; display: flex; justify-content: space-between; align-items: center;">
        <a href="/" style="color: #94a3b8; text-decoration: none; font-size: 0.88rem; display: flex; align-items: center; gap: 6px; font-weight: 500;">
            <span>←</span> Beranda Landing Page
        </a>
        <a href="/register" style="color: #38bdf8; text-decoration: none; font-size: 0.88rem; font-weight: 600;">
            Daftar Akun ➔
        </a>
    </div>

    <div class="login-card">
        <div class="brand">
            <div class="brand-icon">🥋</div>
            <h2>ARENA SILAT SAAS</h2>
            <p>Sovereign Pencak Silat Tournament Operating System</p>
            <span class="badge-sso">🔐 Single Sign-On & RBAC 2026</span>
        </div>

        <div id="alert-box" class="alert-box"></div>

        <form onsubmit="handleLogin(event)">
            <div class="form-group">
                <label for="identitas">Email / Nomor WhatsApp</label>
                <input type="text" id="identitas" placeholder="contoh: coach@jateng.id / 0811..." required autocomplete="username">
            </div>
            <div class="form-group">
                <div style="display: flex; justify-content: space-between; margin-bottom: 6px;">
                    <label for="password" style="margin: 0;">Kata Sandi</label>
                    <a href="javascript:void(0)" onclick="alert('Fitur reset kata sandi via OTP WhatsApp segera hadir!')" style="color: #38bdf8; font-size: 0.8rem; text-decoration: none;">Lupa Sandi?</a>
                </div>
                <input type="password" id="password" placeholder="Masukkan kata sandi..." required autocomplete="current-password">
            </div>
            <button type="submit" class="btn-submit">Masuk ke Sistem ➔</button>
        </form>

        <button class="sso-btn" onclick="alert('🌐 Menghubungkan ke SSO Nasional IPSI & Kemendikdasmen (SIMT)...')">
            <span>🌐</span> Masuk dengan Google SSO / Akun Belajar.id
        </button>

        <div style="text-align: center; margin-top: 18px; font-size: 0.88rem; color: #94a3b8;">
            Belum memiliki akun? <a href="/register" style="color: #38bdf8; font-weight: bold; text-decoration: none;">Daftar Akun Baru</a>
        </div>

        <div class="demo-roles">
            <div style="font-size: 0.8rem; font-weight: 700; color: var(--gold); text-align: center; letter-spacing: 0.5px;">
                🚀 AKSES CEPAT MODE DEMO (AUTO-LOGIN PERAN):
            </div>
            <div class="demo-grid">
                <button type="button" class="demo-btn" onclick="setQuickDemo('admin@arenasilat.id', 'admin123')">👑 Super Admin</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('eo@silat.id', 'eo123')">📝 Panitia / EO</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('td@ipsi.or.id', 'td123')">🏛️ Dewan Hakim/TD</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('kp@gelanggang.id', 'kp123')">⚖️ Ketua Pertandingan</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('juri1@ipsi.id', 'juri123')">📱 Wasit Juri 1</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('medis@rsud.id', 'medis123')">🩺 Dokter Medis</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('marshall@arena.id', 'marshall123')">⚖️ Marshalling BLE</button>
                <button type="button" class="demo-btn" onclick="setQuickDemo('coach@jateng.id', 'coach123')">🥋 Pelatih Kontingen</button>
            </div>
        </div>
    </div>
</body>
</html>"###)
}

fn html_register() -> String {
    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Daftar Akun Baru — Arena Silat SaaS</title>
    <style>
        :root {{
            --bg-dark: #090d16;
            --card-dark: #1e293b;
            --gold: #f59e0b;
            --primary: #2563eb;
            --accent-green: #10b981;
            --border-color: #334155;
            --text-muted: #94a3b8;
        }}
        * {{ box-sizing: border-box; }}
        body {{
            margin: 0;
            padding: 20px;
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
            background: radial-gradient(circle at top center, #1e293b 0%, #090d16 80%);
            color: white;
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 100vh;
            justify-content: center;
        }}
        .reg-card {{
            background: rgba(30, 41, 59, 0.85);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 20px;
            padding: 35px 40px;
            width: 100%;
            max-width: 580px;
            box-shadow: 0 25px 50px -12px rgba(0,0,0,0.7);
        }}
        .brand {{
            text-align: center;
            margin-bottom: 22px;
        }}
        .brand-icon {{
            font-size: 2.8rem;
            margin-bottom: 4px;
        }}
        .brand h2 {{
            margin: 0;
            font-size: 1.5rem;
            font-weight: 800;
            letter-spacing: 1px;
            background: linear-gradient(135deg, #ffffff 40%, #94a3b8);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .brand p {{
            margin: 4px 0 0 0;
            color: var(--text-muted);
            font-size: 0.85rem;
        }}
        .form-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 14px;
        }}
        .form-group {{
            margin-bottom: 14px;
        }}
        .form-group.full {{
            grid-column: 1 / -1;
        }}
        label {{
            display: block;
            font-size: 0.84rem;
            color: #cbd5e1;
            margin-bottom: 5px;
            font-weight: 500;
        }}
        input, select {{
            width: 100%;
            padding: 11px 14px;
            border-radius: 9px;
            border: 1px solid var(--border-color);
            background: rgba(15, 23, 42, 0.85);
            color: white;
            font-size: 0.92rem;
            transition: all 0.2s;
        }}
        input:focus, select:focus {{
            outline: none;
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.25);
            background: rgba(15, 23, 42, 1);
        }}
        select {{
            cursor: pointer;
        }}
        select option {{
            background: #0f172a;
            color: white;
        }}
        .terms {{
            display: flex;
            align-items: center;
            gap: 8px;
            margin: 10px 0 16px 0;
            font-size: 0.82rem;
            color: var(--text-muted);
        }}
        .terms input[type="checkbox"] {{
            width: 16px;
            height: 16px;
            cursor: pointer;
            accent-color: #2563eb;
        }}
        .btn-submit {{
            width: 100%;
            padding: 13px;
            border-radius: 9px;
            border: none;
            background: linear-gradient(135deg, #10b981, #059669);
            color: white;
            font-weight: 700;
            font-size: 1rem;
            cursor: pointer;
            transition: all 0.2s;
            box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
        }}
        .btn-submit:hover {{
            background: linear-gradient(135deg, #34d399, #10b981);
            transform: translateY(-1px);
            box-shadow: 0 6px 16px rgba(16, 185, 129, 0.4);
        }}
        .alert-box {{
            display: none;
            padding: 12px;
            border-radius: 8px;
            margin-bottom: 16px;
            font-size: 0.88rem;
            text-align: center;
        }}
        .alert-error {{
            background: rgba(220, 38, 38, 0.2);
            border: 1px solid #ef4444;
            color: #fca5a5;
        }}
        .alert-success {{
            background: rgba(16, 185, 129, 0.2);
            border: 1px solid #10b981;
            color: #6ee7b7;
        }}
    </style>
    <script>
        function handleRegister(e) {{
            e.preventDefault();
            let nama = document.getElementById('nama').value.trim();
            let email = document.getElementById('email').value.trim();
            let telp = document.getElementById('telp').value.trim();
            let role = document.getElementById('role').value;
            let org = document.getElementById('org').value.trim();
            let pwd = document.getElementById('password').value;
            let pwdConfirm = document.getElementById('password_confirm').value;
            let terms = document.getElementById('terms_agree').checked;
            let alertBox = document.getElementById('alert-box');

            if (pwd !== pwdConfirm) {{
                alertBox.className = 'alert-box alert-error';
                alertBox.innerText = '⚠️ Kata Sandi dan Konfirmasi Kata Sandi tidak cocok!';
                alertBox.style.display = 'block';
                return;
            }}

            if (!terms) {{
                alertBox.className = 'alert-box alert-error';
                alertBox.innerText = '⚠️ Anda wajib menyetujui Peraturan & Etika Pertandingan IPSI.';
                alertBox.style.display = 'block';
                return;
            }}

            fetch('/api/auth/register', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{
                    nama: nama,
                    email: email,
                    telp: telp,
                    role: role,
                    org: org,
                    password: pwd
                }})
            }}).then(r => r.json()).then(d => {{
                if (d.status === 'ok') {{
                    alertBox.className = 'alert-box alert-success';
                    alertBox.innerText = '🎉 Registrasi Berhasil! ID Akun: ' + d.user_id + '. Mengarahkan ke halaman login...';
                    alertBox.style.display = 'block';
                    setTimeout(() => {{
                        window.location.href = '/login';
                    }}, 1200);
                }} else {{
                    alertBox.className = 'alert-box alert-error';
                    alertBox.innerText = '⚠️ Gagal Registrasi: ' + d.message;
                    alertBox.style.display = 'block';
                }}
            }}).catch(err => {{
                alertBox.className = 'alert-box alert-error';
                alertBox.innerText = '⚠️ Terjadi kesalahan jaringan saat registrasi.';
                alertBox.style.display = 'block';
            }});
        }}
    </script>
</head>
<body>
    <div style="width: 100%; max-width: 580px; margin-bottom: 14px; display: flex; justify-content: space-between; align-items: center;">
        <a href="/" style="color: #94a3b8; text-decoration: none; font-size: 0.88rem; display: flex; align-items: center; gap: 6px; font-weight: 500;">
            <span>←</span> Beranda Landing Page
        </a>
        <a href="/login" style="color: #38bdf8; text-decoration: none; font-size: 0.88rem; font-weight: 600;">
            Sudah Punya Akun? Masuk ➔
        </a>
    </div>

    <div class="reg-card">
        <div class="brand">
            <div class="brand-icon">📝</div>
            <h2>PENDAFTARAN AKUN ARENA SILAT</h2>
            <p>Satu Akun Terpadu untuk Pelatih, Atlet, Wasit Juri, dan Panitia</p>
        </div>

        <div id="alert-box" class="alert-box"></div>

        <form onsubmit="handleRegister(event)">
            <div class="form-group full">
                <label for="nama">Nama Lengkap & Gelar Resmi</label>
                <input type="text" id="nama" placeholder="contoh: Bambang Supriyanto, S.Pd." required>
            </div>
            <div class="form-grid">
                <div class="form-group">
                    <label for="email">Alamat Email</label>
                    <input type="email" id="email" placeholder="nama@email.com" required>
                </div>
                <div class="form-group">
                    <label for="telp">No. WhatsApp Aktif</label>
                    <input type="text" id="telp" placeholder="08123456789" required>
                </div>
            </div>
            <div class="form-grid">
                <div class="form-group">
                    <label for="role">Tipe Pendaftar (Role Akses)</label>
                    <select id="role">
                        <option value="CoachOfficial">🥋 Pelatih / Official Kontingen</option>
                        <option value="AthleteUser">🏃 Pesilat / Atlet Mandiri</option>
                        <option value="Jury">⚖️ Wasit Juri Berlisensi</option>
                        <option value="DoctorMedic">🩺 Dokter Pertandingan / Tim Medis</option>
                        <option value="EoOrganizer">📝 Panitia / Event Organizer (EO)</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="org">Perguruan / Pengkab / Kontingen</label>
                    <input type="text" id="org" placeholder="contoh: Tapak Suci Solo / PSHT / Kontingen Jateng">
                </div>
            </div>
            <div class="form-grid">
                <div class="form-group">
                    <label for="password">Kata Sandi Baru</label>
                    <input type="password" id="password" placeholder="Minimal 6 karakter" minlength="6" required>
                </div>
                <div class="form-group">
                    <label for="password_confirm">Konfirmasi Kata Sandi</label>
                    <input type="password" id="password_confirm" placeholder="Ulangi kata sandi..." minlength="6" required>
                </div>
            </div>

            <div class="terms">
                <input type="checkbox" id="terms_agree" required checked>
                <label for="terms_agree" style="margin: 0; font-size: 0.8rem; color: #cbd5e1; cursor: pointer;">
                    Saya menyetujui Peraturan Pertandingan IPSI 2026, Kode Etik Wasit Juri, dan Ketentuan Layanan Arena Silat SaaS.
                </label>
            </div>

            <button type="submit" class="btn-submit">Daftarkan Akun Baru ➔</button>
        </form>

        <div style="text-align: center; margin-top: 18px; font-size: 0.88rem; color: #94a3b8;">
            Sudah memiliki akun terdaftar? <a href="/login" style="color: #38bdf8; font-weight: bold; text-decoration: none;">Masuk di Sini</a>
        </div>
    </div>
</body>
</html>"###)
}

fn html_juri(juri_id: usize) -> String {
    format!(r###"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>Touchpad Juri {} — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 8px; font-family: system-ui, sans-serif; background: #0f172a; color: white; user-select: none; }}
        .header {{ text-align: center; padding: 6px; background: #1e293b; border-radius: 6px; margin-bottom: 8px; font-weight: bold; font-size: 0.9rem; }}
        .grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 8px; height: calc(100vh - 70px); }}
        .column {{ display: flex; flex-direction: column; gap: 8px; }}
        .btn-action {{ flex: 1; border: none; border-radius: 8px; font-size: 1.3rem; font-weight: 800; color: white; cursor: pointer; display: flex; flex-direction: column; align-items: center; justify-content: center; }}
        .btn-action:active {{ transform: scale(0.96); filter: brightness(1.3); }}
        .btn-red {{ background: #dc2626; }}
        .btn-blue {{ background: #2563eb; }}
        .points {{ font-size: 0.8rem; opacity: 0.8; margin-top: 2px; }}
        .notification {{ position: fixed; bottom: 15px; left: 50%; transform: translateX(-50%); background: rgba(0,0,0,0.85); padding: 6px 14px; border-radius: 16px; display: none; border: 1px solid #10b981; color: #10b981; font-weight: bold; font-size: 0.85rem; }}
    </style>
    <script>
        function sendVote(sudut, aksi) {{
            fetch('/api/juri/press', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ juri_id: {}, sudut: sudut, aksi: aksi }})
            }}).then(r => r.json()).then(d => {{
                let n = document.getElementById('notif');
                n.innerText = (d.konsensus ? '✅ KONSENSUS TERCAPAI: ' : '⏳ Input Juri Dikirim: ') + sudut + ' ' + aksi;
                n.style.display = 'block';
                setTimeout(() => {{ n.style.display = 'none'; }}, 1000);
            }});
        }}
    </script>
</head>
<body>
    <div class="header">
        ⚖️ TOUCHPAD JURI {} — IPSI 2026 (BLIND SCORING)
        <a href="/scoreboard" style="color:#94a3b8; margin-left:10px; text-decoration:none;">📺 Gelanggang</a>
    </div>
    <div class="grid">
        <div class="column">
            <button class="btn-action btn-red" onclick="sendVote('Merah', 'Pukulan')">PUKULAN<span class="points">+1 POIN</span></button>
            <button class="btn-action btn-red" onclick="sendVote('Merah', 'Tendangan')">TENDANGAN<span class="points">+2 POIN</span></button>
            <button class="btn-action btn-red" onclick="sendVote('Merah', 'Jatuhan')">JATUHAN<span class="points">+3 POIN</span></button>
            <button class="btn-action btn-red" onclick="sendVote('Merah', 'Kuncian')">KUNCIAN<span class="points">+4 POIN</span></button>
        </div>
        <div class="column">
            <button class="btn-action btn-blue" onclick="sendVote('Biru', 'Pukulan')">PUKULAN<span class="points">+1 POIN</span></button>
            <button class="btn-action btn-blue" onclick="sendVote('Biru', 'Tendangan')">TENDANGAN<span class="points">+2 POIN</span></button>
            <button class="btn-action btn-blue" onclick="sendVote('Biru', 'Jatuhan')">JATUHAN<span class="points">+3 POIN</span></button>
            <button class="btn-action btn-blue" onclick="sendVote('Biru', 'Kuncian')">KUNCIAN<span class="points">+4 POIN</span></button>
        </div>
    </div>
    <div id="notif" class="notification"></div>
</body>
</html>"###, juri_id, juri_id, juri_id)
}

fn html_kp() -> String {
    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Ketua Pertandingan (KP) — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .card {{ background: #1e293b; padding: 15px; border-radius: 8px; border: 1px solid #334155; margin-bottom: 15px; }}
        .btn {{ width: 100%; padding: 10px; margin-bottom: 8px; border: none; border-radius: 6px; font-size: 0.92rem; font-weight: bold; color: white; cursor: pointer; }}
        .btn-yellow {{ background: #ca8a04; }}
        .btn-orange {{ background: #ea580c; }}
        .btn-red {{ background: #dc2626; }}
        .btn-green {{ background: #16a34a; }}
    </style>
    <script>
        function sendKp(action) {{
            fetch('/api/kp/action', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ action: action }})
            }}).then(r => r.json()).then(d => {{ alert('Aksi KP Diproses: ' + action); }});
        }}
    </script>
</head>
<body>
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <h2>⚖️ PANEL KETUA PERTANDINGAN (KP) & MEJA GELANGGANG</h2>
        <a href="/scoreboard" style="color:#38bdf8; text-decoration:none; font-weight:bold;">📺 Gelanggang</a>
    </div>
    {}

    <div class="card" style="text-align:center;">
        <h3>⏱️ KONTROL TIMER GELANGGANG & GONG DIGITAL</h3>
        <div style="display:flex; gap:10px; justify-content:center;">
            <button class="btn btn-green" style="width:auto; padding:10px 24px;" onclick="sendKp('MULAI_BABAK')">▶️ Mulai / Lanjutkan</button>
            <button class="btn btn-yellow" style="width:auto; padding:10px 24px;" onclick="sendKp('JEDA_BABAK')">⏸️ Jeda (Pause)</button>
            <button class="btn btn-orange" style="width:auto; padding:10px 24px;" onclick="sendKp('GONG')">🔔 Pukul Gong</button>
        </div>
    </div>

    <div style="display:grid; grid-template-columns:1fr 1fr; gap:15px;">
        <div class="card">
            <h3 style="color:#ef4444; margin-top:0;">🔴 HUKUMAN SUDUT MERAH</h3>
            <button class="btn btn-yellow" onclick="sendKp('HUKUMAN_MERAH_TEGURAN1')">Teguran 1 (-1 Poin)</button>
            <button class="btn btn-yellow" onclick="sendKp('HUKUMAN_MERAH_TEGURAN2')">Teguran 2 (-2 Poin)</button>
            <button class="btn btn-orange" onclick="sendKp('HUKUMAN_MERAH_PERINGATAN1')">Peringatan 1 (-5 Poin / Kuning)</button>
            <button class="btn btn-red" onclick="sendKp('HUKUMAN_MERAH_PERINGATAN3')">Peringatan 3 (DISKUALIFIKASI / Merah)</button>
        </div>
        <div class="card">
            <h3 style="color:#3b82f6; margin-top:0;">🔵 HUKUMAN SUDUT BIRU</h3>
            <button class="btn btn-yellow" onclick="sendKp('HUKUMAN_BIRU_TEGURAN1')">Teguran 1 (-1 Poin)</button>
            <button class="btn btn-yellow" onclick="sendKp('HUKUMAN_BIRU_TEGURAN2')">Teguran 2 (-2 Poin)</button>
            <button class="btn btn-orange" onclick="sendKp('HUKUMAN_BIRU_PERINGATAN1')">Peringatan 1 (-5 Poin / Kuning)</button>
            <button class="btn btn-red" onclick="sendKp('HUKUMAN_BIRU_PERINGATAN3')">Peringatan 3 (DISKUALIFIKASI / Merah)</button>
        </div>
    </div>
</body>
</html>"#, render_navbar("kp"))
}

fn html_bracket() -> String {
    format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>Bagan Pertandingan — Arena Silat</title>
    <style>
        body {{ margin: 0; padding: 15px; font-family: system-ui, sans-serif; background: #0f172a; color: white; }}
        .tree {{ display: flex; gap: 30px; justify-content: center; align-items: center; margin-top: 20px; }}
        .round {{ display: flex; flex-direction: column; justify-content: space-around; gap: 15px; }}
        .match {{ background: #1e293b; border: 1px solid #334155; border-radius: 6px; padding: 10px 15px; width: 200px; }}
        .athlete {{ padding: 4px 0; border-bottom: 1px solid #334155; font-size: 0.88rem; }}
        .red {{ color: #ef4444; font-weight: bold; }}
        .blue {{ color: #3b82f6; font-weight: bold; }}
    </style>
</head>
<body>
    <h2>🌳 BAGAN PERTANDINGAN (BAGAN 8 — SEPARASI KONTINGEN)</h2>
    {}
    <div class="tree">
        <div class="round">
            <h4 style="text-align:center; color:#94a3b8; margin:0 0 10px 0;">PEREMPAT FINAL</h4>
            <div class="match"><div class="athlete red">🔴 Eko Prasetyo (Jateng)</div><div class="athlete blue">🔵 Ridwan (DKI)</div></div>
            <div class="match"><div class="athlete red">🔴 Hendra (Jabar)</div><div class="athlete blue">🔵 BYE</div></div>
            <div class="match"><div class="athlete red">🔴 Bambang (Jatim)</div><div class="athlete blue">🔵 Joko (Bali)</div></div>
            <div class="match"><div class="athlete red">🔴 Ahmad (Sumbar)</div><div class="athlete blue">🔵 BYE</div></div>
        </div>
        <div class="round">
            <h4 style="text-align:center; color:#f59e0b; margin:0 0 10px 0;">SEMI FINAL</h4>
            <div class="match"><div class="athlete red">🔴 Eko Prasetyo (Jateng)</div><div class="athlete blue">🔵 Hendra (Jabar)</div></div>
            <div class="match"><div class="athlete red">🔴 Bambang (Jatim)</div><div class="athlete blue">🔵 Ahmad (Sumbar)</div></div>
        </div>
        <div class="round">
            <h4 style="text-align:center; color:#10b981; margin:0 0 10px 0;">🏆 FINAL</h4>
            <div class="match" style="border:2px solid #10b981;">
                <div class="athlete red">🔴 Eko Prasetyo (Jateng)</div>
                <div class="athlete blue">🔵 Bambang (Jatim)</div>
            </div>
        </div>
    </div>
</body>
</html>"#, render_navbar("bracket"))
}

fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<MultiRoleAppState>>) {
    let mut buffer = [0; 4096];
    let n = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request = String::from_utf8_lossy(&buffer[..n]);
    let mut lines = request.lines();
    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    // Deteksi Session / Cookie Autentikasi
    let has_auth_cookie = request.lines().any(|l| {
        let lower = l.to_lowercase();
        lower.starts_with("cookie:") && (lower.contains("arena_auth=1") || lower.contains("arena_auth=true"))
    });
    let has_auth_header = request.lines().any(|l| {
        let lower = l.to_lowercase();
        lower.starts_with("authorization:") && lower.contains("bearer")
    });
    let has_auth_query = path.contains("auth=1") || path.contains("arena_auth=1");
    let is_authenticated = has_auth_cookie || has_auth_header || has_auth_query;

    if method == "GET" {
        // 1. Logout Action
        if path == "/logout" {
            let response = "HTTP/1.1 302 Found\r\nSet-Cookie: arena_auth=; Path=/; Max-Age=0; SameSite=Lax\r\nLocation: /\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        // 2. Public Routes (Bisa diakses tanpa login)
        if path == "/" || path == "/index.html" || path == "/landing" {
            let body = html_landing_page();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body
            );
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        if path == "/login" {
            let body = html_login();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body
            );
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        if path == "/register" {
            let body = html_register();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body
            );
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        if path == "/api/status" {
            let st = state.lock().unwrap();
            let json = format!(
                r#"{{"skor_merah":{},"skor_biru":{},"babak":{},"status":"{:?}"}}"#,
                st.engine.skor_merah.total_poin,
                st.engine.skor_biru.total_poin,
                st.engine.babak_aktif,
                st.engine.status
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                json.len(), json
            );
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        // 3. Protected Routes (Wajib Login)
        if !is_authenticated {
            let redirect_target = format!("/login?redirect={}&auth_required=1", path);
            let body = format!(
                r#"<!DOCTYPE html><html><head><meta http-equiv="refresh" content="0;url={}" /></head><body style="background:#090d16;color:white;font-family:sans-serif;text-align:center;padding:50px;"><h2>🔒 Akses Terbatas</h2><p>Halaman internal turnamen memerlukan otentikasi login.</p><p><a href="{}" style="color:#38bdf8;font-weight:bold;">Mengarahkan ke Halaman Login &rarr;</a></p></body></html>"#,
                redirect_target, redirect_target
            );
            let response = format!(
                "HTTP/1.1 302 Found\r\nLocation: {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                redirect_target, body.len(), body
            );
            let _ = stream.write_all(response.as_bytes());
            return;
        }

        // 4. Authenticated Internal Pages
        let (status, content_type, body) = if path == "/scoreboard" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_dashboard(&*st))
        } else if path == "/gelanggang" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_gelanggang(&*st))
        } else if path == "/pendaftaran" || path == "/eo" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_pendaftaran(&*st))
        } else if path == "/kategori" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_kategori(&*st))
        } else if path == "/td" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_td(&*st))
        } else if path == "/medis" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_medis(&*st))
        } else if path == "/marshalling" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_marshalling(&*st))
        } else if path == "/kontingen" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_kontingen(&*st))
        } else if path == "/superadmin" {
            ("200 OK", "text/html", html_superadmin())
        } else if path == "/camera" || path == "/var" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_camera(&*st))
        } else if path == "/laporan" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_laporan(&*st))
        } else if path == "/federasi" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_federasi(&*st))
        } else if path == "/pembayaran" || path == "/payment" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_pembayaran(&*st))
        } else if path == "/ai-vision" || path == "/biomekanik" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_ai_vision(&*st))
        } else if path == "/asuransi" || path == "/bpjs" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_asuransi(&*st))
        } else if path == "/publik" || path == "/live" {
            let st = state.lock().unwrap();
            ("200 OK", "text/html", html_publik(&*st))
        } else if path.starts_with("/juri") {
            let juri_id = if path.contains("id=2") { 2 } else if path.contains("id=3") { 3 } else { 1 };
            ("200 OK", "text/html", html_juri(juri_id))
        } else if path == "/kp" {
            ("200 OK", "text/html", html_kp())
        } else if path == "/bracket" {
            ("200 OK", "text/html", html_bracket())
        } else {
            ("404 NOT FOUND", "text/plain", "Halaman Tidak Ditemukan".to_string())
        };

        let response = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status, content_type, body.len(), body
        );
        let _ = stream.write_all(response.as_bytes());
    } else if method == "POST" {
        let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
        let mut st = state.lock().unwrap();
        let mut res_json = r#"{"status":"ok"}"#.to_string();

        if path == "/api/camera/strategy" {
            if body.contains("MosaicQuadView") {
                st.cam_manager.active_strategy = BandwidthStrategy::MosaicQuadView;
                st.log_aksi.push("🖼️ [BANDWIDTH] Beralih ke Mode Mosaic Quad-View (Hemat 85%)".to_string());
            } else if body.contains("LocalEdgeRingBufferOnly") {
                st.cam_manager.active_strategy = BandwidthStrategy::LocalEdgeRingBufferOnly;
                st.log_aksi.push("📼 [BANDWIDTH] Beralih ke Mode Local Edge Ring Buffer (Hemat 100% VAR)".to_string());
            } else if body.contains("AdaptiveBitrate") {
                st.cam_manager.active_strategy = BandwidthStrategy::AdaptiveBitrate;
                st.log_aksi.push("🎛️ [BANDWIDTH] Beralih ke Mode Dynamic Adaptive Bitrate (ABR)".to_string());
            } else if body.contains("DirectMultiChannel") {
                st.cam_manager.active_strategy = BandwidthStrategy::DirectMultiChannel;
                st.log_aksi.push("⚡ [BANDWIDTH] Beralih ke Mode Direct Multi-Channel (Full HD)".to_string());
            }
            res_json = format!(
                r#"{{"status":"ok","actual_kbps":{},"saving_percentage":{:.1}}}"#,
                st.cam_manager.hitung_total_bandwidth_upload_kbps(),
                st.cam_manager.hitung_persentase_penghematan()
            );
        } else if path == "/api/camera/var_clip" {
            let clip = st.cam_manager.buat_klip_var_replay(1, 30);
            st.log_aksi.push(format!("⚖️ [VAR REPLAY] Eksekusi Putusan: {}", body));
            res_json = format!(
                r#"{{"status":"ok","clip_id":"{}","court_id":{}}}"#,
                clip.clip_id, clip.court_id
            );
        } else if path == "/api/juri/press" {
            let mut juri_id = 1;
            let mut sudut = SudutPesilat::Merah;
            let mut aksi = TipePoin::Pukulan;

            if body.contains(r#""juri_id":2"#) || body.contains(r#""juri_id": 2"#) { juri_id = 2; }
            if body.contains(r#""juri_id":3"#) || body.contains(r#""juri_id": 3"#) { juri_id = 3; }
            if body.contains(r#""Biru""#) { sudut = SudutPesilat::Biru; }
            if body.contains(r#""Tendangan""#) { aksi = TipePoin::Tendangan; }
            if body.contains(r#""Jatuhan""#) { aksi = TipePoin::Jatuhan; }
            if body.contains(r#""Kuncian""#) { aksi = TipePoin::Kuncian; }

            let konsensus = st.engine.tekan_tombol_juri(juri_id, sudut, aksi);
            let sudut_str = if sudut == SudutPesilat::Merah { "🔴 MERAH" } else { "🔵 BIRU" };

            // Record to Black Box Audit Trail
            st.td_engine.rekam_black_box("PARTAI-FINAL-01", juri_id, if sudut == SudutPesilat::Merah { "Merah" } else { "Biru" }, &format!("{:?}", aksi), 100, konsensus.is_some());

            if let Some((_, a, poin)) = konsensus {
                let log = format!("✅ [KONSENSUS JURI SAH] {} +{} Poin ({:?})", sudut_str, poin, a);
                st.log_aksi.push(log);
                res_json = format!(r#"{{"status":"ok","konsensus":true,"poin":{}}}"#, poin);
            } else {
                let log = format!("⏳ [Input Juri {}] {} {:?}", juri_id, sudut_str, aksi);
                st.log_aksi.push(log);
                res_json = r#"{"status":"ok","konsensus":false}"#.to_string();
            }
        } else if path == "/api/kp/action" {
            if body.contains("HUKUMAN_MERAH_TEGURAN1") {
                st.engine.berikan_hukuman(SudutPesilat::Merah, TipeHukuman::Teguran1);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔴 MERAH Teguran 1 (-1)".to_string());
            } else if body.contains("HUKUMAN_MERAH_TEGURAN2") {
                st.engine.berikan_hukuman(SudutPesilat::Merah, TipeHukuman::Teguran2);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔴 MERAH Teguran 2 (-2)".to_string());
            } else if body.contains("HUKUMAN_MERAH_PERINGATAN1") {
                st.engine.berikan_hukuman(SudutPesilat::Merah, TipeHukuman::Peringatan1);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔴 MERAH Peringatan 1 (-5)".to_string());
            } else if body.contains("HUKUMAN_BIRU_TEGURAN1") {
                st.engine.berikan_hukuman(SudutPesilat::Biru, TipeHukuman::Teguran1);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔵 BIRU Teguran 1 (-1)".to_string());
            } else if body.contains("HUKUMAN_BIRU_TEGURAN2") {
                st.engine.berikan_hukuman(SudutPesilat::Biru, TipeHukuman::Teguran2);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔵 BIRU Teguran 2 (-2)".to_string());
            } else if body.contains("HUKUMAN_BIRU_PERINGATAN1") {
                st.engine.berikan_hukuman(SudutPesilat::Biru, TipeHukuman::Peringatan1);
                st.log_aksi.push("⚖️ [KP HUKUMAN] 🔵 BIRU Peringatan 1 (-5)".to_string());
            } else if body.contains("MULAI_BABAK") {
                st.engine.status = StatusPartai::Berlangsung;
                st.log_aksi.push("▶️ [KP] Babak Dimulai / Dilanjutkan!".to_string());
            } else if body.contains("JEDA_BABAK") {
                st.engine.status = StatusPartai::JedaBabak;
                st.log_aksi.push("⏸️ [KP] Babak Dihentikan Sementara (Jeda)".to_string());
            } else if body.contains("GONG") {
                st.log_aksi.push("🔔 [KP] GONG DIBUNYIKAN — Waktu Babak Habis!".to_string());
            }
        } else if path == "/api/td/approve_bracket" {
            st.td_engine.sahkan_bagan_turnamen();
            st.log_aksi.push("🏛️ [TD] Bagan Turnamen Resmi Disetujui & Disahkan!".to_string());
        } else if path == "/api/medis/decision" {
            if body.contains("FIT_TO_CONTINUE") {
                st.med_engine.catat_insiden("PARTAI-FINAL-01", "ATL-01", "Eko Prasetyo", "Merah", BagianTubuhCedera::DadaRusuk, KategoriDiagnosis::MemarRingan, 45, KeputusanDokter::FitLanjutTanding);
                st.engine.status = StatusPartai::Berlangsung;
                st.log_aksi.push("🩺 [DOKTER] Atlet Fit to Continue — Laga Dilanjutkan".to_string());
            } else if body.contains("UNFIT_STOP_MATCH") {
                st.med_engine.catat_insiden("PARTAI-FINAL-01", "ATL-01", "Eko Prasetyo", "Merah", BagianTubuhCedera::KepalaWajah, KategoriDiagnosis::GegarOtakKO, 120, KeputusanDokter::UnfitHentikanLagaWMP);
                st.engine.status = StatusPartai::Selesai;
                st.log_aksi.push("🛑 [DOKTER] WMP Medis — Atlet Dinyatakan Unfit & Skorsing 30 Hari".to_string());
            }
        } else if path == "/api/auth/login" {
            let mut identitas = "";
            let mut pwd = "";
            if let Some(i_pos) = body.find(r#""identitas":""#) {
                let rem = &body[i_pos + 13..];
                if let Some(end) = rem.find('"') { identitas = &rem[..end]; }
            }
            if let Some(p_pos) = body.find(r#""password":""#) {
                let rem = &body[p_pos + 12..];
                if let Some(end) = rem.find('"') { pwd = &rem[..end]; }
            }

            if let Some(user) = st.auth_engine.login(identitas, pwd) {
                let redirect = match user.active_role {
                    UserRole::SuperAdmin => "/superadmin",
                    UserRole::EoOrganizer => "/pendaftaran",
                    UserRole::TechnicalDelegate => "/td",
                    UserRole::KetuaPertandingan => "/kp",
                    UserRole::Jury => "/juri?id=1",
                    UserRole::DoctorMedic => "/medis",
                    UserRole::MarshallingOfficer => "/marshalling",
                    UserRole::CoachOfficial => "/kontingen",
                    UserRole::AthleteUser => "/kontingen",
                    _ => "/scoreboard",
                };
                res_json = format!(
                    r#"{{"status":"ok","user":{{"user_id":"{}","nama_lengkap":"{}","active_role":"{:?}"}},"redirect_url":"{}"}}"#,
                    user.user_id, user.nama_lengkap, user.active_role, redirect
                );
            } else {
                res_json = r#"{"status":"error","message":"Email/No HP atau Kata Sandi Salah!"}"#.to_string();
            }
        } else if path == "/api/auth/register" {
            let mut nama = "User Baru";
            let mut email = "";
            let mut telp = "";
            let mut pwd = "password";
            let mut role = UserRole::CoachOfficial;

            if let Some(pos) = body.find(r#""nama":""#) {
                let rem = &body[pos + 8..];
                if let Some(end) = rem.find('"') { nama = &rem[..end]; }
            }
            if let Some(pos) = body.find(r#""email":""#) {
                let rem = &body[pos + 9..];
                if let Some(end) = rem.find('"') { email = &rem[..end]; }
            }
            if let Some(pos) = body.find(r#""telp":""#) {
                let rem = &body[pos + 8..];
                if let Some(end) = rem.find('"') { telp = &rem[..end]; }
            }
            if let Some(pos) = body.find(r#""password":""#) {
                let rem = &body[pos + 12..];
                if let Some(end) = rem.find('"') { pwd = &rem[..end]; }
            }
            if body.contains(r#""AthleteUser""#) { role = UserRole::AthleteUser; }
            if body.contains(r#""Jury""#) { role = UserRole::Jury; }
            if body.contains(r#""DoctorMedic""#) { role = UserRole::DoctorMedic; }
            if body.contains(r#""EoOrganizer""#) { role = UserRole::EoOrganizer; }

            match st.auth_engine.daftarkan_user(email, telp, nama, pwd, role, None) {
                Ok(uid) => {
                    res_json = format!(r#"{{"status":"ok","user_id":"{}"}}"#, uid);
                },
                Err(e) => {
                    res_json = format!(r#"{{"status":"error","message":"{}"}}"#, e);
                }
            }
        }

        let response = format!(
            "HTTP/1.1 200 OK\r\n{}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            if path == "/api/auth/login" && res_json.contains(r#""status":"ok""#) {
                "Set-Cookie: arena_auth=1; Path=/; Max-Age=86400; SameSite=Lax\r\n"
            } else {
                ""
            },
            res_json.len(),
            res_json
        );
        let _ = stream.write_all(response.as_bytes());
    }
}

fn print_banner() {
    println!("\x1b[1;36m==============================================================================\x1b[0m");
    println!("\x1b[1;33m  🥋 ARENA SILAT SAAS — SOVEREIGN 9-ROLE TOURNAMENT MANAGEMENT SUITE 🥋      \x1b[0m");
    println!("\x1b[1;32m       100% In-House Sovereign Code powered by Widya-Lang Ecosystem           \x1b[0m");
    println!("\x1b[1;36m==============================================================================\x1b[0m");
    println!("\x1b[1;37m  * [1] Super Admin Multi-Tenant & Platform Telemetry (/superadmin)\x1b[0m");
    println!("\x1b[1;37m  * [2] Event Organizer & Pendaftaran Kontingen (/pendaftaran)\x1b[0m");
    println!("\x1b[1;37m  * [3] Technical Delegate & Black Box Audit (/td)\x1b[0m");
    println!("\x1b[1;37m  * [4] Ketua Pertandingan & Meja Gelanggang (/kp)\x1b[0m");
    println!("\x1b[1;37m  * [5] Wasit Juri 1..3 Touchpad (/juri?id=1..3)\x1b[0m");
    println!("\x1b[1;37m  * [6] Dokter Pertandingan & Medis (/medis)\x1b[0m");
    println!("\x1b[1;37m  * [7] Marshalling & Timbangan BLE (/marshalling)\x1b[0m");
    println!("\x1b[1;37m  * [8] Manager Kontingen & Pelatih (/kontingen)\x1b[0m");
    println!("\x1b[1;37m  * [9] Multi-Camera & VAR Replay Control (/camera)\x1b[0m");
    println!("\x1b[1;37m  * [10] Publik, TV Scoreboard, Bagan & Laporan (/scoreboard, /bracket, /laporan)\x1b[0m");
    println!("\x1b[1;36m==============================================================================\x1b[0m\n");
}

fn main() {
    print_banner();

    // 1. Inisialisasi Data Pendaftaran
    let mut reg_engine = RegistrationEngine::baru("KEJURNAS-IPSI-2026", "KEJURNAS PENCAK SILAT INDONESIA 2026");
    reg_engine.daftarkan_kontingen("KTG-JATENG", "Kontingen Jawa Tengah", "Semarang", "Bambang Supriyanto", "08123456789", "jateng@silat.id");
    reg_engine.daftarkan_kontingen("KTG-JATIM", "Kontingen Jawa Timur", "Surabaya", "Suharto Wijaya", "08129876543", "jatim@silat.id");
    reg_engine.daftarkan_kontingen("KTG-JABAR", "Kontingen Jawa Barat", "Bandung", "Asep Sunandar", "08121112223", "jabar@silat.id");
    reg_engine.daftarkan_kontingen("KTG-DKI", "Kontingen DKI Jakarta", "Jakarta", "Rahmat Hidayat", "08124445556", "dki@silat.id");

    reg_engine.konfirmasi_pembayaran_kontingen("KTG-JATENG");
    reg_engine.konfirmasi_pembayaran_kontingen("KTG-JATIM");
    reg_engine.konfirmasi_pembayaran_kontingen("KTG-JABAR");
    reg_engine.konfirmasi_pembayaran_kontingen("KTG-DKI");

    let _ = reg_engine.daftarkan_atlet("ATL-01", "KTG-JATENG", "Eko Prasetyo", JenisKelamin::Putra, "2002-05-12", 22, 68.4, 175.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E (65-70kg)".to_string()));
    let _ = reg_engine.daftarkan_atlet("ATL-02", "KTG-JATIM", "Bambang Sudarsono", JenisKelamin::Putra, "2001-08-20", 23, 67.8, 173.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E (65-70kg)".to_string()));
    let _ = reg_engine.daftarkan_atlet("ATL-03", "KTG-JABAR", "Hendra Gunawan", JenisKelamin::Putra, "2003-02-14", 21, 68.0, 172.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E (65-70kg)".to_string()));
    let _ = reg_engine.daftarkan_atlet("ATL-04", "KTG-DKI", "Ridwan Kamil", JenisKelamin::Putra, "2000-11-09", 24, 69.1, 176.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E (65-70kg)".to_string()));

    reg_engine.verifikasi_atlet("ATL-01");
    reg_engine.verifikasi_atlet("ATL-02");
    reg_engine.verifikasi_atlet("ATL-03");
    reg_engine.verifikasi_atlet("ATL-04");

    // 2. Inisialisasi Data Medali & Nominasi
    let medali_kontingen = vec![
        RekapMedaliKontingen { kontingen_id: "KTG-JATENG".to_string(), nama_kontingen: "Jawa Tengah".to_string(), emas: 4, perak: 2, perunggu: 1, total_poin_klasemen: 0 },
        RekapMedaliKontingen { kontingen_id: "KTG-JATIM".to_string(), nama_kontingen: "Jawa Timur".to_string(), emas: 3, perak: 3, perunggu: 2, total_poin_klasemen: 0 },
        RekapMedaliKontingen { kontingen_id: "KTG-JABAR".to_string(), nama_kontingen: "Jawa Barat".to_string(), emas: 2, perak: 4, perunggu: 3, total_poin_klasemen: 0 },
        RekapMedaliKontingen { kontingen_id: "KTG-DKI".to_string(), nama_kontingen: "DKI Jakarta".to_string(), emas: 1, perak: 1, perunggu: 4, total_poin_klasemen: 0 },
    ];

    let nominasi_terbaik = vec![
        NominasiPesilatTerbaik { atlet_id: "ATL-01".to_string(), nama_atlet: "Eko Prasetyo".to_string(), kontingen: "Jawa Tengah".to_string(), kategori: "Tanding Dewasa Putra Kelas E".to_string(), total_poin_teknik: 28, total_jatuhan: 5, total_tendangan: 8, jumlah_pelanggaran: 0, skor_indeks_kemenangan: 0.0 },
        NominasiPesilatTerbaik { atlet_id: "ATL-02".to_string(), nama_atlet: "Bambang Sudarsono".to_string(), kontingen: "Jawa Timur".to_string(), kategori: "Tanding Dewasa Putra Kelas E".to_string(), total_poin_teknik: 22, total_jatuhan: 3, total_tendangan: 6, jumlah_pelanggaran: 1, skor_indeks_kemenangan: 0.0 },
    ];

    // 3. Inisialisasi Marshalling Call Room & Medis
    let mut marsh_engine = MarshallingEngine::baru();
    marsh_engine.tambah_antrean(1, 1, "ATL-01", "Eko Prasetyo", "Jawa Tengah", "🔴 Merah", 68.4, 65.0, 70.0);
    marsh_engine.tambah_antrean(1, 1, "ATL-02", "Bambang Sudarsono", "Jawa Timur", "🔵 Biru", 67.8, 65.0, 70.0);
    marsh_engine.verifikasi_pelindung("ATL-01");
    marsh_engine.verifikasi_pelindung("ATL-02");

    let med_engine = MedicalEngine::baru();
    let td_engine = DewanHakimEngine::baru("Dr. Ir. H. Sugiyanto, M.Pd.", "IPSI-TD-NASIONAL-001");

    // 4. Inisialisasi Multi-Gelanggang (A s/d F)
    let mut court_manager = MultiCourtManager::baru("TENANT-IPSI-01", "KEJURNAS PENCAK SILAT INDONESIA 2026");
    court_manager.tambah_gelanggang(1, "Gelanggang A", "GOR Jatidiri - Matras Barat");
    court_manager.tambah_gelanggang(2, "Gelanggang B", "GOR Jatidiri - Matras Timur");
    court_manager.tambah_gelanggang(3, "Gelanggang C", "GOR Jatidiri - Matras Utara");
    court_manager.tambah_gelanggang(4, "Gelanggang D", "GOR Jatidiri - Matras Selatan");
    court_manager.tambah_gelanggang(5, "Gelanggang E", "GOR Bulutangkis - Matras Pemasalan 1");
    court_manager.tambah_gelanggang(6, "Gelanggang F", "GOR Bulutangkis - Matras Pemasalan 2");

    // Personel Wasit Juri
    court_manager.daftarkan_personel("P-KP-A", "Drs. H. Mulyono", "WJ-NAS-01", "Wasit Juri Nasional 1", "Jateng", "0812345601");
    court_manager.daftarkan_personel("P-KP-B", "K.H. Ahmad Dahlan", "WJ-NAS-02", "Wasit Juri Nasional 1", "Jatim", "0812345602");
    court_manager.daftarkan_personel("P-KP-C", "Drs. Cecep Hidayat", "WJ-NAS-03", "Wasit Juri Nasional 1", "Jabar", "0812345603");
    court_manager.daftarkan_personel("P-KP-D", "Ir. Bambang Wijaya", "WJ-NAS-04", "Wasit Juri Nasional 1", "DKI", "0812345604");
    court_manager.daftarkan_personel("P-KP-E", "Drs. Slamet Riyadi", "WJ-NAS-05", "Wasit Juri Nasional 2", "DIY", "0812345605");
    court_manager.daftarkan_personel("P-KP-F", "H. Zainal Abidin", "WJ-NAS-06", "Wasit Juri Nasional 2", "Banten", "0812345606");

    court_manager.daftarkan_personel("P-WASIT-01", "Ahmad Fauzi", "WJ-NAS-07", "Wasit Matras", "Jatim", "081235001");
    court_manager.daftarkan_personel("P-JURI-01", "Budi Santoso", "WJ-DAERAH-01", "Wasit Juri Daerah", "Jateng", "081236001");
    court_manager.daftarkan_personel("P-JURI-02", "Cecep Supriatna", "WJ-DAERAH-02", "Wasit Juri Daerah", "Jabar", "081236002");
    court_manager.daftarkan_personel("P-JURI-03", "Dedi Kurniawan", "WJ-DAERAH-03", "Wasit Juri Daerah", "DKI", "081236003");
    court_manager.daftarkan_personel("P-TIMER-01", "Operator Dimas", "OP-01", "Operator IT", "Jateng", "081239001");
    court_manager.daftarkan_personel("P-MEDIS-01", "dr. Anita Rahayu, Sp.KO", "MED-01", "Dokter Pertandingan", "RSUD", "081240001");

    let _ = court_manager.tetapkan_roster_gelanggang(1, "P-KP-A", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");
    let _ = court_manager.tetapkan_roster_gelanggang(2, "P-KP-B", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");
    let _ = court_manager.tetapkan_roster_gelanggang(3, "P-KP-C", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");
    let _ = court_manager.tetapkan_roster_gelanggang(4, "P-KP-D", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");
    let _ = court_manager.tetapkan_roster_gelanggang(5, "P-KP-E", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");
    let _ = court_manager.tetapkan_roster_gelanggang(6, "P-KP-F", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift 1 (08:00 - 12:00)");

    // 5. Inisialisasi Multi-Camera & Bandwidth Manager (4 Kamera per Gelanggang x 6 Gelanggang)
    let mut cam_manager = MultiCameraCourtManager::baru("TENANT-IPSI-01", "KEJURNAS-IPSI-2026", "127.0.0.1");
    for court in 1..=6 {
        cam_manager.daftarkan_kamera(court, CameraAngle::Sudut1DepanKiri, CameraSourceType::WebRtcSmartphone { device_model: format!("HP CAM G{}-1", court), battery_percent: 94 }, 60, 3500);
        cam_manager.daftarkan_kamera(court, CameraAngle::Sudut2DepanKanan, CameraSourceType::WebRtcSmartphone { device_model: format!("HP CAM G{}-2", court), battery_percent: 92 }, 60, 3500);
        cam_manager.daftarkan_kamera(court, CameraAngle::Sudut3BelakangKiri, CameraSourceType::NdiHxBroadcast { ip_address: format!("192.168.1.{}", 50 + court), stream_channel: format!("G{}_BL", court) }, 60, 3500);
        cam_manager.daftarkan_kamera(court, CameraAngle::Sudut4BelakangKanan, CameraSourceType::UsbDirectCapture { device_id: format!("USB_CAP_G{}", court) }, 60, 3500);
    }

    // 6. Engine Pertandingan Live
    let mut engine = ArenaScoringEngine::baru("PARTAI-FINAL-KELAS-E");
    engine.mulai_pertandingan();

    let state = Arc::new(Mutex::new(MultiRoleAppState {
        engine,
        reg_engine,
        med_engine,
        marsh_engine,
        td_engine,
        court_manager,
        auth_engine: AuthEngine::baru(),
        cam_manager,
        medali_kontingen,
        nominasi_terbaik,
        atlet_merah: "Eko Prasetyo".to_string(),
        kontingen_merah: "Kontingen Jawa Tengah".to_string(),
        atlet_biru: "Bambang Sudarsono".to_string(),
        kontingen_biru: "Kontingen Jawa Timur".to_string(),
        kategori: "Tanding Dewasa Putra — Kelas E (65 - 70 kg)".to_string(),
        nomor_partai: "PARTAI-FINAL-01".to_string(),
        log_aksi: vec![
            "🥋 Pertandingan Final Dimulai! Babak 1 Aktif.".to_string(),
        ],
    }));

    let server_state = Arc::clone(&state);
    let port = 8088;
    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("\x1b[1;31m[Galat Socket]: {}\x1b[0m", e);
            return;
        }
    };

    println!("\x1b[1;32m[+] Server Terpadu Ekosistem Sovereign Arena Silat Aktif di: http://127.0.0.1:{}\x1b[0m", port);
    println!("    📺 [1] Gelanggang TV Scoreboard : http://127.0.0.1:{}/scoreboard", port);
    println!("    📷 [2] Multi-Camera & VAR Replay: http://127.0.0.1:{}/camera", port);
    println!("    🏟️  [3] Multi-Gelanggang A-F    : http://127.0.0.1:{}/gelanggang", port);
    println!("    📝 [4] EO & Pendaftaran         : http://127.0.0.1:{}/pendaftaran", port);
    println!("    🎯 [5] Kategori & Drawing       : http://127.0.0.1:{}/kategori", port);
    println!("    🌳 [6] Bagan Pohon 2^n          : http://127.0.0.1:{}/bracket", port);
    println!("    📱 [7] Wasit Juri 1..3          : http://127.0.0.1:{}/juri?id=1", port);
    println!("    ⚖️  [8] Panel KP & Meja Gelanggang: http://127.0.0.1:{}/kp", port);
    println!("    🏛️  [9] Dewan Hakim & TD Audit    : http://127.0.0.1:{}/td", port);
    println!("    🩺 [10] Dokter & Medis          : http://127.0.0.1:{}/medis", port);
    println!("    ⚖️  [11] Marshalling & Timbangan : http://127.0.0.1:{}/marshalling", port);
    println!("    🥋 [12] Portal Kontingen/Coach  : http://127.0.0.1:{}/kontingen", port);
    println!("    👑 [13] Super Admin Platform    : http://127.0.0.1:{}/superadmin", port);
    println!("    📊 [14] Laporan Medali & Neraca : http://127.0.0.1:{}/laporan", port);
    println!("    🏛️ [15] Federasi & Audit IPSI   : http://127.0.0.1:{}/federasi", port);
    println!("    💳 [16] Pembayaran & Split      : http://127.0.0.1:{}/pembayaran", port);
    println!("    ⚡ [17] AI Vision & Biometrik   : http://127.0.0.1:{}/ai-vision", port);
    println!("    🩺 [18] Asuransi BPJS Atlet     : http://127.0.0.1:{}/asuransi", port);
    println!("    🌐 [19] Portal Publik & H2H     : http://127.0.0.1:{}/publik\n", port);

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let st = Arc::clone(&server_state);
                thread::spawn(move || {
                    handle_connection(stream, st);
                });
            }
        }
    });

    println!("\x1b[1;32m==============================================================================\x1b[0m");
    println!("\x1b[1;32m  SISTEM 9-PERAN + MULTI-CAMERA VAR SIAP MELAYANI KEJUARAAN NASIONAL!         \x1b[0m");
    println!("\x1b[1;32m  Buka browser Anda di: http://127.0.0.1:8088/camera                           \x1b[0m");
    println!("\x1b[1;32m==============================================================================\x1b[0m");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
