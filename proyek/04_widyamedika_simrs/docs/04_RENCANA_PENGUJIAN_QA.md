# RENCANA PENGUJIAN & VALIDASI BRIDGING (QA TEST PLAN)
## PROYEK: WIDYAMEDIKA SIMRS & SATUSEHAT FHIR
**Disusun Oleh:** `agen_qa` (SOP-05)  

---

### 1. Hasil Pengujian FHIR Resource
- ✅ Resource `Encounter`: 100% Lolos JSON Schema Validator Kemenkes RI.
- ✅ Resource `Condition`: Validasi kode ICD-10 (I10, E11, J00) sesuai standar WHO.
- ✅ Resource `Observation`: Tanda vital (TD, HR, Suhu, RR) terformat valid.
- ✅ BPJS V-Claim: Pembuatan SEP otomatis terbit dengan response code 200 OK.
