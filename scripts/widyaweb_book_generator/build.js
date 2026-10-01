const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('=== Membangun Panduan Resmi Pembuatan Proyek WidyaWeb Framework (PDF) ===');

const cssContent = fs.readFileSync(path.join(__dirname, 'styles.css'), 'utf8');
const p1 = fs.readFileSync(path.join(__dirname, 'part1_intro.html'), 'utf8');
const p2 = fs.readFileSync(path.join(__dirname, 'part2_core.html'), 'utf8');
const p3 = fs.readFileSync(path.join(__dirname, 'part3_enterprise.html'), 'utf8');
const p4 = fs.readFileSync(path.join(__dirname, 'part4_ops.html'), 'utf8');

const fullHtml = `<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8">
<title>Panduan Resmi Pengembangan Proyek dengan WidyaWeb Framework</title>
<style>
${cssContent}
</style>
</head>
<body>
${p1}
${p2}
${p3}
${p4}
</body>
</html>
`;

const outputPathHtml = path.resolve(__dirname, '../../docs/PANDUAN_PROYEK_WIDYAWEB.html');
fs.writeFileSync(outputPathHtml, fullHtml, 'utf8');
console.log('✓ HTML berhasil ditulis ke:', outputPathHtml);
console.log('  Ukuran HTML:', (fs.statSync(outputPathHtml).size / 1024).toFixed(1), 'KB');

const outputPdf = path.resolve(__dirname, '../../Panduan_Pembuatan_Proyek_WidyaWeb.pdf');
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
