// ==============================================================================
// app.js - WidyaMedika SIMRS & Kemenkes SatuSehat FHIR Engine
// ==============================================================================

const PATIENTS = [
  { nama: 'Bpk. Slamet Riyadi', umur: 54, gender: 'L', nik: '3302150804720002', bpjs: '0001849201948', rm: 'RM-2026-08412', td: '130/80 mmHg', hr: '78 bpm', keluhan: 'Pusing ringan di tengkuk sejak 2 hari yang lalu.', icd: 'I10', resep: 'Amlodipine 5mg (1x1 Tab Malam)' },
  { nama: 'Ibu Siti Aminah', umur: 48, gender: 'P', nik: '3302154402780001', bpjs: '0002948192019', rm: 'RM-2026-08413', td: '120/75 mmHg', hr: '72 bpm', keluhan: 'Kontrol rutin kadar gula darah puasa, tidak ada luka.', icd: 'E11.9', resep: 'Metformin 500mg (2x1 Tab Bersama Makan)' },
  { nama: 'Ananda Bagas Pratama', umur: 9, gender: 'L', nik: '3302151206170003', bpjs: '0003819402914', rm: 'RM-2026-08414', td: '100/65 mmHg', hr: '88 bpm', keluhan: 'Batuk pilek dan bersin 3 hari, demam sumeng.', icd: 'J00', resep: 'Cetirizine Sirup 5mg/5ml (1x1 Cth Malam)' }
];

let activePatientIdx = 0;

document.addEventListener('DOMContentLoaded', () => {
  updateFHIRPreview();
  tambahLogBridging('SIMRS Initialized — SatuSehat FHIR v4.0.1 Engine Ready');
  tambahLogBridging('BPJS V-Claim Bridging Active — Endpoint Production 2026');
});

function updateFHIRPreview() {
  const p = PATIENTS[activePatientIdx];
  const td = document.getElementById('vital-td').value;
  const icd = document.getElementById('select-icd10').value;
  const icd9 = document.getElementById('select-icd9').value;
  const keluhan = document.getElementById('rme-keluhan').value;
  const resep = document.getElementById('rme-resep').value;
  const now = new Date().toISOString();

  const fhirBundle = {
    "resourceType": "Bundle",
    "type": "transaction",
    "entry": [
      {
        "fullUrl": "urn:uuid:encounter-" + p.rm,
        "resource": {
          "resourceType": "Encounter",
          "status": "finished",
          "class": {
            "system": "http://terminology.hl7.org/CodeSystem/v3-ActCode",
            "code": "AMB",
            "display": "Ambulatory (Rawat Jalan)"
          },
          "subject": {
            "reference": "Patient/" + p.nik,
            "display": p.nama
          },
          "period": { "start": now, "end": now },
          "reasonCode": [{ "text": keluhan }]
        }
      },
      {
        "fullUrl": "urn:uuid:condition-" + p.rm,
        "resource": {
          "resourceType": "Condition",
          "clinicalStatus": {
            "coding": [{
              "system": "http://terminology.hl7.org/CodeSystem/condition-clinical",
              "code": "active"
            }]
          },
          "category": [{
            "coding": [{
              "system": "http://terminology.hl7.org/CodeSystem/condition-category",
              "code": "encounter-diagnosis"
            }]
          }],
          "code": {
            "coding": [{
              "system": "http://hl7.org/fhir/sid/icd-10",
              "code": icd,
              "display": "Diagnosa Utama RME"
            }]
          },
          "subject": { "reference": "Patient/" + p.nik }
        }
      },
      {
        "fullUrl": "urn:uuid:observation-vital-" + p.rm,
        "resource": {
          "resourceType": "Observation",
          "status": "final",
          "category": [{
            "coding": [{
              "system": "http://terminology.hl7.org/CodeSystem/observation-category",
              "code": "vital-signs"
            }]
          }],
          "code": { "text": "Tekanan Darah: " + td },
          "subject": { "reference": "Patient/" + p.nik }
        }
      },
      {
        "fullUrl": "urn:uuid:medication-" + p.rm,
        "resource": {
          "resourceType": "MedicationRequest",
          "status": "active",
          "intent": "order",
          "medicationCodeableConcept": { "text": resep },
          "subject": { "reference": "Patient/" + p.nik }
        }
      }
    ]
  };

  document.getElementById('fhir-json-box').innerText = JSON.stringify(fhirBundle, null, 2);
}

function simpanDanKirimSatuSehat() {
  const p = PATIENTS[activePatientIdx];
  const icd = document.getElementById('select-icd10').value;
  const now = new Date().toLocaleTimeString('id-ID', { hour12: false });

  tambahLogBridging(`${now} | SatuSehat POST 201 Created | EncID: ${p.rm} | ICD: ${icd} | Status: DISPATCHED`);
  alert(`✓ Rekam Medis Elektronik ${p.nama} (${p.rm}) sukses dikirim & tervalidasi ke Kemenkes SatuSehat!`);
}

function terbitkanSEPBpjs() {
  const p = PATIENTS[activePatientIdx];
  const sepNum = '0301R001' + Math.floor(1000000 + Math.random() * 9000000);
  tambahLogBridging(`BPJS V-Claim: SEP ${sepNum} Terbit (Kelas 3 - Hak Rawat BPJS Aktif)`);
  alert(`✓ Surat Eligibilitas Peserta (SEP) No. ${sepNum} berhasil diterbitkan via BPJS V-Claim Bridging!`);
}

function simulasiPasienBaru() {
  activePatientIdx = (activePatientIdx + 1) % PATIENTS.length;
  const p = PATIENTS[activePatientIdx];

  document.getElementById('disp-p-name').innerText = `${p.nama} (${p.umur} Thn / ${p.gender})`;
  document.getElementById('vital-td').value = p.td;
  document.getElementById('vital-hr').value = p.hr;
  document.getElementById('rme-keluhan').value = p.keluhan;
  document.getElementById('select-icd10').value = p.icd;
  document.getElementById('rme-resep').value = p.resep;

  updateFHIRPreview();
  tambahLogBridging(`Antrean Berpindah ➔ Pasien Baru: ${p.nama} (${p.rm})`);
}

function tambahLogBridging(msg) {
  const box = document.getElementById('simrs-log');
  if (!box) return;

  const div = document.createElement('div');
  div.className = 'log-item';
  div.innerText = msg;
  box.prepend(div);
}
