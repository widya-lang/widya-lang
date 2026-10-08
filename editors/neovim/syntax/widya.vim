" Vim Syntax File untuk Widya-Lang
" Bahasa: Widya-Lang (.wya, .widya, .wd)
" Lisensi: MIT

if exists("b:current_syntax")
  finish
endif

" Komentar
syntax region widyaCommentLine start="//" end="$" contains=widyaTodo,@Spell
syntax region widyaCommentBlock start="/\*" end="\*/" contains=widyaTodo,@Spell
syntax keyword widyaTodo contained TODO FIXME CATATAN PENTING PERINGATAN BUG

" Nilai Literal & Konstanta
syntax match widyaNumber "\v<\d+(\.\d+)?([eE][+-]?\d+)?>"
syntax match widyaHexNumber "\v<0x[0-9a-fA-F]+>"
syntax keyword widyaBoolean benar salah true false
syntax keyword widyaNull kosong nihil nil null

" Teks & String
syntax region widyaString start='"' skip='\\"' end='"' contains=widyaEscape
syntax match widyaEscape contained "\\[nrt\"'\\]"

" Kata Kunci Kontrol & Deklarasi Dwibahasa
syntax keyword widyaKeyword fn fungsi let tetapkan misal var tetap mut const
syntax keyword widyaKeyword if jika kalau else lain lainnya match cocokkan
syntax keyword widyaKeyword for untuk in dalam while selama ulang berhenti break lanjut continue
syntax keyword widyaKeyword return kembalikan struct struktur enum pilihan impl terapkan trait sifat
syntax keyword widyaKeyword where di_mana type tipe async asinkron await tunggu_hasil
syntax keyword widyaKeyword try coba catch tangkap throw lempar impor import sebagai as
syntax keyword widyaKeyword gunakan use modul module ekspor export pasok vendor
syntax keyword widyaKeyword baru new ini this eksternal volatil pinjam mutabel

" Basis Data WidyaBase Keywords
syntax keyword widyaKeyword basis_data tabel kueri sisip perbarui hapus indeks transaksi komit batal skema kolom

" Operator Logika Dwibahasa
syntax keyword widyaOperator dan atau bukan
syntax match widyaOperator "[\+\-\*/\%\=\!\<\>\&\|\^]"

" Tipe Bawaan Sistem
syntax keyword widyaType Int Angka Float String Teks Bool Bolean Array Larik Map Peta
syntax keyword widyaType Byte Waktu BasisData Hasil Opsi Uuid Blob Tabel Kueri Option Result Any Void Objek

" Fungsi Bawaan Populer
syntax keyword widyaBuiltin cetak pastikan ke_teks ke_angka panjang tipe_dari
syntax keyword widyaBuiltin buka_basis_data eksekusi_kueri kueri mulai_transaksi
syntax keyword widyaBuiltin BuatAplikasiDesktop JendelaDesktop MenuBilah BakiSistem
syntax keyword widyaBuiltin baca_berkas tulis_berkas hapus_berkas

" Anotasi & Atribut
syntax match widyaAnnotation "@\[[a-zA-Z0-9_() ,\t]*\]"
syntax match widyaAnnotation "#\[[a-zA-Z0-9_() ,\t]*\]"

" Hubungkan Grup Warna Sintaksis Vim
highlight default link widyaCommentLine Comment
highlight default link widyaCommentBlock Comment
highlight default link widyaTodo Todo
highlight default link widyaNumber Number
highlight default link widyaHexNumber Number
highlight default link widyaBoolean Boolean
highlight default link widyaNull Constant
highlight default link widyaString String
highlight default link widyaEscape SpecialChar
highlight default link widyaKeyword Keyword
highlight default link widyaOperator Operator
highlight default link widyaType Type
highlight default link widyaBuiltin Function
highlight default link widyaAnnotation PreProc

let b:current_syntax = "widya"
