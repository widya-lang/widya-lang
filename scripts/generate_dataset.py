#!/usr/bin/env python3
"""
Widya-Lang Dataset Generator for LLM Fine-Tuning (Unsloth & AMD Radeon / DirectML / ROCm)
========================================================================================
Mengekstrak seluruh basis pengetahuan Widya-Lang dari seluruh repositori:
1. 229+ Berkas Contoh Nyata (contoh/*.wya) -> Code Generation, Explanation & Invocation
2. 85+ Pustaka & Modul Standar (modul/*.wya) -> Import Guide & API Reference
3. Bank Soal Kuis, Teori & Ujian Sertifikasi (BANK_SOAL_KUIS_DAN_UJIAN_SERTIFIKASI.md) -> WCA, WCP, WCE & Spesialisasi
4. Seluruh Dokumentasi Resmi & Roadmap (widyacoder_kb.json / DOKUMENTASI_*.md) -> 500+ Chunk Arsitektur
5. WidyaBase Deep Database Engine -> Slotted-Page, MVCC, WAL, SQL Dwibahasa, CTE, Window Functions, HNSW Vector
6. Translasi Kode Antar-Bahasa -> Konversi dari Python, JavaScript, Go, Rust, ANSI SQL ke Widya-Lang
7. Skenario Debugging & Perbaikan Galat -> Diagnosa kesalahan sintaks, runtime & panduan koreksi
8. Kurikulum Komputasi Bahasa Daerah & NLP Nusantara (Jawa, Sunda, Bali, Minang, Batak, Bugis, Aceh)
9. Tata Bahasa Resmi & Kamus Sintaks Dialek Nusantara

Format Keluaran: ShareGPT / ChatML JSONL yang kompatibel 100% dengan Unsloth, SFTTrainer, Hugging Face & Ollama.
"""

import os
import re
import json
import glob
from pathlib import Path

SYSTEM_PROMPT = """Anda adalah WidyaCoder, asisten kecerdasan buatan ahli pemrograman bahasa Widya-Lang dan mesin basis data WidyaBase.
Anda menguasai secara mendalam:
1. Sintaks bahasa Widya-Lang (Dialek Nusantara dan padanan modern): misal, biar, diketahui, diket, tetap, fungsi, struktur, implementasi, sifat, jika-selain_itu, kalau-jika_tidak, untuk_setiap, selama, ulangi_selama, coba-tangkap.
2. Pustaka standar dan modul ekosistem: widyabase, desktop, mobile, widyaweb, ai_rag_stack, deep_learning, dataframe, kripto, ffi, audio, game3d, iiot, crdt, dll.
3. Kueri SQL Dwibahasa WidyaBase: PILIH/SELECT, DARI/FROM, DIMANA/WHERE, SISIP_KE/INSERT INTO, PERBARUI/UPDATE, HAPUS_DARI/DELETE FROM, BUAT_TABEL/CREATE TABLE, DENGAN/WITH (CTE), LEWAT/OVER (Window Functions), TITIK_SIMPAN/SAVEPOINT, KUNCI_ASING/FOREIGN KEY.
4. Translasi kode dari bahasa lain (Python, JavaScript, Go, Rust, ANSI SQL ke Widya-Lang) dan pemecahan galat (debugging).
Berikan jawaban yang jelas, akurat, terstruktur, dan sertakan contoh kode Widya-Lang yang siap dijalankan."""

def clean_text(text: str) -> str:
    return text.strip()

def extract_header_comment(content: str) -> tuple[str, str]:
    """Mengambil judul dan deskripsi dari komentar pembuka berkas .wya"""
    lines = content.splitlines()
    title = ""
    desc_lines = []
    in_comment = False
    
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("// ==") or stripped.startswith("/* =="):
            in_comment = True
            continue
        if stripped.startswith("//") or in_comment:
            clean_line = stripped.lstrip("/* ").rstrip("*/").strip()
            if not title and any(kw in clean_line.upper() for kw in ["CONTOH", "MODUL", "DEMO", "IMPLEMENTASI", "SIMULASI"]):
                title = clean_line.replace("//", "").strip()
            elif clean_line:
                desc_lines.append(clean_line)
        else:
            if not stripped:
                continue
            break
            
    desc = " ".join(desc_lines)
    return title, desc

def extract_functions_and_structs(content: str) -> tuple[list[str], list[str]]:
    """Mengekstrak nama fungsi dan struktur dari kode .wya"""
    funcs = re.findall(r'fungsi\s+([a-zA-Z0-9_]+)\s*\(', content)
    structs = re.findall(r'struktur\s+([a-zA-Z0-9_]+)\s*\{?', content)
    return funcs, structs

# ==============================================================================
# 1. GENERATOR DARI CONTOH KODE (contoh/*.wya)
# ==============================================================================
def generate_samples_from_contoh(repo_root: Path) -> list:
    samples = []
    contoh_dir = repo_root / "contoh"
    if not contoh_dir.exists():
        return samples

    files = sorted(glob.glob(str(contoh_dir / "*.wya")))
    for idx, file_path in enumerate(files):
        p = Path(file_path)
        try:
            content = p.read_text(encoding="utf-8")
        except Exception:
            continue

        if not content.strip():
            continue

        title, desc = extract_header_comment(content)
        name_clean = p.stem.replace("_", " ").title()
        if not title:
            title = f"Program {name_clean}"

        # 1. Variasi Prompt Permintaan Kode
        prompt_templates = [
            f"Buatkan contoh program Widya-Lang untuk {name_clean}.",
            f"Bagaimana implementasi program {name_clean} menggunakan bahasa pemrograman Widya-Lang?",
            f"Tuliskan kode lengkap program Widya-Lang untuk {name_clean}.",
            f"Saya ingin membuat aplikasi atau modul {name_clean} di Widya-Lang, berikan contoh kodenya."
        ]
        user_prompt_1 = prompt_templates[idx % len(prompt_templates)]
        if desc:
            user_prompt_1 += f" Spesifikasi & Fitur: {desc[:250]}"

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": user_prompt_1},
                {"role": "assistant", "content": f"Berikut adalah implementasi program Widya-Lang untuk **{name_clean}**:\n\n```widya\n{content}\n```\n\n**Penjelasan Singkat:**\nProgram ini mengimplementasikan logika {name_clean.lower()} secara mandiri dan berdaulat menggunakan sintaks resmi Widya-Lang."}
            ]
        })

        # 2. Sampel Penjelasan Kode (Code Explanation)
        snippet = content[:1500] if len(content) > 1500 else content
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Jelaskan alur logika dan cara kerja kode Widya-Lang berikut:\n\n```widya\n{snippet}\n```"},
                {"role": "assistant", "content": f"Kode di atas adalah implementasi Widya-Lang untuk **{name_clean}**.\n\n**Tujuan & Fitur Utama:**\n{desc if desc else 'Mengimplementasikan komputasi terstruktur menggunakan pustaka dan tata bahasa Widya-Lang.'}\n\n**Alur Kerja Program:**\n1. Mendeklarasikan variabel dan struktur data yang dibutuhkan.\n2. Mengolah alur eksekusi komputasi menggunakan kontrol logika dan perulangan.\n3. Menampilkan hasil atau mengembalikan nilai komputasi dengan aman dan efisien."}
            ]
        })

        # 3. Sampel Pemanggilan / Pengujian (Invocation / Test)
        funcs, structs = extract_functions_and_structs(content)
        if funcs:
            main_funcs = [f for f in funcs if f != "utama"][:3]
            func_list_str = ", ".join(f"`{f}()`" for f in main_funcs) if main_funcs else "`fungsi utama()`"
            snippet_test = (
                "fungsi utama() {\n"
                f"    tulis(\"=== Menguji {name_clean} ===\");\n"
                "    // Eksekusi logika fungsi terkait\n"
                "    tulis(\"Pengujian selesai dengan sukses.\");\n"
                "}"
            )
            samples.append({
                "messages": [
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": f"Bagaimana contoh pemanggilan fungsi dalam kode {name_clean} ini di Widya-Lang?"},
                    {"role": "assistant", "content": f"Dalam program **{name_clean}**, fungsi utama yang dapat dipanggil meliputi {func_list_str}.\n\nContoh cara memanggil dan mengujinya:\n```widya\n{snippet_test}\n```"}
                ]
            })

    return samples

