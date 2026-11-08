# Spesifikasi Persamaan Matematika & OMML Editor 🧮

> **Spesifikasi Teknis Engine Rumus WidyaOffice Desktop**  
> *Translasi Murni: LaTeX Syntax ➔ OpenXML OMML (`<m:oMath>`) ➔ WYSIWYG Rendering*

---

## 1. Pendahuluan

Dalam sebagian besar aplikasi pengolah kata berbasis web atau turunan Markdown sederhana, rumus matematika biasanya disimpan sebagai **gambar raster (PNG/JPEG)** atau SVG statis. Pendekatan ini memiliki kelemahan fatal di lingkungan korporat/akademik:
- Rumus buram saat diperbesar atau dicetak beresolusi tinggi.
- Rumus tidak dapat diedit ulang saat berkas dibuka di Microsoft Word atau LibreOffice.
- Aksesibilitas (screen reader) dan reflow tipografi menjadi rusak.

**WidyaOffice Desktop** menyelesaikan masalah ini dengan memproses rumus secara semantik murni ke dalam standar **Office Math Markup Language (OMML)** sesuai standar internasional **ISO/IEC 29500-1**.

---

## 2. Alur Kerja Dual-Input (Visual Palette & LaTeX)

Pengguna WidyaOffice Desktop dapat memasukkan rumus dengan dua metode yang tersinkronisasi dua arah (*two-way sync*):

```
┌────────────────────────────────────────────────────────┐
│             Modal Sisipkan Persamaan Matematika         │
├────────────────────────────────────────────────────────┤
│ [Palet Visual]:                                        │
│  [ a/b Pecahan ]  [ √x Akar ]  [ xⁿ Pangkat ]  [ xᵢ ]  │
│  [ ∫ Integral ]   [ ∑ Sigma ]  [ π, α, β, θ, λ, Ω ]    │
│  [ ±, ×, ÷, ≠, ≤, ≥, ≈, ∞, →, ⇒, ∈, ⊂, ∇ ]            │
├────────────────────────────────────────────────────────┤
│ Input LaTeX / Plaintext:                               │
│ \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}                     │
├────────────────────────────────────────────────────────┤
│ Pratinjau Langsung (WYSIWYG KaTeX/Vector):             │
│                                                        │
│                    -b ± √(b² - 4ac)                    │
│               x = ──────────────────                   │
│                           2a                           │
│                                                        │
├────────────────────────────────────────────────────────┤
│ Opsi: [x] Tampilkan sebagai Blok (Tengah)  [ ] Inline  │
│ [ Batal ]                             [ Sisipkan (Enter) ]
└────────────────────────────────────────────────────────┘
```

---

## 3. Struktur OpenXML OMML (ISO/IEC 29500)

Ketika rumus disisipkan, modul [src/office/docx.rs](file:///home/devops/widya-lang/src/office/docx.rs) mengompilasi struktur ekspresi menjadi tag XML matematika murni.

### A. Deklarasi Namespace Wajib
Pada berkas `word/document.xml`, elemen induk `<w:document>` dideklarasikan dengan namespace resmi:
```xml
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math">
```

### B. Rumus Blok (Tersendiri di Tengah Halaman)
Rumus blok dibungkus dalam wadah `<m:oMathPara>` di dalam paragraf `<w:p>`:
```xml
<w:p>
  <w:pPr>
    <w:jc w:val="center"/>
  </w:pPr>
  <m:oMathPara>
    <m:oMath>
      <!-- Struktur Elemen Matematika di Sini -->
    </m:oMath>
  </m:oMathPara>
</w:p>
```

### C. Rumus Sejajar (Inline Math dalam Teks)
Rumus sejajar ditempatkan langsung sebagai elemen saudara dari teks `<w:r>`:
```xml
<w:p>
  <w:r><w:t>Persamaan energi relativistik adalah </w:t></w:r>
  <m:oMath>
    <m:r><m:t>E</m:t></m:r>
    <m:r><m:t>=</m:t></m:r>
    <m:sSup>
      <m:e><m:r><m:t>mc</m:t></m:r></m:e>
      <m:sup><m:r><m:t>2</m:t></m:r></m:sup>
    </m:sSup>
  </m:oMath>
  <w:r><w:t> sebagaimana dirumuskan Einstein.</w:t></w:r>
</w:p>
```

---

## 4. Tabel Pemetaan Sintaks LaTeX ke Elemen OMML

Modul [src/office/docx.rs](file:///home/devops/widya-lang/src/office/docx.rs) mengonversi operator matematika melalui pemetaan semantik berikut:

| Sintaks LaTeX | Komponen Rumus | Struktur Tag OpenXML OMML |
| :--- | :--- | :--- |
| `\frac{A}{B}` | Pecahan (*Fraction*) | `<m:f><m:num><m:r><m:t>A</m:t></m:r></m:num><m:den><m:r><m:t>B</m:t></m:r></m:den></m:f>` |
| `\sqrt{X}` | Akar Kuadrat (*Square Root*) | `<m:rad><m:radPr><m:degHide m:val="on"/></m:radPr><m:deg/><m:e><m:r><m:t>X</m:t></m:r></m:e></m:rad>` |
| `\sqrt[N]{X}` | Akar Pangkat-N (*N-th Root*) | `<m:rad><m:deg><m:r><m:t>N</m:t></m:r></m:deg><m:e><m:r><m:t>X</m:t></m:r></m:e></m:rad>` |
| `X^{P}` | Pangkat (*Superscript*) | `<m:sSup><m:e><m:r><m:t>X</m:t></m:r></m:e><m:sup><m:r><m:t>P</m:t></m:r></m:sup></m:sSup>` |
| `X_{I}` | Indeks Bawah (*Subscript*) | `<m:sSub><m:e><m:r><m:t>X</m:t></m:r></m:e><m:sub><m:r><m:t>I</m:t></m:r></m:sub></m:sSub>` |
| `\int_{A}^{B}` | Integral Tentu | Simbol `∫` dengan argumen batas bawah `A` dan batas atas `B` |
| `\sum_{I=1}^{N}` | Notasi Sigma | Simbol `∑` dengan sub-elemen batas bawah dan atas |
| `\alpha`, `\beta`, `\pi` | Simbol Huruf Yunani | Representasi Unicode UTF-8 (`α`, `β`, `π`) dalam elemen `<m:r><m:t>` |
| `\pm`, `\times`, `\leq` | Operator Perhitungan | Representasi Unicode UTF-8 (`±`, `×`, `≤`) dalam elemen `<m:r><m:t>` |

---

## 5. Kompatibilitas Lintas Aplikasi

Setiap berkas DOCX yang dibuat atau diedit di WidyaOffice Desktop diuji untuk menjamin interoperabilitas 100%:
1. **Microsoft Word (Office 2013, 2016, 2019, 2021, Office 365):**
   - Rumus dikenali secara bawaan sebagai objek *"Equation"*.
   - Pengguna dapat mengklik rumus di Word dan mengeditnya menggunakan Equation Tab bawaan Microsoft Word tanpa pesan peringatan kompatibilitas.
2. **LibreOffice Writer (v7.x / v8.x):**
   - Rumus terbaca dan ter-render sempurna sebagai Math Object.
3. **Konversi ke PDF:**
   - Saat diekspor ke PDF via [src/office/converter.rs](file:///home/devops/widya-lang/src/office/converter.rs), rumus di-render dalam format vektor beresolusi tajam tanpa pixelation pada level zoom berapapun (400%+).
