use widya::graphics::{
    BatasanKotak, FontAtlasMurni, FramebufferMurni, GayaTipografi, MetrikTeks,
    PerataanTeks, PersegiKotak, RenderKotakMurni, Titik2D, VektorPath, Warna, WidgetMurni,
};

#[test]
fn test_pure_canvas_color_and_alpha_blending() {
    let red = Warna::rgba(255, 0, 0, 255);
    let semi_blue = Warna::rgba(0, 0, 255, 128);
    let hex_color = Warna::dari_hex("#10B981");

    assert_eq!(hex_color.r, 0x10);
    assert_eq!(hex_color.g, 0xB9);
    assert_eq!(hex_color.b, 0x81);
    assert_eq!(hex_color.a, 255);

    let blended = Warna::blend_src_over(semi_blue, red);
    assert!(blended.b > 100);
    assert!(blended.r > 100);
}

#[test]
fn test_pure_canvas_framebuffer_and_primitives() {
    let mut fb = FramebufferMurni::baru(320, 240);
    fb.bersihkan(Warna::PUTIH);

    // Test gambar kotak dan kotak bulat
    fb.gambar_kotak(10.0, 10.0, 100.0, 50.0, Warna::MERAH);
    assert_eq!(fb.ambil_piksel(20, 20), Warna::MERAH);

    fb.gambar_kotak_bulat(50.0, 50.0, 120.0, 80.0, 12.0, Warna::BIRU);
    assert_eq!(fb.ambil_piksel(60, 60), Warna::BIRU);

    // Test gambar garis & lingkaran
    fb.gambar_garis(0.0, 0.0, 100.0, 100.0, Warna::HITAM);
    fb.gambar_lingkaran(160.0, 120.0, 30.0, Warna::HIJAU);
    assert_eq!(fb.ambil_piksel(160, 120), Warna::HIJAU);

    // Test ekspor PPM format
    let ppm = fb.ekspor_ke_ppm();
    assert!(ppm.starts_with(b"P6\n320 240\n255\n"));
    assert_eq!(ppm.len(), (b"P6\n320 240\n255\n".len() + 320 * 240 * 3));
}

#[test]
fn test_pure_font_metrics_and_rasterization() {
    let mut fb = FramebufferMurni::baru(300, 100);
    fb.bersihkan(Warna::PUTIH);

    let mut gaya = GayaTipografi::default();
    gaya.ukuran_font = 16.0;
    gaya.warna = Warna::HITAM;

    let metrik = FontAtlasMurni::hitung_metrik("Widya Sovereign", &gaya);
    assert!(metrik.lebar > 100.0);
    assert!(metrik.tinggi >= 16.0);

    FontAtlasMurni::gambar_teks(&mut fb, "Widya OS", 20.0, 20.0, &gaya);
}

#[test]
fn test_pure_three_tree_reactive_ui_layout_and_paint() {
    let mut fb = FramebufferMurni::baru(400, 600);
    fb.bersihkan(Warna::rgb(245, 245, 247));

    // Bangun Pohon Widget Deklaratif (Setara Flutter Scaffold + Column + Container)
    let card_widget = WidgetMurni::kontainer(
        "card_omzet",
        Some(360.0),
        Some(120.0),
        Some(Warna::rgb(255, 255, 255)),
        16.0,
        Some(WidgetMurni::kolom(
            "card_col",
            vec![
                WidgetMurni::teks("lbl_title", "Ringkasan Penjualan:", 14.0, Warna::rgb(100, 100, 100)),
                WidgetMurni::teks("lbl_total", "Rp 15.450.000", 20.0, Warna::rgb(16, 185, 129)),
                WidgetMurni::tombol("btn_bayar", "Bayar dengan QRIS", Warna::rgb(37, 99, 235), "AKSI_BAYAR"),
            ],
            8.0,
        )),
    );

    let app_widget = WidgetMurni::scaffold(
        "root_scaffold",
        "WidyaMobile Store",
        Warna::rgb(15, 23, 42),
        WidgetMurni::kontainer(
            "body_container",
            None,
            None,
            None,
            0.0,
            Some(WidgetMurni::kolom("body_col", vec![card_widget], 16.0)),
        ),
    );

    // Hitung Tata Letak BoxConstraints 1-Pass
    let root_batasan = BatasanKotak::ketat(400.0, 600.0);
    let mut root_render = RenderKotakMurni::baru(&app_widget.id, root_batasan);
    root_render.tata_letak(&app_widget, root_batasan);

    assert_eq!(root_render.lebar, 400.0);
    assert_eq!(root_render.tinggi, 600.0);
    assert!(!root_render.anak.is_empty());

    // Paint ke Framebuffer Murni
    root_render.cat(&app_widget, &mut fb);

    // Verifikasi warna AppBar di bagian atas (y=10)
    let appbar_pixel = fb.ambil_piksel(50, 10);
    assert_eq!(appbar_pixel, Warna::rgb(15, 23, 42));
}