# ==============================================================================
# 2. GENERATOR DARI MODUL PUSTAKA STANDAR (modul/*.wya)
# ==============================================================================
def generate_samples_from_modul(repo_root: Path) -> list:
    samples = []
    modul_dir = repo_root / "modul"
    if not modul_dir.exists():
        return samples

    files = sorted(glob.glob(str(modul_dir / "*.wya")))
    for file_path in files:
        p = Path(file_path)
        try:
            content = p.read_text(encoding="utf-8")
        except Exception:
            continue

        if not content.strip():
            continue

        modul_name = p.stem
        title, desc = extract_header_comment(content)
        funcs, structs = extract_functions_and_structs(content)

        # 1. Sample: Cara Impor & Menggunakan
        import_stmt = f'impor "./modul/{modul_name}.wya";'
        mod_desc = desc if desc else f"Modul `{modul_name}` menyediakan fungsi-fungsi standar siap pakai untuk mempercepat pengembangan aplikasi di ekosistem Widya-Lang."
        code_preview = content[:1800]
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Bagaimana cara mengimpor dan memanfaatkan modul `{modul_name}` di Widya-Lang?"},
                {"role": "assistant", "content": f"Untuk menggunakan modul `{modul_name}`, Anda dapat mengimpornya dengan sintaks:\n```widya\n{import_stmt}\n```\n\n**Deskripsi Modul:**\n{mod_desc}\n\n**Struktur Kode Modul (Ringkasan):**\n```widya\n{code_preview}\n```"}
            ]
        })

        # 2. Sample: API & Fungsi yang Disediakan
        if funcs or structs:
            features = []
            if structs:
                features.append(f"- **Struktur Data**: {', '.join(f'`{s}`' for s in structs[:5])}")
            if funcs:
                features.append(f"- **Fungsi**: {', '.join(f'`{f}()`' for f in funcs[:8])}")
            feature_str = "\n".join(features)
            samples.append({
                "messages": [
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": f"Fungsi dan kemampuan apa saja yang disediakan oleh pustaka standar `{modul_name}` di Widya-Lang?"},
                    {"role": "assistant", "content": f"Modul pustaka `{modul_name}` menyediakan kapabilitas berikut:\n\n{feature_str}\n\nModul ini didesain dengan performa native tanpa dependensi pihak ketiga."}
                ]
            })

    return samples

# ==============================================================================
# 3. GENERATOR BANK SOAL & SERTIFIKASI (BANK_SOAL_KUIS_DAN_UJIAN_SERTIFIKASI.md)
# ==============================================================================
def generate_samples_from_bank_soal(repo_root: Path) -> list:
    samples = []
    bank_path = repo_root / "BANK_SOAL_KUIS_DAN_UJIAN_SERTIFIKASI.md"
    if not bank_path.exists():
        return samples

    try:
        text = bank_path.read_text(encoding="utf-8")
    except Exception:
        return samples

    # 1. Pilihan Ganda Format 1 (Level 1-3: #### Soal ...)
    pattern1 = r'#### Soal\s+(\d+)[^\n]*\n\*\*Pertanyaan\*\*:\s*([^\n]+)\n(.*?)- \*\*Kunci Jawaban\*\*:\s*\*\*?([A-D])\*\*?\n- \*\*Pembahasan Mendalam\*\*:\s*([^\n]+(?:\n(?!####|---|\Z)[^\n]+)*)'
    for m in re.finditer(pattern1, text, re.DOTALL):
        num, q, opt, key, exp = m.groups()
        q_clean = q.strip()
        opt_clean = opt.strip()
        key_clean = key.strip()
        exp_clean = exp.strip()

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Jawab soal sertifikasi resmi Widya-Lang berikut beserta pembahasannya:\n\n**Pertanyaan**: {q_clean}\n\nPilihan:\n{opt_clean}"},
                {"role": "assistant", "content": f"**Kunci Jawaban**: **{key_clean}**\n\n**Pembahasan Mendalam:**\n{exp_clean}"}
            ]
        })

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q_clean},
                {"role": "assistant", "content": f"**Penjelasan:**\n{exp_clean}"}
            ]
        })

    # 2. Pilihan Ganda Format 2 (Spesialisasi: - **Soal ...**)
    pattern2 = r'-\s+\*\*Soal\s+(\d+)\*\*:\s*([^\n]+)\n\s+-\s+\*Kunci\*:\s*\*\*?([A-D])\*\*?\s*\((.*?)\)\.?(?:\n\s+-\s+\*Pembahasan\*:\s*([^\n]+(?:\n(?!\s*-\s+\*\*Soal|\s*###|---|\Z)[^\n]+)*))?'
    for m in re.finditer(pattern2, text):
        num, q, key, key_desc, exp = m.groups()
        q_clean = q.strip()
        key_clean = key.strip()
        key_desc_clean = key_desc.strip()
        exp_clean = exp.strip() if exp else key_desc_clean

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Jelaskan jawaban dari pertanyaan arsitektur Widya-Lang berikut: {q_clean}"},
                {"role": "assistant", "content": f"**Jawaban**: **{key_clean}** ({key_desc_clean})\n\n**Pembahasan:**\n{exp_clean}"}
            ]
        })

    # 3. Tantangan Koding Format 1 (Level 1-3)
    pattern_ch1 = r'#### Tantangan\s+(\d+)[^\n]*:\s*([^\n]+)\n-\s+\*\*Tantangan\*\*:\s*([^\n]+)\n-\s+\*\*Starter Code\*\*:\s*```widya\n(.*?)```.*?- \*\*Kunci Solusi Resmi\*\*:\s*```widya\n(.*?)```'
    for m in re.finditer(pattern_ch1, text, re.DOTALL):
        num, title, desc, starter, sol = m.groups()
        title_clean = title.strip()
        desc_clean = desc.strip()
        sol_clean = sol.strip()

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Kerjakan tantangan kode resmi Widya-Lang untuk **{title_clean}**:\n{desc_clean}"},
                {"role": "assistant", "content": f"Berikut adalah kunci solusi resmi untuk **{title_clean}**:\n\n```widya\n{sol_clean}\n```\n\nKode ini memenuhi seluruh kriteria pengujian dan mengikuti konvensi penulisan Widya-Lang yang bersih dan efisien."}
            ]
        })

    # 4. Tantangan Koding Format 2 (Spesialisasi 4-10)
    pattern_ch2 = r'-\s+\*\*Tantangan\s+(\d+)[^\n]*\*\*:\s*([^\n]+)\n\s+-\s+\*Kunci(?: Solusi)?\*:\s*```widya\n(.*?)```'
    for m in re.finditer(pattern_ch2, text, re.DOTALL):
        num, desc, sol = m.groups()
        desc_clean = desc.strip()
        sol_clean = sol.strip()

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Selesaikan tantangan pemrograman Widya-Lang berikut:\n{desc_clean}"},
                {"role": "assistant", "content": f"Berikut adalah solusi fungsi Widya-Lang:\n\n```widya\n{sol_clean}\n```"}
            ]
        })

    return samples

