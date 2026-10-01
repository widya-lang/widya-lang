// ==============================================================================
// app.js - SmartCity ISPU Telemetry & GIS Logic (Permen LHK No. P.14/2020)
// ==============================================================================

let truckX = 280;
let truckDir = 1;
let trafficLightState = 'green';

document.addEventListener('DOMContentLoaded', () => {
  kalkulasiISPU();
  startTruckMovement();
  startTrafficCycle();
});

// INTERPOLASI RESMI ISPU PERMEN LHK No. P.14/2020
function getISPU_PM25(conc) {
  const table = [
    { Xb: 0.0, Xa: 15.5, Ib: 0, Ia: 50, cat: 'BAIK', color: 'text-green', recom: 'Tingkat mutu udara sangat baik, tidak memberikan dampak negatif terhadap manusia, hewan, dan tumbuhan.' },
    { Xb: 15.6, Xa: 55.4, Ib: 51, Ia: 100, cat: 'SEDANG', color: 'text-amber', recom: 'Kualitas udara masih dapat diterima, namun bagi kelompok sensitif disarankan mengurangi aktivitas luar ruang.' },
    { Xb: 55.5, Xa: 150.4, Ib: 101, Ia: 200, cat: 'TIDAK SEHAT', color: 'text-red', recom: 'Dapat merugikan kesehatan pada sejumlah segmen populasi yang terpapar. Gunakan masker saat keluar.' },
    { Xb: 150.5, Xa: 250.4, Ib: 201, Ia: 300, cat: 'SANGAT TIDAK SEHAT', color: 'text-purple', recom: 'Dapat meningkatkan sensitivitas pada penderita asma dan penyakit paru obstruktif kronis.' },
    { Xb: 250.5, Xa: 500.0, Ib: 301, Ia: 500, cat: 'BERBAHAYA', color: 'text-red', recom: 'Tingkat kualitas udara yang dapat merugikan kesehatan serius pada seluruh populasi. Tetap di dalam ruangan!' }
  ];

  for (let row of table) {
    if (conc >= row.Xb && conc <= row.Xa) {
      const ispu = Math.round(((row.Ia - row.Ib) / (row.Xa - row.Xb)) * (conc - row.Xb) + row.Ib);
      return { ispu, ...row };
    }
  }

  if (conc > 500) {
    return { ispu: 500, cat: 'BERBAHAYA', color: 'text-red', recom: 'Tingkat kualitas udara darurat!' };
  }
  return { ispu: 0, cat: 'BAIK', color: 'text-green', recom: 'Kualitas udara sangat bersih.' };
}

function kalkulasiISPU() {
  const conc = Number(document.getElementById('input-pm25').value) || 0;
  const res = getISPU_PM25(conc);

  document.getElementById('calc-ispu-val').innerText = `${res.ispu} (${res.cat})`;
  document.getElementById('calc-ispu-val').className = res.color;
}

// PILIH NODE PADA PETA
function pilihNode(id, title, ispu, detail) {
  document.getElementById('node-title').innerText = title;
  document.getElementById('node-ispu-num').innerText = ispu;

  let cat = 'BAIK';
  let col = 'text-green';
  let recom = 'Kualitas udara sangat baik dan menyegarkan.';

  if (ispu > 50 && ispu <= 100) {
    cat = 'SEDANG'; col = 'text-amber'; recom = 'Kualitas udara masih dapat diterima, kurangi aktivitas berat.';
  } else if (ispu > 100) {
    cat = 'TIDAK SEHAT'; col = 'text-red'; recom = 'Kawasan industri / asap padat. Disarankan mengenakan masker.';
  }

  const catEl = document.getElementById('node-cat');
  catEl.innerText = cat;
  catEl.className = 'score-cat ' + col;
  document.getElementById('node-ispu-num').className = 'score-num ' + col;
  document.getElementById('node-recom').innerText = recom + ` [${detail}]`;
}

// PILIH TRAFFIC
function pilihTraffic(title, vc, green) {
  document.getElementById('node-title').innerText = title;
  document.getElementById('node-ispu-num').innerText = '0.85';
  document.getElementById('node-cat').innerText = 'ATCS V/C RATIO';
  document.getElementById('node-cat').className = 'score-cat text-amber';
  document.getElementById('node-recom').innerText = `Simpang terpantau dengan ${green}. Optimasi rasio volume terhadap kapasitas berjalan dinamis.`;
}

// MOVING FLEET TRUCK ANIMATION
function startTruckMovement() {
  setInterval(() => {
    truckX += truckDir * 4;
    if (truckX > 480) truckDir = -1;
    if (truckX < 180) truckDir = 1;

    const truckEl = document.getElementById('fleet-truck-01');
    if (truckEl) {
      truckEl.setAttribute('transform', `translate(${truckX - 280}, 0)`);
    }
  }, 100);
}

// TRAFFIC LIGHT CYCLE
function startTrafficCycle() {
  setInterval(() => {
    const bulb = document.getElementById('traffic-light-bulb');
    if (!bulb) return;

    if (trafficLightState === 'green') {
      trafficLightState = 'yellow';
      bulb.setAttribute('fill', '#f59e0b');
    } else if (trafficLightState === 'yellow') {
      trafficLightState = 'red';
      bulb.setAttribute('fill', '#ef4444');
    } else {
      trafficLightState = 'green';
      bulb.setAttribute('fill', '#10b981');
    }
  }, 3500);
}

function optimasiLampuLaluLintas() {
  const vc = (0.65 + Math.random() * 0.3).toFixed(2);
  const green = Math.round(30 + vc * 35);

  document.getElementById('disp-vc').innerText = `${vc} (Terkalkulasi)`;
  document.getElementById('disp-green').innerText = `${green} Detik`;
  alert(`⚡ Optimasi ATCS Webster: Rasio V/C ${vc} dihitung ulang. Fase hijau disesuaikan menjadi ${green} detik!`);
}

function simulasiAnomaliSensor() {
  pilihNode('ispu-3', 'Stasiun Timur - Kawasan Industri (ALERT)', 185, 'PM2.5: 112.5 µg/m³ [ASAP TEBAL]');
  document.getElementById('input-pm25').value = '112.5';
  kalkulasiISPU();
  alert('🚨 Peringatan ISPU: Terdeteksi lonjakan emisi PM2.5 di Stasiun Industri! Notifikasi otomatis terkirim ke DLH Kabupaten.');
}

function toggleLayer(layer) {
  alert(`Layer '${layer}' status diperbarui.`);
}
