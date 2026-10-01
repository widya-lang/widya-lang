# PRODUCT REQUIREMENT DOCUMENT (PRD)
## PROYEK: WIDYAMEDIKA SIMRS & SATUSEHAT HL7 FHIR BRIDGING
**Disusun Oleh:** `agen_analis` (SOP-02) & `agen_arsitek` (SOP-03)  
**Standar Regulasi:** Permenkes No. 24/2022 tentang Rekam Medis Elektronik (RME) & HL7 FHIR v4.0.1  

---

### 1. Problem Statement & Tujuan
Rumah Sakit Umum Daerah (RSUD), Puskesmas, dan Klinik Pratama diwajibkan oleh Kemenkes RI untuk menghubungkan sistem Rekam Medis Elektronik (RME) ke platform SatuSehat nasional dan BPJS Kesehatan V-Claim.

**Tujuan**: Menyediakan sistem SIMRS berdaulat dengan validasi otomatis resource FHIR (`Encounter`, `Condition`, `Observation`, `MedicationRequest`), pemetaan kode diagnosis **ICD-10** & tindakan **ICD-9-CM**, serta penerbitan SEP BPJS instan.
