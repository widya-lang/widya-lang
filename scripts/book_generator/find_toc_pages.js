const fs = require('fs');

const buf = fs.readFileSync('Buku_Panduan_Pemrograman_Widya_Lang.pdf');
const pdf = buf.toString('latin1');

// In Edge PDFs, text strings are encoded. Let's find occurrences of chapter titles.
const chapters = [
  { id: "Bab 1", title: "Menyiapkan Lingkungan Pengembangan" },
  { id: "Bab 2", title: "Sistem Tipe Data Primitif" },
  { id: "Bab 3", title: "Percabangan & Pola Perulangan" },
  { id: "Bab 4", title: "Fungsi, Rekursi" },
  { id: "Bab 5", title: "Koleksi Data" },
  { id: "Bab 6", title: "Pemrograman Berorientasi Entitas" },
  { id: "Bab 7", title: "Penanganan Galat" },
  { id: "Bab 8", title: "Modularitas, Sistem Impor" },
  { id: "Bab 9", title: "Rekayasa Aplikasi Desktop GUI" },
  { id: "Bab 10", title: "Rekayasa Aplikasi Mobile" },
  { id: "Bab 11", title: "Integrasi Basis Data Mandiri WidyaBase" },
  { id: "Bab 12", title: "Pemrograman Konkuren" },
  { id: "Bab 13", title: "Komputasi Sains" },
  { id: "Bab 14", title: "Kolaborasi Cerdas dengan AI Lokal" },
  { id: "Bab 15", title: "Keamanan Siber" },
  { id: "Bab 16", title: "Praktik Pengujian Perangkat Lunak" },
  { id: "Bab 17", title: "Panduan Distribusi" },
  { id: "Lampiran A", title: "Kamus Lengkap Sintaks" },
  { id: "Lampiran B", title: "Tabel Komparasi Bahasa" },
  { id: "Lampiran C", title: "Direktori Modul Ekosistem" },
  { id: "Lampiran D", title: "Diagnostik Galat Compiler" }
];

console.log("Analyzing page breaks...");
// In our HTML, each page-break class causes a new page. Let's count page-breaks before each chapter in the HTML!
const html = fs.readFileSync('docs/BUKU_PANDUAN_PEMROGRAMAN_WIDYA.html', 'utf8');

// The cover is page 1
// Kata pengantar has page-break (page 2)
// TOC has page-break (page 3)
// Then each chapter...
// Let's compute actual pages by counting page-break and section heights
