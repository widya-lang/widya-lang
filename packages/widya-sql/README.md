# @widya/sql

Pustaka basis data relasional in-memory dan query builder untuk bahasa pemrograman **Widya-Lang**, setara dengan **SQLite3**, **Knex.js**, dan **Prisma**.

## Fitur Utama

- **In-Memory Store**: Eksekusi kueri ultra-cepat tanpa dependensi eksternal.
- **Relational Operations**: `db_buat_tabel`, `db_sisip` (INSERT), `db_pilih` (SELECT dengan WHERE, PROJECTION, LIMIT), `db_perbarui` (UPDATE), `db_hapus` (DELETE), dan `db_hitung` (COUNT).
- **Zero-Dependency**: Murni ditulis dalam sintaksis Widya.

## Contoh Penggunaan

```widya
impor "@widya/sql" sebagai sql;

misal db = sql.db_buka("sistem_analitika");
sql.db_buat_tabel(db, "pengguna");

sql.db_sisip(db, "pengguna", {"id": 1, "nama": "Ahmad", "skor": 95});
sql.db_sisip(db, "pengguna", {"id": 2, "nama": "Budi", "skor": 70});

fungsi filter_skor_tinggi(row) {
    kembalikan row["skor"] > 80;
}

misal hasil = sql.db_pilih(db, "pengguna", filter_skor_tinggi, ["nama", "skor"]);
sql.db_tutup(db);
```

