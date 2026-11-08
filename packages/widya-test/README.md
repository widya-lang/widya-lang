# @widya/test

Framework penegasan (*assertion*) dan pelari pengujian (*test runner*) resmi untuk **Widya-Lang**, setara dengan **Jest**, **Pytest**, dan `cargo test`.

## Fitur Utama

- **Test Runner Ringan**: `uji_buat_pelari`, `uji_kasus`, `uji_ringkasan`.
- **Assertion Kaya**: `tegaskan_sama`, `tegaskan_tidak_sama`, `tegaskan_benar`, `tegaskan_salah`.
- **Zero-Dependency**: Murni ditulis dalam sintaksis Widya.

## Contoh Penggunaan

```widya
impor "@widya/test" sebagai test;

misal pelari = test.uji_buat_pelari();

fungsi uji_aritmatika(t) {
    test.tegaskan_sama(t, 2 + 2, 4, "Penjumlahan dasar gagal");
    test.tegaskan_tidak_sama(t, 2 * 3, 5, "Perkalian harus 6");
    test.tegaskan_benar(t, 10 > 5, "10 lebih besar dari 5");
}

test.uji_kasus(pelari, "Uji Aritmatika Dasar", uji_aritmatika);

misal hasil = test.uji_ringkasan(pelari);
tulis("Lulus: " + ke_teks(hasil["lulus"]) + "/" + ke_teks(hasil["total"]));
```

