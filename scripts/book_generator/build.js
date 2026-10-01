const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('=== Membangun Buku Panduan Resmi Pemrograman Widya-Lang (Jilid 2) ===');

const cssContent = fs.readFileSync(path.join(__dirname, 'styles.css'), 'utf8');
const p1 = fs.readFileSync(path.join(__dirname, 'part1_foundations.html'), 'utf8');
const p2 = fs.readFileSync(path.join(__dirname, 'part2_structures.html'), 'utf8');
const p3 = fs.readFileSync(path.join(__dirname, 'part3_applications.html'), 'utf8');
const p4 = fs.readFileSync(path.join(__dirname, 'part4_advanced.html'), 'utf8');
const p5 = fs.readFileSync(path.join(__dirname, 'part5_production.html'), 'utf8');

const fullHtml = `<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8">
<title>Buku Panduan Resmi Pemrograman Widya-Lang - Edisi Komprehensif Jilid 2</title>
<style>
${cssContent}
</style>
</head>
<body>
${p1}
${p2}
${p3}
${p4}
${p5}
`;

const outputPathHtml = path.resolve(__dirname, '../../docs/BUKU_PANDUAN_PEMROGRAMAN_WIDYA.html');
fs.writeFileSync(outputPathHtml, fullHtml, 'utf8');
console.log('✓ HTML berhasil ditulis ke:', outputPathHtml);
console.log('  Ukuran HTML:', (fs.statSync(outputPathHtml).size / 1024).toFixed(1), 'KB');

const outputPdf = path.resolve(__dirname, '../../Buku_Panduan_Pemrograman_Widya_Lang.pdf');
const edgePath = 'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe';

console.log('Mencetak PDF via Microsoft Edge Headless...');
const htmlUrl = 'file:///' + outputPathHtml.replace(/\\/g, '/');
const cmd = `"${edgePath}" --headless --disable-gpu --no-pdf-header-footer --print-to-pdf="${outputPdf}" "${htmlUrl}"`;

try {
  execSync(cmd, { stdio: 'inherit' });
  console.log('✓ PDF berhasil dicetak ke:', outputPdf);

  const pdfBuf = fs.readFileSync(outputPdf);
  const pdfStr = pdfBuf.toString('latin1');
  const matches = pdfStr.match(/\/Count\s+(\d+)/g);
  console.log('  Informasi Halaman PDF:', matches);
  if (matches && matches.length > 0) {
    const lastCount = matches[matches.length - 1];
    console.log('  Total Halaman Buku PDF:', lastCount);
  }
} catch (err) {
  console.error('Galat saat mencetak PDF:', err);
  process.exit(1);
}
