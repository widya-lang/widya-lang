/**
 * WidyaOffice Desktop - Unified Command Bus & Catalog Engine
 * Diadaptasi dari filosofi arsitektur WordCraft (400+ discrete command catalog & MCP-drivable).
 * Menghubungkan Ribbon GUI, shortcut keyboard, pencarian "Beri Tahu Saya",
 * IPC Backend Rust (office_bridge.rs), dan Agen AI ke satu bus perintah terpusat.
 */

(function(global) {
  "use strict";

  const OfficeCommands = {
    // Registry penyimpanan definisi perintah
    _registry: new Map(),
    _aliases: new Map(),
    _history: [],
    _maxHistory: 100,
    _shortcuts: [],

    init(core) {
      this.core = core;
      this.registerBuiltinCommands();
      this.bindDataCommandListeners();
      console.log(`[OfficeCommands] Terinisialisasi: ${this._registry.size} perintah terdaftar.`);
    },

    /**
     * Daftarkan satu perintah baru ke dalam katalog
     */
    register(def) {
      if (!def || !def.id) return;
      const id = def.id.toLowerCase();
      this._registry.set(id, def);

      // Daftarkan alias (bahasa Indonesia dan variasi)
      if (Array.isArray(def.aliases)) {
        def.aliases.forEach(alias => {
          this._aliases.set(alias.toLowerCase(), id);
        });
      }

      // Daftarkan shortcut jika tersedia
      if (def.shortcut) {
        this._shortcuts.push({
          shortcut: def.shortcut.toLowerCase(),
          commandId: id
        });
      }
    },

    /**
     * Eksekusi perintah berdasarkan ID atau Alias
     * @param {string} idOrAlias - Contoh: "format.bold", "format.tebal", "insert.page_break"
     * @param {any} payload - Argumen opsional
     * @returns {Object} { success: boolean, command: string, data?: any, error?: string }
     */
    execute(idOrAlias, payload = null) {
      if (!idOrAlias || typeof idOrAlias !== "string") {
        return { success: false, error: "ID perintah tidak valid" };
      }

      const query = idOrAlias.trim().toLowerCase();
      const resolvedId = this._aliases.get(query) || query;
      const cmd = this._registry.get(resolvedId);

      if (!cmd) {
        console.warn(`[OfficeCommands] Perintah tidak ditemukan: "${idOrAlias}"`);
        return { success: false, error: `Perintah "${idOrAlias}" tidak ditemukan dalam katalog` };
      }

      try {
        const result = cmd.run ? cmd.run(payload, this.core || global.OfficeCore) : { success: true };
        const record = {
          command: resolvedId,
          payload: payload,
          timestamp: Date.now(),
          success: result && result.success !== false
        };

        this._history.push(record);
        if (this._history.length > this._maxHistory) {
          this._history.shift();
        }

        // Pancarkan event untuk observer / listener AI agent
        if (typeof window !== "undefined" && window.dispatchEvent) {
          window.dispatchEvent(new CustomEvent("widya:command", { detail: record }));
        }

        return Object.assign({ success: true, command: resolvedId }, result || {});
      } catch (err) {
        console.error(`[OfficeCommands] Galat saat menjalankan "${resolvedId}":`, err);
        return { success: false, command: resolvedId, error: err.toString() };
      }
    },

    /**
     * Dapatkan daftar seluruh perintah (bisa difilter per kategori)
     */
    list(category = null) {
      const items = [];
      for (const [id, cmd] of this._registry.entries()) {
        if (!category || cmd.category === category) {
          items.push({
            id: cmd.id,
            aliases: cmd.aliases || [],
            name: cmd.name || id,
            description: cmd.description || "",
            category: cmd.category || "general",
            shortcut: cmd.shortcut || null
          });
        }
      }
      return items;
    },

    /**
     * Cari perintah berdasarkan teks (untuk pencarian "Beri Tahu Saya" / Tell Me)
     */
    search(query) {
      if (!query || typeof query !== "string") return [];
      const q = query.trim().toLowerCase();
      const results = [];

      for (const [id, cmd] of this._registry.entries()) {
        let matchScore = 0;
        if (id.includes(q)) matchScore += 3;
        if (cmd.name && cmd.name.toLowerCase().includes(q)) matchScore += 5;
        if (cmd.description && cmd.description.toLowerCase().includes(q)) matchScore += 2;
        if (cmd.aliases && cmd.aliases.some(a => a.toLowerCase().includes(q))) matchScore += 4;

        if (matchScore > 0) {
          results.push({
            id: cmd.id,
            name: cmd.name,
            category: cmd.category,
            shortcut: cmd.shortcut,
            score: matchScore
          });
        }
      }

      return results.sort((a, b) => b.score - a.score);
    },

    /**
     * Tangani event keydown dan cocokkan dengan shortcut yang terdaftar
     */
    handleKeyDown(e) {
      // Abaikan jika sedang mengetik di input form teks biasa (kecuali shortcut tombol fungsi atau escape)
      const target = e.target;
      const isInput = target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.tagName === "SELECT");
      const isEscape = e.key === "Escape";
      const isFKey = e.key && e.key.startsWith("F");

      if (isInput && !isEscape && !isFKey && !((e.ctrlKey || e.metaKey) && (e.key === "s" || e.key === "S"))) {
        return false;
      }

      // Bangun string deskripsi shortcut
      const parts = [];
      if (e.ctrlKey || e.metaKey) parts.push("ctrl");
      if (e.altKey) parts.push("alt");
      if (e.shiftKey) parts.push("shift");

      let keyName = e.key.toLowerCase();
      if (keyName === " ") keyName = "space";
      parts.push(keyName);
      const currentShortcut = parts.join("+");

      // Cek pencocokan persis
      for (const item of this._shortcuts) {
        if (item.shortcut === currentShortcut) {
          e.preventDefault();
          this.execute(item.commandId);
          return true;
        }
      }

      return false;
    },

    /**
     * Delegasi event otomatis untuk elemen ber-atribut data-cmd
     */
    bindDataCommandListeners() {
      document.addEventListener("click", (e) => {
        const targetBtn = e.target.closest("[data-cmd]");
        if (targetBtn) {
          const cmdId = targetBtn.getAttribute("data-cmd");
          const cmdArg = targetBtn.getAttribute("data-cmd-arg");
          if (cmdId) {
            e.preventDefault();
            this.execute(cmdId, cmdArg);
          }
        }
      });
    },

    /**
     * Inisialisasi katalog perintah lengkap (WordCraft Parity Suite)
     */
    registerBuiltinCommands() {
      // ----------------------------------------------------
      // 1. FILE / BERKAS
      // ----------------------------------------------------
      this.register({
        id: "file.new",
        aliases: ["berkas.baru", "new_document", "buat_baru"],
        name: "Dokumen Baru",
        description: "Buat lembar kerja dokumen baru yang bersih",
        category: "file",
        shortcut: "ctrl+n",
        run: (p, core) => {
          core.loadBlankDocument();
          return { success: true };
        }
      });

      this.register({
        id: "file.save",
        aliases: ["berkas.simpan", "save_document", "simpan"],
        name: "Simpan Dokumen",
        description: "Simpan dokumen aktif ke penyimpanan lokal",
        category: "file",
        shortcut: "ctrl+s",
        run: (p, core) => {
          core.saveCurrentAppDocument();
          return { success: true };
        }
      });

      this.register({
        id: "file.save_as",
        aliases: ["berkas.simpan_sebagai", "save_as", "simpan_sebagai"],
        name: "Simpan Sebagai...",
        description: "Buka dialog Simpan Sebagai untuk memilih nama dan folder berkas",
        category: "file",
        shortcut: "f12",
        run: (p, core) => {
          core.openSaveAsDialog();
          return { success: true };
        }
      });

      this.register({
        id: "file.browse_folder",
        aliases: ["berkas.jelajahi_folder", "browse_folder", "pilih_folder"],
        name: "Jelajahi Folder",
        description: "Pilih direktori penyimpanan dokumen dengan pemilih folder berdaulat",
        category: "file",
        run: (p, core) => {
          core.browseFolderLocation();
          return { success: true };
        }
      });

      this.register({
        id: "file.export_pdf",
        aliases: ["berkas.ekspor_pdf", "export_pdf", "cetak_pdf"],
        name: "Ekspor ke PDF",
        description: "Konversi dan ekspor dokumen menjadi berkas PDF vektor berkualitas tinggi",
        category: "file",
        shortcut: "ctrl+p",
        run: (p, core) => {
          core.exportDocumentToPdf();
          return { success: true };
        }
      });

      this.register({
        id: "file.export_docx",
        aliases: ["berkas.ekspor_docx", "export_docx"],
        name: "Ekspor ke Word (.docx)",
        description: "Ekspor dokumen ke format standar OpenXML DOCX murni",
        category: "file",
        run: (p, core) => {
          core.exportDocumentToDocx();
          return { success: true };
        }
      });

      this.register({
        id: "file.import_markdown",
        aliases: ["berkas.impor_markdown", "import_markdown", "buka_markdown"],
        name: "Impor Naskah Markdown (.md)",
        description: "Muat naskah Markdown akademik (seperti bab disertasi) langsung ke lembar WidyaWriter",
        category: "file",
        run: async (payload, core) => {
          let mdText = typeof payload === "string" ? payload : (payload && payload.markdown ? payload.markdown : null);
          if (!mdText) {
            mdText = prompt("Tempelkan naskah Markdown atau Bab Disertasi Anda di sini:");
            if (!mdText) return { success: false, message: "Dibatalkan" };
          }
          return await core.importMarkdownContent(mdText, payload && payload.title ? payload.title : null);
        }
      });

      this.register({
        id: "file.import_widya_dsl",
        aliases: ["berkas.impor_dsl", "import_dsl", "kompilasi_widya"],
        name: "Kompilasi Skrip Widya-Lang (.widya)",
        description: "Kompilasi kode deklaratif dokumen Widya-Lang menjadi tata letak visual WordCraft",
        category: "file",
        run: async (payload, core) => {
          let code = typeof payload === "string" ? payload : (payload && payload.code ? payload.code : null);
          if (!code) {
            code = prompt("Masukkan kode dokumen DSL Widya-Lang (dokumen { ... }):");
            if (!code) return { success: false, message: "Dibatalkan" };
          }
          return await core.compileWidyaDslContent(code);
        }
      });

      this.register({
        id: "file.export_widya_dsl",
        aliases: ["berkas.ekspor_dsl", "export_dsl"],
        name: "Ekspor ke Widya-Lang DSL (.widya)",
        description: "Ekstrak struktur dokumen aktif ke format deklaratif kode bahasa Widya-Lang",
        category: "file",
        run: (p, core) => {
          return core.exportActiveDocumentToWidyaDsl();
        }
      });

      // ----------------------------------------------------
      // 2. EDIT / SUNTING
      // ----------------------------------------------------
      this.register({
        id: "edit.undo",
        aliases: ["sunting.urung", "undo"],
        name: "Urungkan (Undo)",
        description: "Batalkan aksi pengetikan atau pemformatan terakhir",
        category: "edit",
        shortcut: "ctrl+z",
        run: (p, core) => {
          core.execFormat("undo");
          return { success: true };
        }
      });

      this.register({
        id: "edit.redo",
        aliases: ["sunting.ulang", "redo"],
        name: "Ulangi (Redo)",
        description: "Ulangi kembali aksi yang sebelumnya dibatalkan",
        category: "edit",
        shortcut: "ctrl+y",
        run: (p, core) => {
          core.execFormat("redo");
          return { success: true };
        }
      });

      this.register({
        id: "edit.select_all",
        aliases: ["sunting.pilih_semua", "select_all"],
        name: "Pilih Semua",
        description: "Pilih seluruh teks dalam dokumen aktif",
        category: "edit",
        shortcut: "ctrl+a",
        run: (p, core) => {
          core.execFormat("selectAll");
          return { success: true };
        }
      });

      this.register({
        id: "edit.format_painter",
        aliases: ["sunting.kuas_format", "format_painter"],
        name: "Kuas Format (Format Painter)",
        description: "Salin dan terapkan pemformatan teks ke bagian lain",
        category: "edit",
        run: (p, core) => {
          core.toggleFormatPainter();
          return { success: true, active: core.formatPainterActive };
        }
      });

      this.register({
        id: "edit.find",
        aliases: ["sunting.cari", "find"],
        name: "Cari Teks",
        description: "Buka panel pencarian kata dan frasa dalam naskah",
        category: "edit",
        shortcut: "ctrl+f",
        run: (p, core) => {
          core.openFindReplaceModal("find");
          return { success: true };
        }
      });

      this.register({
        id: "edit.replace",
        aliases: ["sunting.ganti", "replace"],
        name: "Ganti Teks",
        description: "Buka panel cari dan ganti kata dalam naskah",
        category: "edit",
        shortcut: "ctrl+h",
        run: (p, core) => {
          core.openFindReplaceModal("replace");
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 3. FORMATTING / PEMFORMATAN TEKS
      // ----------------------------------------------------
      this.register({
        id: "format.bold",
        aliases: ["format.tebal", "bold", "tebal"],
        name: "Tebal (Bold)",
        description: "Terapkan gaya tebal pada teks terpilih",
        category: "format",
        shortcut: "ctrl+b",
        run: (p, core) => {
          core.execFormat("bold");
          return { success: true };
        }
      });

      this.register({
        id: "format.italic",
        aliases: ["format.miring", "italic", "miring"],
        name: "Miring (Italic)",
        description: "Terapkan gaya miring pada teks terpilih",
        category: "format",
        shortcut: "ctrl+i",
        run: (p, core) => {
          core.execFormat("italic");
          return { success: true };
        }
      });

      this.register({
        id: "format.underline",
        aliases: ["format.garis_bawah", "underline", "garis_bawah"],
        name: "Garis Bawah (Underline)",
        description: "Terapkan garis bawah pada teks terpilih",
        category: "format",
        shortcut: "ctrl+u",
        run: (p, core) => {
          core.execFormat("underline");
          return { success: true };
        }
      });

      this.register({
        id: "format.strikethrough",
        aliases: ["format.coret", "strikethrough"],
        name: "Coretan Teks",
        description: "Beri garis coret di tengah teks terpilih",
        category: "format",
        run: (p, core) => {
          core.execFormat("strikeThrough");
          return { success: true };
        }
      });

      this.register({
        id: "format.subscript",
        aliases: ["format.subskrip", "subscript"],
        name: "Subskrip",
        description: "Buat huruf kecil di bawah garis dasar teks",
        category: "format",
        run: (p, core) => {
          core.execFormat("subscript");
          return { success: true };
        }
      });

      this.register({
        id: "format.superscript",
        aliases: ["format.superskrip", "superscript"],
        name: "Superskrip",
        description: "Buat huruf kecil di atas garis dasar teks",
        category: "format",
        run: (p, core) => {
          core.execFormat("superscript");
          return { success: true };
        }
      });

      this.register({
        id: "format.font_family",
        aliases: ["format.jenis_font", "font_family"],
        name: "Jenis Font",
        description: "Atur jenis huruf naskah (Calibri, Aptos, Times New Roman, dsb.)",
        category: "format",
        run: (family, core) => {
          if (family) core.setFontFamily(family);
          return { success: true, fontFamily: family };
        }
      });

      this.register({
        id: "format.font_size",
        aliases: ["format.ukuran_font", "font_size"],
        name: "Ukuran Font",
        description: "Atur ukuran poin huruf teks (pt)",
        category: "format",
        run: (size, core) => {
          if (size) core.setFontSize(size);
          return { success: true, fontSize: size };
        }
      });

      this.register({
        id: "format.grow_font",
        aliases: ["format.perbesar_font", "grow_font"],
        name: "Perbesar Ukuran Font",
        description: "Tingkatkan ukuran font teks terpilih 1 poin",
        category: "format",
        run: (p, core) => {
          core.changeFontSize(1);
          return { success: true };
        }
      });

      this.register({
        id: "format.shrink_font",
        aliases: ["format.perkecil_font", "shrink_font"],
        name: "Perkecil Ukuran Font",
        description: "Kurangi ukuran font teks terpilih 1 poin",
        category: "format",
        run: (p, core) => {
          core.changeFontSize(-1);
          return { success: true };
        }
      });

      this.register({
        id: "format.case",
        aliases: ["format.ubah_huruf", "change_case"],
        name: "Ubah Kapitalisasi Huruf",
        description: "Ubah teks ke HURUF BESAR, huruf kecil, atau Huruf Judul",
        category: "format",
        run: (mode, core) => {
          core.changeCase(mode || "title");
          return { success: true };
        }
      });

      this.register({
        id: "format.font_color",
        aliases: ["format.warna_font", "font_color"],
        name: "Warna Font",
        description: "Atur warna teks terpilih",
        category: "format",
        run: (color, core) => {
          if (color) core.setFontColor(color);
          return { success: true, color: color };
        }
      });

      this.register({
        id: "format.highlight",
        aliases: ["format.sorotan", "highlight_color"],
        name: "Warna Sorotan Teks",
        description: "Beri warna stabilo pada latar belakang teks terpilih",
        category: "format",
        run: (color, core) => {
          core.setHighlightColor(color || "yellow");
          return { success: true, color: color };
        }
      });

      this.register({
        id: "format.clear",
        aliases: ["format.hapus_format", "clear_formatting"],
        name: "Hapus Semua Pemformatan",
        description: "Kembalikan teks ke format standar polos",
        category: "format",
        run: (p, core) => {
          core.clearFormatting();
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 4. PARAGRAPH / PARAGRAF
      // ----------------------------------------------------
      this.register({
        id: "paragraph.align_left",
        aliases: ["paragraf.rata_kiri", "align_left"],
        name: "Rata Kiri",
        description: "Ratakan paragraf ke batas margin kiri",
        category: "paragraph",
        shortcut: "ctrl+l",
        run: (p, core) => {
          core.execFormat("justifyLeft");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.align_center",
        aliases: ["paragraf.rata_tengah", "align_center"],
        name: "Rata Tengah",
        description: "Posisikan paragraf tepat di tengah halaman",
        category: "paragraph",
        shortcut: "ctrl+e",
        run: (p, core) => {
          core.execFormat("justifyCenter");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.align_right",
        aliases: ["paragraf.rata_kanan", "align_right"],
        name: "Rata Kanan",
        description: "Ratakan paragraf ke batas margin kanan",
        category: "paragraph",
        shortcut: "ctrl+r",
        run: (p, core) => {
          core.execFormat("justifyRight");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.align_justify",
        aliases: ["paragraf.rata_penuh", "align_justify"],
        name: "Rata Kanan-Kiri (Justify)",
        description: "Ratakan paragraf penuh di kedua sisi margin",
        category: "paragraph",
        shortcut: "ctrl+j",
        run: (p, core) => {
          core.execFormat("justifyFull");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.bullet_list",
        aliases: ["paragraf.daftar_butir", "bullet_list"],
        name: "Daftar Butir (Bullets)",
        description: "Ubah baris menjadi daftar butir bersimbol",
        category: "paragraph",
        run: (p, core) => {
          core.execFormat("insertUnorderedList");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.number_list",
        aliases: ["paragraf.daftar_nomor", "number_list"],
        name: "Daftar Bernomor (Numbering)",
        description: "Ubah baris menjadi daftar urut bertingkat angka",
        category: "paragraph",
        run: (p, core) => {
          core.execFormat("insertOrderedList");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.indent",
        aliases: ["paragraf.tambah_inden", "indent"],
        name: "Tambah Indentasi",
        description: "Geser paragraf masuk lebih dalam ke arah kanan",
        category: "paragraph",
        run: (p, core) => {
          core.execFormat("indent");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.outdent",
        aliases: ["paragraf.kurang_inden", "outdent"],
        name: "Kurangi Indentasi",
        description: "Tarik paragraf keluar mendekati margin kiri",
        category: "paragraph",
        run: (p, core) => {
          core.execFormat("outdent");
          return { success: true };
        }
      });

      this.register({
        id: "paragraph.line_spacing",
        aliases: ["paragraf.spasi_baris", "line_spacing"],
        name: "Spasi Baris",
        description: "Atur kerapatan spasi antar baris (1.0, 1.15, 1.5, 2.0)",
        category: "paragraph",
        run: (val, core) => {
          core.setLineSpacing(val || "1.15");
          return { success: true, spacing: val };
        }
      });

      this.register({
        id: "paragraph.style",
        aliases: ["paragraf.gaya", "apply_style"],
        name: "Terapkan Gaya Paragraf",
        description: "Terapkan gaya judul Heading 1, 2, 3, Subtitle, atau Normal",
        category: "paragraph",
        run: (styleName, core) => {
          core.applyParagraphStyle(styleName || "normal");
          return { success: true, style: styleName };
        }
      });

      // ----------------------------------------------------
      // 5. INSERT / SISIPKAN
      // ----------------------------------------------------
      this.register({
        id: "insert.page_break",
        aliases: ["sisipkan.pemisah_halaman", "page_break"],
        name: "Pemisah Halaman (Page Break)",
        description: "Paksa pemecahan teks ke awal lembar halaman berikutnya",
        category: "insert",
        shortcut: "ctrl+enter",
        run: (p, core) => {
          core.insertPageBreak();
          return { success: true };
        }
      });

      this.register({
        id: "insert.table",
        aliases: ["sisipkan.tabel", "table"],
        name: "Sisipkan Tabel",
        description: "Buat tabel data baru dengan jumlah baris dan kolom yang ditentukan",
        category: "insert",
        run: (p, core) => {
          if (p && p.rows && p.cols) {
            core.insertCustomTable(p.rows, p.cols);
          } else {
            core.openTableInsertModal();
          }
          return { success: true };
        }
      });

      this.register({
        id: "insert.picture",
        aliases: ["sisipkan.gambar", "picture"],
        name: "Sisipkan Gambar",
        description: "Buka dialog untuk menyisipkan berkas gambar ke dokumen",
        category: "insert",
        run: (p, core) => {
          core.openImageInsertModal();
          return { success: true };
        }
      });

      this.register({
        id: "insert.equation",
        aliases: ["sisipkan.persamaan", "equation", "rumus"],
        name: "Sisipkan Rumus Matematika (OMML)",
        description: "Buka editor persamaan matematika berstandar OpenXML OMML",
        category: "insert",
        shortcut: "alt+=",
        run: (p, core) => {
          if (typeof MathEditor !== "undefined") MathEditor.openDialog();
          return { success: true };
        }
      });

      this.register({
        id: "insert.header",
        aliases: ["sisipkan.header", "edit_header"],
        name: "Sunting Header",
        description: "Aktifkan mode penyuntingan area kepala dokumen (header)",
        category: "insert",
        run: (p, core) => {
          core.enterHeaderFooterMode("header");
          return { success: true };
        }
      });

      this.register({
        id: "insert.footer",
        aliases: ["sisipkan.footer", "edit_footer"],
        name: "Sunting Footer",
        description: "Aktifkan mode penyuntingan area kaki dokumen (footer)",
        category: "insert",
        run: (p, core) => {
          core.enterHeaderFooterMode("footer");
          return { success: true };
        }
      });

      this.register({
        id: "insert.page_number",
        aliases: ["sisipkan.nomor_halaman", "page_number"],
        name: "Sisipkan Nomor Halaman",
        description: "Tambahkan bidang penomoran halaman otomatis",
        category: "insert",
        run: (p, core) => {
          core.insertPageNumberField();
          return { success: true };
        }
      });

      this.register({
        id: "insert.mail_merge",
        aliases: ["sisipkan.surat_massal", "mail_merge"],
        name: "Wisaya Surat Massal (Mail Merge)",
        description: "Buat ribuan surat otomatis berbasis integrasi data CSV/Database",
        category: "insert",
        run: (p, core) => {
          if (typeof MailMergeWizard !== "undefined") MailMergeWizard.openDialog();
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 6. LAYOUT / TATA LETAK
      // ----------------------------------------------------
      this.register({
        id: "layout.margins",
        aliases: ["tata_letak.margin", "margins"],
        name: "Atur Margin Halaman",
        description: "Pilih margin standar, sempit, lebar, atau akademik disertasi (4-4-3-3 cm)",
        category: "layout",
        run: (presetOrValues, core) => {
          if (typeof presetOrValues === "string") {
            core.setMargins(presetOrValues);
          } else if (presetOrValues && typeof presetOrValues === "object") {
            core.applyCustomMargins(
              presetOrValues.top || 2.54,
              presetOrValues.left || 2.54,
              presetOrValues.bottom || 2.54,
              presetOrValues.right || 2.54
            );
          }
          return { success: true };
        }
      });

      this.register({
        id: "layout.orientation",
        aliases: ["tata_letak.orientasi", "orientation"],
        name: "Orientasi Kertas",
        description: "Ubah orientasi dokumen antara Potret atau Lanskap",
        category: "layout",
        run: (orient, core) => {
          core.setOrientation(orient || "portrait");
          return { success: true, orientation: orient };
        }
      });

      this.register({
        id: "layout.paper_size",
        aliases: ["tata_letak.ukuran_kertas", "paper_size"],
        name: "Ukuran Kertas",
        description: "Pilih format kertas A4, Letter, F4 (Folio), atau Legal",
        category: "layout",
        run: (size, core) => {
          core.setPaperSize(size || "a4");
          return { success: true, paperSize: size };
        }
      });

      this.register({
        id: "layout.page_setup",
        aliases: ["tata_letak.pengaturan_halaman", "page_setup"],
        name: "Kotak Dialog Penyiapan Halaman",
        description: "Buka modal komprehensif pengaturan margin, kertas, dan orientasi",
        category: "layout",
        run: (p, core) => {
          core.openPageSetupModal();
          return { success: true };
        }
      });

      this.register({
        id: "layout.repaginate",
        aliases: ["tata_letak.paginasi", "repaginate"],
        name: "Hitung Ulang Paginasi Lembar",
        description: "Picu pembagian dan aliran paragraf antar-lembar fisik secara instan",
        category: "layout",
        run: (p, core) => {
          core.repaginateDocument();
          return { success: true, totalPages: document.querySelectorAll(".document-page").length };
        }
      });

      // ----------------------------------------------------
      // 6B. REFERENCES / REFERENSI AKADEMIK
      // ----------------------------------------------------
      this.register({
        id: "references.table_of_contents",
        aliases: ["referensi.daftar_isi", "table_of_contents", "toc"],
        name: "Sisipkan / Buat Daftar Isi",
        description: "Buat Daftar Isi publikasi otomatis lengkap dengan nomor halaman dan tautan bab",
        category: "references",
        run: (p, core) => {
          core.generateTableOfContents();
          return { success: true };
        }
      });

      this.register({
        id: "references.update_toc",
        aliases: ["referensi.perbarui_daftar_isi", "update_toc"],
        name: "Perbarui Daftar Isi",
        description: "Sinkronkan kembali nomor halaman dan daftar judul pada Daftar Isi yang ada",
        category: "references",
        run: (p, core) => {
          core.updateTableOfContents(false);
          return { success: true };
        }
      });

      this.register({
        id: "references.footnote",
        aliases: ["referensi.catatan_kaki", "footnote", "insert_footnote"],
        name: "Sisipkan Catatan Kaki",
        description: "Tambahkan rujukan catatan kaki akademik di dasar lembar halaman yang sama",
        category: "references",
        shortcut: "alt+ctrl+f",
        run: (text, core) => {
          core.insertFootnote(typeof text === "string" ? text : null);
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 7. REVIEW & TINJAU
      // ----------------------------------------------------
      this.register({
        id: "review.word_count",
        aliases: ["tinjau.hitung_kata", "word_count"],
        name: "Statistik & Jumlah Kata",
        description: "Buka dialog statistik kata, karakter, dan jumlah halaman dokumen",
        category: "review",
        run: (p, core) => {
          core.openWordCountModal();
          return { success: true };
        }
      });

      this.register({
        id: "review.spell_check",
        aliases: ["tinjau.pemeriksa_ejaan", "spell_check"],
        name: "Pemeriksa Ejaan & Tata Bahasa",
        description: "Periksa kepatuhan ejaan Bahasa Indonesia baku (KBBI / PUEBI)",
        category: "review",
        shortcut: "f7",
        run: (p, core) => {
          core.toggleSpellCheck();
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 8. VIEW / TAMPILAN
      // ----------------------------------------------------
      this.register({
        id: "view.zoom_in",
        aliases: ["tampilan.perbesar", "zoom_in"],
        name: "Perbesar Tampilan (Zoom In)",
        description: "Tingkatkan skala tampilan lembar dokumen",
        category: "view",
        run: (p, core) => {
          core.zoomIn();
          return { success: true, zoom: core.zoomLevel };
        }
      });

      this.register({
        id: "view.zoom_out",
        aliases: ["tampilan.perkecil", "zoom_out"],
        name: "Perkecil Tampilan (Zoom Out)",
        description: "Kurangi skala tampilan lembar dokumen",
        category: "view",
        run: (p, core) => {
          core.zoomOut();
          return { success: true, zoom: core.zoomLevel };
        }
      });

      this.register({
        id: "view.toggle_ruler",
        aliases: ["tampilan.penggaris", "toggle_ruler"],
        name: "Tampilkan / Sembunyikan Penggaris",
        description: "Alihkan visibilitas mistar penggaris dokumen",
        category: "view",
        run: (p, core) => {
          core.toggleRuler();
          return { success: true };
        }
      });

      this.register({
        id: "view.toggle_navigation",
        aliases: ["tampilan.panel_navigasi", "toggle_navigation"],
        name: "Tampilkan / Sembunyikan Panel Navigasi",
        description: "Buka panel daftar judul dan navigasi cepat bab naskah",
        category: "view",
        run: (p, core) => {
          core.toggleNavigationPane();
          return { success: true };
        }
      });

      this.register({
        id: "view.toggle_ribbon",
        aliases: ["tampilan.ciutkan_pita", "toggle_ribbon"],
        name: "Ciutkan / Buka Pita Perintah",
        description: "Sematkan atau ciutkan bilah menu Ribbon untuk area kerja lebih luas",
        category: "view",
        shortcut: "ctrl+f1",
        run: (p, core) => {
          core.toggleRibbonCollapse();
          return { success: true };
        }
      });

      this.register({
        id: "view.fullscreen",
        aliases: ["tampilan.layar_penuh", "fullscreen", "kiosk"],
        name: "Layar Penuh (Kiosk Mode)",
        description: "Beralih ke mode fokus layar penuh tanpa distraksi",
        category: "view",
        shortcut: "f11",
        run: (p, core) => {
          core.toggleKiosk();
          return { success: true };
        }
      });

      this.register({
        id: "view.escape",
        aliases: ["tampilan.tutup_dialog", "escape"],
        name: "Tutup Modal / Mode Aktif",
        description: "Tutup dialog yang sedang terbuka atau keluar dari mode header/footer",
        category: "view",
        shortcut: "escape",
        run: (p, core) => {
          if (core.isEditingHeaderFooter) {
            core.exitHeaderFooterMode();
          } else {
            core.closeBackstage();
            core.closePageSetupModal();
            core.closeSignatureInspector();
            core.closeParityModal();
            core.closeSaveAsDialog();
            core.closeFolderPickerModal();
            const tellMe = document.getElementById("tellMeDropdown");
            if (tellMe) tellMe.style.display = "none";
          }
          return { success: true };
        }
      });

      // ----------------------------------------------------
      // 9. DOCUMENT INSPECTION & QUERY (UNTUK AGENT AI / MCP)
      // ----------------------------------------------------
      this.register({
        id: "document.inspect",
        aliases: ["dokumen.inspeksi", "inspect_document"],
        name: "Inspeksi Dokumen (Metadata & Struktur)",
        description: "Dapatkan analisis struktural naskah (halaman, kata, karakter, bab)",
        category: "document",
        run: (p, core) => {
          const pages = document.querySelectorAll(".document-page");
          const headings = [];
          document.querySelectorAll(".doc-page-body h1, .doc-page-body h2, .doc-page-body h3").forEach(h => {
            headings.push({
              tag: h.tagName.toLowerCase(),
              text: h.innerText.trim()
            });
          });

          let totalWords = 0;
          let totalChars = 0;
          pages.forEach(p => {
            const body = p.querySelector(".doc-page-body");
            if (body) {
              const text = body.innerText || "";
              totalChars += text.length;
              const words = text.trim().split(/\s+/).filter(w => w.length > 0);
              totalWords += words.length;
            }
          });

          return {
            success: true,
            totalPages: pages.length,
            activePageIndex: core.activePageIndex || 1,
            totalWords: totalWords,
            totalCharacters: totalChars,
            headingsCount: headings.length,
            headings: headings.slice(0, 50),
            docName: core.currentDocName,
            app: core.currentApp
          };
        }
      });

      this.register({
        id: "document.repaginate",
        aliases: ["dokumen.atur_ulang_halaman", "repaginate"],
        name: "Atur Ulang Paginasi Halaman",
        description: "Hitung ulang tata letak paragraf, tabel, dan margin ke lembar-lembar fisik",
        category: "document",
        run: (p, core) => {
          core.repaginateDocument();
          const pages = document.querySelectorAll(".document-page");
          return { success: true, totalPages: pages.length };
        }
      });

      this.register({
        id: "document.get_text",
        aliases: ["dokumen.ambil_teks", "get_text"],
        name: "Ambil Teks Dokumen",
        description: "Ekstrak seluruh teks murni dokumen dari halaman pertama hingga akhir",
        category: "document",
        run: (p, core) => {
          const pages = document.querySelectorAll(".document-page");
          const texts = [];
          pages.forEach((p, idx) => {
            const body = p.querySelector(".doc-page-body");
            texts.push(`--- [Halaman ${idx + 1}] ---\n` + (body ? body.innerText : ""));
          });
          return { success: true, text: texts.join("\n\n") };
        }
      });

      this.register({
        id: "document.insert_text",
        aliases: ["dokumen.sisipkan_teks", "insert_text", "ketik_teks"],
        name: "Ketik / Sisipkan Teks",
        description: "Sisipkan teks pada posisi kursor saat ini",
        category: "document",
        run: (text, core) => {
          if (typeof text === "string") {
            document.execCommand("insertText", false, text);
            core.debouncedRepaginate(50);
            return { success: true, text: text };
          }
          return { success: false, error: "Argumen teks harus berupa string" };
        }
      });

      // ----------------------------------------------------
      // 10. WIDYACALC (SPREADSHEET & KALKULASI BERDAULAT)
      // ----------------------------------------------------
      this.register({
        id: "calc.select_cell",
        aliases: ["kalkulasi.pilih_sel", "select_cell"],
        name: "Pilih Sel Spreadsheet",
        description: "Pindahkan fokus sel aktif pada lembar kerja (contoh: B4, A1)",
        category: "calc",
        run: (payload, core) => {
          if (typeof WidyaCalc !== "undefined") {
            const coord = typeof payload === "string" ? payload : (payload && payload.cell ? payload.cell : null);
            if (coord) {
              WidyaCalc.selectCell(coord.toUpperCase());
              return { success: true, cell: coord.toUpperCase() };
            }
          }
          return { success: false, error: "WidyaCalc belum aktif atau koordinat tidak valid" };
        }
      });

      this.register({
        id: "calc.set_cell",
        aliases: ["kalkulasi.isi_sel", "set_cell"],
        name: "Isi Nilai atau Rumus Sel",
        description: "Tulis angka, teks, atau rumus (e.g. =SUM(B4:E4)) ke dalam sel tertentu",
        category: "calc",
        run: (payload, core) => {
          if (typeof WidyaCalc !== "undefined" && payload && payload.cell) {
            const coord = payload.cell.toUpperCase();
            const val = payload.value !== undefined ? String(payload.value) : "";
            const sheet = WidyaCalc.sheets[WidyaCalc.activeSheet];
            if (sheet) {
              sheet.cells[coord] = val;
              WidyaCalc.renderGrid();
              WidyaCalc.selectCell(coord);
              return { success: true, cell: coord, value: val, evaluated: WidyaCalc.getCellValue(coord) };
            }
          }
          return { success: false, error: "Payload sel tidak lengkap" };
        }
      });

      this.register({
        id: "calc.get_cell",
        aliases: ["kalkulasi.ambil_nilai", "get_cell"],
        name: "Ambil Nilai Sel",
        description: "Ambil nilai kalkulasi dan rumus mentah dari sel tertentu",
        category: "calc",
        run: (coord, core) => {
          if (typeof WidyaCalc !== "undefined" && typeof coord === "string") {
            const c = coord.toUpperCase();
            const sheet = WidyaCalc.sheets[WidyaCalc.activeSheet];
            return {
              success: true,
              cell: c,
              raw: sheet && sheet.cells ? sheet.cells[c] : undefined,
              value: WidyaCalc.getCellValue(c)
            };
          }
          return { success: false, error: "WidyaCalc belum aktif" };
        }
      });

      this.register({
        id: "calc.auto_sum",
        aliases: ["kalkulasi.hitung_total", "auto_sum"],
        name: "AutoSum Otomatis",
        description: "Hitung total cepat pada baris atau kolom sel yang aktif (Alt+=)",
        category: "calc",
        shortcut: "alt+=",
        run: (func, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.autoSum(typeof func === "string" ? func : "SUM");
            return { success: true, cell: WidyaCalc.selectedCell, value: WidyaCalc.getCellValue(WidyaCalc.selectedCell) };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.format_bold",
        aliases: ["kalkulasi.tebal", "calc_bold"],
        name: "Tebal Sel",
        description: "Beri gaya tebal pada sel spreadsheet terpilih",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.formatSelected("bold");
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.format_currency",
        aliases: ["kalkulasi.mata_uang", "calc_currency"],
        name: "Format Rupiah",
        description: "Format angka ke mata uang Rupiah (Rp)",
        category: "calc",
        shortcut: "alt+shift+4",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.formatSelected("currency");
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.format_percent",
        aliases: ["kalkulasi.persen", "calc_percent"],
        name: "Format Persentase",
        description: "Format angka ke bentuk persentase (%)",
        category: "calc",
        shortcut: "ctrl+shift+5",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.formatSelected("percentage");
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.insert_row",
        aliases: ["kalkulasi.sisipkan_baris", "insert_row"],
        name: "Sisipkan Baris",
        description: "Sisipkan baris baru di atas sel yang sedang aktif",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.insertRow();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.delete_row",
        aliases: ["kalkulasi.hapus_baris", "delete_row"],
        name: "Hapus Baris",
        description: "Hapus baris pada posisi sel yang sedang aktif",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.deleteRow();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.insert_col",
        aliases: ["kalkulasi.sisipkan_kolom", "insert_col"],
        name: "Sisipkan Kolom",
        description: "Sisipkan kolom baru di sebelah kiri sel aktif",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.insertCol();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.delete_col",
        aliases: ["kalkulasi.hapus_kolom", "delete_col"],
        name: "Hapus Kolom",
        description: "Hapus kolom pada posisi sel yang aktif",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.deleteCol();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.freeze_header",
        aliases: ["kalkulasi.bekukan_baris", "freeze_header"],
        name: "Bekukan Baris Header",
        description: "Kunci baris header tabel agar tetap terlihat saat menggulir",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.toggleFreezeHeader();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.freeze_col",
        aliases: ["kalkulasi.bekukan_kolom", "freeze_column"],
        name: "Bekukan Kolom A",
        description: "Kunci kolom pertama agar tetap terlihat saat menggulir ke samping",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.toggleFreezeFirstColumn();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.export_xlsx",
        aliases: ["kalkulasi.ekspor_xlsx", "export_xlsx"],
        name: "Ekspor ke OpenXML Excel (.xlsx)",
        description: "Simpan buku kerja aktif ke format biner standar .xlsx murni",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.exportXlsx();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.export_csv",
        aliases: ["kalkulasi.ekspor_csv", "export_csv"],
        name: "Ekspor ke CSV",
        description: "Unduh lembar kerja aktif ke format Comma Separated Values (.csv)",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            WidyaCalc.exportCsv();
            return { success: true };
          }
          return { success: false };
        }
      });

      this.register({
        id: "calc.generate_chart",
        aliases: ["kalkulasi.buat_grafik", "generate_chart"],
        name: "Buat Grafik Vektor SVG",
        description: "Hasilkan grafik visual interaktif (batang, garis, atau lingkaran) dari rentang sel",
        category: "calc",
        run: (payload, core) => {
          if (typeof WidyaCalc !== "undefined") {
            const p = payload || {};
            return WidyaCalc.generateSvgChart(p.type || "batang", p.title || "Grafik Data", p.label_range || "A4:A6", p.data_range || "F4:F6");
          }
          return { success: false, error: "WidyaCalc belum aktif" };
        }
      });

      this.register({
        id: "calc.compile_dsl",
        aliases: ["kalkulasi.kompilasi_dsl", "import_calc_dsl"],
        name: "Kompilasi Skrip Spreadsheet (.widya)",
        description: "Kompilasi kode deklaratif buku_kerja Widya-Lang menjadi tabel data interaktif",
        category: "calc",
        run: (payload, core) => {
          if (typeof WidyaCalc !== "undefined") {
            const code = typeof payload === "string" ? payload : (payload && payload.code ? payload.code : null);
            if (!code) return { success: false, error: "Kode skrip tidak boleh kosong" };
            return WidyaCalc.importDsl(code);
          }
          return { success: false, error: "WidyaCalc belum aktif" };
        }
      });

      this.register({
        id: "calc.export_dsl",
        aliases: ["kalkulasi.ekspor_dsl", "export_calc_dsl"],
        name: "Ekspor ke Widya Spreadsheet DSL",
        description: "Ekstrak status buku kerja ke sintaks deklaratif kode bahasa Widya-Lang",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            return WidyaCalc.exportDsl();
          }
          return { success: false, error: "WidyaCalc belum aktif" };
        }
      });

      this.register({
        id: "calc.inspect",
        aliases: ["kalkulasi.inspeksi", "inspect_calc"],
        name: "Inspeksi Buku Kerja WidyaCalc",
        description: "Dapatkan analisis metadata buku kerja (total lembar, jumlah sel, rumus, sel aktif)",
        category: "calc",
        run: (p, core) => {
          if (typeof WidyaCalc !== "undefined") {
            return { success: true, stats: WidyaCalc.getStats() };
          }
          return { success: false, error: "WidyaCalc belum aktif" };
        }
      });

      // ----------------------------------------------------
      // 11. WIDYAIMPRESS (Perancang Presentasi & Slide DeckCraft)
      // ----------------------------------------------------
      this.register({
        id: "impress.new_slide",
        aliases: ["presentasi.slide_baru", "new_slide"],
        name: "Slide Baru",
        description: "Tambahkan slide baru ke dalam dek presentasi",
        category: "impress",
        shortcut: "ctrl+m",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const layout = typeof payload === "string" ? payload : (payload && payload.layout ? payload.layout : "content");
            WidyaImpress.addNewSlide(layout);
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex, totalSlides: WidyaImpress.slides.length };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.duplicate_slide",
        aliases: ["presentasi.duplikasi_slide", "duplicate_slide"],
        name: "Duplikasi Slide",
        description: "Gandakan slide yang sedang aktif",
        category: "impress",
        shortcut: "ctrl+d",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.duplicateSlide();
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex, totalSlides: WidyaImpress.slides.length };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.delete_slide",
        aliases: ["presentasi.hapus_slide", "delete_slide"],
        name: "Hapus Slide",
        description: "Hapus slide yang sedang aktif dari dek",
        category: "impress",
        shortcut: "delete",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.deleteSlide();
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex, totalSlides: WidyaImpress.slides.length };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.select_slide",
        aliases: ["presentasi.pilih_slide", "select_slide"],
        name: "Pilih Slide",
        description: "Pindahkan tampilan fokus ke nomor slide tertentu (0-based atau 1-based)",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            let idx = 0;
            if (typeof payload === "number") idx = payload;
            else if (payload && typeof payload.index === "number") idx = payload.index;
            else if (payload && typeof payload.slide === "number") idx = payload.slide - 1;
            WidyaImpress.selectSlide(idx);
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.move_slide_up",
        aliases: ["presentasi.geser_atas", "move_slide_up"],
        name: "Geser Slide ke Atas",
        description: "Pindahkan urutan slide satu posisi lebih awal",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.moveSlide(-1);
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.move_slide_down",
        aliases: ["presentasi.geser_bawah", "move_slide_down"],
        name: "Geser Slide ke Bawah",
        description: "Pindahkan urutan slide satu posisi lebih akhir",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.moveSlide(1);
            return { success: true, activeSlide: WidyaImpress.activeSlideIndex };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.set_layout",
        aliases: ["presentasi.atur_tata_letak", "set_layout"],
        name: "Atur Tata Letak Slide",
        description: "Ubah layout slide aktif (title, content, kpi, two-column, blank)",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const layout = typeof payload === "string" ? payload : (payload && payload.layout ? payload.layout : "content");
            WidyaImpress.changeCurrentLayout(layout);
            return { success: true, layout: layout };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.set_theme",
        aliases: ["presentasi.atur_tema", "set_theme"],
        name: "Atur Tema Presentasi",
        description: "Terapkan palet warna visual (navy, dark, emerald, gold)",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            let t = typeof payload === "string" ? payload : (payload && payload.theme ? payload.theme : "navy");
            if (!t.startsWith("theme-")) {
              if (t === "navy") t = "theme-executive-navy";
              else if (t === "dark") t = "theme-dark-modern";
              else if (t === "emerald") t = "theme-emerald-fintech";
              else if (t === "gold") t = "theme-sovereign-gold";
              else t = `theme-${t}`;
            }
            WidyaImpress.applyTheme(t);
            return { success: true, theme: WidyaImpress.theme };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.set_aspect_ratio",
        aliases: ["presentasi.atur_rasio", "set_aspect_ratio"],
        name: "Atur Rasio Slide",
        description: "Ganti aspek rasio slide antara 16:9 (Layar Lebar) atau 4:3 (Klasik)",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const ratio = typeof payload === "string" ? payload : (payload && payload.ratio ? payload.ratio : "16:9");
            WidyaImpress.setAspectRatio(ratio);
            return { success: true, aspectRatio: WidyaImpress.aspectRatio };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.set_transition",
        aliases: ["presentasi.atur_transisi", "set_transition"],
        name: "Atur Efek Transisi",
        description: "Tentukan animasi perpindahan slide (fade, push, wipe, zoom, flip)",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const trans = typeof payload === "string" ? payload : (payload && payload.transition ? payload.transition : "push");
            WidyaImpress.setSlideTransition(trans);
            return { success: true, transition: trans };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.toggle_sorter",
        aliases: ["presentasi.tampilan_penyortir", "toggle_sorter"],
        name: "Beralih Tampilan Sorter",
        description: "Buka tampilan kisi Slide Sorter untuk mengatur tata urutan slide",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const newMode = WidyaImpress.viewMode === "sorter" ? "normal" : "sorter";
            WidyaImpress.toggleViewMode(newMode);
            return { success: true, viewMode: newMode };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.start_slideshow",
        aliases: ["presentasi.mulai_presentasi", "start_slideshow"],
        name: "Mulai Slide Show",
        description: "Luncurkan peragaan slide layar penuh mulai dari awal (F5)",
        category: "impress",
        shortcut: "f5",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.startFullScreenPresentation(true);
            return { success: true };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.start_slideshow_from_current",
        aliases: ["presentasi.mulai_dari_slide_aktif", "start_slideshow_from_current"],
        name: "Slide Show dari Slide Ini",
        description: "Luncurkan peragaan slide layar penuh mulai dari slide aktif (Shift+F5)",
        category: "impress",
        shortcut: "shift+f5",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.startFullScreenPresentation(false);
            return { success: true };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.toggle_presenter_notes",
        aliases: ["presentasi.tampilkan_catatan", "toggle_notes"],
        name: "Tampilkan/Sembunyikan Catatan",
        description: "Buka panel catatan pembicara untuk slide yang sedang diedit",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.toggleNotesDrawer();
            return { success: true };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.export_pptx",
        aliases: ["presentasi.ekspor_pptx", "export_pptx"],
        name: "Ekspor ke PowerPoint (.pptx)",
        description: "Simpan dek presentasi ke format biner standar Microsoft PowerPoint OpenXML",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.exportPptx();
            return { success: true };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.export_pdf",
        aliases: ["presentasi.ekspor_pdf", "export_pdf"],
        name: "Ekspor ke Handout PDF",
        description: "Ekspor seluruh slide ke dokumen PDF beresolusi tinggi untuk handout atau cetak",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            WidyaImpress.exportPdf();
            return { success: true };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.compile_dsl",
        aliases: ["presentasi.kompilasi_dsl", "import_impress_dsl"],
        name: "Kompilasi Skrip Presentasi (.widya)",
        description: "Muat dan susun slide presentasi dari sintaks deklaratif Widya-Lang",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const code = typeof payload === "string" ? payload : (payload && payload.code ? payload.code : null);
            if (!code) return { success: false, error: "Kode skrip tidak boleh kosong" };
            return WidyaImpress.importDsl(code);
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.import_markdown",
        aliases: ["presentasi.impor_markdown", "import_markdown_slides"],
        name: "Impor Presentasi Markdown",
        description: "Konversi berkas presentasi Markdown (Marp/Slidev) ke dalam dek slide WidyaImpress",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const md = typeof payload === "string" ? payload : (payload && payload.markdown ? payload.markdown : null);
            if (!md) return { success: false, error: "Teks Markdown tidak boleh kosong" };
            return WidyaImpress.importMarkdown(md);
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.export_dsl",
        aliases: ["presentasi.ekspor_dsl", "export_impress_dsl"],
        name: "Ekspor ke Widya Presentasi DSL",
        description: "Ekstrak seluruh slide ke format kode deklaratif bahasa Widya-Lang",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            return WidyaImpress.exportDsl();
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.render_svg",
        aliases: ["presentasi.render_svg", "render_slide_svg"],
        name: "Render Slide ke Vektor SVG",
        description: "Hasilkan grafik vektor SVG mandiri dari slide terpilih",
        category: "impress",
        run: (payload, core) => {
          if (typeof WidyaImpress !== "undefined") {
            const idx = typeof payload === "number" ? payload : (payload && typeof payload.slide_index === "number" ? payload.slide_index : WidyaImpress.activeSlideIndex);
            return WidyaImpress.renderSlideSvg(idx);
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      this.register({
        id: "impress.inspect",
        aliases: ["presentasi.inspeksi", "inspect_impress"],
        name: "Inspeksi Presentasi WidyaImpress",
        description: "Dapatkan analisis metadata dek presentasi (jumlah slide, tata letak aktif, tema, rasio)",
        category: "impress",
        run: (p, core) => {
          if (typeof WidyaImpress !== "undefined") {
            return { success: true, stats: WidyaImpress.getStats() };
          }
          return { success: false, error: "WidyaImpress belum aktif" };
        }
      });

      // =========================================================================
      // KATEGORI 12: WIDYAPDF - SOVEREIGN PDF WORKBENCH & REDACTION
      // =========================================================================
      this.register({
        id: "pdf.next_page",
        aliases: ["pdf.halaman_berikutnya", "next_page"],
        name: "Halaman Berikutnya",
        description: "Pindah ke halaman dokumen PDF selanjutnya",
        category: "pdf",
        shortcut: "pagedown",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.nextPage();
            return { success: true, currentPage: WidyaPDF.currentPage };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.prev_page",
        aliases: ["pdf.halaman_sebelumnya", "prev_page"],
        name: "Halaman Sebelumnya",
        description: "Pindah ke halaman dokumen PDF sebelumnya",
        category: "pdf",
        shortcut: "pageup",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.prevPage();
            return { success: true, currentPage: WidyaPDF.currentPage };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.go_to_page",
        aliases: ["pdf.ke_halaman", "goto_page"],
        name: "Buka Halaman Spesifik",
        description: "Lompat langsung ke nomor halaman yang dituju",
        category: "pdf",
        run: (payload, core) => {
          if (typeof WidyaPDF !== "undefined") {
            const pageNum = typeof payload === "number" ? payload : (payload && payload.page ? payload.page : 1);
            WidyaPDF.goToPage(pageNum);
            return { success: true, currentPage: WidyaPDF.currentPage };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.zoom_in",
        aliases: ["pdf.perbesar", "zoom_in"],
        name: "Perbesar Tampilan (Zoom In)",
        description: "Tingkatkan perbesaran tampilan dokumen PDF sebesar 25%",
        category: "pdf",
        shortcut: "ctrl+=",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.zoomIn();
            return { success: true, zoomLevel: WidyaPDF.zoomLevel };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.zoom_out",
        aliases: ["pdf.perkecil", "zoom_out"],
        name: "Perkecil Tampilan (Zoom Out)",
        description: "Turunkan perbesaran tampilan dokumen PDF sebesar 25%",
        category: "pdf",
        shortcut: "ctrl+-",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.zoomOut();
            return { success: true, zoomLevel: WidyaPDF.zoomLevel };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.zoom_fit_page",
        aliases: ["pdf.pas_halaman", "fit_page"],
        name: "Sesuaikan ke Satu Halaman",
        description: "Sesuaikan skala dokumen agar satu lembar penuh terlihat di layar",
        category: "pdf",
        shortcut: "ctrl+0",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.zoomFitPage();
            return { success: true, zoomLevel: WidyaPDF.zoomLevel };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.zoom_fit_width",
        aliases: ["pdf.pas_lebar", "fit_width"],
        name: "Sesuaikan ke Lebar Jendela",
        description: "Sesuaikan skala dokumen agar memenuhi lebar jendela pembaca",
        category: "pdf",
        shortcut: "ctrl+2",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.zoomFitWidth();
            return { success: true, zoomLevel: WidyaPDF.zoomLevel };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.zoom_actual_size",
        aliases: ["pdf.ukuran_asli", "actual_size"],
        name: "Ukuran Sebenarnya 100%",
        description: "Kembalikan tingkat zoom dokumen ke ukuran asli 100%",
        category: "pdf",
        shortcut: "ctrl+1",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.zoomActualSize();
            return { success: true, zoomLevel: WidyaPDF.zoomLevel };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.rotate",
        aliases: ["pdf.putar_halaman", "rotate_page"],
        name: "Putar Orientasi Halaman 90°",
        description: "Putar orientasi tampilan halaman 90 derajat searah jarum jam",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.rotate();
            return { success: true, rotationDeg: WidyaPDF.rotationDeg };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.toggle_two_up",
        aliases: ["pdf.mode_buku", "toggle_two_up"],
        name: "Beralih Mode Dua Halaman (Two-Up)",
        description: "Tampilkan dua halaman berdampingan seperti membaca buku fisik",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.toggleTwoUp();
            return { success: true, isTwoUp: WidyaPDF.isTwoUp };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.toggle_dark_mode",
        aliases: ["pdf.mode_malam", "toggle_dark_mode"],
        name: "Beralih Pembaca Gelap Kontras Tinggi",
        description: "Aktifkan atau nonaktifkan palet warna gelap ramah mata untuk membaca malam hari",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.toggleDarkMode();
            return { success: true, isDarkMode: WidyaPDF.isDarkMode };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.toggle_comments",
        aliases: ["pdf.panel_komentar", "toggle_comments"],
        name: "Tampilkan/Sembunyikan Panel Komentar",
        description: "Buka atau tutup bilah samping anotasi dan catatan dokumen",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.toggleCommentsSidebar();
            return { success: true, isCommentsOpen: WidyaPDF.isCommentsSidebarOpen };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.search",
        aliases: ["pdf.cari_teks", "search_pdf"],
        name: "Cari Teks dalam Dokumen",
        description: "Buka kotak pencarian teks dan navigasi temuan kata kunci",
        category: "pdf",
        shortcut: "ctrl+f",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.toggleSearchBar();
            return { success: true };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.add_sticky_note",
        aliases: ["pdf.tambah_catatan", "add_sticky_note"],
        name: "Bubuhi Catatan Tempel (Sticky Note)",
        description: "Aktifkan alat penusuk pin catatan tempel interaktif pada halaman dokumen",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.toggleStickyNoteTool();
            return { success: true, toolActive: WidyaPDF.stickyNoteToolActive };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.add_stamp",
        aliases: ["pdf.bubuhi_stempel", "add_rubber_stamp"],
        name: "Bubuhi Stempel Resmi",
        description: "Bubuhi stempel cap karet resmi (APPROVED, CONFIDENTIAL, VALID, FINAL)",
        category: "pdf",
        run: (payload, core) => {
          if (typeof WidyaPDF !== "undefined") {
            const stype = typeof payload === "string" ? payload : (payload && payload.type ? payload.type : "APPROVED");
            WidyaPDF.addRubberStamp(stype);
            return { success: true, type: stype };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.sign_pqc",
        aliases: ["pdf.tanda_tangan_pqc", "sign_pqc"],
        name: "Tanda Tangan Elektronik Pasca-Kuantum FIPS 204",
        description: "Bubuhkan stempel tanda tangan digital berdaulat berbasis NIST ML-DSA-87",
        category: "pdf",
        run: (payload, core) => {
          if (typeof WidyaPDF !== "undefined") {
            const signer = payload && payload.signer ? payload.signer : "Prof. Dr. Budi Santoso";
            const role = payload && payload.role ? payload.role : "Kepala Pusat Kriptografi BSSN";
            return WidyaPDF.signWithPqc(signer, role);
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.sensor_pdp",
        aliases: ["pdf.sensor_uu_pdp", "redact_pdp"],
        name: "Sensor Privasi Otomatis (UU PDP No. 27/2022)",
        description: "Pindai dan sensor otomatis NIK, no telepon, dan nomor rekening perbankan",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            return WidyaPDF.applyPdpRedaction();
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.save",
        aliases: ["pdf.simpan", "save_pdf"],
        name: "Simpan Perubahan Dokumen PDF",
        description: "Simpan seluruh halaman, coretan, stempel, dan tanda tangan ke berkas PDF",
        category: "pdf",
        shortcut: "ctrl+s",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            WidyaPDF.savePdf();
            return { success: true };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.compile_dsl",
        aliases: ["pdf.kompilasi_dsl", "import_pdf_dsl"],
        name: "Kompilasi WidyaPDF DSL",
        description: "Muat struktur halaman dokumen PDF dari kode deklaratif Widya-Lang",
        category: "pdf",
        run: (payload, core) => {
          if (typeof WidyaPDF !== "undefined") {
            const code = typeof payload === "string" ? payload : (payload && payload.code ? payload.code : null);
            if (!code) return { success: false, error: "Kode skrip tidak boleh kosong" };
            return WidyaPDF.importDsl(code);
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.export_dsl",
        aliases: ["pdf.ekspor_dsl", "export_pdf_dsl"],
        name: "Ekspor ke WidyaPDF DSL",
        description: "Ekspor seluruh halaman dokumen dan anotasi ke sintaks deklaratif Widya-Lang",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            return WidyaPDF.exportDsl();
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });

      this.register({
        id: "pdf.inspect",
        aliases: ["pdf.inspeksi", "inspect_pdf"],
        name: "Inspeksi Status Dokumen PDF",
        description: "Dapatkan analisis metadata dokumen PDF (jumlah halaman, zoom, rotasi, stempel, TTE)",
        category: "pdf",
        run: (p, core) => {
          if (typeof WidyaPDF !== "undefined") {
            return { success: true, stats: WidyaPDF.getStats() };
          }
          return { success: false, error: "WidyaPDF belum aktif" };
        }
      });
    }
  };

  // Ekspos ke global namespace
  global.OfficeCommands = OfficeCommands;
  global.executeOfficeCommand = function(id, payload) {
    return OfficeCommands.execute(id, payload);
  };
  global.listOfficeCommands = function(category) {
    return OfficeCommands.list(category);
  };

})(typeof window !== "undefined" ? window : globalThis);
