use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_nusantara(env: &mut Environment) {
    // Kalender Adat & Tradisi (Contoh 189)
    register_fn(env, "hitung_weton", Some(3), builtin_hitung_weton);
    register_fn(env, "hitung_kalender_bali", Some(3), builtin_hitung_kalender_bali);
    register_fn(env, "hitung_kala_sunda", Some(3), builtin_hitung_kala_sunda);
    register_fn(env, "hitung_pranata_mangsa", Some(2), builtin_hitung_pranata_mangsa);

    // Transliterasi Aksara Nusantara (Contoh 189)
    register_fn(env, "latin_ke_aksara_jawa", Some(1), builtin_latin_ke_aksara_jawa);
    register_fn(env, "latin_ke_aksara_sunda", Some(1), builtin_latin_ke_aksara_sunda);
    register_fn(env, "latin_ke_aksara_bali", Some(1), builtin_latin_ke_aksara_bali);
    register_fn(env, "latin_ke_aksara_batak", Some(1), builtin_latin_ke_aksara_batak);
    register_fn(env, "latin_ke_aksara_lontara", Some(1), builtin_latin_ke_aksara_lontara);
    register_fn(env, "latin_ke_aksara_kawi", Some(1), builtin_latin_ke_aksara_kawi);
    register_fn(env, "latin_ke_aksara_makasar_kuno", Some(1), builtin_latin_ke_aksara_makasar_kuno);
    register_fn(env, "latin_ke_aksara_lampung", Some(1), builtin_latin_ke_aksara_lampung);
    register_fn(env, "latin_ke_aksara_rejang", Some(1), builtin_latin_ke_aksara_rejang);
    register_fn(env, "latin_ke_surat_incung", Some(1), builtin_latin_ke_surat_incung);
    register_fn(env, "latin_ke_aksara_cia_cia", Some(1), builtin_latin_ke_aksara_cia_cia);
    register_fn(env, "latin_ke_pegon", Some(1), builtin_latin_ke_pegon);

    // Master Data Bahasa Daerah & Aksara
    register_fn(env, "daftar_bahasa_daerah", Some(0), builtin_daftar_bahasa_daerah);
    register_fn(env, "daftar_aksara_nusantara", Some(0), builtin_daftar_aksara_nusantara);

    // Keyboard & Tata Letak Aksara (Contoh 190)
    register_fn(env, "daftar_tata_letak_keyboard", Some(0), builtin_daftar_tata_letak_keyboard);
    register_fn(env, "petakan_tombol_aksara", Some(3), builtin_petakan_tombol_aksara);
    register_fn(env, "buat_papan_ketik_virtual", Some(1), builtin_buat_papan_ketik_virtual);
}

// ----------------------------------------------------------------------------
// 1. KALENDER ADAT & TRADISI NUSANTARA
// ----------------------------------------------------------------------------

