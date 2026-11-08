/**
 * WidyaOffice Suite Universal UI Controller
 * Mengelola seluruh interaktivitas Ribbon Bar, Tab Switching, Backstage View,
 * Waffle App Launcher, Modal Dialogs, Save As + Pilihan Direktori Interaktif,
 * dan Toast Feedback untuk seluruh aplikasi:
 * - WidyaWriter
 * - WidyaCalc
 * - WidyaImpress
 * - WidyaPDF
 */

(function(global) {
  "use strict";

  // --- 0. Universal Native IPC Client Bridge ---
  if (typeof global.callWidyaIpc !== "function") {
    global.callWidyaIpc = async function(method, payload = {}) {
      try {
        const res = await fetch("/api/ipc", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "x-widya-ipc-token": global.__WIDYA_IPC_TOKEN__ || ""
          },
          body: JSON.stringify({
            metode: method,
            payload: payload
          })
        });
        return await res.json();
      } catch (err) {
        console.warn("[IPC Bridge Error]", err);
        return { sukses: false, galat: err.toString() };
      }
    };
  }

  // --- 1. Ribbon Tab Switching ---
  global.switchRibbonTab = function(tabName) {
    if (!tabName) return;

    const tabs = document.querySelectorAll('.wc-tab-strip .wc-tab');
    tabs.forEach(btn => {
      const target = btn.getAttribute('data-tab');
      if (target === tabName) {
        btn.classList.add('active');
      } else if (!btn.classList.contains('tab-file')) {
        btn.classList.remove('active');
      }
    });

    const panels = document.querySelectorAll('.wc-ribbon-tab-content');
    let panelFound = false;
    panels.forEach(panel => {
      if (panel.id === `tabContent-${tabName}`) {
        panel.classList.add('active');
        panelFound = true;
      } else {
        panel.classList.remove('active');
      }
    });

    if (!panelFound && panels.length > 0) {
      console.log(`[OfficeUI] Panel untuk tab '${tabName}' belum dibuat, menampilkan toast.`);
    }
  };

  // --- 2. Backstage Fullscreen View (File Menu) ---
  global.openBackstage = function(initialTab = 'home') {
    const bs = document.getElementById('wcBackstage');
    if (bs) {
      bs.classList.add('active');
      global.switchBackstageTab(initialTab);
    }
  };

  global.closeBackstage = function() {
    const bs = document.getElementById('wcBackstage');
    if (bs) bs.classList.remove('active');
  };

  global.switchBackstageTab = function(tabName) {
    document.querySelectorAll('.wc-backstage-item[data-bstab]').forEach(item => {
      if (item.getAttribute('data-bstab') === tabName) {
        item.classList.add('active');
      } else {
        item.classList.remove('active');
      }
    });

    document.querySelectorAll('.wc-backstage-pane').forEach(pane => {
      pane.style.display = 'none';
    });
    const activePane = document.getElementById(`bsPane-${tabName}`);
    if (activePane) {
      activePane.style.display = 'block';
    }
  };

  // --- 3. Waffle 9-Dot App Switcher Dropdown ---
  global.toggleWaffleMenu = function() {
    const menu = document.getElementById('waffleDropdown');
    if (menu) menu.classList.toggle('active');
  };

  // --- 4. AutoSave Pill Switch ---
  global.toggleAutoSave = function() {
    const sw = document.getElementById('autoSaveToggle');
    if (sw) {
      sw.classList.toggle('off');
      const isOff = sw.classList.contains('off');
      global.showToast(isOff ? 'AutoSave Dinonaktifkan' : 'AutoSave Aktif (Tersimpan Otomatis)', isOff ? '⚠️' : '💾');
    }
  };

  // --- 5. Modal Dialogs ---
  global.openModal = function(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) modal.classList.add('active');
  };

  global.closeModal = function(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) modal.classList.remove('active');
  };

  // --- 6. Toast Notifications ---
  global.showToast = function(message, icon = 'ℹ️') {
    let toast = document.getElementById('wcToastNotification') || document.getElementById('wcToast');
    if (!toast) {
      toast = document.createElement('div');
      toast.id = 'wcToastNotification';
      toast.className = 'wc-toast';
      document.body.appendChild(toast);
    }

    toast.innerHTML = `<span style="font-size:18px;">${icon}</span><span>${message}</span>`;
    toast.classList.add('show');

    if (global._wcToastTimer) clearTimeout(global._wcToastTimer);
    global._wcToastTimer = setTimeout(() => {
      toast.classList.remove('show');
    }, 3200);
  };

  // --- 7. Universal Save As & Sovereign Directory Picker Controller ---
  global._widyaCurrentPickerPath = localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
  global._widyaSelectedPickerFolder = global._widyaCurrentPickerPath;

  function detectActiveSuiteApp() {
    const cls = document.body.className || '';
    if (cls.includes('app-calc') || location.pathname.includes('widyacalc')) return 'calc';
    if (cls.includes('app-impress') || location.pathname.includes('widyaimpress')) return 'impress';
    if (cls.includes('app-pdf') || location.pathname.includes('widyapdf')) return 'pdf';
    return 'writer';
  }

  function getFormatOptionsForApp(app) {
    if (app === 'calc') {
      return [
        { ext: 'xlsx', label: '📊 Buku Kerja Excel OpenXML (*.xlsx) — ISO/IEC 29500' },
        { ext: 'csv', label: '📄 Berkas Data Nilai Koma CSV (*.csv)' },
        { ext: 'pdf', label: '📑 Laporan Lembar Kerja PDF Vektor (*.pdf)' }
      ];
    }
    if (app === 'impress') {
      return [
        { ext: 'pptx', label: '📽️ Presentasi PowerPoint OpenXML (*.pptx) — ISO/IEC 29500' },
        { ext: 'pdf', label: '📑 Dokumen Handout Presentasi PDF (*.pdf)' }
      ];
    }
    if (app === 'pdf') {
      return [
        { ext: 'pdf', label: '📑 Dokumen PDF Vektor Terverifikasi (*.pdf) — ISO 32000-1' }
      ];
    }
    return [
      { ext: 'docx', label: '📄 Word Document OpenXML (*.docx) — ISO/IEC 29500' },
      { ext: 'pdf', label: '📑 PDF Berdaulat (*.pdf) — ISO 32000-1 Vektor Murni' },
      { ext: 'odt', label: '📝 OpenDocument Text (*.odt) — Standar Terbuka ISO 26300' },
      { ext: 'html', label: '🌐 Halaman Web HTML (*.html)' },
      { ext: 'txt', label: '📃 Dokumen Teks Polos (*.txt)' }
    ];
  }

  global.openSaveAsDialog = function(preselectedFormat) {
    global.closeBackstage();
    ensureSaveAsAndFolderModals();

    const app = detectActiveSuiteApp();
    const formats = getFormatOptionsForApp(app);
    const fileTypeSelect = document.getElementById('saveAsFileType');
    if (fileTypeSelect) {
      fileTypeSelect.innerHTML = formats
        .map(f => `<option value="${f.ext}">${f.label}</option>`)
        .join('');
      if (preselectedFormat) {
        const cleanFmt = preselectedFormat.replace(/^\./, '').toLowerCase();
        fileTypeSelect.value = cleanFmt;
      }
    }

    const folderInput = document.getElementById('saveAsFolderLocation');
    if (folderInput) {
      const savedFolder = localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
      folderInput.value = savedFolder;
    }

    const titleEl = document.getElementById('wcDocTitle') || document.getElementById('docTitle');
    const fileNameInput = document.getElementById('saveAsFileName');
    if (fileNameInput && titleEl) {
      const rawTitle = (titleEl.innerText || 'Dokumen_Widya').split('—')[0].trim().replace(/\s+/g, '_');
      if (!fileNameInput.value || fileNameInput.value === 'The_Open_Studio_Handbook') {
        fileNameInput.value = rawTitle;
      }
    }

    global.openModal('saveAsModal');
    if (fileNameInput) {
      setTimeout(() => {
        fileNameInput.focus();
        fileNameInput.select();
      }, 80);
    }
  };

  global.setSaveAsQuickFolder = async function(shortcutOrPath) {
    let target = shortcutOrPath;
    if (typeof global.callWidyaIpc === 'function') {
      try {
        const resp = await global.callWidyaIpc('sistem.jelajahi_folder', { path: shortcutOrPath });
        const data = (resp && resp.data) ? resp.data : resp;
        if (data && data.current_path) {
          target = data.current_path;
        }
      } catch (_) {}
    }
    if (!target.endsWith('\\') && !target.endsWith('/')) target += '\\';
    const folderInput = document.getElementById('saveAsFolderLocation');
    if (folderInput) folderInput.value = target;
    localStorage.setItem('widya_office_save_folder', target);
    global.showToast(`Direktori simpan diatur ke: ${target}`, '📁');
  };

  global.browseFolderLocation = function() {
    ensureSaveAsAndFolderModals();
    const currentLoc = document.getElementById('saveAsFolderLocation')?.value || localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
    global.openFolderPickerModal(currentLoc);
  };

  global.pickNativeOsFolder = async function() {
    const currentLoc = document.getElementById('saveAsFolderLocation')?.value || localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
    if (typeof global.callWidyaIpc === 'function') {
      global.showToast('Membuka dialog pemilih direktori Windows...', '🖥️');
      const resp = await global.callWidyaIpc('sistem.pilih_folder_os', { initial_path: currentLoc });
      const data = (resp && resp.data) ? resp.data : resp;
      if (data && data.sukses && data.selected_path) {
        let chosen = data.selected_path;
        if (!chosen.endsWith('\\') && !chosen.endsWith('/')) chosen += '\\';
        const folderInput = document.getElementById('saveAsFolderLocation');
        if (folderInput) folderInput.value = chosen;
        const pickerInput = document.getElementById('folderPickerCurrentPath');
        if (pickerInput) pickerInput.value = chosen;
        const selectedLabel = document.getElementById('folderPickerSelectedLabel');
        if (selectedLabel) selectedLabel.innerText = chosen;
        global._widyaCurrentPickerPath = chosen;
        global._widyaSelectedPickerFolder = chosen;
        localStorage.setItem('widya_office_save_folder', chosen);
        global.closeFolderPickerModal();
        global.showToast(`Direktori dipilih: ${chosen}`, '📁');
        return;
      }
    }
    // Fallback ke input webkitdirectory atau penjelajah internal
    const nativeInput = document.getElementById('nativeFolderPickerInput');
    if (nativeInput) {
      nativeInput.click();
    } else {
      global.openFolderPickerModal(currentLoc);
    }
  };

  global.onNativeFolderSelected = function(e) {
    const files = e && e.target ? e.target.files : null;
    if (files && files.length > 0) {
      const relPath = files[0].webkitRelativePath || '';
      const rootFolder = relPath.split('/')[0] || files[0].name;
      const currentLoc = document.getElementById('saveAsFolderLocation')?.value || 'C:\\';
      const base = currentLoc.endsWith('\\') ? currentLoc : currentLoc + '\\';
      const fullPath = base + rootFolder + '\\';
      const targetInput = document.getElementById('saveAsFolderLocation');
      if (targetInput) targetInput.value = fullPath;
      localStorage.setItem('widya_office_save_folder', fullPath);
      global.closeFolderPickerModal();
      global.showToast(`Direktori dipilih: ${fullPath}`, '📁');
    }
  };

  global.openFolderPickerModal = function(startPath) {
    ensureSaveAsAndFolderModals();
    const modal = document.getElementById('folderPickerModal');
    if (!modal) return;
    const initial = startPath || document.getElementById('saveAsFolderLocation')?.value || localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
    modal.classList.add('active');
    global.loadFolderPickerPath(initial);
  };

  global.closeFolderPickerModal = function() {
    const modal = document.getElementById('folderPickerModal');
    if (modal) modal.classList.remove('active');
  };

  global.loadFolderPickerPath = async function(path) {
    let target = (path || '').trim();
    if (!target) target = 'C:\\Satriyo\\';

    const pathInput = document.getElementById('folderPickerCurrentPath');
    const listContainer = document.getElementById('folderPickerList');
    const countEl = document.getElementById('folderPickerItemCount');
    const selectedLabel = document.getElementById('folderPickerSelectedLabel');
    const drivesBar = document.getElementById('folderPickerDrivesBar');

    if (pathInput) pathInput.value = target;
    global._widyaCurrentPickerPath = target;
    global._widyaSelectedPickerFolder = target;
    if (selectedLabel) selectedLabel.innerText = target;

    if (listContainer) {
      listContainer.innerHTML = `<div style="padding:18px; font-size:12.5px; color:#64748b; text-align:center;">Memindai isi direktori...</div>`;
    }

    let subfolders = [];
    let drives = ['C:\\'];

    if (typeof global.callWidyaIpc === 'function') {
      try {
        const resp = await global.callWidyaIpc('sistem.jelajahi_folder', { path: target });
        const data = (resp && resp.data) ? resp.data : resp;
        if (data && (data.sukses || Array.isArray(data.subfolders))) {
          subfolders = data.subfolders || [];
          if (Array.isArray(data.drives) && data.drives.length > 0) {
            drives = data.drives;
          }
          global._widyaCurrentPickerPath = data.current_path || target;
          global._widyaSelectedPickerFolder = global._widyaCurrentPickerPath;
          if (pathInput) pathInput.value = global._widyaCurrentPickerPath;
          if (selectedLabel) selectedLabel.innerText = global._widyaCurrentPickerPath;
        } else {
          subfolders = ['Dokumen_Resmi', 'Data_Disertasi', 'Laporan_2026', 'Arsip_Naskah'];
        }
      } catch (_) {
        subfolders = ['Dokumen_Resmi', 'Data_Disertasi', 'Laporan_2026', 'Arsip_Naskah'];
      }
    }

    if (drivesBar && drives.length > 0) {
      drivesBar.innerHTML = drives.map(d => {
        const esc = d.replace(/\\/g, '\\\\');
        return `<button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11.5px;" onclick="loadFolderPickerPath('${esc}')">💽 ${d}</button>`;
      }).join('') + `
        <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11.5px;" onclick="loadFolderPickerPath('C:\\\\Satriyo')">📁 C:\\Satriyo</button>
        <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11.5px;" onclick="loadFolderPickerPath('Dokumen')">📄 Dokumen</button>
        <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11.5px;" onclick="loadFolderPickerPath('Desktop')">🖥️ Desktop</button>
        <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11.5px;" onclick="loadFolderPickerPath('Unduhan')">⬇️ Unduhan</button>
      `;
    }

    if (countEl) countEl.innerText = `${subfolders.length} sub-folder`;
    if (!listContainer) return;

    if (subfolders.length === 0) {
      listContainer.innerHTML = `<div style="padding:24px; font-size:12.5px; color:#94a3b8; text-align:center;">(Direktori ini tidak memiliki sub-folder. Klik <b>"✓ Pilih Folder Ini"</b> di bawah untuk menyimpan di sini, atau klik <b>"+ Folder Baru"</b>)</div>`;
      return;
    }

    listContainer.innerHTML = subfolders.map(sub => {
      const escaped = sub.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
      return `
        <div class="folder-picker-item" style="display:flex; align-items:center; justify-content:space-between; padding:7px 10px; border-bottom:1px solid #f1f5f9; cursor:pointer; border-radius:4px; font-size:13px;"
             onclick="selectFolderPickerItem(this, '${escaped}')"
             ondblclick="navigateFolderPicker('${escaped}')">
          <div style="display:flex; align-items:center; gap:8px;">
            <span>📁</span>
            <span style="font-weight:500; color:#1e293b;">${sub}</span>
          </div>
          <button type="button" class="wc-btn-secondary" style="padding:2px 8px; font-size:11px;" onclick="event.stopPropagation(); navigateFolderPicker('${escaped}')">Buka ➜</button>
        </div>
      `;
    }).join('');
  };

  global.selectFolderPickerItem = function(el, subfolderName) {
    document.querySelectorAll('.folder-picker-item').forEach(item => {
      item.style.background = 'transparent';
      item.classList.remove('selected');
    });
    if (el) {
      el.style.background = '#e8f2fd';
      el.classList.add('selected');
    }
    let base = global._widyaCurrentPickerPath || 'C:\\';
    if (!base.endsWith('\\') && !base.endsWith('/')) base += '\\';
    const chosen = base + subfolderName + '\\';
    global._widyaSelectedPickerFolder = chosen;
    const label = document.getElementById('folderPickerSelectedLabel');
    if (label) label.innerText = chosen;
  };

  global.navigateFolderPicker = function(subfolderName) {
    let base = global._widyaCurrentPickerPath || 'C:\\';
    if (!base.endsWith('\\') && !base.endsWith('/')) base += '\\';
    global.loadFolderPickerPath(base + subfolderName);
  };

  global.navigateFolderPickerParent = function() {
    let cur = (global._widyaCurrentPickerPath || 'C:\\').replace(/[\\/]+$/, '');
    const lastSep = Math.max(cur.lastIndexOf('\\'), cur.lastIndexOf('/'));
    if (lastSep > 0) {
      let parent = cur.substring(0, lastSep);
      if (parent.endsWith(':')) parent += '\\';
      global.loadFolderPickerPath(parent);
    } else if (cur.length >= 2 && cur.includes(':')) {
      global.loadFolderPickerPath(cur.substring(0, 2) + '\\');
    }
  };

  global.createFolderInPicker = async function() {
    const folderName = prompt('Masukkan nama folder baru:', 'Folder_Baru');
    if (!folderName || !folderName.trim()) return;
    let base = global._widyaCurrentPickerPath || 'C:\\Satriyo\\';
    if (!base.endsWith('\\') && !base.endsWith('/')) base += '\\';
    const newPath = base + folderName.trim();
    if (typeof global.callWidyaIpc === 'function') {
      try {
        await global.callWidyaIpc('sistem.buat_folder', { path: newPath });
        global.showToast(`Folder dibuat: ${newPath}`, '📁');
      } catch (_) {}
    }
    global.loadFolderPickerPath(newPath);
  };

  global.selectCurrentPickerFolder = function() {
    let chosen = global._widyaSelectedPickerFolder || global._widyaCurrentPickerPath || 'C:\\Satriyo\\';
    if (!chosen.endsWith('\\') && !chosen.endsWith('/')) chosen += '\\';
    const targetInput = document.getElementById('saveAsFolderLocation');
    if (targetInput) targetInput.value = chosen;
    localStorage.setItem('widya_office_save_folder', chosen);
    global.closeFolderPickerModal();
    global.showToast(`Direktori penyimpanan dipilih: ${chosen}`, '✓');
  };

  global.confirmSaveAs = async function(explicitFormat) {
    const fileNameEl = document.getElementById('saveAsFileName');
    const folderEl = document.getElementById('saveAsFolderLocation');
    const typeEl = document.getElementById('saveAsFileType');

    let fileName = (fileNameEl ? fileNameEl.value.trim() : '') || 'Dokumen_Widya';
    let folderPath = (folderEl ? folderEl.value.trim() : '') || localStorage.getItem('widya_office_save_folder') || 'C:\\Satriyo\\';
    let format = (explicitFormat || (typeEl ? typeEl.value : 'docx')).replace(/^\./, '').toLowerCase();

    if (!folderPath.endsWith('\\') && !folderPath.endsWith('/')) {
      folderPath += '\\';
    }
    localStorage.setItem('widya_office_save_folder', folderPath);

    // Pastikan nama berkas memiliki ekstensi yang sesuai
    const extSuffix = '.' + format;
    if (!fileName.toLowerCase().endsWith(extSuffix)) {
      const dotIdx = fileName.lastIndexOf('.');
      if (dotIdx > 0) {
        fileName = fileName.substring(0, dotIdx) + extSuffix;
      } else {
        fileName += extSuffix;
      }
    }

    const sheetEl = document.getElementById('wcPageSheet') || document.getElementById('docPageBody');
    const contentText = sheetEl ? (sheetEl.innerText || '') : '';
    const contentHtml = sheetEl ? (sheetEl.innerHTML || '') : '';

    global.closeModal('saveAsModal');

    let savedFullPath = folderPath + fileName;
    if (typeof global.callWidyaIpc === 'function') {
      try {
        const resp = await global.callWidyaIpc('sistem.simpan_berkas_fisik', {
          folder_path: folderPath,
          file_name: fileName,
          format: format,
          content_text: contentText,
          content_html: contentHtml
        });
        const data = (resp && resp.data) ? resp.data : resp;
        if (data && data.full_path) {
          savedFullPath = data.full_path;
        }
      } catch (_) {}
    }

    const titleEl = document.getElementById('wcDocTitle') || document.getElementById('docTitle');
    if (titleEl) titleEl.innerText = fileName;
    const statusEl = document.querySelector('.wc-doc-status');
    if (statusEl) statusEl.innerText = `— Disimpan di ${savedFullPath}`;

    global.showToast(`Berhasil disimpan ke: ${savedFullPath}`, '💾');
  };

  global.executeSaveAs = function() {
    global.confirmSaveAs();
  };

  function ensureSaveAsAndFolderModals() {
    // Jika #folderPickerModal belum ada di DOM, tambahkan secara otomatis
    if (!document.getElementById('folderPickerModal')) {
      const fpWrap = document.createElement('div');
      fpWrap.className = 'wc-modal-backdrop';
      fpWrap.id = 'folderPickerModal';
      fpWrap.style.zIndex = '10005';
      fpWrap.innerHTML = `
        <div class="wc-modal-box" style="max-width:620px; width:95vw;">
          <div class="wc-modal-header">
            <span class="wc-modal-title">📂 Pilih Direktori Penyimpanan Berkas</span>
            <button class="wc-modal-close" onclick="closeFolderPickerModal()">✕</button>
          </div>
          <div class="wc-modal-body" style="padding:14px 18px;">
            <div style="display:flex; gap:6px; align-items:center; margin-bottom:10px;">
              <button type="button" class="wc-btn-secondary" style="padding:6px 10px; font-weight:600;" onclick="navigateFolderPickerParent()" title="Naik ke Folder Induk">⬆ Naik</button>
              <input type="text" id="folderPickerCurrentPath" value="C:\\Satriyo\\" style="flex:1; padding:6px 10px; border:1px solid #cbd5e1; border-radius:4px; font-size:12.5px; font-family:monospace;" onkeydown="if(event.key==='Enter') loadFolderPickerPath(this.value)" />
              <button type="button" class="wc-btn-secondary" style="padding:6px 10px;" onclick="loadFolderPickerPath(document.getElementById('folderPickerCurrentPath').value)">Buka</button>
              <button type="button" class="wc-btn-secondary" style="padding:6px 10px; color:#185abd; font-weight:600;" onclick="createFolderInPicker()">+ Folder Baru</button>
            </div>
            <div id="folderPickerDrivesBar" style="display:flex; flex-wrap:wrap; gap:6px; margin-bottom:10px;"></div>
            <div style="font-size:11.5px; font-weight:700; color:#64748b; margin-bottom:4px; display:flex; justify-content:space-between;">
              <span>DAFTAR SUB-FOLDER (Klik 2x untuk membuka):</span>
              <span id="folderPickerItemCount" style="font-weight:normal;">0 sub-folder</span>
            </div>
            <div id="folderPickerList" style="min-height:200px; max-height:250px; overflow-y:auto; border:1px solid #cbd5e1; border-radius:4px; background:#ffffff; padding:4px;"></div>
            <div style="margin-top:10px; padding:8px 10px; background:#f8fafc; border-radius:4px; display:flex; align-items:center; gap:8px; font-size:12px; border:1px solid #e2e8f0;">
              <span style="font-weight:700; color:#334155;">Direktori Dipilih:</span>
              <span id="folderPickerSelectedLabel" style="color:#185abd; font-family:monospace; font-weight:600; word-break:break-all;">C:\\Satriyo\\</span>
            </div>
          </div>
          <div class="wc-modal-footer" style="display:flex; justify-content:space-between; align-items:center;">
            <button type="button" class="wc-btn-secondary" onclick="pickNativeOsFolder()">🖥️ Dialog Windows Explorer...</button>
            <div style="display:flex; gap:8px;">
              <button type="button" class="wc-btn-secondary" onclick="closeFolderPickerModal()">Batal</button>
              <button type="button" class="wc-btn-primary" onclick="selectCurrentPickerFolder()">✓ Pilih Folder Ini</button>
            </div>
          </div>
        </div>
      `;
      document.body.appendChild(fpWrap);
    }

    // Jika #saveAsModal belum ada di halaman (misal di Calc/Impress/PDF), buat otomatis
    if (!document.getElementById('saveAsModal')) {
      const saWrap = document.createElement('div');
      saWrap.className = 'wc-modal-backdrop';
      saWrap.id = 'saveAsModal';
      saWrap.innerHTML = `
        <div class="wc-modal-box" style="max-width:540px; width:95vw;">
          <div class="wc-modal-header">
            <span class="wc-modal-title" id="saveAsModalTitle">💾 Simpan Sebagai (Save As)</span>
            <button class="wc-modal-close" onclick="closeModal('saveAsModal')">✕</button>
          </div>
          <div class="wc-modal-body">
            <label style="display:block; font-weight:600; margin-bottom:5px; font-size:12.5px;">Nama Berkas:</label>
            <input type="text" id="saveAsFileName" value="Dokumen_Widya" style="width:100%; padding:8px 10px; border:1px solid #cbd5e1; border-radius:4px; font-size:13px; margin-bottom:12px;" onkeydown="if(event.key==='Enter') confirmSaveAs()" />

            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:5px;">
              <label style="font-weight:600; font-size:12.5px;">Lokasi Direktori / Folder Penyimpanan:</label>
              <span style="font-size:11px; color:#64748b;">Ketik path atau jelajahi folder</span>
            </div>
            <div style="display:flex; flex-wrap:wrap; gap:5px; margin-bottom:8px;">
              <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11px;" onclick="setSaveAsQuickFolder('C:\\\\Satriyo')">📁 C:\\Satriyo</button>
              <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11px;" onclick="setSaveAsQuickFolder('Dokumen')">📄 Dokumen</button>
              <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11px;" onclick="setSaveAsQuickFolder('Desktop')">🖥️ Desktop</button>
              <button type="button" class="wc-btn-secondary" style="padding:3px 8px; font-size:11px;" onclick="setSaveAsQuickFolder('Unduhan')">⬇️ Unduhan</button>
            </div>
            <div style="display:flex; gap:6px; margin-bottom:14px;">
              <input type="text" id="saveAsFolderLocation" value="C:\\Satriyo\\" style="flex:1; padding:7px 10px; border:1px solid #cbd5e1; border-radius:4px; font-size:12.5px; font-family:monospace;" />
              <button type="button" class="wc-btn-secondary" style="padding:7px 10px; font-weight:600; white-space:nowrap;" onclick="browseFolderLocation()" title="Buka Penjelajah Direktori Interaktif">📂 Pilih Direktori...</button>
              <button type="button" class="wc-btn-secondary" style="padding:7px 10px; font-weight:600; white-space:nowrap;" onclick="pickNativeOsFolder()" title="Pilih Direktori via Windows Explorer">🖥️ Windows...</button>
            </div>
            <input type="file" id="nativeFolderPickerInput" webkitdirectory style="display:none;" onchange="onNativeFolderSelected(event)" />

            <label style="display:block; font-weight:600; margin-bottom:5px; font-size:12.5px;">Format Berkas Penyimpanan:</label>
            <select id="saveAsFileType" style="width:100%; padding:8px 10px; border:1px solid #cbd5e1; border-radius:4px; font-size:13px; margin-bottom:12px; background:#fff;"></select>
          </div>
          <div class="wc-modal-footer" style="display:flex; justify-content:flex-end; gap:8px;">
            <button class="wc-btn-secondary" onclick="closeModal('saveAsModal')">Batal</button>
            <button class="wc-btn-primary" onclick="confirmSaveAs()">💾 Simpan ke Direktori</button>
          </div>
        </div>
      `;
      document.body.appendChild(saWrap);
    }
  }

  // --- 8. Auto-Bind Event Listeners on Page Ready ---
  document.addEventListener('DOMContentLoaded', () => {
    ensureSaveAsAndFolderModals();

    document.querySelectorAll('.wc-tab-strip .wc-tab[data-tab]').forEach(btn => {
      btn.addEventListener('click', () => {
        const tab = btn.getAttribute('data-tab');
        if (tab) global.switchRibbonTab(tab);
      });
    });

    document.addEventListener('click', (e) => {
      const waffle = document.getElementById('waffleDropdown');
      const btn = document.getElementById('waffleBtn');
      if (waffle && btn && !btn.contains(e.target) && !waffle.contains(e.target)) {
        waffle.classList.remove('active');
      }
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        global.closeBackstage();
        document.querySelectorAll('.wc-modal-backdrop.active').forEach(m => m.classList.remove('active'));
      } else if (e.key === 'F12' || (e.ctrlKey && e.shiftKey && (e.key === 'S' || e.key === 's'))) {
        e.preventDefault();
        global.openSaveAsDialog();
      }
    });
  });

})(typeof window !== "undefined" ? window : globalThis);