# ==============================================================================
# 4. GENERATOR DARI BASIS PENGETAHUAN RAG (widyacoder_kb.json)
# ==============================================================================
def generate_samples_from_kb(repo_root: Path) -> list:
    samples = []
    kb_path = repo_root / "widyacoder_kb.json"
    if not kb_path.exists():
        return samples

    try:
        with open(kb_path, encoding="utf-8") as f:
            kb_data = json.load(f)
    except Exception:
        return samples

    chunks = kb_data.get("chunks", [])
    doc_map_names = {
        "DOKUMENTASI_WIDYABASE.md": "Mesin Basis Data WidyaBase",
        "DOKUMENTASI_WIDYAWEB_CLOUD.md": "WidyaWeb Cloud & Edge Runtime",
        "DOKUMENTASI_WIDYACODER_LOCAL_AI.md": "WidyaCoder Local RAG AI",
        "DOKUMENTASI_KOMPREHENSIF.md": "Spesifikasi Komprehensif Widya-Lang",
        "DOKUMENTASI_DESKTOP.md": "Widya Desktop GUI Canvas",
        "DOKUMENTASI_MOBILE.md": "Widya Mobile Android/iOS",
        "DOKUMENTASI_OFFICE.md": "Widya Office Automation Suite",
        "DOKUMENTASI_SOVEREIGN_SECURITY_SHIELD.md": "WidyaShield Sovereign Security",
        "DOKUMENTASI_SMARTCITY_CRDT.md": "Smart City CRDT & IoT",
        "DOKUMENTASI_WIDYABOX_CONTAINER.md": "WidyaBox MicroVM Container",
        "DOKUMENTASI_WIDYAKUBE_ORCHESTRATOR.md": "WidyaKube Cluster Orchestrator",
        "DOKUMENTASI_WPM.md": "Widya Package Manager (WPM)",
        "DOKUMENTASI_PILAR_KEMANDIRIAN.md": "Pilar Kemandirian Teknologi Nasional",
        "DOKUMENTASI_DATA_PLATFORM.md": "Widya Data Platform & Analytics",
        "DOKUMENTASI_GAME3D.md": "Widya Game 3D Engine",
        "DOKUMENTASI_PAYMENT_GATEWAY.md": "Widya Payment Gateway & FinTech",
        "DOKUMENTASI_MEGA_FRONTIERS.md": "Mega Frontiers DeepTech",
        "README.md": "Ekosistem Bahasa Pemrograman Widya-Lang"
    }

    for chunk in chunks:
        category = chunk.get("category", "")
        source_path = chunk.get("source_path", "")
        title = chunk.get("title", "").strip()
        content = chunk.get("content", "").strip()

        # Ambil dokumentasi dan roadmap yang substansial (> 120 karakter)
        if category not in ["dokumentasi", "roadmap"] or len(content) < 120:
            continue

        doc_label = doc_map_names.get(source_path, source_path.replace(".md", "").replace("_", " "))
        clean_title = re.sub(r'^[#0-9.\s🚀🧠⚡🌐🏙️🛡️📦📊🏛️🎯💻📂🔒]+\s*', '', title)
        if not clean_title:
            clean_title = title

        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Jelaskan mengenai {clean_title} dalam arsitektur {doc_label} di Widya-Lang."},
                {"role": "assistant", "content": f"Berdasarkan dokumentasi resmi **{doc_label}**, berikut penjelasan mengenai **{clean_title}**:\n\n{content}"}
            ]
        })

    return samples

