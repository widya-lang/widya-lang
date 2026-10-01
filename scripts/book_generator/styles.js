module.exports = {
  css: `
  @page {
    size: A4 portrait;
    margin: 18mm 16mm 20mm 16mm;
    @bottom-right {
      content: "Halaman " counter(page);
      font-size: 8.5pt;
      color: #64748b;
      font-family: 'Segoe UI', system-ui, sans-serif;
    }
    @bottom-left {
      content: "Buku Panduan Resmi Pemrograman Widya-Lang (Edisi Komprehensif Jilid 2)";
      font-size: 8.5pt;
      color: #94a3b8;
      font-family: 'Segoe UI', system-ui, sans-serif;
    }
  }

  @page :first {
    margin: 0;
    @bottom-right { content: ""; }
    @bottom-left { content: ""; }
  }

  * {
    box-sizing: border-box;
    -webkit-print-color-adjust: exact !important;
    print-color-adjust: exact !important;
  }

  body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Roboto, Helvetica, Arial, sans-serif;
    font-size: 9.1pt;
    line-height: 1.55;
    color: #1e293b;
    margin: 0;
    padding: 0;
    background-color: #ffffff;
  }

  /* Cover Page */
  .cover {
    width: 100vw;
    height: 100vh;
    min-height: 297mm;
    background: linear-gradient(135deg, #06152b 0%, #0f2c59 35%, #1e40af 70%, #0284c7 100%);
    color: #ffffff;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 38mm 24mm 26mm 24mm;
    page-break-after: always;
    break-after: page;
  }

  .cover-badge {
    display: inline-block;
    background: rgba(255, 255, 255, 0.16);
    border: 1px solid rgba(255, 255, 255, 0.35);
    color: #38bdf8;
    padding: 6px 18px;
    border-radius: 20px;
    font-size: 9.8pt;
    font-weight: 700;
    letter-spacing: 1.5px;
    margin-bottom: 22px;
    text-transform: uppercase;
  }

  .cover-title {
    font-size: 33pt;
    font-weight: 800;
    line-height: 1.15;
    margin: 0 0 14px 0;
    color: #ffffff;
    letter-spacing: -0.5px;
  }

  .cover-title span {
    color: #38bdf8;
    background: linear-gradient(to right, #38bdf8, #818cf8);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .cover-subtitle {
    font-size: 12.2pt;
    font-weight: 400;
    line-height: 1.5;
    color: #cbd5e1;
    margin: 0 0 24px 0;
    max-width: 660px;
  }

  .cover-features {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 13px;
    margin-top: 15px;
    max-width: 650px;
  }

  .cover-feature-item {
    background: rgba(15, 23, 42, 0.68);
    border: 1px solid rgba(255, 255, 255, 0.16);
    padding: 12px 15px;
    border-radius: 8px;
    font-size: 8.8pt;
    color: #e2e8f0;
  }

  .cover-feature-item strong {
    color: #38bdf8;
    display: block;
    margin-bottom: 3px;
    font-size: 9.7pt;
  }

  .cover-footer {
    border-top: 1px solid rgba(255, 255, 255, 0.22);
    padding-top: 16px;
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
  }

  .cover-author {
    font-size: 10.2pt;
    color: #e2e8f0;
  }

  .cover-author strong {
    color: #ffffff;
    font-size: 11.2pt;
    display: block;
  }

  .cover-edition {
    text-align: right;
    font-size: 8.8pt;
    color: #94a3b8;
  }

  /* Structural Elements */
  .page-break {
    page-break-before: always;
    break-before: page;
  }

  .no-break {
    page-break-inside: avoid;
    break-inside: avoid;
  }

  h1.chapter-title {
    font-size: 16pt;
    font-weight: 800;
    color: #0f172a;
    border-bottom: 2.5px solid #2563eb;
    padding-bottom: 5px;
    margin-top: 12px;
    margin-bottom: 12px;
    display: flex;
    align-items: center;
    gap: 10px;
    page-break-after: avoid;
    break-after: avoid;
  }

  .chapter-number {
    background: #2563eb;
    color: #ffffff;
    font-size: 10pt;
    padding: 3px 9px;
    border-radius: 6px;
    font-weight: 700;
  }

  h2 {
    font-size: 11.4pt;
    font-weight: 700;
    color: #1e293b;
    margin-top: 12px;
    margin-bottom: 5px;
    border-left: 3.5px solid #0284c7;
    padding-left: 8px;
    page-break-after: avoid;
    break-after: avoid;
  }

  h3 {
    font-size: 9.8pt;
    font-weight: 600;
    color: #334155;
    margin-top: 9px;
    margin-bottom: 3px;
    page-break-after: avoid;
    break-after: avoid;
  }

  p {
    margin: 0 0 7px 0;
    text-align: justify;
  }

  ul, ol {
    margin: 0 0 8px 0;
    padding-left: 18px;
  }

  li {
    margin-bottom: 3px;
  }

  /* Callout Boxes */
  .callout {
    padding: 8px 12px;
    border-radius: 6px;
    margin: 9px 0;
    border-left: 3.5px solid;
    page-break-inside: avoid;
    break-inside: avoid;
  }

  .callout-title {
    font-weight: 700;
    font-size: 8.9pt;
    margin-bottom: 2px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .callout-info {
    background-color: #f0f9ff;
    border-color: #0284c7;
    color: #0c4a6e;
  }
  .callout-info .callout-title { color: #0284c7; }

  .callout-tip {
    background-color: #f0fdf4;
    border-color: #16a34a;
    color: #14532d;
  }
  .callout-tip .callout-title { color: #16a34a; }

  .callout-warn {
    background-color: #fffbeb;
    border-color: #f59e0b;
    color: #78350f;
  }
  .callout-warn .callout-title { color: #d97706; }

  .callout-danger {
    background-color: #fef2f2;
    border-color: #ef4444;
    color: #7f1d1d;
  }
  .callout-danger .callout-title { color: #dc2626; }

  /* Code Blocks */
  pre {
    background: #0f172a;
    color: #f8fafc;
    padding: 9px 12px;
    border-radius: 6px;
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 8.0pt;
    line-height: 1.42;
    overflow-x: auto;
    margin: 6px 0;
    border: 1px solid #334155;
    page-break-inside: avoid;
    break-inside: avoid;
  }

  code {
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 8.3pt;
    background: #f1f5f9;
    color: #0f172a;
    padding: 1.5px 4px;
    border-radius: 4px;
    border: 1px solid #e2e8f0;
  }

  pre code {
    background: transparent;
    color: inherit;
    padding: 0;
    border: none;
    font-size: inherit;
  }

  /* Code Syntax Colors */
  .kw { color: #f43f5e; font-weight: bold; } /* Keywords */
  .str { color: #34d399; } /* Strings */
  .num { color: #fbbf24; } /* Numbers */
  .com { color: #64748b; font-style: italic; } /* Comments */
  .fn { color: #38bdf8; } /* Functions */
  .typ { color: #c084fc; font-weight: 600; } /* Types / Structs */

  /* Tables */
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 8px 0;
    font-size: 8.3pt;
    page-break-inside: avoid;
    break-inside: avoid;
  }

  th {
    background-color: #1e293b;
    color: #ffffff;
    text-align: left;
    padding: 5px 8px;
    font-weight: 600;
    border: 1px solid #334155;
  }

  td {
    padding: 4.5px 8px;
    border: 1px solid #e2e8f0;
    vertical-align: top;
  }

  tr:nth-child(even) {
    background-color: #f8fafc;
  }

  /* Table of Contents */
  .toc {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 14px 18px;
    margin: 12px 0 16px 0;
  }

  .toc-title {
    font-size: 13pt;
    font-weight: 700;
    color: #0f172a;
    margin-bottom: 10px;
    border-bottom: 2px solid #e2e8f0;
    padding-bottom: 5px;
  }

  .toc-list {
    list-style-type: none;
    padding-left: 0;
    margin: 0;
  }

  .toc-item {
    margin-bottom: 4px;
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    font-size: 8.6pt;
  }

  .toc-item a {
    text-decoration: none;
    color: #1e293b;
    font-weight: 600;
  }

  .toc-dots {
    flex-grow: 1;
    border-bottom: 1px dotted #cbd5e1;
    margin: 0 6px;
  }

  .toc-page {
    color: #64748b;
    font-weight: 600;
  }

  .diagram-box {
    background: #f1f5f9;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    padding: 8px 11px;
    font-family: 'Consolas', monospace;
    font-size: 7.6pt;
    line-height: 1.34;
    margin: 7px 0;
    white-space: pre;
    color: #0f172a;
    page-break-inside: avoid;
    break-inside: avoid;
  }
`
};
