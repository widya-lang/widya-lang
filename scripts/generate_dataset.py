#!/usr/bin/env python3
"""
Widya-Lang Dataset Generator for Unsloth LLM Fine-Tuning
=========================================================
Mengekstrak seluruh basis pengetahuan Widya-Lang dari repositori:
1. 176+ berkas contoh nyata (contoh/*.wya)
2. 60+ modul pustaka standar (modul/*.wya)
3. Dokumentasi resmi & database engine (DOKUMENTASI_WIDYABASE.md, README.md, dll.)

Format Keluaran: ShareGPT / ChatML JSONL yang kompatibel 100% dengan Unsloth & SFTTrainer.
"""

import os
import re
import json
import glob
from pathlib import Path

SYSTEM_PROMPT = """Anda adalah WidyaCoder, asisten kecerdasan buatan ahli pemrograman bahasa Widya-Lang dan mesin basis data WidyaBase.
Anda menguasai secara mendalam:
1. Sintaks bahasa Widya-Lang (Dialek Nusantara dan padanan modern): misal, tetapan, fungsi, struktur, sifat, jika-lain, untuk, selama, coba-tangkap.
2. Pustaka standar dan modul ekosistem: widyabase, desktop, mobile, ai_rag_stack, deep_learning, dataframe, dll.
3. Kueri SQL Dwibahasa WidyaBase: PILIH/SELECT, DARI/FROM, DIMANA/WHERE, SISIP_KE/INSERT, PERBARUI/UPDATE, BUAT_TABEL, DENGAN/WITH (CTE), LEWAT/OVER (Window Functions), TITIK_SIMPAN/SAVEPOINT, KUNCI_ASING/FOREIGN KEY.
Berikan jawaban yang jelas, akurat, dan sertakan contoh kode Widya-Lang yang siap dijalankan."""

def clean_text(text: str) -> str:
    return text.strip()

def extract_header_comment(content: str) -> tuple[str, str]:
    """Mengambil judul dan deskripsi dari komentar atas file .wya"""
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
            if "CONTOH" in clean_line or "MODUL" in clean_line or "DEMO" in clean_line:
                title = clean_line
            elif clean_line:
                desc_lines.append(clean_line)
        else:
            if not stripped:
                continue
            break
            
    desc = " ".join(desc_lines)
    return title, desc

def generate_samples_from_contoh(repo_root: Path) -> list:
    samples = []
    contoh_dir = repo_root / "contoh"
    if not contoh_dir.exists():
        return samples

    for file_path in glob.glob(str(contoh_dir / "*.wya")):
        p = Path(file_path)
        try:
            content = p.read_text(encoding="utf-8")
        except Exception:
            continue

        title, desc = extract_header_comment(content)
        name_clean = p.stem.replace("_", " ")

        # 1. Sample: Tuliskan kode untuk...
        user_prompt_1 = f"Buatkan contoh program Widya-Lang untuk {name_clean}."
        if desc:
            user_prompt_1 += f" Spesifikasi: {desc[:250]}"
        
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": user_prompt_1},
                {"role": "assistant", "content": f"Berikut adalah implementasi program Widya-Lang untuk {name_clean}:\n\n```widya\n{content}\n```"}
            ]
        })

        # 2. Sample: Jelaskan cara kerja kode ini
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Jelaskan kode Widya-Lang berikut dan apa tujuannya:\n\n```widya\n{content[:1500]}\n```"},
                {"role": "assistant", "content": f"Kode di atas adalah program Widya-Lang untuk **{name_clean}**.\n\n**Tujuan & Fitur Utama:**\n{desc if desc else 'Mengimplementasikan logika komputasi menggunakan sintaks dan modul Widya-Lang.'}\n\nProgram ini menunjukkan bagaimana Widya-Lang mengelola struktur data, fungsi, dan eksekusi secara berdaulat dan efisien."}
            ]
        })

    return samples

def generate_samples_from_modul(repo_root: Path) -> list:
    samples = []
    modul_dir = repo_root / "modul"
    if not modul_dir.exists():
        return samples

    for file_path in glob.glob(str(modul_dir / "*.wya")):
        p = Path(file_path)
        try:
            content = p.read_text(encoding="utf-8")
        except Exception:
            continue

        modul_name = p.stem
        # Sample: Penggunaan modul
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": f"Bagaimana cara mengimpor dan memanfaatkan modul `{modul_name}` di Widya-Lang?"},
                {"role": "assistant", "content": f"Untuk menggunakan modul `{modul_name}`, Anda dapat mengimpornya dengan perintah `impor \"./modul/{modul_name}.wya\";`.\n\nBerikut struktur kode modul `{modul_name}`:\n\n```widya\n{content[:2000]}\n```\n\nModul ini menyediakan fungsi dan struktur siap pakai untuk mempercepat pengembangan aplikasi di ekosistem Widya-Lang."}
            ]
        })

    return samples

