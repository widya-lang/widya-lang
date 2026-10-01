// ==============================================================================
// app.js - Widya Workspace CRDT Collaborative Document & KaTeX Engine
// ==============================================================================

document.addEventListener('DOMContentLoaded', () => {
  renderPreview();
  startCollabSimulation();
});

// PARSER MARKDOWN & FORMULA KATEX SEDERHANA
function renderPreview() {
  const raw = document.getElementById('markdown-editor').value;
  const paper = document.getElementById('paper-output');

  let html = '';
  const lines = raw.split('\n');
  let inList = false;
  let inMath = false;
  let mathBuffer = '';

  for (let line of lines) {
    line = line.trim();

    // Math block $$
    if (line.startsWith('$$') && line.endsWith('$$') && line.length > 2) {
      const formula = line.replace(/\$\$/g, '');
      html += `<div class="math-display">∑ Formula: ${formatMath(formula)}</div>`;
      continue;
    } else if (line === '$$') {
      if (!inMath) {
        inMath = true;
        mathBuffer = '';
      } else {
        inMath = false;
        html += `<div class="math-display">∑ Formula: ${formatMath(mathBuffer)}</div>`;
      }
      continue;
    }

    if (inMath) {
      mathBuffer += ' ' + line;
      continue;
    }

    // Headers
    if (line.startsWith('# ')) {
      if (inList) { html += '</ul>'; inList = false; }
      html += `<h1>${line.slice(2)}</h1>`;
    } else if (line.startsWith('## ')) {
      if (inList) { html += '</ul>'; inList = false; }
      html += `<h2>${line.slice(3)}</h2>`;
    } else if (line.startsWith('### ')) {
      if (inList) { html += '</ul>'; inList = false; }
      html += `<h3>${line.slice(4)}</h3>`;
    } else if (line.startsWith('- ')) {
      if (!inList) { html += '<ul>'; inList = true; }
      html += `<li>${parseInline(line.slice(2))}</li>`;
    } else if (line === '') {
      if (inList) { html += '</ul>'; inList = false; }
    } else {
      if (inList) { html += '</ul>'; inList = false; }
      html += `<p>${parseInline(line)}</p>`;
    }
  }

  if (inList) html += '</ul>';
  paper.innerHTML = html;
}

function parseInline(text) {
  // Bold **text**
  text = text.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
  // Inline Code `text`
  text = text.replace(/`(.*?)`/g, '<code>$1</code>');
  return text;
}

function formatMath(latex) {
  // Clean up LaTeX formatting for clean display
  return latex
    .replace(/\\text\{(.*?)\}/g, '$1')
    .replace(/\\sum_\{(.*?)\}\^\{(.*?)\}/g, '∑ ($1 to $2)')
    .replace(/\\frac\{(.*?)\}\{(.*?)\}/g, '($1 / $2)')
    .replace(/\\times/g, '×');
}

// SIMULASI KOLABORASI MULTI-USER CRDT
function startCollabSimulation() {
  const users = ['Lead Coder', 'Sistem Arsitek', 'Notulis AI'];
  setInterval(() => {
    const u = users[Math.floor(Math.random() * users.length)];
    // Periodic status updates
  }, 4000);
}

function eksporPDF() {
  window.print();
}