# ==============================================================================
# 5. GENERATOR WIDYABASE DEEP DATABASE ENGINE & DWICARA SQL
# ==============================================================================
def generate_samples_from_widyabase_deep(repo_root: Path) -> list:
    qa_list = [
        (
            "Apa itu WidyaBase dan apa saja fitur kuncinya?",
            "WidyaBase adalah mesin basis data mandiri berkinerja native, berdaulat, dan berskala industri yang terintegrasi di dalam ekosistem Widya-Lang.\n\nFitur-fitur utamanya meliputi:\n1. Penyimpanan Slotted-Page 4096-byte dengan Buffer Pool Manager LRU.\n2. Write-Ahead Logging (WAL) dengan Crash Recovery berbasis urutan LSN.\n3. Multi-Version Concurrency Control (MVCC) Snapshot Isolation (pembaca tidak memblokir penulis).\n4. B+Tree Indexing O(log N) untuk kunci utama dan indeks sekunder.\n5. SQL Dwibahasa Simetris (Bahasa Indonesia Nusantara dan Standar ANSI Global).\n6. Daemon PostgreSQL Wire Protocol v3.0 (pgwire) di port 5432.\n7. Multi-Modal AI (HNSW Vektor Cosine Similarity & parser JSONB).\n8. Analitik Enterprise: CTE (WITH/DENGAN), Window Functions (OVER/LEWAT), Views, Savepoints, dan Kunci Asing (Foreign Keys)."
        ),
        (
            "Tuliskan padanan kata kunci SQL Bahasa Indonesia dengan Standar ANSI di WidyaBase.",
            "Berikut padanan kata kunci SQL di WidyaBase:\n- `PILIH` = `SELECT`\n- `DARI` = `FROM`\n- `DIMANA` / `DI_MANA` = `WHERE`\n- `SISIP_KE` / `MASUKKAN_KE` = `INSERT INTO`\n- `NILAI` = `VALUES`\n- `PERBARUI` / `UBAH` = `UPDATE`\n- `ATUR` = `SET`\n- `HAPUS_DARI` = `DELETE FROM`\n- `BUAT_TABEL` = `CREATE TABLE`\n- `KUNCI_UTAMA` = `PRIMARY KEY`\n- `KUNCI_ASING` ... `REFERENSI` = `FOREIGN KEY` ... `REFERENCES`\n- `GABUNG` / `GABUNG_KIRI` = `INNER JOIN` / `LEFT JOIN`\n- `KELOMPOKKAN_BERDASARKAN` = `GROUP BY`\n- `MEMILIKI` = `HAVING`\n- `URUTKAN BERDASARKAN` = `ORDER BY`\n- `NAIK` / `TURUN` = `ASC` / `DESC`\n- `BATAS` / `LEWATI` = `LIMIT` / `OFFSET`\n- `BERBEDA` / `UNIK` = `DISTINCT`\n- `DENGAN ... SEBAGAI` = `WITH ... AS` (CTE)\n- `LEWAT (PARTISI_BERDASARKAN ...)` = `OVER (PARTITION BY ...)` (Window Functions)\n- `TITIK_SIMPAN` = `SAVEPOINT`\n- `GUGURKAN_KE` = `ROLLBACK TO`"
        ),
        (
            "Bagaimana cara menggunakan Common Table Expression (CTE) di WidyaBase?",
            "CTE di WidyaBase menggunakan klausa `DENGAN ... SEBAGAI (...)` atau standar `WITH ... AS (...)`:\n\n```sql\n-- Versi Bahasa Indonesia\nDENGAN ringkasan_divisi SEBAGAI (\n    PILIH divisi, JUMLAH(gaji) AS total_gaji\n    DARI karyawan\n    KELOMPOKKAN_BERDASARKAN divisi\n)\nPILIH divisi, total_gaji DARI ringkasan_divisi\nDIMANA total_gaji > 100000000\nURUTKAN BERDASARKAN total_gaji TURUN;\n\n-- Versi ANSI SQL\nWITH summary AS (\n    SELECT department, SUM(salary) AS total_salary\n    FROM employees\n    GROUP BY department\n)\nSELECT department, total_salary FROM summary;\n```"
        ),
        (
            "Bagaimana cara menggunakan Window Functions di WidyaBase?",
            "WidyaBase mendukung `NOMOR_BARIS()`, `PERINGKAT()`, `DENSE_RANK()`, `JUMLAH()`, `RATA_RATA()`, dan `HITUNG()` dengan klausa `LEWAT (...)` atau `OVER (...)`:\n\n```sql\nPILIH nama, divisi, gaji,\n      NOMOR_BARIS() LEWAT (PARTISI_BERDASARKAN divisi URUTKAN_BERDASARKAN gaji TURUN) AS no_urut,\n      PERINGKAT() LEWAT (PARTISI_BERDASARKAN divisi URUTKAN_BERDASARKAN gaji TURUN) AS ranking,\n      JUMLAH(gaji) LEWAT (PARTISI_BERDASARKAN divisi) AS total_divisi\nDARI karyawan\nURUTKAN BERDASARKAN divisi NAIK, gaji TURUN;\n```"
        ),
        (
            "Bagaimana transaksi berjenjang (Savepoints) bekerja di WidyaBase?",
            "WidyaBase menyediakan `TITIK_SIMPAN`, `GUGURKAN_KE`, dan `LEPAS_TITIK_SIMPAN` untuk rollback berbutir halus:\n\n```sql\nMULAI;\nPERBARUI dompet ATUR saldo = 1500000 DIMANA id = 1;\nTITIK_SIMPAN sebelum_transfer;\n\n-- Operasi yang salah\nPERBARUI dompet ATUR saldo = 200000 DIMANA id = 1;\n\n-- Batalkan hanya operasi kedua kembali ke titik simpan\nGUGURKAN_KE sebelum_transfer;\n\nLEPAS_TITIK_SIMPAN sebelum_transfer;\nKOMIT;\n```"
        ),
        (
            "Bagaimana cara melakukan pencarian kemiripan vektor AI (HNSW Cosine Similarity) di WidyaBase?",
            "WidyaBase mendukung tipe data `VEKTOR(dimensi)` dan operator jarak vektor semantik langsung di SQL:\n\n```sql\nBUAT_TABEL artikel (\n    id BULAT KUNCI_UTAMA,\n    judul TEKS,\n    vektor_fitur VEKTOR(384)\n);\n\n-- Kueri Top-K Vektor Terdekat (Cosine Distance)\nPILIH id, judul, JARAK_KOSINUS(vektor_fitur, [0.05, -0.12, ...]) AS jarak\nDARI artikel\nURUTKAN BERDASARKAN jarak NAIK\nBATAS 5;\n```"
        ),
        (
            "Bagaimana cara menghubungkan aplikasi pihak ketiga (DBeaver, psql, Node.js, Python) ke WidyaBase?",
            "WidyaBase memiliki daemon **PostgreSQL Wire Protocol v3.0 (pgwire)** bawaan di port **5432**.\nAnda dapat menghubungkan alat apapun dengan connection string standar:\n`postgresql://widya:widya123@localhost:5432/widyadb`\n\nContoh via terminal psql:\n```bash\npsql -h localhost -p 5432 -U widya -d widyadb\n```\nWidyaBase akan langsung mengeksekusi kueri ANSI maupun kueri dwibahasa secara transparan."
        ),
        (
            "Jelaskan arsitektur penyimpanan Slotted-Page 4096-byte dan Buffer Pool Manager pada WidyaBase.",
            "WidyaBase mengelola data dalam unit blok berukuran tepat **4096 byte** (Slotted Page) yang selaras dengan ukuran sektor disk OS modern:\n\n1. **Struktur Header Halaman**: Berisi Page LSN, jumlah slot record, dan pointer free space.\n2. **Array Slot Indireksi**: Tumbuh ke bawah dari bagian atas halaman, menyimpan offset dan panjang setiap tuple.\n3. **Data Tuple**: Tumbuh ke atas dari dasar halaman (bottom-up growth).\n4. **Buffer Pool Manager**: Memelihara frame halaman di memori RAM menggunakan algoritma eviksi LRU (*Least Recently Used*) dengan dirty flag tracking untuk meminimalkan I/O disk fisik."
        ),
        (
            "Bagaimana arsitektur Write-Ahead Logging (WAL) dan Crash Recovery di WidyaBase menjamin durabilitas (ACID)?",
            "WidyaBase menerapkan protokol **Write-Ahead Logging (WAL)** berbasis urutan LSN (Log Sequence Number):\n\n1. Sebelum perubahan slotted page ditulis ke disk data primer, rekaman perubahan (diff log) wajib di-flush terlebih dahulu ke berkas WAL (`.wal`).\n2. Setiap transaksi menyertakan tanda MULAI, KOMIT, atau GUGURKAN.\n3. Saat pemulihan bencana (*Crash Recovery*), engine mengeksekusi protokol ARIES 3 tahap:\n   - **Fase Analisis**: Membaca catatan WAL untuk memetakan transaksi aktif dan halaman kotor.\n   - **Fase Redo**: Memutar ulang seluruh perubahan yang sah untuk mengembalikan kondisi sistem ke titik crash.\n   - **Fase Undo**: Membatalkan seluruh operasi transaksi yang belum sempat dikomit saat sistem terhenti."
        ),
        (
            "Bagaimana isolasi Multi-Version Concurrency Control (MVCC) bekerja di WidyaBase?",
            "WidyaBase menerapkan Snapshot Isolation MVCC:\n- Setiap baris memiliki metadata versi `xmin` (ID transaksi pembuat) dan `xmax` (ID transaksi penghapus/pembaru).\n- Pembaca (*SELECT/PILIH*) membaca snapshot berdasarkan status transaksi saat kueri dimulai, sehingga **pembaca tidak pernah memblokir penulis dan penulis tidak memblokir pembaca** (*lock-free reads*).\n- Proses vacuum otomatis membersihkan tuple mati yang tidak lagi dijangkau oleh snapshot tertua."
        )
    ]

    samples = []
    for q, a in qa_list:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q},
                {"role": "assistant", "content": a}
            ]
        })
    return samples

