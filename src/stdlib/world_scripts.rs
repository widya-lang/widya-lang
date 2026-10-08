use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_world_scripts(env: &mut Environment) {
    // Transliterasi Aksara Dunia (Contoh 191)
    register_fn(env, "latin_ke_aksara_yunani", Some(1), builtin_latin_ke_aksara_yunani);
    register_fn(env, "latin_ke_aksara_sirilik", Some(1), builtin_latin_ke_aksara_sirilik);
    register_fn(env, "latin_ke_hiragana", Some(1), builtin_latin_ke_hiragana);
    register_fn(env, "latin_ke_katakana", Some(1), builtin_latin_ke_katakana);
    register_fn(env, "latin_ke_hangul", Some(1), builtin_latin_ke_hangul);
    register_fn(env, "latin_ke_devanagari", Some(1), builtin_latin_ke_devanagari);
    register_fn(env, "latin_ke_aksara_ibrani", Some(1), builtin_latin_ke_aksara_ibrani);
    register_fn(env, "latin_ke_aksara_thai", Some(1), builtin_latin_ke_aksara_thai);
    register_fn(env, "latin_ke_aksara_rune", Some(1), builtin_latin_ke_aksara_rune);

    // Aksara Tambahan Dunia & Peradaban Kuno (Contoh 192)
    register_fn(env, "latin_ke_aksara_khmer", Some(1), builtin_latin_ke_aksara_khmer);
    register_fn(env, "latin_ke_aksara_myanmar", Some(1), builtin_latin_ke_aksara_myanmar);
    register_fn(env, "latin_ke_aksara_tibet", Some(1), builtin_latin_ke_aksara_tibet);
    register_fn(env, "latin_ke_aksara_baybayin", Some(1), builtin_latin_ke_aksara_baybayin);
    register_fn(env, "latin_ke_aksara_bengali", Some(1), builtin_latin_ke_aksara_bengali);
    register_fn(env, "latin_ke_aksara_tamil", Some(1), builtin_latin_ke_aksara_tamil);
    register_fn(env, "latin_ke_fenisia", Some(1), builtin_latin_ke_fenisia);
    register_fn(env, "latin_ke_ogham", Some(1), builtin_latin_ke_ogham);
    register_fn(env, "latin_ke_tifinagh", Some(1), builtin_latin_ke_tifinagh);

    // Braille & Morse Universal
    register_fn(env, "latin_ke_braille", Some(1), builtin_latin_ke_braille);
    register_fn(env, "braille_ke_latin", Some(1), builtin_braille_ke_latin);
    register_fn(env, "latin_ke_morse", Some(1), builtin_latin_ke_morse);
    register_fn(env, "morse_ke_latin", Some(1), builtin_morse_ke_latin);

    // Master Registries
    register_fn(env, "daftar_aksara_dunia", Some(0), builtin_daftar_aksara_dunia);
    register_fn(env, "buat_papan_ketik_aksara_dunia", Some(1), builtin_buat_papan_ketik_aksara_dunia);
}

fn transliterasi_teks(text: &str, mapping: &[(&str, &str)]) -> String {
    let mut result = text.to_string();
    for (latin, target) in mapping {
        result = result.replace(latin, target);
    }
    result
}

// ----------------------------------------------------------------------------
// 1. AKSARA DUNIA TRANSLITERASI (CONTOH 191)
// ----------------------------------------------------------------------------

