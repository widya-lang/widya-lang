const fs = require('fs');
const html = fs.readFileSync('docs/BUKU_PANDUAN_PEMROGRAMAN_WIDYA.html', 'utf8');
const titles = html.match(/<h1 class="chapter-title">[\s\S]*?<\/h1>/g);
console.log(`Ditemukan ${titles.length} judul bab & lampiran:`);
titles.forEach((t, i) => {
  const clean = t.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim();
  console.log(`${i + 1}. ${clean}`);
});