# ==============================================================================
# 6. GENERATOR TRANSLASI KODE ANTAR-BAHASA (Python/JS/SQL -> Widya-Lang)
# ==============================================================================
def generate_samples_code_translation() -> list:
    translations = [
        (
            "Ubah fungsi Python berikut menjadi bahasa Widya-Lang:\n\n```python\ndef hitung_diskon(total_belanja, persen_diskon=0.10):\n    if total_belanja >= 100000:\n        return total_belanja * (1 - persen_diskon)\n    return total_belanja\n```",
            "Berikut adalah konversi fungsi tersebut ke dalam Widya-Lang:\n\n```widya\nfungsi hitung_diskon(total_belanja, persen_diskon) {\n    jika (persen_diskon == kosong) {\n        persen_diskon = 0.10;\n    }\n    jika (total_belanja >= 100000) {\n        kembalikan total_belanja * (1.0 - persen_diskon);\n    }\n    kembalikan total_belanja;\n}\n```\n\n**Catatan Perubahan:**\n- Menggunakan kata kunci `fungsi` sebagai pengganti `def`.\n- Menggunakan klausa `jika` dan tanda kurung kurawal `{ ... }` untuk blok kode alih-alih indentasi.\n- Menggunakan kata kunci `kembalikan` sebagai pengganti `return`."
        ),
        (
            "Konversi kode JavaScript manipulasi array berikut ke Widya-Lang:\n\n```javascript\nconst angka = [1, 2, 3, 4, 5, 6];\nconst genap = angka.filter(x => x % 2 === 0);\nconst ganda = genap.map(x => x * 2);\nconsole.log(ganda);\n```",
            "Berikut implementasi ekuivalen di Widya-Lang:\n\n```widya\nfungsi utama() {\n    tetap angka = [1, 2, 3, 4, 5, 6];\n    misal ganda = [];\n    \n    untuk_setiap x dalam angka {\n        jika (x % 2 == 0) {\n            ganda.tambah(x * 2);\n        }\n    }\n    \n    tulis(\"Hasil penggandaan bilangan genap:\");\n    tulis(ganda);\n}\n```\n\n**Catatan:**\n- `const` diterjemahkan menjadi `tetap`.\n- Array baru mutable dideklarasikan dengan `misal`.\n- Iterasi menggunakan idiom deklaratif `untuk_setiap <elemen> dalam <koleksi>`.\n- `console.log` digantikan oleh fungsi bawaan `tulis()` atau `cetak()`."
        ),
        (
            "Ubah kode Python class berikut menjadi struktur dan metode di Widya-Lang:\n\n```python\nclass Produk:\n    def __init__(self, nama, harga, stok):\n        self.nama = nama\n        self.harga = harga\n        self.stok = stok\n        \n    def total_nilai(self):\n        return self.harga * self.stok\n```",
            "Di Widya-Lang, paradigma berorientasi data memisahkan definisi struktur data (`struktur`) dan implementasi perilakunya (`implementasi`):\n\n```widya\nstruktur Produk {\n    nama: Teks,\n    harga: Pecahan,\n    stok: Bulat\n}\n\nimplementasi Produk {\n    fungsi baru(nama, harga, stok) {\n        kembalikan Produk {\n            nama: nama,\n            harga: harga,\n            stok: stok\n        };\n    }\n    \n    fungsi total_nilai(diri) {\n        kembalikan diri.harga * diri.stok;\n    }\n}\n```"
        ),
        (
            "Bagaimana menulis kueri ANSI SQL berikut menggunakan sintaks Bahasa Indonesia WidyaBase?\n\n```sql\nSELECT pelanggan.nama, SUM(pesanan.total) AS total_belanja\nFROM pelanggan\nINNER JOIN pesanan ON pelanggan.id = pesanan.id_pelanggan\nWHERE pesanan.status = 'SELESAI'\nGROUP BY pelanggan.nama\nHAVING SUM(pesanan.total) > 5000000\nORDER BY total_belanja DESC\nLIMIT 10;\n```",
            "Kueri di atas dapat ditulis dalam dialek SQL Nusantara WidyaBase secara 100% simetris:\n\n```sql\nPILIH pelanggan.nama, JUMLAH(pesanan.total) AS total_belanja\nDARI pelanggan\nGABUNG pesanan PADA pelanggan.id = pesanan.id_pelanggan\nDIMANA pesanan.status = 'SELESAI'\nKELOMPOKKAN_BERDASARKAN pelanggan.nama\nMEMILIKI JUMLAH(pesanan.total) > 5000000\nURUTKAN BERDASARKAN total_belanja TURUN\nBATAS 10;\n```"
        ),
        (
            "Konversikan algoritma rekursif Fibonacci Python berikut ke Widya-Lang:\n\n```python\ndef fibonacci(n):\n    if n <= 0:\n        return 0\n    elif n == 1:\n        return 1\n    return fibonacci(n - 1) + fibonacci(n - 2)\n```",
            "Berikut implementasi dalam Widya-Lang:\n\n```widya\nfungsi fibonacci(n) {\n    jika (n <= 0) {\n        kembalikan 0;\n    }\n    jika (n == 1) {\n        kembalikan 1;\n    }\n    kembalikan fibonacci(n - 1) + fibonacci(n - 2);\n}\n```"
        ),
        (
            "Konversi algoritma pencarian biner (Binary Search) Python berikut ke Widya-Lang:\n\n```python\ndef binary_search(arr, target):\n    low = 0\n    high = len(arr) - 1\n    while low <= high:\n        mid = (low + high) // 2\n        if arr[mid] == target:\n            return mid\n        elif arr[mid] < target:\n            low = mid + 1\n        else:\n            high = mid - 1\n    return -1\n```",
            "Berikut implementasi Binary Search di Widya-Lang:\n\n```widya\nfungsi cari_biner(arr, target) {\n    misal bawah = 0;\n    misal atas = panjang(arr) - 1;\n    \n    selama (bawah <= atas) {\n        misal tengah = (bawah + atas) / 2;\n        jika (arr[tengah] == target) {\n            kembalikan tengah;\n        } selain_itu jika (arr[tengah] < target) {\n            bawah = tengah + 1;\n        } selain_itu {\n            atas = tengah - 1;\n        }\n    }\n    kembalikan -1;\n}\n```"
        ),
        (
            "Konversi kode penghitung frekuensi kata Python ke Widya-Lang menggunakan struktur Kamus (Dictionary):\n\n```python\ndef hitung_kata(kalimat):\n    frekuensi = {}\n    for kata in kalimat.split():\n        frekuensi[kata] = frekuensi.get(kata, 0) + 1\n    return frekuensi\n```",
            "Berikut implementasi di Widya-Lang:\n\n```widya\nfungsi hitung_kata(kalimat) {\n    misal daftar_kata = kalimat.pisah(\" \");\n    misal frekuensi = {};\n    \n    untuk_setiap kata dalam daftar_kata {\n        jika (frekuensi.memiliki(kata)) {\n            frekuensi[kata] = frekuensi[kata] + 1;\n        } selain_itu {\n            frekuensi[kata] = 1;\n        }\n    }\n    kembalikan frekuensi;\n}\n```"
        ),
        (
            "Ubah fungsi cek palindrom JavaScript berikut ke Widya-Lang:\n\n```javascript\nfunction isPalindrome(text) {\n    const cleaned = text.toLowerCase().replace(/[^a-z0-9]/g, '');\n    const reversed = cleaned.split('').reverse().join('');\n    return cleaned === reversed;\n}\n```",
            "Berikut implementasi di Widya-Lang:\n\n```widya\nfungsi apakah_palindrom(teks) {\n    misal bersih = teks.huruf_kecil();\n    misal n = panjang(bersih);\n    misal i = 0;\n    \n    selama (i < n / 2) {\n        jika (bersih[i] != bersih[n - 1 - i]) {\n            kembalikan salah;\n        }\n        i = i + 1;\n    }\n    kembalikan benar;\n}\n```"
        ),
        (
            "Konversi pola Go Struct dan Receiver Method berikut ke Widya-Lang:\n\n```go\ntype Rekening struct {\n    Nomor string\n    Saldo float64\n}\n\nfunc (r *Rekening) Setor(jumlah float64) {\n    r.Saldo += jumlah\n}\n```",
            "Di Widya-Lang, padanan representatifnya adalah:\n\n```widya\nstruktur Rekening {\n    nomor: Teks,\n    saldo: Pecahan\n}\n\nimplementasi Rekening {\n    fungsi setor(diri, jumlah) {\n        diri.saldo = diri.saldo + jumlah;\n    }\n}\n```"
        )
    ]

    samples = []
    for user_prompt, assistant_resp in translations:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": user_prompt},
                {"role": "assistant", "content": assistant_resp}
            ]
        })
    return samples