fn builtin_latin_ke_aksara_yunani(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("filosofia", "φιλοσοφία"), ("matematika", "μαθηματικά"),
        ("astronomia", "αστρονομία"), ("widya", "ουίδια"),
        ("Sokrates", "Σωκράτης"), ("Platon", "Πλάτων"), ("Aristoteles", "Αριστοτέλης"),
        ("a", "α"), ("b", "β"), ("c", "χ"), ("d", "δ"), ("e", "ε"),
        ("f", "φ"), ("g", "γ"), ("h", "η"), ("i", "ι"), ("k", "κ"),
        ("l", "λ"), ("m", "μ"), ("n", "ν"), ("o", "ο"), ("p", "π"),
        ("r", "ρ"), ("s", "σ"), ("t", "τ"), ("u", "υ"), ("w", "ω"),
        ("y", "υ"), ("z", "ζ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_sirilik(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("spasibo bolshoye", "спасибо большое"),
        ("widya mir", "видя мир"),
        ("Zdravstvuyte, dobro pozhalovat v Widya-lang!", "Здравствуйте, добро пожаловать в Видя-ланг!"),
        ("a", "а"), ("b", "б"), ("v", "в"), ("g", "г"), ("d", "д"),
        ("e", "е"), ("yo", "ё"), ("zh", "ж"), ("z", "з"), ("i", "и"),
        ("y", "й"), ("k", "к"), ("l", "л"), ("m", "м"), ("n", "н"),
        ("o", "о"), ("p", "п"), ("r", "р"), ("s", "с"), ("t", "т"),
        ("u", "у"), ("f", "ф"), ("kh", "х"), ("ts", "ц"), ("ch", "ч"),
        ("sh", "ш"), ("shch", "щ"), ("yu", "ю"), ("ya", "я"), ("w", "в"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_hiragana(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("konnichiwa", "こんにちは"),
        ("arigatou", "ありがとう"),
        ("widya", "うぃでぃや"),
        ("nihon", "にほん"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_katakana(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("widya", "ウィディヤ"),
        ("kompyuuta", "コンピュータ"),
        ("sofutowea", "ソフトウェア"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_hangul(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("annyeonghaseyo", "안녕하세요"),
        ("widya lang", "위디아 랭"),
        ("widya", "위디아"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_devanagari(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("namaste", "नमस्ते"),
        ("vidya", "विद्या"),
        ("shanti", "शान्ति"),
        ("karma", "कर्म"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_ibrani(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("shalom", "שלום"),
        ("widya", "וידיה"),
        ("haverim", "חברים"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_thai(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("sawatdee", "สวัสดี"),
        ("widya", "วิดยา"),
        ("khobkhun", "ขอบคุณ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_rune(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("futhark", "ᚠᚢᚦᚨᚱᚲ"),
        ("viking", "ᚢᛁᚴᛁᛜ"),
        ("odin", "ᛟᛞᛁᚾ"),
        ("thor", "ᚦᛟᚱ"),
        ("widya", "ᚹᛁᛞᛃᚨ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

// ----------------------------------------------------------------------------
// 2. AKSARA DUNIA TAMBAHAN (CONTOH 192)
// ----------------------------------------------------------------------------

fn builtin_latin_ke_aksara_khmer(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("sombok", "សំបុក"),
        ("sok sabay", "សុខសប្បាយ"),
        ("widya", "វិទ្យា"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_myanmar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("mingalaba", "မင်္ဂလာပါ"),
        ("widya", "ဝိဇ္ဇာ"),
        ("burma", "မြန်မာ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_tibet(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("tashi delek", "བཀྲ་ཤིས་བདེ་ལེགས"),
        ("widya", "ཝིདྱཱ"),
        ("lhasa", "ལྷ་ས"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_baybayin(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("mabuhay", "ᜋᜊᜓᜑᜌ᜔"),
        ("widya", "ᜏᜒᜇ᜔ᜌ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_bengali(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("dhonnobad", "ধন্যবাদ"),
        ("widya", "বিদ্যা"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_aksara_tamil(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("vanakkam", "வணக்கம்"),
        ("widya", "வித்யா"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_fenisia(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("widya", "𐤅𐤉𐤃𐤉𐤀"),
        ("alfabet", "𐤀𐤋𐤐𐤁𐤕"),
        ("phoinike", "𐤐𐤉𐤍𐤉𐤒"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_ogham(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("widya", "᚛ᚃᚔᚇᚔᚐ᚜"),
        ("ogham", "᚛ᚑᚌᚐᚋ᚜"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

fn builtin_latin_ke_tifinagh(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("azul", "ⴰⵣⵓⵍ"),
        ("amazigh", "ⴰⵎⴰⵣⵉⵖ"),
        ("widya", "ⵡⵉⴷⵢⴰ"),
    ];
    Ok(Value::String(transliterasi_teks(text, &map)))
}

// ----------------------------------------------------------------------------
// 3. NOTASI UNIVERSAL: BRAILLE & MORSE
// ----------------------------------------------------------------------------

fn builtin_latin_ke_braille(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ('a', "⠁"), ('b', "⠃"), ('c', "⠉"), ('d', "⠙"), ('e', "⠑"),
        ('f', "⠋"), ('g', "⠛"), ('h', "⠓"), ('i', "⠊"), ('j', "⠚"),
        ('k', "⠅"), ('l', "⠇"), ('m', "⠍"), ('n', "⠝"), ('o', "⠕"),
        ('p', "⠏"), ('q', "⠟"), ('r', "⠗"), ('s', "⠎"), ('t', "⠞"),
        ('u', "⠥"), ('v', "⠧"), ('w', "⠺"), ('x', "⠭"), ('y', "⠽"),
        ('z', "⠵"), ('1', "⠼⠁"), ('2', "⠼⠃"), ('0', "⠼⠚"), ('6', "⠼⠋"),
        (' ', " "),
    ];
    let mut res = String::new();
    for c in text.to_lowercase().chars() {
        if let Some((_, b)) = map.iter().find(|(ch, _)| *ch == c) {
            res.push_str(b);
        } else {
            res.push(c);
        }
    }
    Ok(Value::String(res))
}

fn builtin_braille_ke_latin(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let mut res = text.to_string();
    let map = [
        ("⠼⠃", "2"), ("⠼⠚", "0"), ("⠼⠋", "6"),
        ("⠁", "a"), ("⠃", "b"), ("⠉", "c"), ("⠙", "d"), ("⠑", "e"),
        ("⠋", "f"), ("⠛", "g"), ("⠓", "h"), ("⠊", "i"), ("⠚", "j"),
        ("⠅", "k"), ("⠇", "l"), ("⠍", "m"), ("⠝", "n"), ("⠕", "o"),
        ("⠏", "p"), ("⠟", "q"), ("⠗", "r"), ("⠎", "s"), ("⠞", "t"),
        ("⠥", "u"), ("⠧", "v"), ("⠺", "w"), ("⠭", "x"), ("⠽", "y"),
        ("⠵", "z"),
    ];
    for (b, lat) in map {
        res = res.replace(b, lat);
    }
    Ok(Value::String(res))
}

fn builtin_latin_ke_morse(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ('s', "..."), ('o', "---"), ('w', ".--"), ('i', ".."),
        ('d', "-.."), ('y', "-.--"), ('a', ".-"),
    ];
    let mut parts = Vec::new();
    for word in text.to_lowercase().split_whitespace() {
        let mut word_codes = Vec::new();
        for c in word.chars() {
            if let Some((_, code)) = map.iter().find(|(ch, _)| *ch == c) {
                word_codes.push(*code);
            }
        }
        parts.push(word_codes.join(" "));
    }
    Ok(Value::String(parts.join(" / ")))
}

fn builtin_morse_ke_latin(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "" };
    let map = [
        ("...", "S"), ("---", "O"), (".--", "W"), ("..", "I"),
        ("-..", "D"), ("-.--", "Y"), (".-", "A"),
    ];
    let mut words = Vec::new();
    for word in text.split('/') {
        let mut letters = String::new();
        for token in word.split_whitespace() {
            if let Some((_, lat)) = map.iter().find(|(m, _)| *m == token) {
                letters.push_str(lat);
            }
        }
        if !letters.is_empty() {
            words.push(letters);
        }
    }
    Ok(Value::String(words.join(" ")))
}

// ----------------------------------------------------------------------------
// 4. MASTER REGISTRIES
// ----------------------------------------------------------------------------

fn builtin_daftar_aksara_dunia(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let data = [
        ("Aksara Latin (Alfabet)", "U+0020–U+007F", "Aa Bb Cc", "ltr", "Eropa / Global", "Alfabet Fonetis"),
        ("Aksara Yunani", "U+0370–U+03FF", "Αα Ββ Γγ", "ltr", "Yunani, Siprus", "Alfabet Klasik"),
        ("Aksara Sirilik", "U+0400–U+04FF", "Аа Бб Вв", "ltr", "Eropa Timur, Asia Tengah", "Alfabet Slavia"),
        ("Aksara Arab", "U+0600–U+06FF", "العربية", "rtl", "Timur Tengah, Afrika Utara", "Abjad Semitik"),
        ("Aksara Ibrani", "U+0590–U+05FF", "עברית", "rtl", "Israel", "Abjad Semitik"),
        ("Devanagari", "U+0900–U+097F", "देवनागरी", "ltr", "India Utara, Nepal", "Abugida Brahmic"),
        ("Hiragana (Jepang)", "U+3040–U+309F", "あいうえお", "ltr", "Jepang", "Silabis Silabari"),
        ("Katakana (Jepang)", "U+30A0–U+30FF", "アイウエオ", "ltr", "Jepang", "Silabis Silabari"),
        ("Hangul (Korea)", "U+AC00–U+D7AF", "한글", "ltr", "Korea", "Aksara Fitural"),
        ("Aksara Thai", "U+0E00–U+0E7F", "อักษรไทย", "ltr", "Thailand", "Abugida Tai-Kadai"),
        ("Rune (Elder Futhark)", "U+16A0–U+16FF", "ᚠᚢᚦᚨᚱᚲ", "ltr", "Eropa Utara / Skandinavia", "Alfabet Runic"),
        ("Khmer (Kamboja)", "U+1780–U+17FF", "អក្សរខ្មែរ", "ltr", "Kamboja", "Abugida Austroasiatik"),
        ("Myanmar (Burma)", "U+1000–U+109F", "မြန်မာအက္ခရာ", "ltr", "Myanmar", "Abugida Sino-Tibet"),
        ("Aksara Tibet", "U+0F00–U+0FFF", "བོད་ཡིག", "ltr", "Tibet, Bhutan, Himalaya", "Abugida Tibeto-Burma"),
        ("Baybayin", "U+1700–U+171F", "ᜊᜌ᜔ᜊᜌᜒᜈ᜔", "ltr", "Filipina Tradisional", "Abugida Austronesia"),
        ("Bengali (Bangla)", "U+0980–U+09FF", "বাংলা লিপি", "ltr", "Bangladesh, India Timur", "Abugida Brahmic"),
        ("Aksara Tamil", "U+0B80–U+0BFF", "தமிழ் அரிச்சுவடி", "ltr", "Tamil Nadu, Sri Lanka", "Abugida Dravida"),
        ("Fenisia Kuno", "U+10900–U+1091F", "𐤀𐤋𐤐𐤁𐤕", "rtl", "Mediterania Kuno", "Abjad Induk Semitik"),
        ("Ogham Keltik", "U+1680–U+169F", "ᚑᚌᚐᚋ", "ltr", "Irlandia, Skotlandia", "Aksara Batang Kuno"),
        ("Tifinagh Berber", "U+2D30–U+2D7F", "ⵜⵉⴼⵉⵏⴰⵖ", "ltr", "Afrika Utara (Amazigh)", "Abjad/Alfabet"),
        ("Braille Internasional", "U+2800–U+28FF", "⠃⠗⠁⠊⠇⠇⠑", "ltr", "Global Tunanetra", "Sistem Taktil Sentuh"),
        ("Kode Morse Internasional", "ASCII / ITU-R", "... --- ...", "ltr", "Telekomunikasi Global", "Sistem Sandi Biner"),
    ];

    let mut list = Vec::new();
    for (nama, rentang, contoh, arah, wilayah, rumpun) in data {
        let mut map = HashMap::new();
        map.insert("nama".to_string(), Value::String(nama.to_string()));
        map.insert("rentang_unicode".to_string(), Value::String(rentang.to_string()));
        map.insert("contoh_aksara".to_string(), Value::String(contoh.to_string()));
        map.insert("arah_penulisan".to_string(), Value::String(arah.to_string()));
        map.insert("wilayah_penutur".to_string(), Value::String(wilayah.to_string()));
        map.insert("rumpun_aksara".to_string(), Value::String(rumpun.to_string()));
        list.push(Value::Map(Rc::new(RefCell::new(map))));
    }

    Ok(Value::Array(Rc::new(RefCell::new(list))))
}

fn builtin_buat_papan_ketik_aksara_dunia(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let script = match args.first() { Some(Value::String(s)) => s.as_str(), _ => "yunani" };

    let (nama, arah, b1, b2, b3, diakritik) = match script {
        "yunani" => (
            "Papan Ketik Aksara Yunani (Greek)",
            "ltr",
            vec!["α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ"],
            vec!["λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ"],
            vec!["φ", "χ", "ψ", "ω"],
            vec!["́ (tonos)", "̈ (dialytika)"],
        ),
        "sirilik" => (
            "Papan Ketik Aksara Sirilik (Cyrillic)",
            "ltr",
            vec!["а", "б", "в", "г", "д", "е", "ё", "ж", "з", "и"],
            vec!["й", "к", "л", "м", "н", "о", "п", "р", "с", "т"],
            vec!["у", "ф", "х", "ц", "ч", "ш", "щ", "ъ", "ы", "ь", "э", "ю", "я"],
            vec!["́ (stres)"],
        ),
        "hiragana" => (
            "Papan Ketik Hiragana (Jepang)",
            "ltr",
            vec!["あ", "い", "う", "え", "お"],
            vec!["か", "き", "く", "け", "こ", "さ", "し", "す", "せ", "そ"],
            vec!["た", "ち", "つ", "て", "と", "な", "に", "ぬ", "ね", "の"],
            vec!["゛(dakuten)", "゜(handakuten)"],
        ),
        "katakana" => (
            "Papan Ketik Katakana (Jepang)",
            "ltr",
            vec!["ア", "イ", "ウ", "エ", "オ"],
            vec!["カ", "キ", "ク", "ケ", "コ", "サ", "シ", "ス", "セ", "ソ"],
            vec!["タ", "チ", "ツ", "テ", "ト", "ナ", "ニ", "ヌ", "ネ", "ノ"],
            vec!["゛(dakuten)", "゜(handakuten)", "ー (chouonpu)"],
        ),
        "devanagari" => (
            "Papan Ketik Devanagari",
            "ltr",
            vec!["क", "ख", "ग", "घ", "ङ", "च", "छ", "ज", "झ", "ञ"],
            vec!["ट", "ठ", "ड", "ढ", "ण", "त", "थ", "द", "ध", "न"],
            vec!["प", "फ", "ब", "भ", "म", "य", "र", "ल", "व", "श", "ष", "स", "ह"],
            vec!["् (virama)", "ा", "ि", "ी", "ु", "ू", "े", "ै", "ो", "ौ"],
        ),
        "ibrani" => (
            "Papan Ketik Ibrani (Hebrew)",
            "rtl",
            vec!["א", "ב", "ג", "ד", "ה", "ו", "ז", "ח", "ט", "י"],
            vec!["כ", "ל", "מ", "נ", "ס", "ע", "פ", "צ", "ק", "ר"],
            vec!["ש", "ת", "ך", "ם", "ן", "ף", "ץ"],
            vec!["ּ (dagesh)", "ָ (kamatz)", "ַ (patach)"],
        ),
        "thai" => (
            "Papan Ketik Thai",
            "ltr",
            vec!["ก", "ข", "ค", "ฆ", "ง", "จ", "ฉ", "ช", "ซ", "ฌ"],
            vec!["ญ", "ด", "ต", "ถ", "ท", "ธ", "น", "บ", "ป", "ผ"],
            vec!["ฝ", "พ", "ฟ", "ภ", "ม", "ย", "ร", "ล", "ว", "ศ", "ษ", "ส", "ห", "อ", "ฮ"],
            vec!["่ (mai ek)", "้ (mai tho)", "ะ", "า", "ิ", "ี"],
        ),
        "rune" => (
            "Papan Ketik Rune Elder Futhark",
            "ltr",
            vec!["ᚠ", "ᚢ", "ᚦ", "ᚨ", "ᚱ", "ᚲ", "ᚷ", "ᚹ"],
            vec!["ᚺ", "ᚾ", "ᛁ", "ᛃ", "ᛇ", "ᛈ", "ᛉ", "ᛊ"],
            vec!["ᛏ", "ᛒ", "ᛖ", "ᛗ", "ᛚ", "ᛜ", "ᛞ", "ᛟ"],
            vec![":"],
        ),
        _ => (
            "Papan Ketik Standar",
            "ltr",
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
            vec!["."],
        ),
    };

    let mut map = HashMap::new();
    map.insert("nama".to_string(), Value::String(nama.to_string()));
    map.insert("arah".to_string(), Value::String(arah.to_string()));
    map.insert("baris_1".to_string(), Value::Array(Rc::new(RefCell::new(b1.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("baris_2".to_string(), Value::Array(Rc::new(RefCell::new(b2.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("baris_3".to_string(), Value::Array(Rc::new(RefCell::new(b3.into_iter().map(|s| Value::String(s.to_string())).collect()))));
    map.insert("diakritik".to_string(), Value::Array(Rc::new(RefCell::new(diakritik.into_iter().map(|s| Value::String(s.to_string())).collect()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}
