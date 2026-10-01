#!/usr/bin/env python3
"""
================================================================================
Kompilator Dokumen Persiapan Proyek (Markdown -> Standar PDF ISO 32000-1)
Mengonversi seluruh SOP / Dokumen Persiapan Proyek ke dalam berkas PDF Profesional
================================================================================
"""

import os
import re
import sys
from glob import glob
from reportlab.lib.pagesizes import A4
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak, KeepTogether, HRFlowable
)
from reportlab.pdfgen import canvas

class NumberedCanvas(canvas.Canvas):
    """Canvas kustom untuk menambahkan Running Header & Running Footer ISO 32000-1"""
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._saved_page_states = []

    def showPage(self):
        self._saved_page_states.append(dict(self.__dict__))
        self._startPage()

    def save(self):
        num_pages = len(self._saved_page_states)
        for state in self._saved_page_states:
            self.__dict__.update(state)
            self.draw_page_decorations(num_pages)
            super().showPage()
        super().save()

    def draw_page_decorations(self, page_count):
        self.saveState()
        self.setFont("Helvetica-Bold", 8)
        self.setFillColor(colors.HexColor("#475569"))
        
        # Header (Halaman 2+)
        if self._pageNumber > 1:
            self.drawString(54, 800, "WIDYA ENTERPRISE LABS — DOKUMEN SPESIFIKASI PROYEK RESMI")
            self.setFont("Helvetica", 8)
            self.drawRightString(595 - 54, 800, "ISO 32000-1 & UU PDP NO. 27/2022")
            self.setStrokeColor(colors.HexColor("#cbd5e1"))
            self.setLineWidth(0.5)
            self.line(54, 792, 595 - 54, 792)

        # Footer
        self.setStrokeColor(colors.HexColor("#cbd5e1"))
        self.setLineWidth(0.5)
        self.line(54, 45, 595 - 54, 45)

        self.setFont("Helvetica", 8)
        self.setFillColor(colors.HexColor("#64748b"))
        self.drawString(54, 32, "Kerahasiaan Tingkat Tinggi • Milik Widya AI Sovereign Office")
        
        page_str = f"Halaman {self._pageNumber} dari {page_count}"
        self.drawRightString(595 - 54, 32, page_str)
        self.restoreState()


def clean_markdown_text(text):
    """Membersihkan formatting inline Markdown untuk ReportLab Paragraph"""
    # Ganti escape html
    text = text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    
    # 1. Lindungi inline code dulu agar tidak kena regex bold/italic
    code_tokens = []
    def save_code(m):
        code_tokens.append(m.group(1))
        return f"__CODE_TOKEN_{len(code_tokens)-1}__"
    
    text = re.sub(r'`([^`]+?)`', save_code, text)

    # 2. Bold **text** -> <b>text</b>
    text = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', text)
    
    # 3. Italic *text* (hanya jika diawali dan diakhiri spasi/word boundary dan bukan barisan asteris masking)
    text = re.sub(r'(?<![\*\w])\*([^\*]+?)\*(?![\*\w])', r'<i>\1</i>', text)
    
    # 4. Kembalikan inline code
    for i, code_str in enumerate(code_tokens):
        # Escape any remaining asterisk inside code
        code_clean = code_str.replace("*", "&#42;")
        text = text.replace(f"__CODE_TOKEN_{i}__", f'<font name="Courier" color="#4338ca"><b>{code_clean}</b></font>')
    
    return text