def generate_samples_from_widyabase_doc(repo_root: Path) -> list:
    samples = []
    doc_path = repo_root / "DOKUMENTASI_WIDYABASE.md"
    if not doc_path.exists():
        return samples

    # Pertanyaan terstruktur tentang WidyaBase
    qa_list = [
        (
            "Apa itu WidyaBase dan apa saja fitur kuncinya?",
            "WidyaBase adalah mesin basis data mandiri berkinerja native, berdaulat, dan berskala industri yang terintegrasi di dalam ekosistem Widya-Lang.\n\nFitur-fitur utamanya meliputi:\n1. Penyimpanan Slotted-Page 4096-byte dengan Buffer Pool Manager LRU.\n2. Write-Ahead Logging (WAL) dengan Crash Recovery berbasis urutan LSN.\n3. Multi-Version Concurrency Control (MVCC) Snapshot Isolation (pembaca tidak memblokir penulis).\n4. B+Tree Indexing O(log N) untuk kunci utama dan indeks sekunder.\n5. SQL Dwibahasa Simetris (Bahasa Indonesia Nusantara dan Standar ANSI Global).\n6. Daemon PostgreSQL Wire Protocol v3.0 (pgwire) di port 5432.\n7. Multi-Modal AI (HNSW Vektor Cosine Similarity & parser JSONB).\n8. Analitik Enterprise: CTE (WITH/DENGAN), Window Functions (OVER/LEWAT), Views, Savepoints, dan Kunci Asing (Foreign Keys)."
        ),
        (
            "Tuliskan padanan kata kunci SQL Bahasa Indonesia dengan Standar ANSI di WidyaBase.",
            "Berikut padanan kata kunci SQL di WidyaBase:\n- `PILIH` = `SELECT`\n- `DARI` = `FROM`\n- `DIMANA` / `DI_MANA` = `WHERE`\n- `SISIP_KE` / `MASUKKAN_KE` = `INSERT INTO`\n- `NILAI` = `VALUES`\n- `PERBARUI` / `UBAH` = `UPDATE`\n- `ATUR` = `SET`\n- `HAPUS_DARI` = `DELETE FROM`\n- `BUAT_TABEL` = `CREATE TABLE`\n- `KUNCI_UTAMA` = `PRIMARY KEY`\n- `KUNCI_ASING` ... `REFERENSI` = `FOREIGN KEY` ... `REFERENCES`\n- `GABUNG` / `GABUNG_KIRI` = `INNER JOIN` / `LEFT JOIN`\n- `KELOMPOKKAN_BERDASARKAN` = `GROUP BY`\n- `MEMILIKI` = `HAVING`\n- `URUTKAN BERDASARKAN` = `ORDER BY`\n- `NAIK` / `TURUN` = `ASC` / `DESC`\n- `BERBEDA` / `UNIK` = `DISTINCT`\n- `DENGAN ... SEBAGAI` = `WITH ... AS` (CTE)\n- `LEWAT (PARTISI_BERDASARKAN ...)` = `OVER (PARTITION BY ...)` (Window Functions)\n- `TITIK_SIMPAN` = `SAVEPOINT`\n- `GUGURKAN_KE` = `ROLLBACK TO`"
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
        )
    ]

    for q, a in qa_list:
        samples.append({
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": q},
                {"role": "assistant", "content": a}
            ]
        })

    return samples

def main():
    repo_root = Path(__file__).resolve().parent.parent
    output_path = repo_root / "widya_ai_dataset.jsonl"
    
    print(f"[*] Mengekstrak dataset dari repositori: {repo_root}")
    all_samples = []
    
    # 1. Dari contoh
    contoh_samples = generate_samples_from_contoh(repo_root)
    print(f"[+] Diekstrak dari contoh/*.wya: {len(contoh_samples)} sampel")
    all_samples.extend(contoh_samples)
    
    # 2. Dari modul
    modul_samples = generate_samples_from_modul(repo_root)
    print(f"[+] Diekstrak dari modul/*.wya: {len(modul_samples)} sampel")
    all_samples.extend(modul_samples)
    
    # 3. Dari dokumentasi WidyaBase
    doc_samples = generate_samples_from_widyabase_doc(repo_root)
    print(f"[+] Diekstrak dari dokumentasi: {len(doc_samples)} sampel")
    all_samples.extend(doc_samples)

    # Simpan ke JSONL
    with open(output_path, "w", encoding="utf-8") as f:
        for item in all_samples:
            f.write(json.dumps(item, ensure_ascii=False) + "\n")
            
    print(f"\n[✓] Sukses! Total {len(all_samples)} sampel dataset tersimpan di: {output_path}")
    print(f"[*] Ukuran berkas: {output_path.stat().st_size / 1024:.2f} KB")

if __name__ == "__main__":
    main()
