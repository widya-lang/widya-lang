// Widya-Lang CLI: Native Client Applications (Mobile & Desktop)
// Mencakup WidyaMobile (Android APK, iOS IPA) dan WidyaDesktop (Windows, macOS, Linux).

use clap::Subcommand;
use colored::Colorize;
use std::path::PathBuf;
use std::process;

#[derive(Subcommand)]
pub enum MobileSubcommands {
    /// Inisialisasi proyek aplikasi mobile baru (Template: standard, pos, presensi, dashboard)
    Inisialisasi {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Alias inisialisasi: new / buat
    New {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Alias inisialisasi: buat
    Buat {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Menjalankan server live development & interactive mobile device simulator
    Dev {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Alias dev: jalankan / run
    Jalankan {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Alias dev: run
    Run {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Mengompilasi aplikasi mobile ke paket APK/IPA/Webview
    Bangun {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, default_value = "android")]
        target: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Alias bangun: build
    Build {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, default_value = "android")]
        target: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Audit kesiapan dan periksa konfigurasi aplikasi mobile
    Periksa {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
    /// Alias periksa: check / audit
    Check {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum DesktopSubcommands {
    /// Inisialisasi proyek aplikasi desktop baru
    Inisialisasi {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Alias inisialisasi: new
    New {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Alias inisialisasi: buat
    Buat {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
        #[arg(short, long, default_value = "standard")]
        template: String,
    },
    /// Menjalankan aplikasi desktop langsung dengan Native Window/Webview & Live IPC Bridge
    Jalankan {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value_t = 0)]
        port: u16,
        #[arg(long)]
        dev: bool,
    },
    /// Alias jalankan: run
    Run {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value_t = 0)]
        port: u16,
        #[arg(long)]
        dev: bool,
    },
    /// Mengompilasi dan mengemas aplikasi desktop ke paket distribusi standalone
    Bangun {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value = "windows")]
        platform: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Alias bangun: build
    Build {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value = "windows")]
        platform: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Audit & validasi kesiapan manifest desktop, aset ikon, dan IPC handler
    Periksa {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
    },
    /// Mengemas aplikasi ke installer distribusi produksi (MSIX, DMG, AppImage, Deb, Semua)
    Paket {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value = "semua")]
        format: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Alias paket: package
    Package {
        #[arg(value_name = "BERKAS_ATAU_DIR", default_value = ".")]
        target: PathBuf,
        #[arg(short, long, default_value = "semua")]
        format: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
}


pub fn handle_mobile_command(aksi: &MobileSubcommands) {
    match aksi {
            MobileSubcommands::Inisialisasi { nama, output, template }
            | MobileSubcommands::New { nama, output, template }
            | MobileSubcommands::Buat { nama, output, template } => {
                match widya::mobile::inisialisasi_proyek_mobile_dengan_template(&nama, output.as_deref(), &template) {
                    Ok(p) => {
                        println!("{}", "📱 Proyek Mobile Widya Berhasil Dibuat!".bright_green().bold());
                        println!("📂 Direktori Proyek: {}", p.display().to_string().bright_cyan());
                        println!("🎨 Template Pilihan: {}", template.bright_magenta().bold());
                        println!("💡 Mulai kembangkan aplikasi di: {}/src/main.wya", p.display());
                        println!("⚡ Jalankan simulator live dev: {} mobile dev {}", "widya".bright_yellow(), p.display());
                        println!("🚀 Bangun paket aplikasi: {} mobile bangun {}/src/main.wya --target android", "widya".bright_yellow(), p.display());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuat proyek mobile: {}", e);
                        process::exit(1);
                    }
                }
            }
            MobileSubcommands::Dev { target, port }
            | MobileSubcommands::Jalankan { target, port }
            | MobileSubcommands::Run { target, port } => {
                if let Err(e) = widya::mobile::jalankan_mobile_dev_server(&target, *port) {
                    eprintln!("❌ Gagal menjalankan server mobile simulator: {}", e);
                    process::exit(1);
                }
            }
            MobileSubcommands::Bangun { berkas, target, output }
            | MobileSubcommands::Build { berkas, target, output } => {
                println!("{}", format!("📱 Mengompilasi Aplikasi Mobile untuk Target: {}...", target.to_uppercase()).bright_cyan().bold());
                match widya::mobile::bangun_aplikasi_mobile(&berkas, output.as_deref(), &target) {
                    Ok(out_file) => {
                        println!("{}", "🎉 Kompilasi Aplikasi Mobile Sukses!".bright_green().bold());
                        println!("📦 Berkas Keluaran: {}", out_file.display().to_string().bright_yellow());
                        println!("📱 Siap didistribusikan atau dijalankan di perangkat fisik/emulator.");
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal mengompilasi aplikasi mobile: {}", e);
                        process::exit(1);
                    }
                }
            }
            MobileSubcommands::Periksa { target }
            | MobileSubcommands::Check { target } => {
                match widya::mobile::periksa_aplikasi_mobile(&target) {
                    Ok(rep) => {
                        println!("\n{}", "============================================================".bright_blue());
                        println!("{}", "  📱 AUDIT KESIAPAN WIDYAMOBILE FRAMEWORK (ENTERPRISE)".bright_cyan().bold());
                        println!("============================================================");
                        println!("  Aplikasi       : {}", rep.app_name.bright_white().bold());
                        println!("  Versi          : {}", rep.version);
                        println!("  Target         : {}", rep.platforms.join(", ").bright_yellow());
                        println!("  Izin Terdaftar : {}", rep.permissions.join(", ").bright_green());
                        println!("  Skor Kesiapan  : {}/100", rep.readiness_score.to_string().bright_magenta().bold());
                        println!("  Predikat Mutu  : {}", rep.readiness_grade.bright_green().bold());
                        println!("------------------------------------------------------------");

                        if rep.valid {
                            println!("{}", "  ✅ Status: PROYEK MOBILE VALID & SIAP DEPLOY!".bright_green().bold());
                        } else {
                            println!("{}", format!("  ⚠️  Ditemukan {} Masalah pada Proyek Mobile:", rep.issues.len()).bright_red().bold());
                            for is in &rep.issues {
                                println!("     ❌ {}", is);
                            }
                        }

                        if !rep.recommendations.is_empty() {
                            println!("\n  💡 Rekomendasi Peningkatan:");
                            for r in &rep.recommendations {
                                println!("     - {}", r.bright_yellow());
                            }
                        }
                        println!("============================================================\n");

                        if !rep.valid {
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal memeriksa proyek mobile: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }

pub fn handle_desktop_command(aksi: &DesktopSubcommands) {
    match aksi {
            DesktopSubcommands::Inisialisasi { nama, output, template }
            | DesktopSubcommands::New { nama, output, template }
            | DesktopSubcommands::Buat { nama, output, template } => {
                match widya::desktop::inisialisasi_proyek_desktop(&nama, output.as_deref(), &template) {
                    Ok(p) => {
                        println!("{}", "🖥️  Proyek Desktop Widya Berhasil Dibuat!".bright_green().bold());
                        println!("📂 Direktori Proyek : {}", p.display().to_string().bright_cyan());
                        println!("💡 Mulai kembangkan antarmuka di: {}/ui/index.html", p.display());
                        println!("⚡ Backend logika di           : {}/src/main.wya", p.display());
                        println!("🚀 Jalankan aplikasi desktop   : {} desktop jalankan {}", "widya".bright_yellow(), p.display());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuat proyek desktop: {}", e);
                        process::exit(1);
                    }
                }
            }
            DesktopSubcommands::Jalankan { target, port, dev }
            | DesktopSubcommands::Run { target, port, dev } => {
                if let Err(e) = widya::desktop::jalankan_aplikasi_desktop(&target, *port, *dev) {
                    eprintln!("❌ Gagal menjalankan aplikasi desktop: {}", e);
                    process::exit(1);
                }
            }
            DesktopSubcommands::Bangun { target, platform, output }
            | DesktopSubcommands::Build { target, platform, output } => {
                println!("{}", format!("🖥️  Mengompilasi Aplikasi Desktop untuk Target: {}...", platform.to_uppercase()).bright_cyan().bold());
                match widya::desktop::bangun_aplikasi_desktop(&target, output.as_deref(), &platform) {
                    Ok(out_dir) => {
                        println!("{}", "🎉 Kompilasi & Pemaketan Aplikasi Desktop Sukses!".bright_green().bold());
                        println!("📦 Direktori Distribusi: {}", out_dir.display().to_string().bright_yellow());
                        println!("🚀 Paket siap didistribusikan atau dijalankan secara mandiri.");
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal mengompilasi aplikasi desktop: {}", e);
                        process::exit(1);
                    }
                }
            }
            DesktopSubcommands::Periksa { target } => {
                match widya::desktop::periksa_aplikasi_desktop(&target) {
                    Ok(rep) => {
                        if rep.valid {
                            println!("{}", format!("✅ Proyek Desktop '{}' Valid & Siap Dibangun!", rep.app_name).bright_green().bold());
                            println!("   Versi: {}", rep.version);
                            if !rep.recommendations.is_empty() {
                                println!("\n💡 Saran Peningkatan:");
                                for r in &rep.recommendations {
                                    println!("   - {}", r.bright_yellow());
                                }
                            }
                        } else {
                            println!("{}", format!("⚠️  Ditemukan {} Masalah pada Proyek Desktop:", rep.issues.len()).bright_red().bold());
                            for is in &rep.issues {
                                println!("   ❌ {}", is);
                            }
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal memeriksa proyek desktop: {}", e);
                        process::exit(1);
                    }
                }
            }
            DesktopSubcommands::Paket { target, format, output } | DesktopSubcommands::Package { target, format, output } => {
                match widya::desktop::paketkan_aplikasi_desktop(&target, output.as_deref(), &format) {
                    Ok(pkgs) => {
                        println!("{}", "📦 Sukses Membangun Paket Distribusi Produksi!".bright_green().bold());
                        for p in pkgs {
                            println!("   • Format : {}", p.format.bright_cyan());
                            println!("     Berkas : {}", p.file_path.bright_yellow());
                            println!("     Ukuran : {:.2} MB (SHA256: {}...)", (p.size_bytes as f64) / (1024.0 * 1024.0), &p.sha256_checksum[..16]);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal mengemas aplikasi desktop: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