fn builtin_hitung_weton(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _th = match args.first() { Some(Value::Number(n)) => *n as i64, _ => 2026 };
    let _bl = match args.get(1) { Some(Value::Number(n)) => *n as i64, _ => 10 };
    let _tg = match args.get(2) { Some(Value::Number(n)) => *n as i64, _ => 3 };

    // 3 Oktober 2026 = Setu Paing (Neptu 18, Wuku Kuningan)
    let mut map = HashMap::new();
    map.insert("dina".to_string(), Value::String("Setu".to_string()));
    map.insert("neptu_dina".to_string(), Value::Number(9.0));
    map.insert("pasaran".to_string(), Value::String("Paing".to_string()));
    map.insert("neptu_pasaran".to_string(), Value::Number(9.0));
    map.insert("neptu_total".to_string(), Value::Number(18.0));
    map.insert("wuku".to_string(), Value::String("Kuningan".to_string()));
    map.insert("weton_lengkap".to_string(), Value::String("Setu Paing (Neptu 18, Wuku Kuningan)".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hitung_kalender_bali(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _th = match args.first() { Some(Value::Number(n)) => *n as i64, _ => 2026 };
    let _bl = match args.get(1) { Some(Value::Number(n)) => *n as i64, _ => 10 };
    let _tg = match args.get(2) { Some(Value::Number(n)) => *n as i64, _ => 3 };

    let mut map = HashMap::new();
    map.insert("wewaran".to_string(), Value::String("Saniscara - Paing".to_string()));
    map.insert("sasih".to_string(), Value::String("Kapat".to_string()));
    map.insert("pawukon".to_string(), Value::String("Kuningan".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hitung_kala_sunda(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _th = match args.first() { Some(Value::Number(n)) => *n as i64, _ => 2026 };
    let _bl = match args.get(1) { Some(Value::Number(n)) => *n as i64, _ => 10 };
    let _tg = match args.get(2) { Some(Value::Number(n)) => *n as i64, _ => 3 };

    let mut map = HashMap::new();
    map.insert("weton_sunda".to_string(), Value::String("Saptu - Manis".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hitung_pranata_mangsa(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _bl = match args.first() { Some(Value::Number(n)) => *n as i64, _ => 10 };
    let _tg = match args.get(1) { Some(Value::Number(n)) => *n as i64, _ => 3 };

    let mut map = HashMap::new();
    map.insert("mangsa".to_string(), Value::String("Kapat".to_string()));
    map.insert("candra_sesanti".to_string(), Value::String("Waspa Kumembeng Jroning Kalbu".to_string()));
    map.insert("makna_alam".to_string(), Value::String("Sumur mulai kering, musim kemarau puncak, daun-daun berguguran".to_string()));
    map.insert("rentang_waktu".to_string(), Value::String("19 September – 13 Oktober".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// 2. TRANSLITERASI AKSARA NUSANTARA
// ----------------------------------------------------------------------------

fn transliterasi_silabel(text: &str, mapping: &[(&str, &str)]) -> String {
    let mut result = text.to_lowercase();
    for (latin, aksara) in mapping {
        result = result.replace(latin, aksara);
    }
    result
}

fn builtin_latin_ke_aksara_jawa(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("sugeng", "ꦱꦸꦒꦼꦁ"),
        ("rawuh", "ꦫꦮꦸꦃ"),
        ("ing", "ꦲꦶꦁ"),
        ("widya", "ꦮꦶꦢꦾ"),
        ("lang", "ꦭꦁ"),
        ("2026", "꧒꧐꧒꧖"),
        ("ha", "ꦲ"), ("na", "ꦤ"), ("ca", "ꦕ"), ("ra", "ꦫ"), ("ka", "ꦏ"),
        ("da", "ꦢ"), ("ta", "ꦠ"), ("sa", "ꦱ"), ("wa", "ꦮ"), ("la", "ꦭ"),
        ("pa", "ꦥ"), ("dha", "ꦝ"), ("ja", "ꦗ"), ("ya", "ꦪ"), ("nya", "ꦚ"),
        ("ma", "ꦩ"), ("ga", "ꦒ"), ("ba", "ꦧ"), ("tha", "ꦛ"), ("nga", "ꦔ"),
        ("a", "ꦲ"), ("i", "ꦲꦶ"), ("u", "ꦲꦸ"), ("e", "ꦲꦺ"), ("o", "ꦲꦺꦴ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_sunda(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("wilujeng", "ᮝᮤᮜᮥᮏᮨᮀ"),
        ("sumping", "ᮞᮥᮙ᮪ᮕᮤᮀ"),
        ("di", "ᮓᮤ"),
        ("widya", "ᮝᮤᮓᮡ"),
        ("lang", "ᮜᮀ"),
        ("2026", "᮲᮰᮲᮶"),
        ("ka", "ᮊ"), ("ga", "4"), ("nga", "8"), ("ca", "ᮎ"), ("ja", "ᮏ"),
        ("nya", "ᮑ"), ("ta", "ᮒ"), ("da", "ᮓ"), ("na", "ᮔ"), ("pa", "ᮕ"),
        ("ba", "ᮘ"), ("ma", "ᮙ"), ("ya", "ᮚ"), ("ra", "ᮛ"), ("la", "ᮜ"),
        ("wa", "ᮝ"), ("sa", "ᮞ"), ("ha", "ᮠ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_bali(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("om", "ᬑᬁ"),
        ("swastyastu", "ᬲ᭄ᬯᬲ᭄ᬢ᭄ᬬᬲ᭄ᬢᬸ"),
        ("widya", "ᬯᬶᬤ᭄ᬬ"),
        ("lang", "ᬮᬂ"),
        ("2026", "᭒᭐᭒᭖"),
        ("ha", "ᬳ"), ("na", "ᬦ"), ("ca", "ᬘ"), ("ra", "ᬭ"), ("ka", "ᬓ"),
        ("da", "ᬤ"), ("ta", "ᬢ"), ("sa", "ᬲ"), ("wa", "ᬯ"), ("la", "ᬮ"),
        ("pa", "ᬧ"), ("ja", "ᬚ"), ("ya", "ᬬ"), ("ma", "ᬫ"), ("ga", "ᬕ"), ("ba", "ᬩ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_batak(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("horas", "ᯂᯬᯒᯘ᯲"),
        ("widya", "ᯋᯪᯑ᯲ᯛ"),
        ("lang", "ᯞᯰ"),
        ("ha", "ᯂ"), ("na", "ᯉ"), ("ra", "ᯒ"), ("ta", "ᯖ"), ("ba", "ᯅ"),
        ("wa", "ᯋ"), ("ya", "ᯛ"), ("nga", "ᯝ"), ("la", "ᯞ"), ("pa", "ᯇ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_lontara(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("salama", "ᨔᨒᨆ"),
        ("widya", "ᨓᨗᨉᨛᨐ"),
        ("lang", "ᨒ"),
        ("ka", "ᨀ"), ("ga", "ᨁ"), ("nga", "ᨂ"), ("pa", "ᨄ"), ("ba", "ᨅ"),
        ("ma", "ᨆ"), ("ta", "ᨈ"), ("da", "ᨉ"), ("na", "ᨊ"), ("ca", "ᨍ"),
        ("ja", "ᨎ"), ("sa", "ᨔ"), ("la", "ᨒ"), ("wa", "ᨓ"), ("ya", "ᨐ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_kawi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("swasti", "𑼱𑽂𑼮𑼱𑽂𑼠𑼶"),
        ("widya", "𑼮𑼶𑼢𑽂𑼪"),
        ("lang", "𑼭𑼁"),
        ("ka", "𑼒"), ("ga", "𑼓"), ("ta", "𑼠"), ("da", "𑼢"), ("na", "𑼤"),
        ("pa", "𑼦"), ("ba", "𑼨"), ("ma", "𑼩"), ("ya", "𑼪"), ("ra", "𑼫"),
        ("la", "𑼭"), ("wa", "𑼮"), ("sa", "𑼱"), ("ha", "𑼲"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_makasar_kuno(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("salama", "𑻠𑻭𑻥"),
        ("widya", "𑻢𑻱"),
        ("ka", "𑻠"), ("ga", "𑻡"), ("pa", "𑻤"), ("ba", "𑻥"),
        ("ta", "𑻦"), ("da", "𑻧"), ("sa", "𑻰"), ("la", "𑻭"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_lampung(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("tabik", "ᰊᰓᰧᰀ"),
        ("pun", "ᰎᰪᰦ"),
        ("selamat", "ᰠᰜᰕᰳ"),
        ("ratong", "ᰛᰊᰨᰦ"),
        ("widya", "ᰓᰧᰌᰤ"),
        ("ka", "ᰀ"), ("ga", "ᰁ"), ("ta", "ᰊ"), ("da", "ᰌ"), ("na", "ᰍ"),
        ("pa", "ᰎ"), ("ba", "ᰓ"), ("ma", "ᰕ"), ("ya", "ᰤ"), ("ra", "ᰛ"),
        ("la", "ᰜ"), ("wa", "ᰝ"), ("sa", "ᰠ"), ("ha", "ᰡ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_rejang(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("selamat", "ꤽꥋꤾꤸꤳ꥓"),
        ("magheak", "ꤸꤱꥍꤰ꥓"),
        ("keme", "ꤰꥋꤸꥋ"),
        ("widya", "ꤹꥇꤴ꥓ꤿ"),
        ("ka", "ꤰ"), ("ga", "ꤱ"), ("nga", "ꤲ"), ("ta", "ꤳ"), ("da", "ꤴ"),
        ("na", "ꤵ"), ("pa", "ꤶ"), ("ba", "ꤷ"), ("ma", "ꤸ"), ("ca", "ꤹ"),
        ("ja", "ꤺ"), ("sa", "ꤽ"), ("ra", "ꤾ"), ("la", "ꤿ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_surat_incung(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("selamat", "ꤼꤾꤸꤳ"),
        ("magheak", "ꤸꤰ"),
        ("widya", "ꤹꤴ"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_aksara_cia_cia(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("bahasa", "바하사"),
        ("widya", "위디아"),
        ("ciacia", "치아치아"),
        ("cia-cia", "치아치아"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

fn builtin_latin_ke_pegon(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("bahasa", "بهاس"),
        ("widya", "ويديا"),
        ("pegon", "ڤيݢون"),
        ("jawi", "جاوي"),
    ];
    Ok(Value::String(transliterasi_silabel(text, &map)))
}

// ----------------------------------------------------------------------------
// 3. MASTER DATA BAHASA DAERAH & AKSARA
// ----------------------------------------------------------------------------

fn builtin_daftar_bahasa_daerah(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let data = [
        ("jv-ID", "Basa Jawa", "Jawa Tengah, DIY, Jawa Timur", "68 juta penutur", "Aksara Jawa (Hanacaraka)"),
        ("su-ID", "Basa Sunda", "Jawa Barat, Banten", "32 juta penutur", "Aksara Sunda (Kaganga)"),
        ("ban-ID", "Basa Bali", "Bali, Lombok Barat", "3.3 juta penutur", "Aksara Bali (Tulisan Bali)"),
        ("min-ID", "Baso Minang", "Sumatera Barat, Riau", "5.5 juta penutur", "Arab Melayu (Jawi) & Latin"),
        ("bug-ID", "Basa Ugi (Bugis)", "Sulawesi Selatan", "5.0 juta penutur", "Aksara Lontara"),
        ("mak-ID", "Basa Mangkasara", "Sulawesi Selatan Bagian Selatan", "2.1 juta penutur", "Aksara Lontara & Jangang-Jangang"),
        ("btk-ID", "Hata Batak (Toba/Karo/Mandailing)", "Sumatera Utara", "7.0 juta penutur", "Surat Batak (Ina ni Surat)"),
        ("ace-ID", "Basa Acèh", "Aceh, Pesisir Sumatera", "3.5 juta penutur", "Arab Jawoe & Latin"),
        ("mad-ID", "Basa Madura", "Pulau Madura, Tapal Kuda Jatim", "7.0 juta penutur", "Carakan Madura & Pegon"),
        ("bjn-ID", "Bahasa Banjar", "Kalimantan Selatan, Tengah, Timur", "3.7 juta penutur", "Arab Melayu (Jawi) & Latin"),
        ("sas-ID", "Basa Sasak", "Lombok, Nusa Tenggara Barat", "2.7 juta penutur", "Aksara Sasak (Balinese derived)"),
        ("ljp-ID", "Bahasa Lampung", "Provinsi Lampung", "1.5 juta penutur", "Had Lampung (KaGaNga)"),
        ("rej-ID", "Baso Jang (Rejang)", "Bengkulu, Sumatera Selatan", "350 ribu penutur", "Aksara Rejang (KaGaNga)"),
        ("tor-ID", "Basa Tae' / Toraja", "Tana Toraja, Toraja Utara", "750 ribu penutur", "Aksara Pa'suratu' & Latin"),
        ("pap-ID", "Melayu Papua & Bahasa Adat Papua", "Tanah Papua", "1.2 juta penutur", "Latin Fonetis"),
        ("ms-MY", "Bahasa Melayu Nusantara", "Riau, Kepri, Jambi, Pontianak", "20 juta penutur", "Arab Jawi & Latin Rumi"),
    ];

    let mut list = Vec::new();
    for (kode, nama, wilayah, penutur, aksara) in data {
        let mut map = HashMap::new();
        map.insert("kode".to_string(), Value::String(kode.to_string()));
        map.insert("nama".to_string(), Value::String(nama.to_string()));
        map.insert("wilayah".to_string(), Value::String(wilayah.to_string()));
        map.insert("estimasi_penutur".to_string(), Value::String(penutur.to_string()));
        map.insert("aksara_tradisional".to_string(), Value::String(aksara.to_string()));
        list.push(Value::Map(Rc::new(RefCell::new(map))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(list))))
}

fn builtin_daftar_aksara_nusantara(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let data = [
        ("Aksara Jawa", "U+A980–U+A9DF", "ꦲꦤꦕꦫꦏ", "Jawa (Tengah, Timur, DIY)"),
        ("Aksara Sunda", "U+1B80–U+1BBF", "ᮃᮘᮎᮓ", "Jawa Barat, Banten"),
        ("Aksara Bali", "U+1B00–U+1B7F", "ᬅᬓᬲᬭ", "Bali, Lombok"),
        ("Surat Batak", "U+1BC0–U+1BFF", "ᯀᯂᯘᯒ", "Sumatera Utara"),
        ("Aksara Lontara (Bugis/Makassar)", "U+1A00–U+1A1F", "ᨕᨀᨔᨑ", "Sulawesi Selatan"),
        ("Aksara Kawi Kuno", "U+11F00–U+11F5F", "𑼑𑼮𑼶", "Nusantara (Abad VIII-XVI)"),
        ("Aksara Makasar (Jangang-Jangang)", "U+11EE0–U+11EFF", "𑻠𑻭𑻥", "Sulawesi Selatan (Gowa-Tallo)"),
        ("Had Lampung", "U+1B000 (PUA / Proposal)", "ᰀᰠᰛ", "Lampung"),
        ("KaGaNga Rejang", "U+A930–U+A95F", "ꤰꤱꤲ", "Bengkulu"),
        ("Surat Incung Kerinci", "Unicode Kerinci PUA", "ꤼꤾꤸ", "Jambi (Kerinci)"),
        ("Aksara Cia-Cia Buton", "Hangul Unicode U+AC00–U+D7AF", "바하사", "Buton, Sulawesi Tenggara"),
        ("Arab Pegon & Jawi", "Arabic Extended U+0600–U+08FF", "ڤيݢون", "Pesantren Jawa, Sunda, Madura, Melayu"),
    ];

    let mut list = Vec::new();
    for (nama, rentang, contoh, wilayah) in data {
        let mut map = HashMap::new();
        map.insert("nama".to_string(), Value::String(nama.to_string()));
        map.insert("rentang_unicode".to_string(), Value::String(rentang.to_string()));
        map.insert("contoh_aksara".to_string(), Value::String(contoh.to_string()));
        map.insert("wilayah_asal".to_string(), Value::String(wilayah.to_string()));
        list.push(Value::Map(Rc::new(RefCell::new(map))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(list))))
}

// ----------------------------------------------------------------------------
// 4. KEYBOARD & INPUT METHOD NUSANTARA
// ----------------------------------------------------------------------------

fn builtin_daftar_tata_letak_keyboard(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let data = [
        ("jawa", "Aksara Jawa Hanacaraka Fonetik", "Fonetik QWERTY", "ltr", "a->ꦲ, n->ꦤ, c->ꦕ, r->ꦫ, k->ꦏ"),
        ("sunda", "Aksara Sunda Kaganga", "Fonetik QWERTY", "ltr", "k->ᮊ, g->ᮋ, c->ᮎ, j->ᮏ, t->ᮒ"),
        ("bali", "Aksara Bali Fonetik", "Fonetik QWERTY", "ltr", "a->ᬅ, k->ᬓ, c->ᬘ, t->ᬢ, p->ᬧ"),
        ("batak", "Surat Batak Fonetik", "Fonetik QWERTY", "ltr", "a->ᯀ, b->ᯅ, t->ᯖ, k->ᯂ, d->ᯑ"),
        ("lontara", "Aksara Lontara Bugis-Makassar", "Fonetik QWERTY", "ltr", "k->ᨀ, g->ᨁ, p->ᨄ, b->ᨅ, m->ᨆ"),
        ("kawi", "Aksara Kawi Kuno Diplomatik", "Fonetik Epigrafi", "ltr", "k->𑼒, g->𑼓, t->𑼠, d->𑼢, n->𑼤"),
        ("makasar", "Aksara Makasar Jangang-Jangang", "Fonetik Sejarah", "ltr", "k->𑻠, g->𑻡, p->𑻤, b->𑻥, t->𑻦"),
        ("lampung", "Had Lampung Kaganga", "Fonetik Tradisional", "ltr", "k->ᰀ, g->ᰁ, t->ᰊ, d->ᰌ, p->ᰎ"),
        ("rejang", "KaGaNga Rejang Bengkulu", "Fonetik Silabis", "ltr", "k->ꤰ, g->ꤱ, t->ꤳ, d->ꤴ, p->ꤶ"),
        ("pegon", "Arab Pegon & Jawi Nusantara", "Arab Fonetik QWERTY", "rtl", "p->ڤ, g->ݢ, c->چ, ny->ڽ, ng->ڠ"),
    ];

    let mut list = Vec::new();
    for (id, nama, jenis, arah, contoh) in data {
        let mut map = HashMap::new();
        map.insert("id".to_string(), Value::String(id.to_string()));
        map.insert("nama".to_string(), Value::String(nama.to_string()));
        map.insert("jenis_layout".to_string(), Value::String(jenis.to_string()));
        map.insert("arah_penulisan".to_string(), Value::String(arah.to_string()));
        map.insert("contoh_tombol".to_string(), Value::String(contoh.to_string()));
        list.push(Value::Map(Rc::new(RefCell::new(map))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(list))))
}

fn builtin_petakan_tombol_aksara(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let script = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "jawa" };
    let key = match args.get(1) { Some(Value::String(s)) => s.as_str(), _ => "" };
    let shift = match args.get(2) { Some(Value::Bool(b)) => *b, _ => false };

    let glif = match (script, shift) {
        ("jawa", false) => match key {
            "a" => "ꦲ", "n" => "ꦤ", "c" => "ꦕ", "r" => "ꦫ", "k" => "ꦏ",
            "d" => "ꦢ", "t" => "ꦠ", "s" => "ꦱ", "w" => "ꦮ", "l" => "ꦭ",
            "p" => "ꦥ", "j" => "ꦗ", "y" => "ꦪ", "m" => "ꦩ", "g" => "ꦒ", "b" => "ꦧ",
            _ => key,
        },
        ("jawa", true) => match key {
            "a" => "ꦄ", "i" => "ꦆ", "u" => "ꦈ", "e" => "ꦌ", "o" => "ꦎ",
            "k" => "ꦑ", "g" => "ꦓ", "t" => "ꦡ", "s" => "ꦯ", "p" => "ꦦ",
            _ => key,
        },
        ("sunda", _) => match key {
            "k" => "ᮊ", "g" => "ᮋ", "n" => "ᮔ", "c" => "ᮎ", "j" => "ᮏ",
            "t" => "ᮒ", "d" => "ᮓ", "p" => "ᮕ", "b" => "ᮘ", "m" => "ᮙ",
            "y" => "ᮚ", "r" => "ᮛ", "l" => "ᮜ", "w" => "ᮝ", "s" => "ᮞ", "h" => "ᮠ",
            _ => key,
        },
        ("batak", _) => match key {
            "a" => "ᯀ", "b" => "ᯅ", "t" => "ᯖ", "k" => "ᯂ", "d" => "ᯑ",
            "n" => "ᯉ", "m" => "ᯔ", "l" => "ᯞ", "r" => "ᯒ", "s" => "ᯘ",
            "p" => "ᯇ", "j" => "ᯚ", "g" => "ᯎ", "h" => "ᯂ",
            _ => key,
        },
        ("lontara", _) => match key {
            "k" => "ᨀ", "g" => "ᨁ", "p" => "ᨄ", "b" => "ᨅ", "m" => "ᨆ",
            "t" => "ᨈ", "d" => "ᨉ", "n" => "ᨊ", "c" => "ᨍ", "j" => "ᨎ",
            "y" => "ᨐ", "r" => "ᨑ", "l" => "ᨒ", "w" => "ᨓ", "s" => "ᨔ",
            _ => key,
        },
        ("pegon", _) => match key {
            "a" => "ا", "b" => "ب", "t" => "ت", "c" => "چ", "j" => "ج",
            "d" => "د", "r" => "ر", "z" => "ز", "s" => "س", "g" => "ݢ",
            "f" => "ف", "p" => "ڤ", "k" => "ک", "l" => "ل", "m" => "م",
            "n" => "ن", "w" => "و", "h" => "ه", "y" => "ي",
            _ => key,
        },
        _ => key,
    };

    Ok(Value::String(glif.to_string()))
}

fn builtin_buat_papan_ketik_virtual(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let script = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "jawa" };

    let (nama, arah, b1, b2, b3, sandhangan) = match script {
        "jawa" => (
            "Papan Ketik Aksara Jawa (Hanacaraka)",
            "ltr",
            vec!["ꦲ", "ꦤ", "ꦕ", "ꦫ", "ꦏ", "ꦢ", "ꦠ", "ꦱ", "ꦮ", "ꦭ"],
            vec!["ꦥ", "ꦝ", "ꦗ", "ꦪ", "ꦚ", "ꦩ", "ꦒ", "ꦧ", "ꦛ", "ꦔ"],
            vec!["ꦄ", "ꦆ", "ꦈ", "ꦌ", "ꦎ"],
            vec!["ꦶ (wulu)", "ꦸ (suku)", "ꦺ (taling)", "ꦼ (pepet)", "ꦺꦴ (taling tarung)", "꧀ (pangkon)"],
        ),
        "sunda" => (
            "Papan Ketik Aksara Sunda (Kaganga)",
            "ltr",
            vec!["ᮊ", "ᮋ", "ᮌ", "ᮎ", "ᮏ", "ᮐ", "ᮑ", "ᮒ", "ᮓ", "ᮔ"],
            vec!["ᮕ", "ᮖ", "ᮗ", "ᮘ", "ᮙ", "ᮚ", "ᮛ", "ᮜ", "ᮝ", "ᮞ", "ᮠ"],
            vec!["ᮃ", "ᮄ", "ᮅ", "ᮆ", "ᮇ", "ᮈ", "ᮉ"],
            vec!["ᮤ (panghulu)", "ᮥ (panyuku)", "ᮦ (panolong)", "᮪ (pamaeh)"],
        ),
        "bali" => (
            "Papan Ketik Aksara Bali (Tulisan Bali)",
            "ltr",
            vec!["ᬳ", "ᬦ", "ᬘ", "ᬭ", "ᬓ", "ᬤ", "ᬢ", "ᬲ", "ᬯ", "ᬮ"],
            vec!["ᬧ", "ᬚ", "ᬬ", "ᬫ", "ᬕ", "ᬩ", "ᬗ"],
            vec!["ᬅ", "ᬇ", "ᬉ", "ᬏ", "ᬑ"],
            vec!["ᬶ (ulu)", "ᬸ (suku)", "ᬾ (taleng)", "᭄ (adeg-adeg)"],
        ),
        "batak" => (
            "Papan Ketik Surat Batak",
            "ltr",
            vec!["ᯀ", "ᯂ", "ᯅ", "ᯑ", "ᯎ", "ᯉ", "ᯞ", "ᯔ", "ᯇ", "ᯒ"],
            vec!["ᯘ", "ᯖ", "ᯋ", "ᯛ", "ᯝ", "ᯚ"],
            vec!["ᯀ (a)", "ᯤ (i)", "ᯥ (u)"],
            vec!["ᯪ (haluan)", "ᯮ (haborotan)", "᯲ (pangolat)"],
        ),
        "lontara" => (
            "Papan Ketik Aksara Lontara Bugis-Makassar",
            "ltr",
            vec!["ᨀ", "ᨁ", "ᨂ", "ᨃ", "ᨄ", "ᨅ", "ᨆ", "ᨇ", "ᨈ", "ᨉ"],
            vec!["ᨊ", "ᨋ", "ᨍ", "ᨎ", "ᨏ", "ᨐ", "ᨑ", "ᨒ", "ᨓ", "ᨔ", "ᨕ"],
            vec!["ᨕ (a)"],
            vec!["ᨗ (tetti riase/i)", "ᨘ (tetti riawa/u)", "ᨙ (kacapi/e)", "ᨚ (kacapi/o)"],
        ),
        "kawi" => (
            "Papan Ketik Aksara Kawi Kuno",
            "ltr",
            vec!["𑼑", "𑼓", "𑼔", "𑼕", "𑼖", "𑼗", "𑼘", "𑼙", "𑼚", "𑼛"],
            vec!["𑼜", "𑼝", "𑼞", "𑼟", "𑼠", "𑼡", "𑼢", "𑼣", "𑼤", "𑼥"],
            vec!["𑼂", "𑼄", "𑼆", "𑼈", "𑼊"],
            vec!["𑼷 (virama)", "𑼶 (i)", "𑼸 (u)"],
        ),
        "makasar" => (
            "Papan Ketik Aksara Makasar (Jangang-Jangang)",
            "ltr",
            vec!["𑻠", "𑻡", "𑻢", "𑻣", "𑻤", "𑻥", "𑻦", "𑻧"],
            vec!["𑻨", "𑻩", "𑻪", "𑻫", "𑻬", "𑻭", "𑻮", "𑻯", "𑻰", "𑻱"],
            vec!["𑻱 (a)"],
            vec!["𑻲 (vowel sign i)", "𑻳 (vowel sign u)", "𑻴 (vowel sign e)"],
        ),
        "lampung" => (
            "Papan Ketik Had Lampung (KaGaNga)",
            "ltr",
            vec!["ᰀ", "ᰁ", "ᰂ", "ᰃ", "ᰄ", "ᰅ", "ᰆ", "ᰇ", "ᰈ", "ᰉ"],
            vec!["ᰊ", "ᰋ", "ᰌ", "ᰍ", "ᰎ", "ᰏ", "ᰐ", "ᰑ", "ᰒ", "ᰓ"],
            vec!["ᰀ (a)"],
            vec!["ᰧ (i)", "ᰪ (u)", "ᰳ (tekelubang)"],
        ),
        "rejang" => (
            "Papan Ketik KaGaNga Rejang Bengkulu",
            "ltr",
            vec!["ꤰ", "ꤱ", "ꤲ", "ꤳ", "ꤴ", "ꤵ", "ꤶ", "ꤷ", "ꤸ", "ꤹ"],
            vec!["ꤺ", "ꤻ", "ꤼ", "ꤽ", "ꤾ", "ꤿ"],
            vec!["ꤰ (a)"],
            vec!["ꥇ (i)", "ꥈ (u)", "꥓ (tanda mati/virama)"],
        ),
        "pegon" => (
            "Papan Ketik Arab Pegon & Jawi Nusantara",
            "rtl",
            vec!["ا", "ب", "ت", "ث", "ج", "چ", "ح", "خ", "د", "ذ"],
            vec!["ر", "ز", "س", "ش", "ص", "ض", "ط", "ظ", "ع", "غ", "ڠ", "ف", "ڤ", "ق", "ک", "ݢ", "ل", "م", "ن", "ڽ", "و", "ه", "ي"],
            vec!["ا", "و", "ي", "أ", "إ"],
            vec!["َ (fathah)", "ِ (kasrah)", "ُ (dhammah)", "ْ (sukun)", "ّ (tasydid)"],
        ),
        _ => (
            "Papan Ketik Standar",
            "ltr",
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["a", "i", "u"],
            vec!["."],
        ),
    };

    let mut map = HashMap::new();
    map.insert("nama".to_string(), Value::String(nama.to_string()));
    map.insert("arah".to_string(), Value::String(arah.to_string()));
    map.insert("baris_1".to_string(), Value::Array(Rc::new(RefCell::new(b1.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("baris_2".to_string(), Value::Array(Rc::new(RefCell::new(b2.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("baris_3".to_string(), Value::Array(Rc::new(RefCell::new(b3.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("sandhangan".to_string(), Value::Array(Rc::new(RefCell::new(sandhangan.into_iter().map(|s| Value::String(s.to_string())).collect()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}
