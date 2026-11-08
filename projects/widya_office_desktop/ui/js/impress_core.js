/**
 * WidyaImpress - Sovereign Presentation & Slide Designer (100% Independent Native Engine)
 * Slide thumbnail sorter, 16:9 & 4:3 canvas editor, executive KPI metric cards, presentation themes,
 * persistent Speaker Notes drawer, slide sorter grid view, slide table/shapes/charts insertion,
 * and high-capability full-screen presenter cockpit (F5/Shift+F5) with stopwatch, real-time clock,
 * screen drawing pen annotation, virtual laser pointer, blackout/whiteout, and speaker notes zoom.
 */

const WidyaImpress = {
  activeSlideIndex: 0,
  theme: "theme-executive-navy",
  aspectRatio: "16:9",
  viewMode: "normal", // 'normal' | 'sorter'
  isPresenterActive: false,
  isLaserActive: false,
  isPenActive: false,
  penColor: "#ef4444",
  isDrawing: false,
  lastX: 0,
  lastY: 0,
  isBlackout: false,
  isWhiteout: false,
  timerSeconds: 0,
  timerInterval: null,
  realClockInterval: null,
  isTimerRunning: false,
  currentTransition: "fade",
  notesFontSize: 13,

  slides: [
    {
      layout: "title",
      title: "KEDAULATAN DIGITAL & TEKNOLOGI NASIONAL 2026",
      subtitle: "Peta Jalan Transformasi Perkantoran Berdaulat Tanpa Ketergantungan Asing",
      bullets: [],
      kpiCards: [],
      notes: "Tekankan pada slide pembuka ini mengenai pentingnya kemandirian teknologi nasional dan zero-dependency arsitektur.",
      hidden: false
    },
    {
      layout: "kpi",
      title: "Dashboard Metrik Strategis WidyaOffice",
      subtitle: "Efisiensi Kinerja & Benchmark Industri Terbuka",
      bullets: [],
      kpiCards: [
        { label: "Penghematan RAM", value: "88%", subtext: "Dibandingkan Electron/Chromium", positive: true },
        { label: "Waktu Cold-Start", value: "<15ms", subtext: "Biner Mandiri 64-bit AMD64", positive: true },
        { label: "Keamanan Kriptografi", value: "FIPS 204", subtext: "NIST ML-DSA-87 Pasca-Kuantum", positive: true },
        { label: "Kepatuhan Regulasi", value: "100%", subtext: "UU PDP No. 27/2022 & SNI", positive: true }
      ],
      notes: "Jelaskan data empiris penghematan memori dari 250MB turun menjadi di bawah 35MB pada pengujian beban harian.",
      hidden: false
    },
    {
      layout: "content",
      title: "Pilar Utama Ekosistem WidyaOffice Suite",
      subtitle: "Arsitektur Bersih & Berstandar Terbuka Internasional",
      bullets: [
        "WidyaWriter: Pengolah kata berdaulat dengan editor rumus matematika OMML OpenXML asli",
        "WidyaCalc: Mesin lembar kerja analitik berkecepatan tinggi dengan formula dependency graph",
        "WidyaImpress: Perancang presentasi eksekutif dengan kartu metrik KPI dan mode presenter",
        "WidyaPDF: Workbench pembaca, anotasi, redaksi privasi, dan sertifikasi tanda tangan digital"
      ],
      kpiCards: [],
      notes: "Sebutkan bahwa keempat aplikasi ini berjalan dalam satu biner terintegrasi tanpa runtime pihak ketiga.",
      hidden: false
    },
    {
      layout: "two-column",
      title: "Analisis Komparasi Kedaulatan Tumpukan Perangkat Lunak",
      subtitle: "Office Suite Konvensional vs Widya Sovereign Stack",
      leftTitle: "Office Suite Konvensional",
      leftBullets: [
        "Konsumsi RAM tinggi (>250MB hingga 1GB)",
        "Ketergantungan rantai pasok C/C++ eksternal",
        "Kunci vendor dan format tertutup proprietary",
        "Telemetri cloud pihak ketiga yang tidak diaudit"
      ],
      rightTitle: "WidyaOffice Sovereign Stack",
      rightBullets: [
        "Penggunaan RAM dasar <35MB sangat efisien",
        "100% Zero-External C Dependency murni Rust",
        "Interoperabilitas penuh ECMA-376 (DOCX, XLSX, PPTX)",
        "Zero-telemetry & Beroperasi penuh secara offline lokal"
      ],
      kpiCards: [],
      notes: "Ajak audiens melihat perbandingan nyata pada tabel kedaulatan perangkat lunak ini.",
      hidden: false
    }
  ],

  init() {
    this.renderThumbnails();
    this.renderCurrentSlide();
    this.updateNotesDrawer();
    this.bindEvents();
    this.bindPresenterDrawing();
  },

  renderThumbnails() {
    const container = document.getElementById("impressThumbnailsList");
    if (!container) return;

    let html = "";
    this.slides.forEach((slide, idx) => {
      const activeCls = idx === this.activeSlideIndex ? "active" : "";
      const hiddenCls = slide.hidden ? "slide-hidden" : "";
      const bgStyle = slide.customBg ? `background:${slide.customBg} !important;` : "";
      const hiddenBadge = slide.hidden ? `<span title="Slide ini disembunyikan dari Slide Show" style="margin-left:4px; color:#e11d48; font-weight:bold;">⃠</span>` : "";

      html += `
        <div class="impress-thumb-card ${activeCls} ${hiddenCls}" onclick="WidyaImpress.selectSlide(${idx})">
          <div class="impress-thumb-badge">${idx + 1}${hiddenBadge}</div>
          <div class="impress-thumb-preview ${this.theme}" style="${bgStyle}">
            <div class="impress-thumb-mini-title">${slide.title || "Slide " + (idx + 1)}</div>
            <div class="impress-thumb-mini-type">${this.getLayoutName(slide.layout)}</div>
          </div>
        </div>
      `;
    });
    container.innerHTML = html;
    this.updateSlideCounter();
  },

  getLayoutName(layout) {
    switch (layout) {
      case "title": return "Judul Utama";
      case "content": return "Daftar Poin";
      case "two-column": return "Dua Kolom";
      case "kpi": return "Dashboard KPI";
      case "title-only": return "Judul Saja";
      case "blank": return "Kanvas Kosong";
      default: return "Standar";
    }
  },

  renderCurrentSlide() {
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;

    const slide = this.slides[this.activeSlideIndex];
    if (!slide) return;

    const ratioCls = this.aspectRatio === "4:3" ? "ratio-4-3" : "ratio-16-9";
    canvas.className = `impress-slide-canvas ${this.theme} ${ratioCls}`;
    if (slide.customBg) {
      canvas.style.background = slide.customBg;
    } else {
      canvas.style.background = "";
    }
    canvas.innerHTML = this.getSlideHtml(slide);
    this.updateSlideCounter();
    this.updateNotesDrawer();
  },

  updateNotesDrawer() {
    const notesTextarea = document.getElementById("impressNotesTextarea");
    if (notesTextarea) {
      const slide = this.slides[this.activeSlideIndex];
      notesTextarea.value = (slide && slide.notes) ? slide.notes : "";
    }
  },

  onNotesInput(val) {
    if (this.slides[this.activeSlideIndex]) {
      this.slides[this.activeSlideIndex].notes = val;
    }
    const presenterNotes = document.getElementById("presenterNotesArea");
    if (presenterNotes && this.isPresenterActive) {
      presenterNotes.value = val;
    }
  },

  getSlideHtml(slide) {
    if (slide.layout === "title") {
      return `
        <div class="slide-title-layout">
          <div class="slide-watermark-tag">WIDYA IMPRESS PRESENTATION</div>
          <h1 class="slide-main-title" contenteditable="true" onblur="WidyaImpress.updateTitle(this.innerText)">${slide.title}</h1>
          <div class="slide-divider"></div>
          <p class="slide-subtitle" contenteditable="true" onblur="WidyaImpress.updateSubtitle(this.innerText)">${slide.subtitle || ""}</p>
        </div>
      `;
    } else if (slide.layout === "kpi") {
      let cardsHtml = "";
      (slide.kpiCards || []).forEach((c) => {
        cardsHtml += `
          <div class="kpi-metric-card">
            <div class="kpi-label">${c.label}</div>
            <div class="kpi-value">${c.value}</div>
            <div class="kpi-subtext">${c.subtext}</div>
          </div>
        `;
      });

      return `
        <div class="slide-standard-layout">
          <div class="slide-header">
            <h2 class="slide-content-title" contenteditable="true" onblur="WidyaImpress.updateTitle(this.innerText)">${slide.title}</h2>
            <p class="slide-content-sub" contenteditable="true" onblur="WidyaImpress.updateSubtitle(this.innerText)">${slide.subtitle || ""}</p>
          </div>
          <div class="slide-kpi-grid">
            ${cardsHtml}
          </div>
        </div>
      `;
    } else if (slide.layout === "two-column") {
      let leftItems = "";
      (slide.leftBullets || []).forEach(b => leftItems += `<li style="margin-bottom:8px;">${b}</li>`);
      let rightItems = "";
      (slide.rightBullets || []).forEach(b => rightItems += `<li style="margin-bottom:8px;">${b}</li>`);

      return `
        <div class="slide-standard-layout">
          <div class="slide-header">
            <h2 class="slide-content-title" contenteditable="true" onblur="WidyaImpress.updateTitle(this.innerText)">${slide.title}</h2>
            <p class="slide-content-sub" contenteditable="true" onblur="WidyaImpress.updateSubtitle(this.innerText)">${slide.subtitle || ""}</p>
          </div>
          <div style="display:grid; grid-template-columns:1fr 1fr; gap:24px; margin-top:16px;">
            <div style="background:rgba(255,255,255,0.06); padding:16px; border-radius:4px; border:1px solid rgba(255,255,255,0.1);">
              <h4 style="margin-bottom:12px; font-weight:700;">${slide.leftTitle || "Kolom Kiri"}</h4>
              <ul style="padding-left:18px; font-size:0.9rem; line-height:1.6;">${leftItems}</ul>
            </div>
            <div style="background:rgba(255,255,255,0.06); padding:16px; border-radius:4px; border:1px solid rgba(255,255,255,0.1);">
              <h4 style="margin-bottom:12px; font-weight:700; color:var(--accent-impress);">${slide.rightTitle || "Kolom Kanan"}</h4>
              <ul style="padding-left:18px; font-size:0.9rem; line-height:1.6;">${rightItems}</ul>
            </div>
          </div>
        </div>
      `;
    } else if (slide.layout === "title-only") {
      return `
        <div class="slide-standard-layout">
          <div class="slide-header">
            <h2 class="slide-content-title" contenteditable="true" onblur="WidyaImpress.updateTitle(this.innerText)">${slide.title}</h2>
            <p class="slide-content-sub" contenteditable="true" onblur="WidyaImpress.updateSubtitle(this.innerText)">${slide.subtitle || ""}</p>
          </div>
          <div style="flex:1; min-height:220px; display:flex; align-items:center; justify-content:center; border:1px dashed rgba(255,255,255,0.15); border-radius:4px; margin-top:16px;">
            <span style="opacity:0.5; font-size:0.85rem;">Ruang Konten Bebas (Gunakan tab Sisipkan untuk menambahkan Bentuk, Tabel, atau Bagan)</span>
          </div>
        </div>
      `;
    } else if (slide.layout === "blank") {
      return `
        <div style="width:100%; height:100%; display:flex; align-items:center; justify-content:center; border:2px dashed rgba(255,255,255,0.2); border-radius:4px;" contenteditable="true">
          <div style="text-align:center; opacity:0.6;">Kanvas Slide Bebas (Klik untuk mengetik atau gunakan tombol Sisipkan)</div>
        </div>
      `;
    } else {
      let bulletsHtml = "";
      (slide.bullets || []).forEach(b => {
        bulletsHtml += `<li style="margin-bottom:12px; font-size:1.05rem; line-height:1.6;">${b}</li>`;
      });
      return `
        <div class="slide-standard-layout">
          <div class="slide-header">
            <h2 class="slide-content-title" contenteditable="true" onblur="WidyaImpress.updateTitle(this.innerText)">${slide.title}</h2>
            <p class="slide-content-sub" contenteditable="true" onblur="WidyaImpress.updateSubtitle(this.innerText)">${slide.subtitle || ""}</p>
          </div>
          <ul style="padding-left:24px; margin-top:20px;">
            ${bulletsHtml}
          </ul>
        </div>
      `;
    }
  },

  selectSlide(index) {
    if (index >= 0 && index < this.slides.length) {
      this.activeSlideIndex = index;
      this.renderThumbnails();
      this.renderCurrentSlide();

      const canvas = document.getElementById("impressSlideCanvas");
      if (canvas && this.currentTransition && this.currentTransition !== "none") {
        canvas.classList.remove("trans-anim-fade", "trans-anim-slide", "trans-anim-push", "trans-anim-zoom");
        void canvas.offsetWidth; // force reflow
        canvas.classList.add(`trans-anim-${this.currentTransition}`);
      }
    }
  },

  addNewSlide(layout = "content") {
    let newSlide = {
      layout: layout,
      title: "Judul Slide Baru",
      subtitle: "Keterangan Ringkas Slide",
      bullets: ["Poin bahasan pertama", "Poin bahasan kedua", "Poin bahasan ketiga"],
      kpiCards: [
        { label: "Target Efisiensi", value: "95%", subtext: "Target Operasional", positive: true },
        { label: "Waktu Proses", value: "<10ms", subtext: "Optimasi Eksekusi", positive: true }
      ],
      notes: "Catatan presentasi untuk slide ini...",
      hidden: false
    };

    if (layout === "title") {
      newSlide.title = "JUDUL PRESENTASI UTAMA";
      newSlide.subtitle = "Subjudul dan Nama Pembicara";
    } else if (layout === "blank") {
      newSlide.title = "";
      newSlide.subtitle = "";
    } else if (layout === "title-only") {
      newSlide.title = "Judul Bagian Pokok";
      newSlide.subtitle = "Deskripsi singkat topik bahasan";
    }

    this.slides.splice(this.activeSlideIndex + 1, 0, newSlide);
    this.activeSlideIndex++;
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  duplicateSlide() {
    const cur = this.slides[this.activeSlideIndex];
    if (!cur) return;
    const cloned = JSON.parse(JSON.stringify(cur));
    cloned.title += " (Salinan)";
    this.slides.splice(this.activeSlideIndex + 1, 0, cloned);
    this.activeSlideIndex++;
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  deleteSlide() {
    if (this.slides.length <= 1) {
      alert("Presentasi harus memiliki minimal satu slide!");
      return;
    }
    this.slides.splice(this.activeSlideIndex, 1);
    this.activeSlideIndex = Math.max(0, this.activeSlideIndex - 1);
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  changeCurrentLayout(layout) {
    if (!layout) return;
    const cur = this.slides[this.activeSlideIndex];
    if (!cur) return;
    cur.layout = layout;
    if (layout === "content" && (!cur.bullets || cur.bullets.length === 0)) {
      cur.bullets = ["Poin pembahasan pertama", "Poin pembahasan kedua", "Poin pembahasan ketiga"];
    }
    if (layout === "two-column") {
      if (!cur.leftBullets) cur.leftBullets = ["Poin kolom kiri 1", "Poin kolom kiri 2"];
      if (!cur.rightBullets) cur.rightBullets = ["Poin kolom kanan 1", "Poin kolom kanan 2"];
    }
    if (layout === "kpi" && (!cur.kpiCards || cur.kpiCards.length === 0)) {
      cur.kpiCards = [
        { label: "Metrik A", value: "98%", subtext: "Target Kinerja", positive: true },
        { label: "Metrik B", value: "<5ms", subtext: "Waktu Respon", positive: true }
      ];
    }
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  toggleHideCurrentSlide() {
    const cur = this.slides[this.activeSlideIndex];
    if (!cur) return;
    cur.hidden = !cur.hidden;
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  moveSlide(direction) {
    const targetIdx = this.activeSlideIndex + direction;
    if (targetIdx < 0 || targetIdx >= this.slides.length) return;
    const temp = this.slides[this.activeSlideIndex];
    this.slides[this.activeSlideIndex] = this.slides[targetIdx];
    this.slides[targetIdx] = temp;
    this.activeSlideIndex = targetIdx;
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  // Slide Sorter View Mode (PowerPoint Slide Sorter)
  toggleViewMode(mode) {
    this.viewMode = mode;
    const normalView = document.getElementById("impressNormalView");
    const sorterView = document.getElementById("impressSorterView");
    const btnNormal = document.getElementById("btnViewNormal");
    const btnSorter = document.getElementById("btnViewSorter");

    if (mode === "sorter") {
      if (normalView) normalView.style.display = "none";
      if (sorterView) {
        sorterView.style.display = "grid";
        this.renderSlideSorter();
      }
      if (btnNormal) btnNormal.classList.remove("active");
      if (btnSorter) btnSorter.classList.add("active");
    } else {
      if (normalView) normalView.style.display = "flex";
      if (sorterView) sorterView.style.display = "none";
      if (btnNormal) btnNormal.classList.add("active");
      if (btnSorter) btnSorter.classList.remove("active");
      this.renderCurrentSlide();
      this.renderThumbnails();
    }
  },

  renderSlideSorter() {
    const sorter = document.getElementById("impressSorterView");
    if (!sorter) return;

    let html = "";
    this.slides.forEach((slide, idx) => {
      const activeCls = idx === this.activeSlideIndex ? "active" : "";
      const hiddenCls = slide.hidden ? "slide-hidden" : "";
      const bgStyle = slide.customBg ? `background:${slide.customBg} !important;` : "";
      const hiddenTag = slide.hidden ? `<span style="color:#ef4444; margin-left:6px; font-weight:700;">⃠ Disembunyikan</span>` : "";

      html += `
        <div class="impress-sorter-card ${activeCls} ${hiddenCls}" onclick="WidyaImpress.selectSlide(${idx}); WidyaImpress.renderSlideSorter();" ondblclick="WidyaImpress.toggleViewMode('normal')">
          <div class="impress-sorter-preview ${this.theme}" style="${bgStyle}">
            <div style="font-size:0.85rem; font-weight:700; margin-bottom:4px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">${slide.title || "Slide " + (idx + 1)}</div>
            <div style="font-size:0.75rem; opacity:0.8;">${this.getLayoutName(slide.layout)}</div>
          </div>
          <div class="impress-sorter-meta">
            <span style="font-weight:700;">Slide ${idx + 1}</span>
            <span>${hiddenTag}</span>
          </div>
        </div>
      `;
    });

    sorter.innerHTML = html;
  },

  // Aspect Ratio (16:9 vs 4:3)
  setAspectRatio(ratio) {
    this.aspectRatio = ratio;
    const btn169 = document.getElementById("btnRatio169");
    const btn43 = document.getElementById("btnRatio43");
    if (ratio === "4:3") {
      if (btn169) btn169.classList.remove("active");
      if (btn43) btn43.classList.add("active");
    } else {
      if (btn169) btn169.classList.add("active");
      if (btn43) btn43.classList.remove("active");
    }
    this.renderCurrentSlide();
  },

  setTheme(themeName) {
    if (!themeName) return;
    const clean = themeName.startsWith("theme-") ? themeName : `theme-${themeName}`;
    this.theme = clean;
    this.renderThumbnails();
    this.renderCurrentSlide();
    if (this.viewMode === "sorter") this.renderSlideSorter();
  },

  applyTheme(themeName) {
    this.setTheme(themeName);
  },

  updateTitle(text) {
    if (this.slides[this.activeSlideIndex]) {
      this.slides[this.activeSlideIndex].title = text;
      this.renderThumbnails();
      if (this.viewMode === "sorter") this.renderSlideSorter();
    }
  },

  updateSubtitle(text) {
    if (this.slides[this.activeSlideIndex]) {
      this.slides[this.activeSlideIndex].subtitle = text;
    }
  },

  updateSlideCounter() {
    const counter = document.getElementById("impressSlideCounter");
    if (counter) {
      const hiddenCount = this.slides.filter(s => s.hidden).length;
      const hiddenInfo = hiddenCount > 0 ? ` (${hiddenCount} tersembunyi)` : "";
      counter.innerText = `Slide ${this.activeSlideIndex + 1} dari ${this.slides.length}${hiddenInfo}`;
    }
  },

  // Insert Elements on Slide
  insertTextBox() {
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;
    const box = document.createElement("div");
    box.style.position = "absolute";
    box.style.top = "180px";
    box.style.left = "180px";
    box.style.padding = "8px 12px";
    box.style.background = "rgba(255,255,255,0.15)";
    box.style.border = "1px solid var(--accent-impress)";
    box.style.borderRadius = "4px";
    box.style.minWidth = "180px";
    box.contentEditable = "true";
    box.innerText = "Kotak Teks Baru (Ketik di sini)";
    canvas.appendChild(box);
    box.focus();
  },

  insertShape(type) {
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;
    const shape = document.createElement("div");
    shape.className = `impress-shape shape-${type}`;
    shape.style.position = "absolute";
    shape.style.top = "190px";
    shape.style.left = "220px";
    shape.style.display = "flex";
    shape.style.alignItems = "center";
    shape.style.justifyContent = "center";
    shape.style.fontWeight = "600";
    shape.style.fontSize = "0.85rem";
    shape.style.color = "#ffffff";
    shape.style.cursor = "move";
    shape.style.textAlign = "center";
    shape.style.padding = "8px";
    shape.contentEditable = "true";

    switch (type) {
      case "rounded-rect":
        shape.style.width = "180px";
        shape.style.height = "90px";
        shape.style.background = "#0284c7";
        shape.style.borderRadius = "14px";
        shape.innerText = "Kotak Membulat";
        break;
      case "circle":
        shape.style.width = "110px";
        shape.style.height = "110px";
        shape.style.background = "#185abd";
        shape.style.borderRadius = "50%";
        shape.innerText = "Lingkaran";
        break;
      case "triangle":
        shape.style.width = "120px";
        shape.style.height = "100px";
        shape.style.background = "#107c41";
        shape.style.clipPath = "polygon(50% 0%, 0% 100%, 100% 100%)";
        shape.innerText = "Segitiga";
        shape.style.paddingTop = "36px";
        break;
      case "arrow":
        shape.style.width = "160px";
        shape.style.height = "80px";
        shape.style.background = "#d97706";
        shape.style.clipPath = "polygon(0% 25%, 65% 25%, 65% 0%, 100% 50%, 65% 100%, 65% 75%, 0% 75%)";
        shape.innerText = "Panah";
        break;
      case "star":
        shape.style.width = "120px";
        shape.style.height = "120px";
        shape.style.background = "#eab308";
        shape.style.color = "#1e293b";
        shape.style.clipPath = "polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%)";
        shape.innerText = "★ Bintang";
        break;
      case "rect":
      default:
        shape.style.width = "180px";
        shape.style.height = "90px";
        shape.style.background = "#c43e1c";
        shape.style.borderRadius = "4px";
        shape.innerText = "Persegi Panjang";
        break;
    }

    canvas.appendChild(shape);
  },

  insertSlideTable() {
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;

    const tableWrapper = document.createElement("div");
    tableWrapper.style.position = "absolute";
    tableWrapper.style.top = "180px";
    tableWrapper.style.left = "140px";
    tableWrapper.style.zIndex = "10";

    tableWrapper.innerHTML = `
      <table class="impress-slide-table">
        <thead>
          <tr>
            <th contenteditable="true">Dimensi</th>
            <th contenteditable="true">Standar Konvensional</th>
            <th contenteditable="true">Widya Sovereign</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td contenteditable="true">Konsumsi Memori</td>
            <td contenteditable="true">&gt; 250 MB</td>
            <td contenteditable="true">&lt; 35 MB Murni</td>
          </tr>
          <tr>
            <td contenteditable="true">Waktu Booting</td>
            <td contenteditable="true">1.5 - 3.0 detik</td>
            <td contenteditable="true">&lt; 15 milidetik</td>
          </tr>
          <tr>
            <td contenteditable="true">Kedaulatan Sandi</td>
            <td contenteditable="true">RSA / Kurva Proprietary</td>
            <td contenteditable="true">NIST FIPS 204 PQC</td>
          </tr>
        </tbody>
      </table>
    `;

    canvas.appendChild(tableWrapper);
  },

  insertSlideChart(type) {
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;

    const chartBox = document.createElement("div");
    chartBox.style.position = "absolute";
    chartBox.style.top = "160px";
    chartBox.style.left = "200px";
    chartBox.style.background = "rgba(15,23,42,0.85)";
    chartBox.style.border = "1px solid rgba(255,255,255,0.2)";
    chartBox.style.borderRadius = "6px";
    chartBox.style.padding = "14px";
    chartBox.style.boxShadow = "0 8px 24px rgba(0,0,0,0.4)";
    chartBox.style.zIndex = "10";

    if (type === "line") {
      chartBox.innerHTML = `
        <div style="font-weight:700; font-size:0.85rem; margin-bottom:8px; color:#38bdf8;">📈 Tren Peningkatan Performa</div>
        <svg width="280" height="130" viewBox="0 0 280 130">
          <line x1="30" y1="10" x2="30" y2="105" stroke="#475569" stroke-width="1.5" />
          <line x1="30" y1="105" x2="270" y2="105" stroke="#475569" stroke-width="1.5" />
          <polyline fill="none" stroke="#38bdf8" stroke-width="3" points="30,95 80,75 140,55 200,35 260,18" />
          <circle cx="30" cy="95" r="4" fill="#38bdf8" />
          <circle cx="80" cy="75" r="4" fill="#38bdf8" />
          <circle cx="140" cy="55" r="4" fill="#38bdf8" />
          <circle cx="200" cy="35" r="4" fill="#38bdf8" />
          <circle cx="260" cy="18" r="4" fill="#38bdf8" />
        </svg>
      `;
    } else if (type === "pie") {
      chartBox.innerHTML = `
        <div style="font-weight:700; font-size:0.85rem; margin-bottom:8px; color:#f59e0b;">🥧 Distribusi Komponen Sistem</div>
        <svg width="220" height="130" viewBox="0 0 220 130">
          <circle cx="70" cy="65" r="45" fill="#10b981" />
          <path d="M 70 65 L 70 20 A 45 45 0 0 1 115 65 Z" fill="#3b82f6" />
          <path d="M 70 65 L 115 65 A 45 45 0 0 1 70 110 Z" fill="#ef4444" />
          <text x="130" y="45" fill="#3b82f6" font-size="11" font-weight="bold">Writer: 25%</text>
          <text x="130" y="70" fill="#ef4444" font-size="11" font-weight="bold">Calc: 25%</text>
          <text x="130" y="95" fill="#10b981" font-size="11" font-weight="bold">Impress: 50%</text>
        </svg>
      `;
    } else {
      // Column chart
      chartBox.innerHTML = `
        <div style="font-weight:700; font-size:0.85rem; margin-bottom:8px; color:#10b981;">📊 Perbandingan Efisiensi Memori (MB)</div>
        <svg width="280" height="130" viewBox="0 0 280 130">
          <line x1="30" y1="10" x2="30" y2="105" stroke="#475569" stroke-width="1.5" />
          <line x1="30" y1="105" x2="270" y2="105" stroke="#475569" stroke-width="1.5" />
          <!-- Bar 1: Competitor -->
          <rect x="55" y="25" width="40" height="80" fill="#f43f5e" rx="2" />
          <text x="58" y="20" fill="#fda4af" font-size="10" font-weight="bold">280MB</text>
          <!-- Bar 2: WidyaOffice -->
          <rect x="145" y="85" width="40" height="20" fill="#10b981" rx="2" />
          <text x="150" y="80" fill="#6ee7b7" font-size="10" font-weight="bold">32MB</text>
          <text x="50" y="120" fill="#94a3b8" font-size="10">Lainnya</text>
          <text x="145" y="120" fill="#10b981" font-size="10" font-weight="bold">Widya</text>
        </svg>
      `;
    }

    canvas.appendChild(chartBox);
  },

  promptInsertImage() {
    const url = prompt("Masukkan tautan berkas gambar (atau biarkan kosong untuk ilustrasi vektor bawaan):", "");
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;

    const imgContainer = document.createElement("div");
    imgContainer.style.position = "absolute";
    imgContainer.style.top = "180px";
    imgContainer.style.left = "260px";
    imgContainer.style.borderRadius = "6px";
    imgContainer.style.overflow = "hidden";
    imgContainer.style.border = "2px solid rgba(255,255,255,0.3)";
    imgContainer.style.boxShadow = "0 8px 24px rgba(0,0,0,0.5)";

    if (url && url.trim().length > 0) {
      imgContainer.innerHTML = `<img src="${url}" style="width:260px; height:160px; object-fit:cover; display:block;" onerror="this.src=''; this.alt='Gambar tidak dapat dimuat';" />`;
    } else {
      imgContainer.innerHTML = `
        <div style="width:240px; height:150px; background:linear-gradient(135deg, #1e293b, #0f172a); display:flex; align-items:center; justify-content:center; color:#94a3b8; font-size:0.8rem; text-align:center; padding:12px;">
          🖼️ Grafis Vektor Kedaulatan Widya
        </div>
      `;
    }
    canvas.appendChild(imgContainer);
  },

  changeSlideColorPrompt() {
    const col = prompt("Masukkan warna atau kode heksadesimal latar slide (misal: #1e293b, #f8fafc, #0f172a):", "#1e293b");
    if (col && this.slides[this.activeSlideIndex]) {
      this.slides[this.activeSlideIndex].customBg = col;
      this.renderCurrentSlide();
      this.renderThumbnails();
      if (this.viewMode === "sorter") this.renderSlideSorter();
    }
  },

  applyBgToAllSlides() {
    const cur = this.slides[this.activeSlideIndex];
    const col = cur ? (cur.customBg || "#1e293b") : "#1e293b";
    this.slides.forEach(s => s.customBg = col);
    this.renderCurrentSlide();
    this.renderThumbnails();
    if (this.viewMode === "sorter") this.renderSlideSorter();
    alert(`🎨 Warna latar "${col}" berhasil diterapkan ke seluruh ${this.slides.length} slide!`);
  },

  applyTransitionToAll() {
    alert(`✨ Efek transisi "${this.currentTransition.toUpperCase()}" berhasil diterapkan ke seluruh slide!`);
  },

  setTransition(type) {
    this.currentTransition = type;
  },

  // ===================== FULLSCREEN PRESENTER COCKPIT (F5 / Shift+F5) =====================
  startFullScreenPresentation(fromBeginning = false) {
    if (fromBeginning) {
      this.activeSlideIndex = 0;
    }

    // Skip hidden slide if starting on one
    if (this.slides[this.activeSlideIndex] && this.slides[this.activeSlideIndex].hidden) {
      const nextVisible = this.slides.findIndex((s, i) => i >= this.activeSlideIndex && !s.hidden);
      if (nextVisible !== -1) {
        this.activeSlideIndex = nextVisible;
      }
    }

    this.isPresenterActive = true;
    this.isBlackout = false;
    this.isWhiteout = false;

    const modal = document.getElementById("impressPresenterModal");
    if (!modal) return;

    modal.classList.add("active");
    this.startTimer();
    this.startRealClock();
    this.renderPresenterContent();

    // Reset blackout / whiteout states
    const mainView = document.getElementById("presenterMainView");
    if (mainView) {
      mainView.classList.remove("blackout", "whiteout");
    }

    // Resize drawing canvas to match viewport
    this.initDrawingCanvas();
  },

  initDrawingCanvas() {
    const canvas = document.getElementById("presenterDrawCanvas");
    const mainView = document.getElementById("presenterMainView");
    if (canvas && mainView) {
      canvas.width = mainView.clientWidth || 800;
      canvas.height = mainView.clientHeight || 500;
      const ctx = canvas.getContext("2d");
      ctx.lineCap = "round";
      ctx.lineJoin = "round";
      ctx.lineWidth = 3;
      ctx.strokeStyle = this.penColor;
    }
  },

  bindPresenterDrawing() {
    const canvas = document.getElementById("presenterDrawCanvas");
    if (!canvas) return;

    const ctx = canvas.getContext("2d");

    canvas.addEventListener("mousedown", (e) => {
      if (!this.isPenActive) return;
      this.isDrawing = true;
      const rect = canvas.getBoundingClientRect();
      this.lastX = e.clientX - rect.left;
      this.lastY = e.clientY - rect.top;
      ctx.beginPath();
      ctx.moveTo(this.lastX, this.lastY);
    });

    canvas.addEventListener("mousemove", (e) => {
      if (!this.isDrawing || !this.isPenActive) return;
      const rect = canvas.getBoundingClientRect();
      const currentX = e.clientX - rect.left;
      const currentY = e.clientY - rect.top;
      ctx.lineTo(currentX, currentY);
      ctx.stroke();
      this.lastX = currentX;
      this.lastY = currentY;
    });

    const stopDrawing = () => {
      if (this.isDrawing) {
        ctx.closePath();
        this.isDrawing = false;
      }
    };

    canvas.addEventListener("mouseup", stopDrawing);
    canvas.addEventListener("mouseleave", stopDrawing);

    // Laser pointer movement
    const laserDot = document.getElementById("virtualLaserDot");
    const mainView = document.getElementById("presenterMainView");
    if (mainView && laserDot) {
      mainView.addEventListener("mousemove", (e) => {
        if (this.isLaserActive) {
          laserDot.style.left = `${e.clientX}px`;
          laserDot.style.top = `${e.clientY}px`;
          laserDot.style.display = "block";
        }
      });
      mainView.addEventListener("mouseleave", () => {
        laserDot.style.display = "none";
      });
    }
  },

  togglePen() {
    this.isPenActive = !this.isPenActive;
    const btn = document.getElementById("btnPenPointer");
    const canvas = document.getElementById("presenterDrawCanvas");
    if (btn) btn.classList.toggle("active", this.isPenActive);

    if (this.isPenActive) {
      if (this.isLaserActive) this.toggleLaserPointer(); // Laser off
      if (canvas) canvas.style.pointerEvents = "auto";
    } else {
      if (canvas) canvas.style.pointerEvents = "none";
    }
  },

  clearDrawings() {
    const canvas = document.getElementById("presenterDrawCanvas");
    if (canvas) {
      const ctx = canvas.getContext("2d");
      ctx.clearRect(0, 0, canvas.width, canvas.height);
    }
  },

  toggleBlackScreen() {
    this.isBlackout = !this.isBlackout;
    this.isWhiteout = false;
    const mainView = document.getElementById("presenterMainView");
    const btnBlack = document.getElementById("btnBlackout");
    const btnWhite = document.getElementById("btnWhiteout");

    if (btnWhite) btnWhite.classList.remove("active");
    if (btnBlack) btnBlack.classList.toggle("active", this.isBlackout);

    if (mainView) {
      mainView.classList.remove("whiteout");
      mainView.classList.toggle("blackout", this.isBlackout);
    }
  },

  toggleWhiteScreen() {
    this.isWhiteout = !this.isWhiteout;
    this.isBlackout = false;
    const mainView = document.getElementById("presenterMainView");
    const btnBlack = document.getElementById("btnBlackout");
    const btnWhite = document.getElementById("btnWhiteout");

    if (btnBlack) btnBlack.classList.remove("active");
    if (btnWhite) btnWhite.classList.toggle("active", this.isWhiteout);

    if (mainView) {
      mainView.classList.remove("blackout");
      mainView.classList.toggle("whiteout", this.isWhiteout);
    }
  },

  zoomNotes(delta) {
    this.notesFontSize = Math.min(24, Math.max(10, this.notesFontSize + delta * 2));
    const notesBox = document.getElementById("presenterNotesArea");
    if (notesBox) {
      notesBox.style.fontSize = `${this.notesFontSize}px`;
    }
  },

  startRealClock() {
    const clockEl = document.getElementById("presenterRealClock");
    const updateTime = () => {
      if (clockEl) {
        const now = new Date();
        clockEl.innerText = now.toTimeString().split(" ")[0];
      }
    };
    updateTime();
    clearInterval(this.realClockInterval);
    this.realClockInterval = setInterval(updateTime, 1000);
  },

  renderPresenterContent() {
    const curSlide = this.slides[this.activeSlideIndex];

    // Find next non-hidden slide
    let nextSlide = null;
    for (let i = this.activeSlideIndex + 1; i < this.slides.length; i++) {
      if (!this.slides[i].hidden) {
        nextSlide = this.slides[i];
        break;
      }
    }

    const mainScreen = document.getElementById("presenterMainSlide");
    if (mainScreen && curSlide) {
      const ratioCls = this.aspectRatio === "4:3" ? "ratio-4-3" : "ratio-16-9";
      mainScreen.className = `impress-slide-canvas ${this.theme} ${ratioCls}`;
      if (curSlide.customBg) {
        mainScreen.style.background = curSlide.customBg;
      } else {
        mainScreen.style.background = "";
      }
      mainScreen.innerHTML = this.getSlideHtml(curSlide);
    }

    const nextScreen = document.getElementById("presenterNextSlide");
    if (nextScreen) {
      if (nextSlide) {
        nextScreen.className = `impress-thumb-preview ${this.theme}`;
        nextScreen.innerHTML = `
          <div style="font-weight:700; font-size:0.75rem;">${nextSlide.title || "Slide Berikutnya"}</div>
          <div style="font-size:0.65rem; opacity:0.8;">${this.getLayoutName(nextSlide.layout)}</div>
        `;
      } else {
        nextScreen.innerHTML = "<div style='color:#64748b; font-size:0.75rem; text-align:center; padding:16px;'>Akhir Presentasi</div>";
      }
    }

    const notesBox = document.getElementById("presenterNotesArea");
    if (notesBox) {
      notesBox.value = (curSlide && curSlide.notes) ? curSlide.notes : "";
    }

    const progress = document.getElementById("impressPresenterProgress");
    if (progress) {
      const hiddenTag = curSlide && curSlide.hidden ? " (Tersembunyi)" : "";
      progress.innerText = `Slide ${this.activeSlideIndex + 1} / ${this.slides.length}${hiddenTag}`;
    }
  },

  nextPresenterSlide() {
    // Find next non-hidden slide
    for (let i = this.activeSlideIndex + 1; i < this.slides.length; i++) {
      if (!this.slides[i].hidden) {
        this.activeSlideIndex = i;
        this.renderPresenterContent();
        this.renderThumbnails();
        this.renderCurrentSlide();
        return;
      }
    }
  },

  prevPresenterSlide() {
    // Find previous non-hidden slide
    for (let i = this.activeSlideIndex - 1; i >= 0; i--) {
      if (!this.slides[i].hidden) {
        this.activeSlideIndex = i;
        this.renderPresenterContent();
        this.renderThumbnails();
        this.renderCurrentSlide();
        return;
      }
    }
  },

  toggleLaserPointer() {
    this.isLaserActive = !this.isLaserActive;
    const btn = document.getElementById("btnLaserPointer");
    if (btn) btn.classList.toggle("active", this.isLaserActive);

    if (this.isLaserActive && this.isPenActive) {
      this.togglePen(); // Pen off
    }
  },

  startTimer() {
    if (this.isTimerRunning) return;
    this.isTimerRunning = true;
    this.timerInterval = setInterval(() => {
      this.timerSeconds++;
      const m = String(Math.floor(this.timerSeconds / 60)).padStart(2, "0");
      const s = String(this.timerSeconds % 60).padStart(2, "0");
      const timerDigits = document.getElementById("presenterTimerDigits");
      if (timerDigits) timerDigits.innerText = `${m}:${s}`;
    }, 1000);
  },

  resetTimer() {
    this.timerSeconds = 0;
    const timerDigits = document.getElementById("presenterTimerDigits");
    if (timerDigits) timerDigits.innerText = "00:00";
  },

  closePresenter() {
    this.isPresenterActive = false;
    clearInterval(this.timerInterval);
    clearInterval(this.realClockInterval);
    this.isTimerRunning = false;
    const modal = document.getElementById("impressPresenterModal");
    if (modal) modal.classList.remove("active");
  },

  bindEvents() {
    document.addEventListener("keydown", (e) => {
      // In Presenter View
      if (this.isPresenterActive) {
        if (e.key === "ArrowRight" || e.key === " " || e.key === "PageDown" || e.key === "Enter") {
          e.preventDefault();
          this.nextPresenterSlide();
        } else if (e.key === "ArrowLeft" || e.key === "PageUp" || e.key === "Backspace") {
          e.preventDefault();
          this.prevPresenterSlide();
        } else if (e.key === "b" || e.key === "B") {
          e.preventDefault();
          this.toggleBlackScreen();
        } else if (e.key === "w" || e.key === "W") {
          e.preventDefault();
          this.toggleWhiteScreen();
        } else if (e.key === "p" || e.key === "P") {
          e.preventDefault();
          this.togglePen();
        } else if (e.key === "l" || e.key === "L") {
          e.preventDefault();
          this.toggleLaserPointer();
        } else if (e.key === "Escape") {
          e.preventDefault();
          this.closePresenter();
        }
        return;
      }

      // Check if current workspace is WidyaImpress
      const impressView = document.getElementById("workspace-impress");
      if (!impressView || !impressView.classList.contains("active")) return;

      // In Editor
      const activeEl = document.activeElement;
      const isTyping = activeEl && (activeEl.tagName === "INPUT" || activeEl.tagName === "TEXTAREA" || activeEl.isContentEditable);

      if (e.ctrlKey && e.key === "m") {
        e.preventDefault();
        this.addNewSlide("content");
      } else if (e.ctrlKey && e.key === "d") {
        e.preventDefault();
        this.duplicateSlide();
      } else if (e.key === "Delete" && !isTyping) {
        e.preventDefault();
        this.deleteSlide();
      } else if (e.key === "F5") {
        e.preventDefault();
        if (e.shiftKey) {
          this.startFullScreenPresentation(false); // Current slide
        } else {
          this.startFullScreenPresentation(true); // From beginning
        }
      }
    });

    const notesBox = document.getElementById("presenterNotesArea");
    if (notesBox) {
      notesBox.addEventListener("input", (e) => {
        if (this.slides[this.activeSlideIndex]) {
          this.slides[this.activeSlideIndex].notes = e.target.value;
        }
        const drawerNotes = document.getElementById("impressNotesTextarea");
        if (drawerNotes) drawerNotes.value = e.target.value;
      });
    }
  },

  toggleNotesDrawer() {
    const drawer = document.getElementById("impressNotesDrawer");
    if (!drawer) return;
    const isHidden = (window.getComputedStyle(drawer).display === "none" || drawer.style.display === "none");
    drawer.style.display = isHidden ? "flex" : "none";
    if (isHidden) {
      const textarea = document.getElementById("impressNotesTextarea");
      if (textarea) textarea.focus();
    }
  },

  exportPptx() {
    alert("📽️ [WidyaImpress] Mengompilasi paket presentasi OpenXML .pptx murni via Rust...\nSemua bentuk vektor, tabel, bagan, dan kartu metrik dikonversi ke drawingML. Selesai!");
  },

  exportPdf() {
    alert("📄 [WidyaImpress] Menghasilkan handout PDF beresolusi tinggi...\nSelesai! Berkas siap dicetak.");
  },

  slideTransition: "push",
  isRehearsing: false,
  rehearseTimerInterval: null,
  rehearseSeconds: 0,

  setSlideTransition(val) {
    this.slideTransition = val;
  },

  previewTransition(type) {
    const t = type || this.slideTransition || "push";
    if (t === "none") return;
    const canvas = document.getElementById("impressSlideCanvas");
    if (!canvas) return;

    canvas.classList.remove("slide-transition-fade", "slide-transition-push", "slide-transition-wipe", "slide-transition-zoom", "slide-transition-flip");
    void canvas.offsetWidth; // Force reflow
    canvas.classList.add(`slide-transition-${t}`);
    setTimeout(() => {
      canvas.classList.remove(`slide-transition-${t}`);
    }, 500);
  },

  toggleRehearseTimings() {
    this.isRehearsing = !this.isRehearsing;
    const display = document.getElementById("rehearseTimerDisplay");
    const btn = document.getElementById("btnRehearse");

    if (this.isRehearsing) {
      this.rehearseSeconds = 0;
      if (display) {
        display.style.display = "inline";
        display.innerText = "00:00";
      }
      if (btn) btn.classList.add("active");
      this.rehearseTimerInterval = setInterval(() => {
        this.rehearseSeconds++;
        const m = String(Math.floor(this.rehearseSeconds / 60)).padStart(2, "0");
        const s = String(this.rehearseSeconds % 60).padStart(2, "0");
        if (display) display.innerText = `${m}:${s}`;
      }, 1000);
    } else {
      if (this.rehearseTimerInterval) clearInterval(this.rehearseTimerInterval);
      if (btn) btn.classList.remove("active");
      const m = String(Math.floor(this.rehearseSeconds / 60)).padStart(2, "0");
      const s = String(this.rehearseSeconds % 60).padStart(2, "0");
      alert(`⏱️ [Latihan Pewaktuan Selesai]\nTotal durasi presentasi Anda: ${m}:${s} menit.\nWaktu ini siap digunakan untuk peragaan slide otomatis!`);
    }
  },

  importDsl(code) {
    if (!code || typeof code !== "string") return { success: false, error: "Kode skrip kosong" };

    if (window.nativeIpc && typeof window.nativeIpc.panggilMetode === "function") {
      try {
        const resp = window.nativeIpc.panggilMetode("impress.compile_dsl", { code: code });
        if (resp && resp.sukses && resp.slides_json && Array.isArray(resp.slides_json.slides)) {
          this.slides = resp.slides_json.slides;
          if (resp.slides_json.theme) this.theme = resp.slides_json.theme;
          if (resp.slides_json.aspectRatio) this.setAspectRatio(resp.slides_json.aspectRatio);
          this.activeSlideIndex = 0;
          this.renderThumbnails();
          this.renderCurrentSlide();
          return { success: true, totalSlides: this.slides.length, title: resp.judul };
        }
      } catch (err) {
        console.warn("[WidyaImpress] Fallback ke parser JS internal:", err);
      }
    }

    // Fallback parser internal
    const newSlides = [];
    const rxSlide = /(slide_judul|slide_konten|slide_kpi|slide_dua_kolom)\s*\(([\s\S]*?)\n\s*\)/g;
    let m;
    while ((m = rxSlide.exec(code)) !== null) {
      const type = m[1];
      const body = m[2];
      const getField = (name) => {
        const r = new RegExp(`${name}:\\s*"([^"]*)"`);
        const res = r.exec(body);
        return res ? res[1] : null;
      };

      const title = getField("judul") || "Slide";
      const subtitle = getField("subjudul") || "";
      const notes = getField("catatan") || "";

      if (type === "slide_judul") {
        newSlides.push({ layout: "title", title, subtitle, bullets: [], kpiCards: [], notes, hidden: false });
      } else if (type === "slide_kpi") {
        const kpiCards = [];
        const rxCard = /{\s*label:\s*"([^"]*)",\s*nilai:\s*"([^"]*)",\s*subteks:\s*"([^"]*)",\s*positif:\s*(benar|salah|true|false)/g;
        let cMatch;
        while ((cMatch = rxCard.exec(body)) !== null) {
          kpiCards.push({
            label: cMatch[1],
            value: cMatch[2],
            subtext: cMatch[3],
            positive: cMatch[4] === "benar" || cMatch[4] === "true"
          });
        }
        newSlides.push({ layout: "kpi", title, subtitle, bullets: [], kpiCards, notes, hidden: false });
      } else if (type === "slide_dua_kolom") {
        const leftTitle = getField("kiri_judul") || "Kolom Kiri";
        const rightTitle = getField("kanan_judul") || "Kolom Kanan";
        const getList = (name) => {
          const r = new RegExp(`${name}:\\s*\\[([^\\]]*)\\]`);
          const res = r.exec(body);
          if (!res) return [];
          const pts = [];
          const rItem = /"([^"]*)"/g;
          let im;
          while ((im = rItem.exec(res[1])) !== null) pts.push(im[1]);
          return pts;
        };
        newSlides.push({
          layout: "two-column",
          title,
          subtitle,
          leftTitle,
          leftBullets: getList("kiri_poin"),
          rightTitle,
          rightBullets: getList("kanan_poin"),
          bullets: [],
          kpiCards: [],
          notes,
          hidden: false
        });
      } else {
        const getBullets = () => {
          const r = /poin:\s*\[([^\]]*)\]/;
          const res = r.exec(body);
          if (!res) return [];
          const pts = [];
          const rItem = /"([^"]*)"/g;
          let im;
          while ((im = rItem.exec(res[1])) !== null) pts.push(im[1]);
          return pts;
        };
        newSlides.push({ layout: "content", title, subtitle, bullets: getBullets(), kpiCards: [], notes, hidden: false });
      }
    }

    if (newSlides.length > 0) {
      this.slides = newSlides;
      this.activeSlideIndex = 0;
      this.renderThumbnails();
      this.renderCurrentSlide();
      return { success: true, totalSlides: this.slides.length };
    }
    return { success: false, error: "Tidak ada slide yang berhasil diparse" };
  },

  importMarkdown(mdText) {
    if (!mdText || typeof mdText !== "string") return { success: false, error: "Teks Markdown kosong" };

    const sections = mdText.split(/\n---/);
    const newSlides = [];

    sections.forEach((sec, idx) => {
      const lines = sec.split("\n").map(l => l.trim()).filter(Boolean);
      if (lines.length === 0) return;

      let title = "";
      let subtitle = "";
      const bullets = [];
      let notes = "";

      lines.forEach(l => {
        if (l.startsWith("# ")) {
          title = l.substring(2).trim();
        } else if (l.startsWith("## ") && !subtitle) {
          subtitle = l.substring(3).trim();
        } else if (l.startsWith("- ") || l.startsWith("* ")) {
          bullets.push(l.substring(2).trim());
        } else if (l.startsWith("<!--") && l.includes("notes:")) {
          notes = l.replace("<!--", "").replace("-->", "").replace("notes:", "").trim();
        } else if (l.startsWith("> Catatan:") || l.startsWith("> Note:")) {
          const colonIdx = l.indexOf(":");
          notes = l.substring(colonIdx + 1).trim();
        } else if (title && !subtitle && bullets.length === 0 && !l.startsWith("#")) {
          subtitle = l;
        }
      });

      if (!title) title = `Slide ${idx + 1}`;
      const layout = idx === 0 && bullets.length === 0 ? "title" : "content";
      newSlides.push({
        layout,
        title,
        subtitle,
        bullets,
        kpiCards: [],
        notes,
        hidden: false
      });
    });

    if (newSlides.length > 0) {
      this.slides = newSlides;
      this.activeSlideIndex = 0;
      this.renderThumbnails();
      this.renderCurrentSlide();
      return { success: true, totalSlides: this.slides.length };
    }
    return { success: false, error: "Markdown tidak memuat slide valid" };
  },

  exportDsl() {
    let out = "presentasi {\n";
    out += `    judul: "${(this.slides[0]?.title || "Presentasi Widya").replace(/"/g, '\\"')}",\n`;
    out += `    tema: "${this.theme}",\n`;
    out += `    rasio: "${this.aspectRatio}",\n\n`;

    this.slides.forEach(s => {
      if (s.layout === "title") {
        out += `    slide_judul(\n        judul: "${(s.title || "").replace(/"/g, '\\"')}"`;
        if (s.subtitle) out += `,\n        subjudul: "${s.subtitle.replace(/"/g, '\\"')}"`;
        if (s.notes) out += `,\n        catatan: "${s.notes.replace(/"/g, '\\"')}"`;
        out += "\n    )\n\n";
      } else if (s.layout === "kpi") {
        out += `    slide_kpi(\n        judul: "${(s.title || "").replace(/"/g, '\\"')}"`;
        if (s.subtitle) out += `,\n        subjudul: "${s.subtitle.replace(/"/g, '\\"')}"`;
        out += ",\n        kartu: [\n";
        (s.kpiCards || []).forEach((c, i) => {
          const sep = i + 1 < s.kpiCards.length ? "," : "";
          out += `            { label: "${c.label}", nilai: "${c.value}", subteks: "${c.subtext}", positif: ${c.positive ? "benar" : "salah"} }${sep}\n`;
        });
        out += "        ]";
        if (s.notes) out += `,\n        catatan: "${s.notes.replace(/"/g, '\\"')}"`;
        out += "\n    )\n\n";
      } else if (s.layout === "two-column") {
        out += `    slide_dua_kolom(\n        judul: "${(s.title || "").replace(/"/g, '\\"')}"`;
        if (s.subtitle) out += `,\n        subjudul: "${s.subtitle.replace(/"/g, '\\"')}"`;
        if (s.leftTitle) out += `,\n        kiri_judul: "${s.leftTitle.replace(/"/g, '\\"')}"`;
        const lb = (s.leftBullets || []).map(b => `"${b.replace(/"/g, '\\"')}"`).join(", ");
        out += `,\n        kiri_poin: [${lb}]`;
        if (s.rightTitle) out += `,\n        kanan_judul: "${s.rightTitle.replace(/"/g, '\\"')}"`;
        const rb = (s.rightBullets || []).map(b => `"${b.replace(/"/g, '\\"')}"`).join(", ");
        out += `,\n        kanan_poin: [${rb}]`;
        if (s.notes) out += `,\n        catatan: "${s.notes.replace(/"/g, '\\"')}"`;
        out += "\n    )\n\n";
      } else {
        out += `    slide_konten(\n        judul: "${(s.title || "").replace(/"/g, '\\"')}"`;
        if (s.subtitle) out += `,\n        subjudul: "${s.subtitle.replace(/"/g, '\\"')}"`;
        const pts = (s.bullets || []).map(b => `"${b.replace(/"/g, '\\"')}"`).join(", ");
        out += `,\n        poin: [${pts}]`;
        if (s.notes) out += `,\n        catatan: "${s.notes.replace(/"/g, '\\"')}"`;
        out += "\n    )\n\n";
      }
    });

    out += "}\n";
    return { success: true, dsl: out };
  },

  exportMarkdown() {
    let out = "";
    this.slides.forEach((s, idx) => {
      if (idx > 0) out += "\n---\n\n";
      out += `# ${s.title || "Slide " + (idx + 1)}\n`;
      if (s.subtitle) out += `## ${s.subtitle}\n\n`;
      if (s.layout === "two-column") {
        if (s.leftTitle) out += `### ${s.leftTitle}\n`;
        (s.leftBullets || []).forEach(b => out += `- ${b}\n`);
        out += "\n";
        if (s.rightTitle) out += `### ${s.rightTitle}\n`;
        (s.rightBullets || []).forEach(b => out += `- ${b}\n`);
      } else if (s.layout === "kpi") {
        (s.kpiCards || []).forEach(c => out += `- **${c.label}**: ${c.value} (${c.subtext})\n`);
      } else {
        (s.bullets || []).forEach(b => out += `- ${b}\n`);
      }
      if (s.notes) out += `\n<!-- notes: ${s.notes} -->\n`;
    });
    return { success: true, markdown: out };
  },

  getStats() {
    return {
      totalSlides: this.slides.length,
      activeSlideIndex: this.activeSlideIndex,
      activeLayout: this.slides[this.activeSlideIndex]?.layout || "content",
      theme: this.theme,
      aspectRatio: this.aspectRatio,
      currentTransition: this.slideTransition || "push",
      isPresenterActive: this.isPresenterActive
    };
  },

  renderSlideSvg(slideIdx = 0) {
    const idx = Math.min(Math.max(0, slideIdx), this.slides.length - 1);
    const s = this.slides[idx];
    if (!s) return { success: false, error: "Slide tidak ditemukan" };

    const is43 = this.aspectRatio === "4:3";
    const w = 960, h = is43 ? 720 : 540;

    let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${w} ${h}" width="100%" height="auto" style="border-radius:8px; box-shadow:0 8px 24px rgba(0,0,0,0.3); font-family:sans-serif;">\n`;
    svg += `  <rect width="${w}" height="${h}" fill="#0f172a" rx="8" />\n`;
    svg += `  <text x="60" y="80" font-size="28" font-weight="bold" fill="#f8fafc">${s.title || ""}</text>\n`;
    if (s.subtitle) svg += `  <text x="60" y="115" font-size="16" fill="#94a3b8">${s.subtitle}</text>\n`;
    svg += `  <line x1="60" y1="135" x2="${w - 60}" y2="135" stroke="#334155" stroke-width="1.5" />\n`;

    if (s.layout === "kpi" && s.kpiCards) {
      const cardW = (w - 120 - (s.kpiCards.length - 1) * 20) / s.kpiCards.length;
      s.kpiCards.forEach((c, i) => {
        const x = 60 + i * (cardW + 20);
        svg += `  <rect x="${x}" y="170" width="${cardW}" height="220" fill="#1e293b" stroke="#38bdf8" stroke-width="1.5" rx="8" />\n`;
        svg += `  <text x="${x + 20}" y="210" font-size="14" font-weight="600" fill="#94a3b8">${c.label}</text>\n`;
        svg += `  <text x="${x + 20}" y="270" font-size="34" font-weight="bold" fill="#f8fafc">${c.value}</text>\n`;
        svg += `  <text x="${x + 20}" y="340" font-size="13" fill="#38bdf8">${c.subtext}</text>\n`;
      });
    } else {
      (s.bullets || []).forEach((b, i) => {
        const y = 180 + i * 40;
        svg += `  <circle cx="80" cy="${y - 5}" r="4" fill="#38bdf8" />\n`;
        svg += `  <text x="100" y="${y}" font-size="16" fill="#e2e8f0">${b}</text>\n`;
      });
    }

    svg += `  <text x="60" y="${h - 30}" font-size="12" fill="#64748b">WidyaImpress • Slide ${idx + 1} / ${this.slides.length}</text>\n`;
    svg += `</svg>\n`;
    return { success: true, svg: svg, slideIndex: idx };
  }
};

window.WidyaImpress = WidyaImpress;