#[test]
fn test_pure_events_hit_test_and_animation_engine() {
    use widya::graphics::{HitTestDispatcher, JenisKurva, PengontrolAnimasi, StatusAnimasi, Tween};

    // 1. Uji Hit-Testing Sentuhan
    let button_widget = WidgetMurni::tombol("btn_login", "Masuk", Warna::MERAH, "AKSI_LOGIN");
    let root_scaffold = WidgetMurni::scaffold(
        "root_app",
        "Aplikasi Ku",
        Warna::BIRU,
        WidgetMurni::kontainer(
            "btn_wrapper",
            Some(300.0),
            Some(200.0),
            None,
            0.0,
            Some(button_widget),
        ),
    );

    let batasan = BatasanKotak::ketat(360.0, 640.0);
    let mut render_tree = RenderKotakMurni::baru(&root_scaffold.id, batasan);
    render_tree.tata_letak(&root_scaffold, batasan);

    // Sentuh di koordinat AppBar (y=20) -> Harus mengenai root_app
    let hit_appbar = HitTestDispatcher::temukan_target(&render_tree, 100.0, 20.0);
    assert_eq!(hit_appbar, Some("root_app"));

    // Sentuh di koordinat Tombol (y=70, x=50) -> Harus mengenai btn_login
    let hit_btn = HitTestDispatcher::temukan_target(&render_tree, 50.0, 70.0);
    assert_eq!(hit_btn, Some("btn_login"));

    // 2. Uji Kurva Easing Animasi & Tween
    let tween_pos = Tween::baru(0.0, 100.0, JenisKurva::EaseOut);
    let val_half = tween_pos.transformasikan(0.5);
    assert!(val_half > 50.0, "EaseOut harus lebih cepat di awal (>50 pada t=0.5): {}", val_half);

    let tween_warna = Tween::warna(Warna::HITAM, Warna::PUTIH, JenisKurva::Linear);
    let warna_tengah = tween_warna.transformasikan(0.5);
    assert_eq!(warna_tengah.r, 127);
    assert_eq!(warna_tengah.g, 127);
    assert_eq!(warna_tengah.b, 127);

    // 3. Uji Pengontrol Animasi 60 FPS
    let mut ctrl = PengontrolAnimasi::baru("ctrl_fade", 500);
    ctrl.maju();
    assert_eq!(ctrl.status, StatusAnimasi::Maju);

    // Simulasi 15 frame @ 16.6ms (~250ms) -> Nilai t harus ~0.5
    for _ in 0..15 {
        ctrl.perbarui(16);
    }
    assert!(ctrl.nilai_t >= 0.45 && ctrl.nilai_t <= 0.55, "Nilai t harus di sekitar 0.5: {}", ctrl.nilai_t);

    // Simulasi hingga selesai 500ms
    for _ in 0..20 {
        ctrl.perbarui(16);
    }
    assert_eq!(ctrl.status, StatusAnimasi::Selesai);
    assert_eq!(ctrl.nilai_t, 1.0);
}