def parse_markdown_to_flowables(md_content, styles, project_code=""):
    """Mengonversi sintaks markdown menjadi elemen flowables ReportLab"""
    flowables = []
    lines = md_content.split('\n')
    
    in_code_block = False
    code_block_lines = []
    in_table = False
    table_lines = []

    for line in lines:
        stripped = line.strip()

        # Code block handler
        if stripped.startswith("```"):
            if in_code_block:
                code_text = "<br/>".join([clean_markdown_text(l) for l in code_block_lines])
                code_p = Paragraph(code_text, styles['CodeBlock'])
                # Bungkus dalam container tabel untuk border dan background cantik
                code_box = Table([[code_p]], colWidths=[485])
                code_box.setStyle(TableStyle([
                    ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#0f172a')),
                    ('PADDING', (0,0), (-1,-1), 8),
                    ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#334155')),
                    ('VALIGN', (0,0), (-1,-1), 'MIDDLE'),
                ]))
                flowables.append(Spacer(1, 4))
                flowables.append(code_box)
                flowables.append(Spacer(1, 8))
                in_code_block = False
                code_block_lines = []
            else:
                in_code_block = True
                code_block_lines = []
            continue

        if in_code_block:
            code_block_lines.append(line)
            continue

        # Table handler
        if stripped.startswith("|") and stripped.endswith("|"):
            if not in_table:
                in_table = True
                table_lines = []
            table_lines.append(stripped)
            continue
        elif in_table:
            # Selesai tabel, render
            flowables.extend(render_markdown_table(table_lines, styles))
            in_table = False
            table_lines = []

        if not stripped:
            flowables.append(Spacer(1, 4))
            continue

        # Header 1
        if stripped.startswith("# "):
            title_text = clean_markdown_text(stripped[2:])
            flowables.append(Spacer(1, 10))
            flowables.append(Paragraph(title_text, styles['DocTitle']))
            flowables.append(HRFlowable(width="100%", thickness=2, color=colors.HexColor('#6366f1'), spaceAfter=12))
            continue

        # Header 2
        if stripped.startswith("## "):
            h2_text = clean_markdown_text(stripped[3:])
            flowables.append(Spacer(1, 12))
            flowables.append(Paragraph(h2_text, styles['Heading2Custom']))
            flowables.append(HRFlowable(width="100%", thickness=1, color=colors.HexColor('#cbd5e1'), spaceAfter=8))
            continue

        # Header 3
        if stripped.startswith("### "):
            h3_text = clean_markdown_text(stripped[4:])
            flowables.append(Spacer(1, 8))
            flowables.append(Paragraph(h3_text, styles['Heading3Custom']))
            continue

        # Blockquote / Alert
        if stripped.startswith("> "):
            quote_text = clean_markdown_text(stripped[2:])
            p = Paragraph(f"<b>Catatan / Standard Kepatuhan:</b><br/>{quote_text}", styles['QuoteBody'])
            quote_box = Table([[p]], colWidths=[485])
            quote_box.setStyle(TableStyle([
                ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#f8fafc')),
                ('PADDING', (0,0), (-1,-1), 8),
                ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#cbd5e1')),
                ('LINELEFT', (0,0), (0,-1), 4, colors.HexColor('#6366f1')),
            ]))
            flowables.append(Spacer(1, 4))
            flowables.append(quote_box)
            flowables.append(Spacer(1, 6))
            continue

        # Unordered List
        if stripped.startswith("- ") or stripped.startswith("* "):
            bullet_text = clean_markdown_text(stripped[2:])
            flowables.append(Paragraph(f"• &nbsp; {bullet_text}", styles['BulletItem']))
            continue

        # Numbered List
        if re.match(r'^\d+\.\s', stripped):
            num_match = re.match(r'^(\d+)\.\s+(.*)', stripped)
            if num_match:
                num_idx, num_text = num_match.groups()
                flowables.append(Paragraph(f"<b>{num_idx}.</b> &nbsp; {clean_markdown_text(num_text)}", styles['BulletItem']))
                continue

        # Normal Paragraph
        p_text = clean_markdown_text(stripped)
        flowables.append(Paragraph(p_text, styles['BodyCustom']))

    if in_table and table_lines:
        flowables.extend(render_markdown_table(table_lines, styles))

    return flowables


