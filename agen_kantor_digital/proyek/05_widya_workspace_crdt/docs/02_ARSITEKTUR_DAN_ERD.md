# ARSITEKTUR CRDT & ENTITY RELATIONSHIP DIAGRAM (ERD)
## PROYEK: WIDYA WORKSPACE COLLABORATIVE CRDT
**Disusun Oleh:** `agen_arsitek` (SOP-03)  

---

### 1. Model State Machine CRDT (Logoot / LWW)

```mermaid
graph TD
    ClientA[User A: Lead Coder] -->|Insert Op ID 1.2| CRDT_Core[CRDT State Matrix]
    ClientB[User B: Arsitek] -->|Insert Op ID 1.5| CRDT_Core
    ClientC[User C: Notulis] -->|Delete Op ID 2.0| CRDT_Core
    CRDT_Core -->|Deterministic Merge| UnifiedState[Unified Document AST]
    UnifiedState --> RenderPaper[IEEE Document View & PDF]
```

### 2. Skema Basis Data Workspace
```sql
BUAT_TABEL JIKA_TIDAK_ADA dokumen_workspace (
    id_dokumen TEKS KUNCI_UTAMA,
    judul TEKS,
    konten_markdown TEKS,
    penulis_terakhir TEKS,
    versi INT,
    waktu_update TEKS
);
```