#[test]
fn test_material3_and_cupertino_widgets() {
    use widya::graphics::WidyaWidgets;

    let mut fb = FramebufferMurni::baru(380, 700);
    fb.bersihkan(Warna::rgb(240, 242, 245));

    // Bangun Layout lengkap dengan Material 3 & Cupertino Widgets
    let card = WidyaWidgets::card(
        "user_card",
        WidgetMurni::kolom(
            "card_body",
            vec![
                WidyaWidgets::badge("badge_vip", "VIP MEMBER", Warna::rgb(245, 158, 11)),
                WidyaWidgets::text_field("input_nama", "Masukkan Nama Lengkap...", "Budi Santoso"),
                WidyaWidgets::switch("sw_notif", true, "TOGGLE_NOTIF"),
                WidyaWidgets::checkbox("cb_setuju", true, "Saya setuju dengan Syarat & Ketentuan", "TOGGLE_TOS"),
                WidyaWidgets::progress_bar("sync_bar", 75.0, Warna::rgb(37, 99, 235)),
                WidyaWidgets::elevated_button("btn_submit", "Kirim Data Sekarang", Warna::rgb(16, 185, 129), "SUBMIT_FORM"),
            ],
            10.0,
        ),
        Some(Warna::rgb(255, 255, 255)),
        16.0,
    );

    let nav_bar = WidyaWidgets::cupertino_nav_bar("nav_bar", "Pengaturan Akun", Some("NAV_BACK"));

    let screen = WidgetMurni::scaffold(
        "main_screen",
        "Dashboard",
        Warna::rgb(30, 41, 59),
        WidgetMurni::kolom("screen_col", vec![nav_bar, card], 16.0),
    );

    let batasan = BatasanKotak::ketat(380.0, 700.0);
    let mut render_tree = RenderKotakMurni::baru(&screen.id, batasan);
    render_tree.tata_letak(&screen, batasan);

    assert_eq!(render_tree.lebar, 380.0);
    assert_eq!(render_tree.tinggi, 700.0);

    // Paint ke Framebuffer
    render_tree.cat(&screen, &mut fb);
}

#[test]
fn test_reactive_signals_and_navigator_router() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use widya::graphics::{PemberitahuNilai, RuteHalaman, Sinyal, WidyaNavigator};

    // 1. Uji Reactive Signal & ValueNotifier
    let hitungan_sinyal = Sinyal::baru("counter", 10);
    assert_eq!(hitungan_sinyal.baca(), 10);

    let notify_count = Arc::new(AtomicU32::new(0));
    let nc_clone = notify_count.clone();
    hitungan_sinyal.notifier.dengarkan(move |_val| {
        nc_clone.fetch_add(1, Ordering::SeqCst);
    });

    hitungan_sinyal.tulis(25);
    assert_eq!(hitungan_sinyal.baca(), 25);
    assert_eq!(notify_count.load(Ordering::SeqCst), 1);

    // 2. Uji Navigator Push/Pop Stack
    let home_page = RuteHalaman {
        nama_rute: "/beranda".to_string(),
        widget_halaman: WidgetMurni::teks("home_txt", "Selamat Datang di Beranda", 16.0, Warna::HITAM),
        parameter: HashMap::new(),
    };

    let mut nav = WidyaNavigator::baru(home_page);
    assert_eq!(nav.halaman_aktif().unwrap().nama_rute, "/beranda");

    let detail_page = RuteHalaman {
        nama_rute: "/produk/102".to_string(),
        widget_halaman: WidgetMurni::teks("detail_txt", "Detail Produk Kopi", 16.0, Warna::HITAM),
        parameter: HashMap::new(),
    };

    nav.push(detail_page);
    assert_eq!(nav.halaman_aktif().unwrap().nama_rute, "/produk/102");

    let popped = nav.pop();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().nama_rute, "/produk/102");
    assert_eq!(nav.halaman_aktif().unwrap().nama_rute, "/beranda");

    // 3. Uji URL Pattern Matching / Deep Linking
    let params = WidyaNavigator::cocokan_pola_url("/produk/:kategori/:id", "/produk/kopi/449").unwrap();
    assert_eq!(params.get("kategori").unwrap(), "kopi");
    assert_eq!(params.get("id").unwrap(), "449");
}