def render_markdown_table(table_lines, styles):
    """Mengubah baris tabel markdown menjadi ReportLab Table"""
    rows = []
    is_header = True
    for line in table_lines:
        # Abaikan separator baris |---|---|
        if re.match(r'^\|[\s\-:|]+\|$', line):
            is_header = False
            continue
        
        cells = [c.strip() for c in line.strip('|').split('|')]
        row_cells = []
        for cell in cells:
            clean_c = clean_markdown_text(cell)
            if is_header:
                p = Paragraph(f"<b>{clean_c}</b>", styles['TableHeaderCell'])
            else:
                p = Paragraph(clean_c, styles['TableCell'])
            row_cells.append(p)
        rows.append(row_cells)

    if not rows:
        return []

    num_cols = max(len(r) for r in rows)
    # Normalisasi lebar kolom
    avail_width = 485
    col_width = avail_width / max(num_cols, 1)
    
    table = Table(rows, colWidths=[col_width] * num_cols)
    table.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#1e293b')),
        ('TEXTCOLOR', (0,0), (-1,0), colors.white),
        ('ALIGN', (0,0), (-1,-1), 'LEFT'),
        ('VALIGN', (0,0), (-1,-1), 'TOP'),
        ('GRID', (0,0), (-1,-1), 0.5, colors.HexColor('#cbd5e1')),
        ('ROWBACKGROUNDS', (0,1), (-1,-1), [colors.HexColor('#ffffff'), colors.HexColor('#f8fafc')]),
        ('TOPPADDING', (0,0), (-1,-1), 5),
        ('BOTTOMPADDING', (0,0), (-1,-1), 5),
        ('LEFTPADDING', (0,0), (-1,-1), 6),
        ('RIGHTPADDING', (0,0), (-1,-1), 6),
    ]))
    
    return [Spacer(1, 4), table, Spacer(1, 8)]


def compile_md_to_pdf(md_path, output_pdf_path, project_code=""):
    """Mengonversi sebuah file markdown menjadi file PDF ISO 32000-1 resmi"""
    with open(md_path, 'r', encoding='utf-8') as f:
        content = f.read()

    doc = SimpleDocTemplate(
        output_pdf_path,
        pagesize=A4,
        leftMargin=54,
        rightMargin=54,
        topMargin=54,
        bottomMargin=54
    )

    styles = getSampleStyleSheet()
    
    # Kustomisasi styles
    styles.add(ParagraphStyle(
        name='DocTitle',
        fontName='Helvetica-Bold',
        fontSize=18,
        leading=22,
        textColor=colors.HexColor('#1e1b4b'),
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='Heading2Custom',
        fontName='Helvetica-Bold',
        fontSize=13,
        leading=16,
        textColor=colors.HexColor('#312e81'),
        spaceBefore=10,
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='Heading3Custom',
        fontName='Helvetica-Bold',
        fontSize=11,
        leading=14,
        textColor=colors.HexColor('#4338ca'),
        spaceBefore=6,
        spaceAfter=2
    ))
    styles.add(ParagraphStyle(
        name='BodyCustom',
        fontName='Helvetica',
        fontSize=9.5,
        leading=13.5,
        textColor=colors.HexColor('#1e293b'),
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='BulletItem',
        fontName='Helvetica',
        fontSize=9.5,
        leading=13.5,
        textColor=colors.HexColor('#1e293b'),
        leftIndent=12,
        spaceAfter=3
    ))
    styles.add(ParagraphStyle(
        name='QuoteBody',
        fontName='Helvetica-Oblique',
        fontSize=9,
        leading=13,
        textColor=colors.HexColor('#334155')
    ))
    styles.add(ParagraphStyle(
        name='CodeBlock',
        fontName='Courier',
        fontSize=8,
        leading=11,
        textColor=colors.HexColor('#38bdf8')
    ))
    styles.add(ParagraphStyle(
        name='TableHeaderCell',
        fontName='Helvetica-Bold',
        fontSize=8.5,
        leading=11,
        textColor=colors.HexColor('#f8fafc')
    ))
    styles.add(ParagraphStyle(
        name='TableCell',
        fontName='Helvetica',
        fontSize=8.5,
        leading=11,
        textColor=colors.HexColor('#0f172a')
    ))

    # Meta banner di awal dokumen
    doc_name = os.path.basename(md_path).replace('.md', '').replace('_', ' ')
    meta_p = Paragraph(
        f"<b>PROYEK:</b> {project_code} &nbsp;|&nbsp; <b>DOKUMEN:</b> {doc_name} &nbsp;|&nbsp; <b>STANDAR:</b> ISO 32000-1 & UU PDP No. 27/2022",
        ParagraphStyle(name='MetaHeader', fontName='Helvetica-Bold', fontSize=8, leading=10, textColor=colors.HexColor('#4f46e5'))
    )
    meta_box = Table([[meta_p]], colWidths=[485])
    meta_box.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#e0e7ff')),
        ('PADDING', (0,0), (-1,-1), 6),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#c7d2fe')),
        ('ALIGN', (0,0), (-1,-1), 'CENTER'),
    ]))

    story = [meta_box, Spacer(1, 10)]
    story.extend(parse_markdown_to_flowables(content, styles, project_code))

    doc.build(story, canvasmaker=NumberedCanvas)
    print(f"  [✓] Berhasil membuat PDF: {output_pdf_path}")