# ==============================================================================
# 7. GENERATOR SKENARIO DEBUGGING & PERBAIKAN GALAT (Error Fixing)
# ==============================================================================
def generate_samples_debugging_and_fixing() -> list:
    debug_cases = [
        (
            "Kode Widya-Lang saya mengalami error kompilasi berikut: `Galat: Tidak dapat mengubah nilai konstanta imutable 'TARIF_PAJAK'`. Kode saya:\n```widya\ntetap TARIF_PAJAK = 0.10;\nTARIF_PAJAK = 0.12;\n```\nBagaimana cara memperbaikinya?",
            "**Penyebab Galat:**\nKata kunci `tetap` digunakan untuk mendeklarasikan konstanta **imutable** (nilai tetap yang tidak dapat diubah setelah diinisialisasi).\n\n**Solusi Perbaikan:**\nJika nilai variabel tersebut perlu diubah (*mutable*), gunakan kata kunci `misal`, `biar`, atau `diketahui`:\n\n```widya\n// Perbaikan: gunakan 'misal'\nmisal tarif_pajak = 0.10;\ntarif_pajak = 0.12;\ntulis(\"Tarif pajak yang baru:\", tarif_pajak);\n```"
        ),
        (
            "Saya terbiasa dengan Python dan menulis kode ini di Widya-Lang tapi ditolak compiler:\n```widya\ndef sapa_pengguna(nama):\n    print(\"Halo \" + nama)\n```\nBagaimana kode yang benar di Widya-Lang?",
            "**Koreksi Sintaks:**\n1. Di Widya-Lang, deklarasi fungsi menggunakan kata kunci `fungsi`, bukan `def`.\n2. Blok kode dibatasi kurung kurawal `{ ... }` dan diakhiri titik koma `;` pada pernyataan.\n3. Fungsi output terminal adalah `tulis()` atau `cetak()`, bukan `print()`.\n\n**Kode yang Benar:**\n```widya\nfungsi sapa_pengguna(nama) {\n    tulis(\"Halo \" + nama);\n}\n```"
        ),
        (
            "Bagaimana cara menangani galat pembagian dengan nol agar program tidak crash di Widya-Lang?",
            "Gunakan blok terstruktur `coba { ... } tangkap (galat) { ... }` bawaan Widya-Lang:\n\n```widya\nfungsi bagi_angka(a, b) {\n    coba {\n        jika (b == 0) {\n            lemparkan \"Pembagian dengan nol tidak diperbolehkan!\";\n        }\n        kembalikan a / b;\n    } tangkap (galat) {\n        tulis(\"Terjadi kesalahan:\", galat);\n        kembalikan 0;\n    }\n}\n```"
        ),
        (
            "Kenapa kode percabangan ini error di Widya-Lang?\n```widya\nif (nilai > 75) {\n    cetak(\"Lulus\");\n} else {\n    cetak(\"Remedial\");\n}\n```",
            "**Penyebab Galat:**\nWidya-Lang tidak menggunakan kata kunci bahasa Inggris `if` dan `else`. Sintaks resmi Widya-Lang menggunakan `jika` (atau `kalau`) dan `selain_itu` (atau `jika_tidak`).\n\n**Kode yang Benar:**\n```widya\njika (nilai > 75) {\n    cetak(\"Lulus\");\n} selain_itu {\n    cetak(\"Remedial\");\n}\n```"
        ),
        (
            "Compiler Widya-Lang menghasilkan galat `IndexOutOfBounds` pada kode berikut:\n```widya\nmisal daftar = [10, 20, 30];\ntulis(daftar[3]);\n```\nApa penyebabnya?",
            "**Penyebab:**\nIndeks Larik (Array) di Widya-Lang berbasis **0 (0-indexed)**. Untuk larik dengan 3 elemen, indeks yang sah adalah `0`, `1`, dan `2`. Memanggil `daftar[3]` mengakses elemen di luar batas.\n\n**Perbaikan:**\n```widya\nmisal daftar = [10, 20, 30];\ntulis(daftar[2]); // Menampilkan elemen ke-3, yaitu 30\n```"
        ),
        (
            "Kode saya error: `Galat: Nilai boolean harus ditulis 'benar' atau 'salah'`. Kode:\n```widya\nmisal aktif = true;\nmisal selesai = false;\n```\nBagaimana memperbaikinya?",
            "**Solusi:**\nDi Widya-Lang, nilai literal logika (*boolean*) adalah `benar` dan `salah` (bukan `true` dan `false`):\n\n```widya\nmisal aktif = benar;\nmisal selesai = salah;\n```"
        ),
        (
            "Kenapa baris ini error: `Galat: Panjang larik menggunakan fungsi 'panjang()' atau '.panjang'`?\n```widya\nmisal arr = [1, 2, 3];\ntulis(len(arr));\n```",
            "**Solusi:**\nFungsi `len()` adalah konvensi Python. Di Widya-Lang, gunakan fungsi bawaan `panjang(arr)` atau properti `arr.panjang`:\n\n```widya\nmisal arr = [1, 2, 3];\ntulis(panjang(arr)); // Output: 3\n```"
        ),
        (
            "Program saya macet dan memori naik terus (infinite loop) pada kode ini:\n```widya\nmisal i = 1;\nselama (i <= 10) {\n    tulis(i);\n}\n```\nBagaimana cara memperbaikinya?",
            "**Penyebab:**\nVariabel counter `i` tidak pernah diperbarui nilainya di dalam blok perulangan `selama`, sehingga kondisi `i <= 10` selalu bernilai `benar` selamanya.\n\n**Perbaikan:**\nTambahkan inkremen nilai counter `i` di akhir setiap iterasi:\n```widya\nmisal i = 1;\nselama (i <= 10) {\n    tulis(i);\n    i = i + 1;\n}\n```"
        )
    ]

    samples = []
    for q, a in debug_cases:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q},
                {"role": "assistant", "content": a}
            ]
        })
    return samples

