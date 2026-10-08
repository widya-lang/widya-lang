// ============================================================================
// WIDYABASE FULL-TEXT SEARCH & INDONESIAN STEMMER ENGINE (fulltext.rs)
// ============================================================================
// Menyediakan pengindeksan teks terbalik (Inverted Index), pemeringkatan BM25,
// stemmer morfologi Bahasa Indonesia (Algoritma Nazief-Adriani / Sastrawi),
// tokenisasi N-Gram (autocomplete), serta pencarian typo-tolerant (Damerau-Levenshtein).
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// INDONESIAN MORPHOLOGICAL STEMMER (ALGORITMA NAZIEF-ADRIANI & SASTRAWI)
// ============================================================================

/// Pengupas kata berimbuhan Bahasa Indonesia ke kata dasar
pub struct IndonesianStemmer {
    dictionary: HashSet<String>,
}

impl IndonesianStemmer {
    pub fn new() -> Self {
        let mut dict = HashSet::new();
        // Kamus kata dasar fundamental Bahasa Indonesia
        let roots = [
            "ajar", "ambil", "antar", "atur", "baca", "bagi", "banting", "bantu", "bayar", "beli",
            "bentur", "buat", "buka", "cari", "catat", "cetak", "daftar", "dapat", "daya", "didik",
            "ganti", "guna", "hambat", "hantar", "hapus", "harap", "hitung", "hubung", "ikat",
            "ikut", "ingat", "isi", "jaga", "jalan", "jual", "jatuh", "juang", "kaji", "kembang",
            "kenang", "kerja", "kirim", "kunci", "lapor", "layat", "lepas", "lihat", "lompat",
            "main", "makan", "mampu", "masuk", "minum", "minta", "mulai", "olahrag", "pantau",
            "pasang", "pegang", "pelihara", "pilih", "pukul", "putus", "rancang", "rawat", "rebut",
            "saji", "sambung", "sampai", "sapa", "sapu", "satu", "sayang", "sebut", "sehat", "selesai",
            "serang", "siap", "siar", "simpan", "siram", "tambah", "tampak", "tanding", "tanam",
            "tangkap", "tarik", "tekan", "tendang", "tentu", "terbang", "terima", "tetap", "tingkat",
            "tolak", "tonton", "tulis", "tumbuh", "tunjuk", "turun", "tutup", "ubah", "ukur", "ulang",
            "unduh", "unggah", "untung", "uji", "urus", "waspada", "yakin",
            // Istilah teknis, persilatan & olahraga
            "atlet", "wasit", "juri", "gelanggang", "pesilat", "sudut", "merah", "biru", "poin",
            "skor", "babak", "tanding", "seni", "jurus", "medali", "juara", "kamera", "siar",
        ];
        for r in roots {
            dict.insert(r.to_string());
        }

        Self { dictionary: dict }
    }

    /// Menambahkan kata dasar ke dalam kamus internal
    pub fn add_root_word(&mut self, word: &str) {
        self.dictionary.insert(word.to_lowercase());
    }