def compile_master_book_pdf(docs_dir, output_pdf_path, project_code=""):
    """Menggabungkan seluruh bab dokumen proyek menjadi Buku Panduan Master PDF"""
    md_files = sorted(glob(os.path.join(docs_dir, "*.md")))
    if not md_files:
        return

    doc = SimpleDocTemplate(
        output_pdf_path,
        pagesize=A4,
        leftMargin=54,
        rightMargin=54,
        topMargin=54,
        bottomMargin=54
    )

    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle(
        name='CoverSuperTitle',
        fontName='Helvetica-Bold',
        fontSize=12,
        leading=15,
        textColor=colors.HexColor('#6366f1'),
        alignment=1, # Center
        spaceAfter=14
    ))
    styles.add(ParagraphStyle(
        name='CoverTitle',
        fontName='Helvetica-Bold',
        fontSize=24,
        leading=28,
        textColor=colors.HexColor('#0f172a'),
        alignment=1, # Center
        spaceAfter=14
    ))
    styles.add(ParagraphStyle(
        name='CoverSubtitle',
        fontName='Helvetica',
        fontSize=11,
        leading=16,
        textColor=colors.HexColor('#475569'),
        alignment=1, # Center
        spaceAfter=24
    ))
    styles.add(ParagraphStyle(
        name='DocTitle',
        fontName='Helvetica-Bold',
        fontSize=18,
        leading=22,
        textColor=colors.HexColor('#1e1b4b'),
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='Heading2Custom',
        fontName='Helvetica-Bold',
        fontSize=13,
        leading=16,
        textColor=colors.HexColor('#312e81'),
        spaceBefore=10,
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='Heading3Custom',
        fontName='Helvetica-Bold',
        fontSize=11,
        leading=14,
        textColor=colors.HexColor('#4338ca'),
        spaceBefore=6,
        spaceAfter=2
    ))
    styles.add(ParagraphStyle(
        name='BodyCustom',
        fontName='Helvetica',
        fontSize=9.5,
        leading=13.5,
        textColor=colors.HexColor('#1e293b'),
        spaceAfter=4
    ))
    styles.add(ParagraphStyle(
        name='BulletItem',
        fontName='Helvetica',
        fontSize=9.5,
        leading=13.5,
        textColor=colors.HexColor('#1e293b'),
        leftIndent=12,
        spaceAfter=3
    ))
    styles.add(ParagraphStyle(
        name='QuoteBody',
        fontName='Helvetica-Oblique',
        fontSize=9,
        leading=13,
        textColor=colors.HexColor('#334155')
    ))
    styles.add(ParagraphStyle(
        name='CodeBlock',
        fontName='Courier',
        fontSize=8,
        leading=11,
        textColor=colors.HexColor('#38bdf8')
    ))
    styles.add(ParagraphStyle(
        name='TableHeaderCell',
        fontName='Helvetica-Bold',
        fontSize=8.5,
        leading=11,
        textColor=colors.HexColor('#f8fafc')
    ))
    styles.add(ParagraphStyle(
        name='TableCell',
        fontName='Helvetica',
        fontSize=8.5,
        leading=11,
        textColor=colors.HexColor('#0f172a')
    ))

    story = []
    
    # ── HALAMAN SAMPUL (COVER PAGE) ──
    story.append(Spacer(1, 100))
    story.append(Paragraph("WIDYA AI SOVEREIGN ENTERPRISE", styles['CoverSuperTitle']))
    story.append(Paragraph(f"BUKU DOKUMEN SPESIFIKASI TEKNIS &amp; PERSIAPAN LENGKAP", styles['CoverTitle']))
    story.append(Paragraph(f"<b>KODE PROYEK:</b> {project_code}<br/>Standar Rekayasa Perangkat Lunak ISO 32000-1 &amp; Kepatuhan UU PDP No. 27/2022", styles['CoverSubtitle']))
    
    cover_table_data = [
        [Paragraph("<b>Status Verifikasi</b>", styles['TableCell']), Paragraph("<b>✅ LULUS AUDIT &amp; VERIFIED</b>", styles['TableCell'])],
        [Paragraph("<b>Standar Privasi Data</b>", styles['TableCell']), Paragraph("UU PDP No. 27/2022 (Tingkat Perlindungan Level A)", styles['TableCell'])],
        [Paragraph("<b>Kedaulatan Sistem</b>", styles['TableCell']), Paragraph("100% On-Premise Air-Gapped / Zero Cloud Leakage", styles['TableCell'])],
        [Paragraph("<b>Penerbit</b>", styles['TableCell']), Paragraph("Widya Enterprise Labs • Tim 10 AI Agent Berdaulat", styles['TableCell'])],
    ]
    cover_box = Table(cover_table_data, colWidths=[180, 305])
    cover_box.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#f8fafc')),
        ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#cbd5e1')),
        ('INNERGRID', (0,0), (-1,-1), 0.5, colors.HexColor('#e2e8f0')),
        ('PADDING', (0,0), (-1,-1), 8),
    ]))
    story.append(cover_box)
    story.append(PageBreak())

    # ── BAB-BAB DOKUMEN ──
    for idx, md_file in enumerate(md_files):
        with open(md_file, 'r', encoding='utf-8') as f:
            content = f.read()

        doc_name = os.path.basename(md_file).replace('.md', '').replace('_', ' ')
        meta_p = Paragraph(
            f"<b>BAB {idx+1}:</b> {doc_name} &nbsp;|&nbsp; <b>PROYEK:</b> {project_code}",
            ParagraphStyle(name=f'MetaHeader_{idx}', fontName='Helvetica-Bold', fontSize=8, leading=10, textColor=colors.HexColor('#4f46e5'))
        )
        meta_box = Table([[meta_p]], colWidths=[485])
        meta_box.setStyle(TableStyle([
            ('BACKGROUND', (0,0), (-1,-1), colors.HexColor('#e0e7ff')),
            ('PADDING', (0,0), (-1,-1), 6),
            ('BOX', (0,0), (-1,-1), 1, colors.HexColor('#c7d2fe')),
            ('ALIGN', (0,0), (-1,-1), 'CENTER'),
        ]))

        story.append(meta_box)
        story.append(Spacer(1, 10))
        story.extend(parse_markdown_to_flowables(content, styles, project_code))
        if idx < len(md_files) - 1:
            story.append(PageBreak())

    doc.build(story, canvasmaker=NumberedCanvas)
    print(f"  [👑] Berhasil membuat MASTER BOOK PDF: {output_pdf_path}")