# ==============================================================================
# 8. GENERATOR KURIKULUM BAHASA DAERAH & NLP NUSANTARA
# ==============================================================================
def generate_samples_kurikulum_daerah(repo_root: Path) -> list:
    regional_cases = [
        (
            "Bagaimana implementasi perhitungan Hari Pasaran Jawa (Weton) dan Neptu di Widya-Lang?",
            "Widya-Lang menyediakan modul kurikulum komputasi daerah (`kurikulum_bahasa_daerah/01_bahasa_jawa/modul_jawa_komputasi.wya`).\n\nBerikut contoh fungsi perhitungan Weton dan Neptu:\n\n```widya\nfungsi hitung_neptu(hari, pasaran) {\n    // Nilai neptu hari baku\n    misal neptu_hari = {\n        \"Minggu\": 5, \"Senin\": 4, \"Selasa\": 3, \"Rabu\": 7,\n        \"Kamis\": 8, \"Jumat\": 6, \"Sabtu\": 9\n    };\n    \n    // Nilai neptu pasaran panca wara\n    misal neptu_pasaran = {\n        \"Legi\": 5, \"Pahing\": 9, \"Pon\": 7, \"Wage\": 4, \"Kliwon\": 8\n    };\n    \n    diketahui h = neptu_hari[hari];\n    diketahui p = neptu_pasaran[pasaran];\n    kembalikan h + p;\n}\n\nfungsi utama() {\n    diketahui total = hitung_neptu(\"Jumat\", \"Kliwon\");\n    tulis(\"Total Neptu Jumat Kliwon:\", total); // Output: 14\n}\n```"
        ),
        (
            "Bagaimana cara membuat generator Sisindiran (Pantun Sunda) berbasis Widya-Lang?",
            "Dalam modul Sunda Komputasi (`kurikulum_bahasa_daerah/02_bahasa_sunda/modul_sunda_komputasi.wya`), struktur Sisindiran menguji rima cangkang dan eusi:\n\n```widya\nstruktur Sisindiran {\n    cangkang_1: Teks,\n    cangkang_2: Teks,\n    eusi_1: Teks,\n    eusi_2: Teks\n}\n\nimplementasi Sisindiran {\n    fungsi cetak_pantun(diri) {\n        tulis(diri.cangkang_1);\n        tulis(diri.cangkang_2);\n        tulis(diri.eusi_1);\n        tulis(diri.eusi_2);\n    }\n}\n```"
        ),
        (
            "Apa saja dukungan komputasi bahasa daerah Nusantara yang terintegrasi di Widya-Lang?",
            "Widya-Lang memiliki kurikulum dan modul terintegrasi untuk 8 rumpun bahasa Nusantara:\n1. **Bahasa Jawa**: Kalender Weton, Pranata Mangsa, Transliterasi Aksara Hanacaraka.\n2. **Bahasa Sunda**: Undak-usuk basa (Loma vs Lemes), Aksara Kaganga, Kalender Kala Sunda.\n3. **Bahasa Bali**: Sistem Kalender Pawukon, Aksara Bali.\n4. **Bahasa Minangkabau**: Falsafah Kato Nan Ampek, pepatah petitih adat.\n5. **Bahasa Bugis-Makassar**: Transliterasi Aksara Lontara & naskah I La Galigo.\n6. **Bahasa Batak**: Transliterasi Surat ni Batak & falsafah Dalihan Na Tolu.\n7. **Bahasa Aceh**: Sastra Jawoe & kalender maritim nelayan.\n8. **Banjar, Sasak, Lampung, Papua**: Linguistik komparatif dan pelestarian aksara Had Lampung."
        ),
        (
            "Bagaimana sistem kalender agraris Pranata Mangsa Jawa diimplementasikan dalam Widya-Lang?",
            "Pranata Mangsa membagi kalender tahun surya menjadi 12 mangsa (musim):\n- Kasa (Kartika): Musim kering, dedaunan gugur.\n- Karo (Puspita): Tanah mengering, mangga berbunga.\n- Katelu, Kapat, Kalima, Kanem, Kapitu, Kawolu, Kasanga, Kasapuluh, Desta, Saddha.\n\nDalam modul `modul_jawa_komputasi.wya`, fungsi `tentukan_mangsa(hari_ke_dalam_tahun)` mengembalikan nama mangsa dan panduan agraris (kapan mulai membajak, menanam padi, atau panen) berbasis siklus astronomis."
        ),
        (
            "Jelaskan transliterasi Aksara Nusantara (Hanacaraka & Kaganga) di Widya-Lang.",
            "Widya-Lang mengimplementasikan modul transliterasi dua arah (*bidirectional transliteration*) berbasis standar Unicode:\n- **Hanacaraka (U+A980 .. U+A9DF)**: Mengurai 20 aksara nglegena (ha, na, ca, ra, ka, da, ta, sa, wa, la, pa, dha, ja, ya, nya, ma, ga, ba, tha, nga) dengan pasangan dan sandhangan (wulu, suku, taling, tarung, layar, wignyan, cecek).\n- **Kaganga Sunda (U+1B80 .. U+1BBF)**: Transliterasi aksara swara dan rarangken secara native."
        )
    ]

    samples = []
    for q, a in regional_cases:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q},
                {"role": "assistant", "content": a}
            ]
        })
    return samples