    /// Memeriksa apakah kata merupakan kata dasar yang sah
    pub fn is_root(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    /// Melakukan proses stemming morfologi Bahasa Indonesia
    pub fn stem(&self, text: &str) -> String {
        let word = text.to_lowercase().trim().to_string();
        if word.len() <= 3 || self.is_root(&word) {
            return word;
        }

        let mut current = word.clone();

        // 1. Hapus Partikel (-lah, -kah, -tah, -pun)
        current = self.remove_particle(&current);
        if self.is_root(&current) {
            return current;
        }

        // 2. Hapus Kata Ganti Kepunyaan / Pronoun (-ku, -mu, -nya)
        current = self.remove_possessive_pronoun(&current);
        if self.is_root(&current) {
            return current;
        }

        // 3. Hapus Awalan & Akhiran (Derivational Affixes)
        let stemmed = self.remove_derivational_affixes(&current);
        if self.is_root(&stemmed) {
            return stemmed;
        }

        // Fallback: jika pemotongan aturan menghasilkan kata dasar yang dikenal, gunakan
        stemmed
    }

    fn remove_particle(&self, word: &str) -> String {
        for p in &["lah", "kah", "tah", "pun"] {
            if word.ends_with(p) && word.len() > p.len() + 2 {
                return word[..word.len() - p.len()].to_string();
            }
        }
        word.to_string()
    }

    fn remove_possessive_pronoun(&self, word: &str) -> String {
        for p in &["ku", "mu", "nya"] {
            if word.ends_with(p) && word.len() > p.len() + 2 {
                return word[..word.len() - p.len()].to_string();
            }
        }
        word.to_string()
    }

    fn remove_suffix(&self, word: &str) -> String {
        for s in &["kan", "an", "i"] {
            if word.ends_with(s) && word.len() > s.len() + 2 {
                let candidate = &word[..word.len() - s.len()];
                if self.is_root(candidate) {
                    return candidate.to_string();
                }
                return candidate.to_string();
            }
        }
        word.to_string()
    }

    fn try_strip_prefix(&self, cur: &str) -> Option<String> {
        // 1. di-, ke-, se-
        for p in &["di", "ke", "se"] {
            if cur.starts_with(p) && cur.len() > p.len() + 2 {
                let cand = &cur[p.len()..];
                if self.is_root(cand) { return Some(cand.to_string()); }
                let cand_no_suffix = self.remove_suffix(cand);
                if self.is_root(&cand_no_suffix) { return Some(cand_no_suffix); }
            }
        }

        // 2. ber-, ter-, per-
        for p in &["ber", "ter", "per"] {
            if cur.starts_with(p) && cur.len() > 5 {
                let cand = &cur[3..];
                if self.is_root(cand) { return Some(cand.to_string()); }
                let s = self.remove_suffix(cand);
                if self.is_root(&s) { return Some(s); }
            }
        }

        // pel- (khusus kata ajar: pelajar -> ajar, pelajaran -> ajar)
        if cur.starts_with("pel") && cur.len() >= 5 {
            let cand = &cur[3..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
        }

        // 3. me- & pe- varian (meng-, mem-, men-, meny-, peng-, pem-, pen-, peny-)
        if cur.starts_with("meng") || cur.starts_with("peng") {
            let prefix_len = 4;
            let cand = &cur[prefix_len..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let with_k = format!("k{}", cand);
            if self.is_root(&with_k) { return Some(with_k); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
            let sk = self.remove_suffix(&with_k);
            if self.is_root(&sk) { return Some(sk); }
        }

        if cur.starts_with("meny") || cur.starts_with("peny") {
            let cand = &cur[4..];
            let with_s = format!("s{}", cand);
            if self.is_root(&with_s) { return Some(with_s); }
            let s = self.remove_suffix(&with_s);
            if self.is_root(&s) { return Some(s); }
        }

        if cur.starts_with("mem") || cur.starts_with("pem") {
            let cand = &cur[3..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let with_p = format!("p{}", cand);
            if self.is_root(&with_p) { return Some(with_p); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
            let sp = self.remove_suffix(&with_p);
            if self.is_root(&sp) { return Some(sp); }
        }

        if cur.starts_with("men") || cur.starts_with("pen") {
            let cand = &cur[3..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let with_t = format!("t{}", cand);
            if self.is_root(&with_t) { return Some(with_t); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
            let st = self.remove_suffix(&with_t);
            if self.is_root(&st) { return Some(st); }
        }

        if cur.starts_with("me") && cur.len() > 4 {
            let cand = &cur[2..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
        }

        if cur.starts_with("pe") && cur.len() > 4 {
            let cand = &cur[2..];
            if self.is_root(cand) { return Some(cand.to_string()); }
            let s = self.remove_suffix(cand);
            if self.is_root(&s) { return Some(s); }
        }

        None
    }

    fn remove_derivational_affixes(&self, word: &str) -> String {
        let cur = word.to_string();

        // 1. Cek akhiran terlebih dahulu
        let without_suffix = self.remove_suffix(&cur);
        if self.is_root(&without_suffix) {
            return without_suffix;
        }

        // 2. Coba stripping awalan pada kata tanpa akhiran (contoh: pelajaran -> pelajar -> ajar)
        if let Some(r) = self.try_strip_prefix(&without_suffix) {
            return r;
        }

        // 3. Coba stripping awalan pada kata asal (contoh: mengajar -> ajar)
        if let Some(r) = self.try_strip_prefix(&cur) {
            return r;
        }

        // Jika tidak ada awalan yang cocok, kembalikan kata tanpa akhiran
        without_suffix
    }
}

impl Default for IndonesianStemmer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TYPO TOLERANCE: DAMERAU-LEVENSHTEIN DISTANCE
// ============================================================================

/// Menghitung jarak sunting Damerau-Levenshtein (insert, delete, replace, transpose)
pub fn damerau_levenshtein_distance(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let len_a = a.len();
    let len_b = b.len();

    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }

    let mut d = vec![vec![0usize; len_b + 1]; len_a + 1];

    for i in 0..=len_a { d[i][0] = i; }
    for j in 0..=len_b { d[0][j] = j; }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            d[i][j] = (d[i - 1][j] + 1)             // deletion
                .min(d[i][j - 1] + 1)               // insertion
                .min(d[i - 1][j - 1] + cost);       // substitution

            // Transposition (tukar huruf berdampingan: misal "sialt" -> "silat")
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }

    d[len_a][len_b]
}

/// Menghitung skor kemiripan antara 0.0 (berbeda total) hingga 1.0 (identik)
pub fn fuzzy_similarity_score(s1: &str, s2: &str) -> f64 {
    let max_len = s1.chars().count().max(s2.chars().count());
    if max_len == 0 { return 1.0; }
    let dist = damerau_levenshtein_distance(s1, s2);
    (1.0 - (dist as f64 / max_len as f64)).max(0.0)
}

// ============================================================================
// TOKENIZER & N-GRAM / AUTOCOMPLETE ENGINE
// ============================================================================

/// Memecah teks menjadi token-token bersih
pub fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

/// Menghasilkan Edge N-Grams untuk fitur autocomplete / search-as-you-type
pub fn generate_edge_ngrams(word: &str, min_len: usize, max_len: usize) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let total = chars.len();
    let mut ngrams = Vec::new();

    for len in min_len..=max_len.min(total) {
        let prefix: String = chars[..len].iter().collect();
        ngrams.push(prefix);
    }

    ngrams
}

// ============================================================================
// FULL-TEXT INVERTED INDEX & BM25 SCORING
// ============================================================================

/// Hasil pencarian teks lengkap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullTextHit {
    pub document_id: String,
    pub score: f64,
    pub matched_terms: Vec<String>,
    pub snippet: String,
}

/// Indeks Teks Terbalik (Inverted Index) dengan Stemmer Bahasa Indonesia
pub struct FullTextIndex {
    stemmer: IndonesianStemmer,
    // term -> (doc_id -> term_frequency)
    postings: HashMap<String, HashMap<String, u32>>,
    // doc_id -> total token count
    doc_lengths: HashMap<String, u32>,
    // doc_id -> raw content
    documents: HashMap<String, String>,
    // n-gram prefix -> Set<canonical_word> untuk autocomplete
    ngram_trie: HashMap<String, HashSet<String>>,
    // Parameter BM25
    k1: f64,
    b: f64,
}

impl FullTextIndex {
    pub fn new() -> Self {
        Self {
            stemmer: IndonesianStemmer::new(),
            postings: HashMap::new(),
            doc_lengths: HashMap::new(),
            documents: HashMap::new(),
            ngram_trie: HashMap::new(),
            k1: 1.2,
            b: 0.75,
        }
    }

    /// Menambahkan dokumen ke dalam indeks teks
    pub fn add_document(&mut self, doc_id: &str, content: &str) {
        let tokens = tokenize(content);
        let doc_len = tokens.len() as u32;

        self.doc_lengths.insert(doc_id.to_string(), doc_len);
        self.documents.insert(doc_id.to_string(), content.to_string());

        for token in &tokens {
            let stemmed = self.stemmer.stem(token);

            // Perbarui posting list untuk kata dasar
            let entry = self.postings.entry(stemmed.clone()).or_default();
            *entry.entry(doc_id.to_string()).or_insert(0) += 1;

            // Masukkan juga token asli jika berbeda dari stemmed
            if stemmed != *token {
                let raw_entry = self.postings.entry(token.clone()).or_default();
                *raw_entry.entry(doc_id.to_string()).or_insert(0) += 1;
            }

            // Bangun N-Grams untuk autocomplete
            for ngram in generate_edge_ngrams(token, 2, 8) {
                self.ngram_trie.entry(ngram).or_default().insert(token.clone());
            }
        }
    }

    /// Menghapus dokumen dari indeks
    pub fn delete_document(&mut self, doc_id: &str) {
        self.doc_lengths.remove(doc_id);
        self.documents.remove(doc_id);

        for posting in self.postings.values_mut() {
            posting.remove(doc_id);
        }
    }

    /// Melakukan pencarian teks menggunakan algoritma BM25 dengan toleransi salah ketik (Fuzzy)
    pub fn search(&self, query: &str, fuzzy_tolerance: usize, max_results: usize) -> Vec<FullTextHit> {
        let raw_query_tokens = tokenize(query);
        if raw_query_tokens.is_empty() || self.doc_lengths.is_empty() {
            return Vec::new();
        }

        let total_docs = self.doc_lengths.len() as f64;
        let avg_doc_len: f64 = self.doc_lengths.values().copied().sum::<u32>() as f64 / total_docs;

        let mut scores: HashMap<String, f64> = HashMap::new();
        let mut doc_matched_terms: HashMap<String, HashSet<String>> = HashMap::new();

        for q_token in &raw_query_tokens {
            let stemmed_q = self.stemmer.stem(q_token);

            // Kumpulkan kata-kata posting yang cocok persis atau fuzzy
            let mut matched_postings: Vec<(&String, &HashMap<String, u32>)> = Vec::new();

            for (indexed_term, doc_map) in &self.postings {
                if indexed_term == &stemmed_q || indexed_term == q_token {
                    matched_postings.push((indexed_term, doc_map));
                } else if fuzzy_tolerance > 0 && indexed_term.len() >= 4 && q_token.len() >= 4 {
                    if damerau_levenshtein_distance(indexed_term, q_token) <= fuzzy_tolerance {
                        matched_postings.push((indexed_term, doc_map));
                    }
                }
            }

            for (term, doc_map) in matched_postings {
                let doc_freq = doc_map.len() as f64;
                // IDF formula: ln(1 + (N - n + 0.5) / (n + 0.5))
                let idf = ((total_docs - doc_freq + 0.5) / (doc_freq + 0.5) + 1.0).ln().max(0.1);

                for (doc_id, &tf) in doc_map {
                    let doc_len = *self.doc_lengths.get(doc_id).unwrap_or(&1) as f64;
                    // TF formula dengan normalisasi panjang dokumen
                    let tf_norm = (tf as f64 * (self.k1 + 1.0))
                        / (tf as f64 + self.k1 * (1.0 - self.b + self.b * (doc_len / avg_doc_len)));

                    *scores.entry(doc_id.clone()).or_insert(0.0) += idf * tf_norm;
                    doc_matched_terms.entry(doc_id.clone()).or_default().insert(term.clone());
                }
            }
        }

        let mut hits: Vec<FullTextHit> = scores
            .into_iter()
            .map(|(doc_id, score)| {
                let content = self.documents.get(&doc_id).cloned().unwrap_or_default();
                let snippet = if content.len() > 140 {
                    format!("{}...", &content[..140])
                } else {
                    content
                };
                let terms = doc_matched_terms
                    .remove(&doc_id)
                    .map(|s| s.into_iter().collect())
                    .unwrap_or_default();

                FullTextHit {
                    document_id: doc_id,
                    score,
                    matched_terms: terms,
                    snippet,
                }
            })
            .collect();

        // Urutkan berdasarkan skor BM25 tertinggi
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        hits.truncate(max_results);
        hits
    }

    /// Autocomplete / Saran pencarian instan berbasis awalan kata (Edge N-Gram)
    pub fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<String> {
        let p = prefix.to_lowercase();
        if let Some(words) = self.ngram_trie.get(&p) {
            let mut list: Vec<String> = words.iter().cloned().collect();
            list.sort();
            list.truncate(limit);
            list
        } else {
            Vec::new()
        }
    }

    /// Mengakses referensi stemmer Bahasa Indonesia
    pub fn stemmer(&self) -> &IndonesianStemmer {
        &self.stemmer
    }
}

impl Default for FullTextIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indonesian_morphological_stemmer() {
        let stemmer = IndonesianStemmer::new();

        // Uji berbagai variasi imbuhan kata "bayar"
        assert_eq!(stemmer.stem("pembayaran"), "bayar");
        assert_eq!(stemmer.stem("membayar"), "bayar");
        assert_eq!(stemmer.stem("dibayarkan"), "bayar");
        assert_eq!(stemmer.stem("bayarlah"), "bayar");
        assert_eq!(stemmer.stem("bayarku"), "bayar");

        // Uji kata "ajar"
        assert_eq!(stemmer.stem("mengajar"), "ajar");
        assert_eq!(stemmer.stem("pelajaran"), "ajar");
        assert_eq!(stemmer.stem("pengajaran"), "ajar");

        // Uji kata "pukul" & "tendang"
        assert_eq!(stemmer.stem("memukul"), "pukul");
        assert_eq!(stemmer.stem("pukulan"), "pukul");
        assert_eq!(stemmer.stem("menendang"), "tendang");
        assert_eq!(stemmer.stem("tendangan"), "tendang");

        // Uji kata "tulis"
        assert_eq!(stemmer.stem("menuliskan"), "tulis");
        assert_eq!(stemmer.stem("tulisan"), "tulis");
    }

    #[test]
    fn test_damerau_levenshtein_distance() {
        // Identik
        assert_eq!(damerau_levenshtein_distance("silat", "silat"), 0);
        // Transposition (tukar huruf berdekatan)
        assert_eq!(damerau_levenshtein_distance("sialt", "silat"), 1);
        // Substitution
        assert_eq!(damerau_levenshtein_distance("solat", "silat"), 1);
        // Insertion & deletion
        assert_eq!(damerau_levenshtein_distance("silatt", "silat"), 1);
        assert_eq!(damerau_levenshtein_distance("sila", "silat"), 1);

        // Skor kemiripan fuzzy
        let sim = fuzzy_similarity_score("pembayran", "pembayaran"); // typo missing 'a'
        assert!(sim > 0.85);
    }

    #[test]
    fn test_full_text_search_with_stemming_and_fuzzy() {
        let mut idx = FullTextIndex::new();

        idx.add_document("doc1", "Sistem pembayaran kasir digital QRIS ASPI telah aktif.");
        idx.add_document("doc2", "Pesilat sudut merah melancarkan tendangan sabit dan bantingan telak.");
        idx.add_document("doc3", "Instruksi sutradara penyiaran untuk menyiarkan pertandingan final.");

        // 1. Kueri kata dasar "bayar" harus menemukan "pembayaran"
        let res1 = idx.search("bayar", 0, 5);
        assert_eq!(res1.len(), 1);
        assert_eq!(res1[0].document_id, "doc1");

        // 2. Kueri berimbuhan "menendang" harus menemukan "tendangan"
        let res2 = idx.search("menendang", 0, 5);
        assert_eq!(res2.len(), 1);
        assert_eq!(res2[0].document_id, "doc2");

        // 3. Kueri typo "bantingn" (toleransi 1) harus menemukan "bantingan"
        let res3 = idx.search("bantingn", 1, 5);
        assert_eq!(res3.len(), 1);
        assert_eq!(res3[0].document_id, "doc2");

        // 4. Autocomplete
        let suggestions = idx.autocomplete("pemb", 5);
        assert!(suggestions.contains(&"pembayaran".to_string()));
    }
}