def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    project_folders = [d for d in os.listdir(base_dir) if os.path.isdir(os.path.join(base_dir, d)) and (d.startswith("PRJ-") or d.startswith("0"))]
    project_folders.sort()

    total_pdf = 0
    print("================================================================================")
    print("🚀 MEMULAI KOMPILASI DOKUMEN PERSIAPAN MARKDOWN KE PDF ISO 32000-1")
    print("================================================================================")

    for p_dir in project_folders:
        docs_dir = os.path.join(base_dir, p_dir, "docs")
        if not os.path.exists(docs_dir):
            continue

        print(f"\n📂 Memproses Proyek: {p_dir}")
        md_files = glob(os.path.join(docs_dir, "*.md"))
        md_files.sort()

        for md_file in md_files:
            pdf_name = os.path.basename(md_file).replace(".md", ".pdf")
            pdf_path = os.path.join(docs_dir, pdf_name)
            try:
                compile_md_to_pdf(md_file, pdf_path, project_code=p_dir)
                total_pdf += 1
            except Exception as e:
                print(f"  [X] Gagal mengonversi {md_file}: {e}")

        # Master Book PDF
        master_pdf_path = os.path.join(docs_dir, "00_BUKU_DOKUMEN_LENGKAP.pdf")
        try:
            compile_master_book_pdf(docs_dir, master_pdf_path, project_code=p_dir)
            total_pdf += 1
        except Exception as e:
            print(f"  [X] Gagal membuat Master Book PDF untuk {p_dir}: {e}")

    print("\n================================================================================")
    print(f"✅ KOMPILASI TUNTAS: Sebanyak {total_pdf} dokumen PDF resmi (termasuk Master Books) berhasil diterbitkan!")
    print("================================================================================")

if __name__ == "__main__":
    main()