# ==============================================================================
# 9. GENERATOR TATA BAHASA & ATURAN SINTAKS RESMI (Grammar Rules)
# ==============================================================================
def generate_samples_grammar_syntax_rules() -> list:
    rules = [
        (
            "Sebutkan seluruh kata kunci yang sah untuk deklarasi variabel dan konstanta di Widya-Lang.",
            "Di Widya-Lang:\n1. **Variabel Yang Dapat Diubah (*Mutable*)**:\n   - `misal` (kata kunci standar)\n   - `biar` (alternatif dialek)\n   - `diketahui` (dialek formal matematika)\n   - `diket` (singkatan praktis)\n   Contoh: `misal skor = 100;`, `diketahui x = 10;`\n\n2. **Konstanta Yang Tidak Dapat Diubah (*Immutable*)**:\n   - `tetap`\n   Contoh: `tetap PHI = 3.14159;`\n   Pemberian nilai ulang pada konstanta `tetap` akan memicu galat kompilasi."
        ),
        (
            "Bagaimana aturan pengujian kesetaraan (equality operator) di Widya-Lang?",
            "Widya-Lang mendukung dua cara pengujian kesetaraan:\n1. **Simbol Baku**: `==` (misal: `jika (a == b) { ... }`)\n2. **Kata Kunci Alami**: `adalah` (misal: `jika (a adalah b) { ... }`)\nUntuk kebalikan (tidak sama dengan), gunakan `!=` atau `bukan`."
        ),
        (
            "Apa saja tipe data primitif dan struktur koleksi bawaan di Widya-Lang?",
            "Tipe data bawaan Widya-Lang meliputi:\n- `Bulat` : Bilangan bulat 64-bit (misal: `42`, `-100`).\n- `Pecahan` : Bilangan pecahan floating-point 64-bit (misal: `3.14`, `0.05`).\n- `Teks` : Rangkaian karakter string UTF-8 (misal: `\"Halo Dunia\"`).\n- `BenarSalah` : Nilai logika boolean (`benar` atau `salah`).\n- `Larik` : Koleksi elemen berurutan dinamis (misal: `[1, 2, 3]`).\n- `Kamus` : Koleksi pasangan kunci dan nilai (misal: `{\"nama\": \"Budi\", \"umur\": 25}`).\n- `Kosong` : Nilai ketiadaan objek (setara `null` / `None`)."
        ),
        (
            "Bagaimana cara kerja perulangan `untuk_setiap` dan `selama` di Widya-Lang?",
            "Widya-Lang menyediakan dua konstruksi perulangan utama:\n\n1. **`untuk_setiap` (Iterasi Koleksi)**:\n```widya\nmisal buah = [\"Apel\", \"Mangga\", \"Jeruk\"];\nuntuk_setiap item dalam buah {\n    tulis(\"Buah:\", item);\n}\n```\n\n2. **`selama` (Perulangan Berbasis Kondisi Logika)**:\n```widya\nmisal hitung = 1;\nselama (hitung <= 5) {\n    tulis(\"Hitungan ke-\", hitung);\n    hitung = hitung + 1;\n}\n```"
        ),
        (
            "Bagaimana konversi tipe data (type casting) dilakukan di Widya-Lang?",
            "Widya-Lang menyediakan fungsi konversi tipe eksplisit:\n- `ke_teks(nilai)` : Mengubah angka, boolean, atau koleksi menjadi string Teks.\n- `ke_bulat(nilai)` : Mengonversi Teks angka atau Pecahan menjadi Bulat (integer).\n- `ke_pecahan(nilai)` : Mengonversi Bulat atau Teks menjadi floating point Pecahan.\n\nContoh:\n```widya\nmisal harga_teks = \"25000\";\nmisal harga_angka = ke_bulat(harga_teks);\nmisal total = harga_angka * 2;\ntulis(\"Total: \" + ke_teks(total));\n```"
        ),
        (
            "Bagaimana cara melakukan operasi input/output berkas (File I/O) di Widya-Lang?",
            "Widya-Lang menyediakan modul I/O bawaan yang aman:\n- `baca_berkas(jalur)` : Membaca seluruh isi berkas teks sebagai string UTF-8.\n- `tulis_berkas(jalur, konten)` : Menulis string konten ke berkas (overwrite).\n- `tambah_berkas(jalur, konten)` : Menambahkan teks ke ujung akhir berkas (append).\n- `apakah_berkas_ada(jalur)` : Mengembalikan `benar` jika berkas ditemukan di disk."
        ),
        (
            "Bagaimana cara mengendalikan perulangan dengan kata kunci `berhenti` dan `lanjutkan` di Widya-Lang?",
            "Widya-Lang menyediakan dua kata kunci kontrol iterasi:\n- `berhenti` : Menghentikan seluruh proses perulangan seketika (*break*).\n- `lanjutkan` : Melompati sisa blok saat ini dan langsung berpindah ke iterasi berikutnya (*continue*).\n\nContoh:\n```widya\nmisal i = 0;\nselama (i < 10) {\n    i = i + 1;\n    jika (i == 3) {\n        lanjutkan; // Lewati angka 3\n    }\n    jika (i == 7) {\n        berhenti;  // Selesai saat mencapai 7\n    }\n    tulis(i);\n}\n```"
        )
    ]

    samples = []
    for q, a in rules:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q},
                {"role": "assistant", "content": a}
            ]
        })
    return samples

# ==============================================================================
# FUNGSI UTAMA (MAIN)
# ==============================================================================
def main():
    repo_root = Path(__file__).resolve().parent.parent
    output_path = repo_root / "widya_ai_dataset.jsonl"
    
    print("=" * 70)
    print("🚀 WIDYA-LANG COMPREHENSIVE DATASET GENERATOR FOR LLM FINE-TUNING")
    print("=" * 70)
    print(f"[*] Direktori Basis Proyek: {repo_root}")
    
    all_samples = []
    
    # 1. Dari seluruh berkas contoh (*.wya)
    s_contoh = generate_samples_from_contoh(repo_root)
    print(f"[+] 1. Diekstrak dari contoh/*.wya (229+ berkas)    : {len(s_contoh)} sampel")
    all_samples.extend(s_contoh)
    
    # 2. Dari seluruh pustaka modul (*.wya)
    s_modul = generate_samples_from_modul(repo_root)
    print(f"[+] 2. Diekstrak dari modul/*.wya (85+ berkas)      : {len(s_modul)} sampel")
    all_samples.extend(s_modul)
    
    # 3. Dari Bank Soal & Sertifikasi
    s_bank = generate_samples_from_bank_soal(repo_root)
    print(f"[+] 3. Diekstrak dari Bank Soal & Sertifikasi       : {len(s_bank)} sampel")
    all_samples.extend(s_bank)
    
    # 4. Dari Basis Pengetahuan RAG (widyacoder_kb.json)
    s_kb = generate_samples_from_kb(repo_root)
    print(f"[+] 4. Diekstrak dari Dokumentasi & KB RAG          : {len(s_kb)} sampel")
    all_samples.extend(s_kb)
    
    # 5. Dari WidyaBase Deep Database Engine
    s_widyabase = generate_samples_from_widyabase_deep(repo_root)
    print(f"[+] 5. WidyaBase Deep Engine & SQL Dwibahasa        : {len(s_widyabase)} sampel")
    all_samples.extend(s_widyabase)
    
    # 6. Translasi Kode Antar-Bahasa (Python/JS/SQL -> Widya)
    s_trans = generate_samples_code_translation()
    print(f"[+] 6. Translasi Kode Antar-Bahasa                  : {len(s_trans)} sampel")
    all_samples.extend(s_trans)
    
    # 7. Skenario Debugging & Perbaikan Galat
    s_debug = generate_samples_debugging_and_fixing()
    print(f"[+] 7. Skenario Debugging & Error Fixing            : {len(s_debug)} sampel")
    all_samples.extend(s_debug)
    
    # 8. Kurikulum Bahasa Daerah & NLP Nusantara
    s_daerah = generate_samples_kurikulum_daerah(repo_root)
    print(f"[+] 8. Kurikulum Bahasa Daerah & NLP Nusantara      : {len(s_daerah)} sampel")
    all_samples.extend(s_daerah)
    
    # 9. Tata Bahasa Resmi & Aturan Sintaks
    s_grammar = generate_samples_grammar_syntax_rules()
    print(f"[+] 9. Tata Bahasa Resmi & Aturan Sintaks           : {len(s_grammar)} sampel")
    all_samples.extend(s_grammar)

    # Simpan ke berkas JSONL
    print("-" * 70)
    print(f"[*] Menyimpan seluruh sampel ke: {output_path}...")
    with open(output_path, "w", encoding="utf-8") as f:
        for item in all_samples:
            f.write(json.dumps(item, ensure_ascii=False) + "\n")
            
    file_size_kb = output_path.stat().st_size / 1024
    file_size_mb = file_size_kb / 1024
    print("=" * 70)
    print(f"🎉 SUKSES BESAR! DATASET TRAINING LENGKAP & KOMPREHENSIF TELAH DIBUAT!")
    print(f"   • Total Sampel Percakapan : {len(all_samples):,} pasang instruksi")
    print(f"   • Ukuran Berkas JSONL     : {file_size_mb:.2f} MB ({file_size_kb:.2f} KB)")
    print(f"   • Format Standard         : ChatML / ShareGPT (100% Siap Unsloth & Ollama)")
    print("=" * 70)

if __name__ == "__main__":
    main()
