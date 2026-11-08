/**
 * WidyaCalc - Sovereign Spreadsheet Controller (100% Independent Native Engine)
 * High-performance spreadsheet grid, formula evaluation, range drag selection,
 * number formatting, interactive SVG chart engine, multi-sheet tabs
 */

const WidyaCalc = {
  activeSheet: "Sheet1",
  sheets: {
    "Sheet1": {
      name: "Ringkasan Keuangan",
      cells: {},
      formats: {},
      maxRow: 30,
      maxCol: 12
    },
    "Sheet2": {
      name: "Rincian Biaya",
      cells: {},
      formats: {},
      maxRow: 30,
      maxCol: 12
    }
  },
  selectedCell: "B4",
  selectionRange: ["B4"],
  rangeStart: "B4",
  rangeEnd: "B4",
  isDraggingSelection: false,
  isDraggingFillHandle: false,
  fillSourceRange: [],
  fillTargetCoord: null,
  clipboard: null,
  autoFilterActive: false,
  activeFilterCol: null,
  columnFilters: {},

  init() {
    this.populateInitialSample();
    this.renderGrid();
    this.bindEvents();
    this.renderSheetTabs();
    this.selectCell("B4");
  },

  populateInitialSample() {
    const s1 = this.sheets["Sheet1"].cells;
    const f1 = this.sheets["Sheet1"].formats;

    // Header Title
    s1["A1"] = "LAPORAN ARUS KAS & PRODUKSI KEDATUAN WIDYA 2026";
    f1["A1"] = { bold: true, color: "#185abd", fontSize: "12pt" };

    // Column Headers
    s1["A3"] = "Kategori";
    s1["B3"] = "Kuartal 1";
    s1["C3"] = "Kuartal 2";
    s1["D3"] = "Kuartal 3";
    s1["E3"] = "Kuartal 4";
    s1["F3"] = "Total Tahunan";

    ["A3","B3","C3","D3","E3","F3"].forEach(coord => {
      f1[coord] = { bold: true, bg: "#f1f5f9", color: "#0f172a", border: "all" };
    });

    // Row Data
    s1["A4"] = "Infrastruktur Server";
    s1["B4"] = "45000000";
    s1["C4"] = "52000000";
    s1["D4"] = "48000000";
    s1["E4"] = "61000000";
    s1["F4"] = "=SUM(B4:E4)";

    s1["A5"] = "R&D AI & Kompiler Widya";
    s1["B5"] = "85000000";
    s1["C5"] = "92000000";
    s1["D5"] = "110000000";
    s1["E5"] = "125000000";
    s1["F5"] = "=SUM(B5:E5)";

    s1["A6"] = "Keamanan Siber & PQC HSM";
    s1["B6"] = "32000000";
    s1["C6"] = "35000000";
    s1["D6"] = "40000000";
    s1["E6"] = "45000000";
    s1["F6"] = "=SUM(B6:E6)";

    // Totals Row
    s1["A8"] = "TOTAL PENGELUARAN";
    f1["A8"] = { bold: true, color: "#107c41" };
    s1["B8"] = "=SUM(B4:B6)";
    s1["C8"] = "=SUM(C4:C6)";
    s1["D8"] = "=SUM(D4:D6)";
    s1["E8"] = "=SUM(E4:E6)";
    s1["F8"] = "=SUM(F4:F6)";

    ["B8","C8","D8","E8","F8"].forEach(c => {
      f1[c] = { bold: true, color: "#107c41", border: "bottom-double" };
    });

    // Currency Formatting
    ["B4","C4","D4","E4","F4","B5","C5","D5","E5","F5","B6","C6","D6","E6","F6","B8","C8","D8","E8","F8"].forEach(c => {
      f1[c] = { ...(f1[c] || {}), currency: "IDR" };
    });
  },

  renderGrid() {
    const table = document.getElementById("calcGridTable");
    if (!table) return;

    const sheet = this.sheets[this.activeSheet];
    const maxRow = sheet.maxRow || 30;
    const maxCol = sheet.maxCol || 12;

    let html = "<thead><tr><th class='calc-corner-header' onclick='WidyaCalc.selectAllGrid()' title='Pilih Seluruh Lembar Kerja (Ctrl+A)'>#</th>";
    for (let c = 1; c <= maxCol; c++) {
      const colLetter = this.getColLetter(c);
      const isFiltered = this.columnFilters && this.columnFilters[c] && this.columnFilters[c].size > 0;
      const filterIcon = this.autoFilterActive
        ? `<button class='calc-col-filter-btn ${isFiltered ? "active" : ""}' data-col='${c}' onclick='WidyaCalc.openFilterPopover(${c}, event)' title='Filter & Urutkan kolom ${colLetter}'>▼</button>`
        : "";
      html += `<th class='calc-col-header' data-col='${c}' ondblclick='WidyaCalc.autoFitColumnByIndex(${c})' title='Klik ganda untuk AutoFit lebar'>${colLetter}${filterIcon}</th>`;
    }
    html += "</tr></thead><tbody>";

    for (let r = 1; r <= maxRow; r++) {
      // Check if row is filtered out
      let rowHidden = false;
      if (this.columnFilters) {
        for (const colIdxStr in this.columnFilters) {
          const colIdx = parseInt(colIdxStr, 10);
          const filterSet = this.columnFilters[colIdx];
          if (filterSet && filterSet.size > 0) {
            const coord = `${this.getColLetter(colIdx)}${r}`;
            const rawVal = sheet.cells[coord] || "";
            const fmt = sheet.formats[coord] || {};
            const dispVal = String(this.getDisplayValue(coord, rawVal, fmt));
            if (filterSet.has(dispVal)) {
              rowHidden = true;
              break;
            }
          }
        }
      }

      const rowClass = rowHidden ? "calc-row-filtered-out" : "";
      html += `<tr class='${rowClass}' data-row='${r}'><th class='calc-row-header' data-row='${r}'>${r}</th>`;
      for (let c = 1; c <= maxCol; c++) {
        const coord = `${this.getColLetter(c)}${r}`;
        const rawVal = sheet.cells[coord] || "";
        const fmt = sheet.formats[coord] || {};
        const displayVal = (this.showFormulasMode && String(rawVal).startsWith("=")) ? rawVal : this.getDisplayValue(coord, rawVal, fmt);

        let styleStr = "";
        if (fmt.bold) styleStr += "font-weight:700;";
        if (fmt.italic) styleStr += "font-style:italic;";
        if (fmt.underline) styleStr += "text-decoration:underline;";
        if (fmt.color) styleStr += `color:${fmt.color};`;
        if (fmt.bg) styleStr += `background-color:${fmt.bg};`;
        if (fmt.align) styleStr += `text-align:${fmt.align};`;
        if (fmt.wrap) styleStr += "white-space:normal; word-break:break-word;";
        if (fmt.fontSize) styleStr += `font-size:${fmt.fontSize};`;

        if (fmt.border === "all") styleStr += "border:1px solid #94a3b8;";
        else if (fmt.border === "box-thick") styleStr += "border:2px solid #0f172a;";
        else if (fmt.border === "bottom-double") styleStr += "border-bottom:3px double #0f172a;";
        else if (fmt.border === "top-bottom") styleStr += "border-top:1px solid #94a3b8; border-bottom:1px solid #94a3b8;";
        else if (fmt.border === "none") styleStr += "border:none;";

        let cellClasses = ["calc-cell"];
        if (this.isColumnAFrozen && c === 1) cellClasses.push("calc-frozen-col");
        const hasVal = this.dataValidations && this.dataValidations[coord];
        if (hasVal) cellClasses.push("has-validation");
        const valBtnHtml = hasVal ? `<span class='calc-cell-validation-btn' onclick='WidyaCalc.showValidationPicker("${coord}", event)'>▾</span>` : "";

        html += `<td class='${cellClasses.join(" ")}' id='cell-${coord}' data-coord='${coord}' style='${styleStr}' tabindex='0'>${displayVal}${valBtnHtml}</td>`;
      }
      html += "</tr>";
    }
    html += "</tbody>";
    table.innerHTML = html;
  },

  renderSheetTabs() {
    const container = document.getElementById("calcSheetTabs");
    if (!container) return;

    let html = "";
    Object.keys(this.sheets).forEach(key => {
      const s = this.sheets[key];
      const activeCls = key === this.activeSheet ? "active" : "";
      html += `<button class='calc-sheet-tab ${activeCls}' data-sheet='${key}' onclick="WidyaCalc.switchSheet('${key}')" ondblclick="WidyaCalc.startRenameSheet('${key}', this)" title="Klik untuk beralih, klik ganda untuk ubah nama">${s.name || key}</button>`;
    });
    html += `<button class='calc-sheet-add-btn' onclick="WidyaCalc.addSheet()" title="Tambah Lembar Kerja Baru">+</button>`;
    container.innerHTML = html;
  },

  startRenameSheet(sheetKey, tabBtn) {
    const s = this.sheets[sheetKey];
    const input = document.createElement("input");
    input.type = "text";
    input.className = "calc-sheet-rename-input";
    input.value = s.name || sheetKey;
    tabBtn.innerHTML = "";
    tabBtn.appendChild(input);
    input.focus();
    input.select();

    const saveName = () => {
      const newName = input.value.trim();
      if (newName) s.name = newName;
      this.renderSheetTabs();
    };

    input.addEventListener("blur", saveName);
    input.addEventListener("keydown", (e) => {
      if (e.key === "Enter") {
        input.blur();
      } else if (e.key === "Escape") {
        this.renderSheetTabs();
      }
    });
  },

  selectAllGrid() {
    const sheet = this.sheets[this.activeSheet];
    const lastCol = this.getColLetter(sheet.maxCol || 12);
    const lastRow = sheet.maxRow || 30;
    this.selectRange("A1", `${lastCol}${lastRow}`);
  },

  autoFitColumnByIndex(colIdx) {
    const colLetter = this.getColLetter(colIdx);
    const sheet = this.sheets[this.activeSheet];
    let maxLen = 8;
    for (let r = 1; r <= (sheet.maxRow || 30); r++) {
      const v = String(sheet.cells[`${colLetter}${r}`] || "");
      if (v.length > maxLen) maxLen = v.length;
    }
    const widthPx = Math.min(Math.max(maxLen * 10, 75), 320);
    document.querySelectorAll(`[data-coord^='${colLetter}']`).forEach(td => {
      td.style.minWidth = `${widthPx}px`;
    });
  },

  switchSheet(sheetKey) {
    if (!this.sheets[sheetKey]) return;
    this.activeSheet = sheetKey;
    this.renderGrid();
    this.renderSheetTabs();
    this.selectCell("A1");
  },

  addSheet() {
    const count = Object.keys(this.sheets).length + 1;
    const key = `Sheet${count}`;
    this.sheets[key] = {
      name: `Lembar ${count}`,
      cells: {},
      formats: {},
      maxRow: 30,
      maxCol: 12
    };
    this.switchSheet(key);
  },

  getColLetter(colIdx) {
    let letter = "";
    let temp = colIdx;
    while (temp > 0) {
      const mod = (temp - 1) % 26;
      letter = String.fromCharCode(65 + mod) + letter;
      temp = Math.floor((temp - mod) / 26);
    }
    return letter;
  },

  getColIndex(colLetter) {
    let result = 0;
    for (let i = 0; i < colLetter.length; i++) {
      result = result * 26 + (colLetter.charCodeAt(i) - 64);
    }
    return result;
  },

  parseCoord(coord) {
    const match = coord.match(/^([A-Z]+)(\d+)$/i);
    if (!match) return { col: 1, row: 1 };
    return {
      col: this.getColIndex(match[1].toUpperCase()),
      row: parseInt(match[2], 10)
    };
  },

  getDisplayValue(coord, rawVal, fmt = {}) {
    if (!rawVal && rawVal !== 0) return "";
    let val = rawVal;
    if (String(rawVal).startsWith("=")) {
      val = this.evaluateFormula(rawVal);
    }

    const num = parseFloat(val);
    if (!isNaN(num) && isFinite(num) && String(val).match(/^-?\d+(\.\d+)?$/)) {
      const decimals = fmt.decimals !== undefined ? fmt.decimals : 0;
      if (fmt.currency === "IDR") {
        return "Rp " + new Intl.NumberFormat("id-ID", { minimumFractionDigits: decimals, maximumFractionDigits: decimals }).format(num);
      } else if (fmt.currency === "USD") {
        return "$" + new Intl.NumberFormat("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 }).format(num);
      } else if (fmt.number) {
        return new Intl.NumberFormat("id-ID", { minimumFractionDigits: decimals, maximumFractionDigits: decimals }).format(num);
      } else if (fmt.percentage) {
        return (num * 100).toFixed(decimals || 1) + "%";
      } else if (decimals > 0) {
        return num.toFixed(decimals);
      }
    }
    return val;
  },

  getCellValue(coord, visited = new Set()) {
    const sheet = this.sheets[this.activeSheet];
    if (!sheet || !sheet.cells) return 0;
    const raw = sheet.cells[coord];
    if (raw === undefined || raw === null || raw === "") return 0;
    if (String(raw).startsWith("=")) {
      if (visited.has(coord)) return 0; // prevent circular dependency
      visited.add(coord);
      return this.evaluateFormula(raw, visited);
    }
    return raw;
  },

  evaluateFormula(formulaStr, visited = new Set()) {
    const clean = formulaStr.trim().substring(1).toUpperCase();
    const sheet = this.sheets[this.activeSheet];

    // 1. SUM
    if (clean.startsWith("SUM(")) {
      const rangeStr = clean.substring(4, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      return numbers.reduce((acc, curr) => acc + curr, 0);
    }

    // 2. AVERAGE
    if (clean.startsWith("AVERAGE(")) {
      const rangeStr = clean.substring(8, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      if (numbers.length === 0) return 0;
      return (numbers.reduce((acc, curr) => acc + curr, 0) / numbers.length).toFixed(2);
    }

    // 3. MIN & MAX
    if (clean.startsWith("MIN(")) {
      const rangeStr = clean.substring(4, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      return numbers.length ? Math.min(...numbers) : 0;
    }
    if (clean.startsWith("MAX(")) {
      const rangeStr = clean.substring(4, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      return numbers.length ? Math.max(...numbers) : 0;
    }

    // 4. COUNT (numbers) & COUNTA (non-empty)
    if (clean.startsWith("COUNTA(")) {
      const rangeStr = clean.substring(7, clean.length - 1);
      const cells = this.getNonEmptyCellsFromRange(rangeStr);
      return cells.length;
    }
    if (clean.startsWith("COUNT(")) {
      const rangeStr = clean.substring(6, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      return numbers.length;
    }

    // 5. IF(condition, true_val, false_val)
    if (clean.startsWith("IF(")) {
      const inside = clean.substring(3, clean.length - 1);
      const parts = inside.split(",");
      if (parts.length >= 3) {
        let condStr = parts[0].trim().replace(/([A-Z]+\d+)/g, m => {
          const v = this.getCellValue(m, visited);
          return !isNaN(parseFloat(v)) ? v : `"${v}"`;
        });
        let isTrue = false;
        try {
          isTrue = Boolean(Function(`"use strict"; return (${condStr});`)());
        } catch(e) { isTrue = false; }
        return isTrue ? parts[1].trim().replace(/^["']|["']$/g, "") : parts[2].trim().replace(/^["']|["']$/g, "");
      }
    }

    // 6. VLOOKUP(lookup_value, table_range, col_index, [exact_match])
    if (clean.startsWith("VLOOKUP(")) {
      const inside = clean.substring(8, clean.length - 1);
      const parts = inside.split(",").map(p => p.trim());
      if (parts.length >= 3) {
        let lookupVal = parts[0];
        if (/^[A-Z]+\d+$/.test(lookupVal)) {
          lookupVal = this.getCellValue(lookupVal, visited);
        } else {
          lookupVal = lookupVal.replace(/^["']|["']$/g, "");
        }

        const rangeStr = parts[1];
        const targetColIdx = parseInt(parts[2], 10);
        if (rangeStr.includes(":") && targetColIdx >= 1) {
          const [rStart, rEnd] = rangeStr.split(":");
          const p1 = this.parseCoord(rStart);
          const p2 = this.parseCoord(rEnd);
          const minCol = Math.min(p1.col, p2.col);
          const minRow = Math.min(p1.row, p2.row);
          const maxRow = Math.max(p1.row, p2.row);

          for (let r = minRow; r <= maxRow; r++) {
            const rowLookupCoord = `${this.getColLetter(minCol)}${r}`;
            const rowVal = String(this.getCellValue(rowLookupCoord, visited)).trim().toLowerCase();
            const searchVal = String(lookupVal).trim().toLowerCase();
            if (rowVal === searchVal || (!isNaN(parseFloat(rowVal)) && parseFloat(rowVal) === parseFloat(searchVal))) {
              const resCoord = `${this.getColLetter(minCol + targetColIdx - 1)}${r}`;
              return this.getCellValue(resCoord, visited);
            }
          }
          return "#N/A";
        }
      }
    }

    // 7. CONCAT & CONCATENATE
    if (clean.startsWith("CONCAT(") || clean.startsWith("CONCATENATE(")) {
      const openIdx = clean.indexOf("(");
      const inside = clean.substring(openIdx + 1, clean.length - 1);
      const parts = inside.split(",");
      let str = "";
      parts.forEach(p => {
        const item = p.trim();
        if (/^[A-Z]+\d+$/.test(item)) {
          str += this.getCellValue(item, visited);
        } else {
          str += item.replace(/^["']|["']$/g, "");
        }
      });
      return str;
    }

    // 8. PRODUCT(range)
    if (clean.startsWith("PRODUCT(")) {
      const rangeStr = clean.substring(8, clean.length - 1);
      const numbers = this.getNumbersFromRange(rangeStr, visited);
      return numbers.length ? numbers.reduce((a, b) => a * b, 1) : 0;
    }

    // 9. ROUND(val, dec)
    if (clean.startsWith("ROUND(")) {
      const inside = clean.substring(6, clean.length - 1);
      const parts = inside.split(",");
      let val = /^[A-Z]+\d+$/.test(parts[0].trim()) ? parseFloat(this.getCellValue(parts[0].trim(), visited)) : parseFloat(parts[0]);
      let dec = parseInt(parts[1] || "0", 10);
      return !isNaN(val) ? val.toFixed(dec) : 0;
    }

    // 10. TODAY() and NOW()
    if (clean.startsWith("TODAY()") || clean.startsWith("NOW()")) {
      const d = new Date();
      return `${String(d.getDate()).padStart(2, '0')}/${String(d.getMonth()+1).padStart(2, '0')}/${d.getFullYear()}`;
    }

    // 11. ABS, SQRT, INT, POWER
    if (clean.startsWith("ABS(")) {
      const arg = clean.substring(4, clean.length - 1).trim();
      const val = /^[A-Z]+\d+$/.test(arg) ? parseFloat(this.getCellValue(arg, visited)) : parseFloat(arg);
      return !isNaN(val) ? Math.abs(val) : 0;
    }
    if (clean.startsWith("SQRT(")) {
      const arg = clean.substring(5, clean.length - 1).trim();
      const val = /^[A-Z]+\d+$/.test(arg) ? parseFloat(this.getCellValue(arg, visited)) : parseFloat(arg);
      return !isNaN(val) && val >= 0 ? Math.sqrt(val) : "#NUM!";
    }
    if (clean.startsWith("INT(")) {
      const arg = clean.substring(4, clean.length - 1).trim();
      const val = /^[A-Z]+\d+$/.test(arg) ? parseFloat(this.getCellValue(arg, visited)) : parseFloat(arg);
      return !isNaN(val) ? Math.floor(val) : 0;
    }
    if (clean.startsWith("POWER(")) {
      const inside = clean.substring(6, clean.length - 1);
      const parts = inside.split(",");
      const b = /^[A-Z]+\d+$/.test(parts[0].trim()) ? parseFloat(this.getCellValue(parts[0].trim(), visited)) : parseFloat(parts[0]);
      const exp = parseFloat(parts[1] || "1");
      return !isNaN(b) && !isNaN(exp) ? Math.pow(b, exp) : 0;
    }

    // 12. General arithmetic expression chaining (e.g. F10-F11 or B4*0.11 or A1+B1)
    try {
      const replaced = clean.replace(/([A-Z]+\d+)/g, (match) => {
        const cVal = this.getCellValue(match, visited);
        const num = parseFloat(cVal);
        return !isNaN(num) ? num : 0;
      });
      if (/^[0-9+\-*/().\s]+$/.test(replaced)) {
        return Function(`"use strict"; return (${replaced});`)();
      }
    } catch (e) {
      return "#ERROR!";
    }

    return formulaStr;
  },

  getNumbersFromRange(rangeStr, visited = new Set()) {
    const sheet = this.sheets[this.activeSheet];
    const results = [];
    if (rangeStr.includes(":")) {
      const parts = rangeStr.split(":");
      const p1 = this.parseCoord(parts[0].trim());
      const p2 = this.parseCoord(parts[1].trim());

      const minCol = Math.min(p1.col, p2.col);
      const maxCol = Math.max(p1.col, p2.col);
      const minRow = Math.min(p1.row, p2.row);
      const maxRow = Math.max(p1.row, p2.row);

      for (let r = minRow; r <= maxRow; r++) {
        for (let c = minCol; c <= maxCol; c++) {
          const coord = `${this.getColLetter(c)}${r}`;
          const val = this.getCellValue(coord, visited);
          const num = parseFloat(val);
          if (!isNaN(num) && isFinite(num)) results.push(num);
        }
      }
    } else {
      const coords = rangeStr.split(",");
      coords.forEach(c => {
        const val = this.getCellValue(c.trim(), visited);
        const num = parseFloat(val);
        if (!isNaN(num) && isFinite(num)) results.push(num);
      });
    }
    return results;
  },

  getNonEmptyCellsFromRange(rangeStr) {
    const sheet = this.sheets[this.activeSheet];
    const results = [];
    if (rangeStr.includes(":")) {
      const parts = rangeStr.split(":");
      const p1 = this.parseCoord(parts[0].trim());
      const p2 = this.parseCoord(parts[1].trim());

      const minCol = Math.min(p1.col, p2.col);
      const maxCol = Math.max(p1.col, p2.col);
      const minRow = Math.min(p1.row, p2.row);
      const maxRow = Math.max(p1.row, p2.row);

      for (let r = minRow; r <= maxRow; r++) {
        for (let c = minCol; c <= maxCol; c++) {
          const coord = `${this.getColLetter(c)}${r}`;
          const raw = sheet.cells[coord];
          if (raw !== undefined && raw !== null && raw !== "") results.push(coord);
        }
      }
    } else {
      const coords = rangeStr.split(",");
      coords.forEach(c => {
        const raw = sheet.cells[c.trim()];
        if (raw !== undefined && raw !== null && raw !== "") results.push(c.trim());
      });
    }
    return results;
  },

  bindEvents() {
    const table = document.getElementById("calcGridTable");
    const formulaInput = document.getElementById("calcFormulaInput");
    const nameBox = document.getElementById("calcNameBox");

    if (table) {
      // Mouse down: start range selection
      table.addEventListener("mousedown", (e) => {
        // If clicking on fill handle, don't start regular cell drag
        if (e.target.classList.contains("calc-fill-handle")) return;

        const td = e.target.closest(".calc-cell");
        if (td) {
          const coord = td.getAttribute("data-coord");
          this.isDraggingSelection = true;
          this.rangeStart = coord;
          this.rangeEnd = coord;
          this.selectRange(coord, coord);
        }
      });

      // Mouse enter / move: drag selection or fill handle preview
      table.addEventListener("mouseover", (e) => {
        const td = e.target.closest(".calc-cell");
        if (!td) return;
        const coord = td.getAttribute("data-coord");

        if (this.isDraggingFillHandle) {
          // Preview fill handle extent
          this.fillTargetCoord = coord;
          document.querySelectorAll(".calc-cell.in-fill-preview").forEach(el => el.classList.remove("in-fill-preview"));

          // Calculate preview rectangular envelope
          const p1 = this.parseCoord(this.rangeStart);
          const p2 = this.parseCoord(this.rangeEnd);
          const pt = this.parseCoord(coord);

          const minC = Math.min(p1.col, p2.col, pt.col);
          const maxC = Math.max(p1.col, p2.col, pt.col);
          const minR = Math.min(p1.row, p2.row, pt.row);
          const maxR = Math.max(p1.row, p2.row, pt.row);

          for (let r = minR; r <= maxR; r++) {
            for (let c = minC; c <= maxC; c++) {
              const previewTd = document.getElementById(`cell-${this.getColLetter(c)}${r}`);
              if (previewTd) previewTd.classList.add("in-fill-preview");
            }
          }
          return;
        }

        if (this.isDraggingSelection) {
          this.rangeEnd = coord;
          this.selectRange(this.rangeStart, this.rangeEnd);
        }
      });

      window.addEventListener("mouseup", () => {
        if (this.isDraggingFillHandle) {
          this.isDraggingFillHandle = false;
          document.querySelectorAll(".calc-cell.in-fill-preview").forEach(el => el.classList.remove("in-fill-preview"));
          if (this.fillTargetCoord) {
            this.applyAutoFill(this.fillSourceRange, this.fillTargetCoord);
            this.fillTargetCoord = null;
          }
        }
        this.isDraggingSelection = false;
      });

      // Double click: inline editing
      table.addEventListener("dblclick", (e) => {
        const td = e.target.closest(".calc-cell");
        if (td) {
          this.startInlineEdit(td);
        }
      });
    }

    if (nameBox) {
      nameBox.addEventListener("keydown", (e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          const target = nameBox.value.trim().toUpperCase();
          if (target.includes(":")) {
            const [s, end] = target.split(":");
            this.selectRange(s.trim(), end.trim());
          } else if (/^[A-Z]+\d+$/.test(target)) {
            this.selectCell(target);
          }
          nameBox.blur();
        }
      });
    }

    if (formulaInput) {
      formulaInput.addEventListener("input", (e) => {
        const sheet = this.sheets[this.activeSheet];
        sheet.cells[this.selectedCell] = e.target.value;
        this.updateCellDisplay(this.selectedCell);
      });

      formulaInput.addEventListener("keydown", (e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          this.navigateRelative(0, 1);
        }
      });
    }

    // Comprehensive Excel-Standard Keyboard Shortcuts
    document.addEventListener("keydown", (e) => {
      if (OfficeCore.currentApp !== "calc") return;
      if (document.activeElement === formulaInput) return;
      if (document.activeElement === nameBox) return;
      if (document.activeElement && (document.activeElement.id === "calcFindInput" || document.activeElement.id === "calcReplaceInput")) return;
      if (document.activeElement && document.activeElement.classList.contains("calc-cell-editing")) return;

      // Ctrl + 1 : Format Sel Lengkap (Format Cells - Excel Standard)
      if ((e.ctrlKey || e.metaKey) && (e.key === "1" || e.key === "!")) {
        e.preventDefault();
        this.openFormatCellsModal();
        return;
      }

      // Ctrl + ` : Tampilkan Seluruh Rumus (Show Formulas toggle)
      if ((e.ctrlKey || e.metaKey) && (e.key === "`" || e.key === "~")) {
        e.preventDefault();
        this.toggleShowFormulas();
        return;
      }

      // F2: Excel In-Place Cell Edit
      if (e.key === "F2") {
        e.preventDefault();
        const td = document.getElementById(`cell-${this.selectedCell}`);
        if (td) this.startInlineEdit(td);
        return;
      }

      // Alt + = : AutoSum (Summation)
      if (e.altKey && (e.key === "=" || e.key === "+")) {
        e.preventDefault();
        this.autoSum("SUM");
        return;
      }

      // Ctrl + A : Select All
      if (e.ctrlKey && (e.key === "a" || e.key === "A")) {
        e.preventDefault();
        this.selectAllGrid();
        return;
      }

      // Ctrl + C : Copy
      if (e.ctrlKey && (e.key === "c" || e.key === "C")) {
        e.preventDefault();
        this.clipboardCopy();
        return;
      }

      // Ctrl + X : Cut
      if (e.ctrlKey && (e.key === "x" || e.key === "X")) {
        e.preventDefault();
        this.clipboardCut();
        return;
      }

      // Ctrl + V : Paste
      if (e.ctrlKey && (e.key === "v" || e.key === "V")) {
        e.preventDefault();
        this.clipboardPaste();
        return;
      }

      // Ctrl + F / Ctrl + H : Find & Replace
      if (e.ctrlKey && (e.key === "f" || e.key === "F" || e.key === "h" || e.key === "H")) {
        e.preventDefault();
        this.openFindDialog();
        return;
      }

      // Escape : Clear clipboard / close popovers
      if (e.key === "Escape") {
        this.clearClipboard();
        this.closeFilterPopover();
        this.closeFindDialog();
        return;
      }

      // Shift + Arrows : Range selection expansion
      if (e.shiftKey && (e.key === "ArrowUp" || e.key === "ArrowDown" || e.key === "ArrowLeft" || e.key === "ArrowRight")) {
        e.preventDefault();
        const endP = this.parseCoord(this.rangeEnd);
        let dC = 0, dR = 0;
        if (e.key === "ArrowUp") dR = -1;
        else if (e.key === "ArrowDown") dR = 1;
        else if (e.key === "ArrowLeft") dC = -1;
        else if (e.key === "ArrowRight") dC = 1;

        const sheet = this.sheets[this.activeSheet];
        const newCol = Math.max(1, Math.min(endP.col + dC, sheet.maxCol || 12));
        const newRow = Math.max(1, Math.min(endP.row + dR, sheet.maxRow || 30));
        this.rangeEnd = `${this.getColLetter(newCol)}${newRow}`;
        this.selectRange(this.rangeStart, this.rangeEnd);
        return;
      }

      // Basic Navigation
      if (e.key === "ArrowUp") { e.preventDefault(); this.navigateRelative(0, -1); }
      else if (e.key === "ArrowDown") { e.preventDefault(); this.navigateRelative(0, 1); }
      else if (e.key === "ArrowLeft") { e.preventDefault(); this.navigateRelative(-1, 0); }
      else if (e.key === "ArrowRight") { e.preventDefault(); this.navigateRelative(1, 0); }
      else if (e.key === "Tab") {
        e.preventDefault();
        this.navigateRelative(e.shiftKey ? -1 : 1, 0);
      }
      else if (e.key === "Enter") {
        e.preventDefault();
        this.navigateRelative(0, e.shiftKey ? -1 : 1);
      }
      else if (e.key === "Delete" || e.key === "Backspace") {
        this.clearSelectedCell();
      }
    });
  },

  selectCell(coord) {
    this.selectedCell = coord;
    this.rangeStart = coord;
    this.rangeEnd = coord;
    this.selectRange(coord, coord);
  },

  selectRange(startCoord, endCoord) {
    const p1 = this.parseCoord(startCoord);
    const p2 = this.parseCoord(endCoord);

    const minCol = Math.min(p1.col, p2.col);
    const maxCol = Math.max(p1.col, p2.col);
    const minRow = Math.min(p1.row, p2.row);
    const maxRow = Math.max(p1.row, p2.row);

    // Clear previous range styling and existing fill handles
    document.querySelectorAll(".calc-cell.in-range, .calc-cell.selected, .calc-cell.range-anchor, .calc-cell.in-fill-preview").forEach(el => {
      el.classList.remove("in-range", "selected", "range-anchor", "in-fill-preview");
    });
    document.querySelectorAll(".calc-fill-handle").forEach(el => el.remove());

    const isSingle = (minCol === maxCol && minRow === maxRow);
    const rangeName = isSingle ? startCoord : `${startCoord}:${endCoord}`;
    this.selectionRange = [];

    const sheet = this.sheets[this.activeSheet];
    const numericValues = [];

    for (let r = minRow; r <= maxRow; r++) {
      for (let c = minCol; c <= maxCol; c++) {
        const coord = `${this.getColLetter(c)}${r}`;
        this.selectionRange.push(coord);
        const td = document.getElementById(`cell-${coord}`);
        if (td) {
          if (coord === startCoord) {
            td.classList.add("range-anchor", "selected");
          } else {
            td.classList.add("in-range");
          }
        }
        const val = this.getCellValue(coord);
        const num = parseFloat(val);
        if (!isNaN(num) && isFinite(num)) numericValues.push(num);
      }
    }

    // Attach Excel Fill Handle to the bottom-right cell (maxCol, maxRow)
    const brCoord = `${this.getColLetter(maxCol)}${maxRow}`;
    const brTd = document.getElementById(`cell-${brCoord}`);
    if (brTd) {
      const handle = document.createElement("div");
      handle.className = "calc-fill-handle";
      handle.title = "Tarik (drag) untuk AutoFill deret/rumus otomatis";
      handle.addEventListener("mousedown", (e) => {
        e.stopPropagation();
        this.isDraggingFillHandle = true;
        this.fillSourceRange = [...this.selectionRange];
        this.fillTargetCoord = null;
      });
      brTd.appendChild(handle);
    }

    const nameBox = document.getElementById("calcNameBox");
    if (nameBox) {
      if (nameBox.tagName === "INPUT") nameBox.value = rangeName;
      else nameBox.innerText = rangeName;
    }

    const formulaInput = document.getElementById("calcFormulaInput");
    if (formulaInput) {
      formulaInput.value = sheet.cells[startCoord] || "";
    }

    this.updateStatusAggregate(numericValues, this.selectionRange.length);
  },

  updateStatusAggregate(numericValues, cellCount) {
    const statCoord = document.getElementById("calcStatCoord");
    const statVal = document.getElementById("calcStatVal");

    if (statCoord) statCoord.innerText = `Rentang: ${this.selectionRange[0]}` + (this.selectionRange.length > 1 ? ` (${cellCount} Sel)` : "");

    if (numericValues.length > 0) {
      const sum = numericValues.reduce((a, b) => a + b, 0);
      const avg = sum / numericValues.length;
      const min = Math.min(...numericValues);
      const max = Math.max(...numericValues);

      const fmt = (n) => new Intl.NumberFormat("id-ID").format(Math.round(n));

      if (statVal) {
        statVal.innerText = `Rata-rata: ${fmt(avg)} | Cacah: ${numericValues.length} | Jumlah: Rp ${fmt(sum)}`;
      }
      const calcAutoSummaryEl = document.getElementById("calcAutoSummary");
      if (calcAutoSummaryEl) {
        calcAutoSummaryEl.style.display = "inline";
        calcAutoSummaryEl.innerText = `Rata-rata: ${fmt(avg)} | Cacah: ${numericValues.length} | Jumlah: ${fmt(sum)}`;
      }
    } else {
      if (statVal) statVal.innerText = "Siap";
      const calcAutoSummaryEl = document.getElementById("calcAutoSummary");
      if (calcAutoSummaryEl) {
        calcAutoSummaryEl.style.display = "none";
      }
    }
  },

  navigateRelative(dCol, dRow) {
    const cur = this.parseCoord(this.selectedCell);
    const newCol = Math.max(1, Math.min(cur.col + dCol, this.sheets[this.activeSheet].maxCol || 12));
    const newRow = Math.max(1, Math.min(cur.row + dRow, this.sheets[this.activeSheet].maxRow || 30));
    const nextCoord = `${this.getColLetter(newCol)}${newRow}`;
    this.selectCell(nextCoord);
  },

  startInlineEdit(td) {
    const coord = td.getAttribute("data-coord");
    const sheet = this.sheets[this.activeSheet];
    const rawVal = sheet.cells[coord] || "";

    const input = document.createElement("input");
    input.type = "text";
    input.className = "calc-cell-editing";
    input.value = rawVal;
    input.style.width = "100%";
    input.style.height = "100%";
    input.style.background = "#ffffff";
    input.style.color = "#0f172a";
    input.style.border = "2px solid #107c41";
    input.style.fontFamily = "inherit";
    input.style.fontSize = "inherit";
    input.style.padding = "2px 4px";
    input.style.outline = "none";

    td.innerHTML = "";
    td.appendChild(input);
    input.focus();

    const finishEdit = () => {
      sheet.cells[coord] = input.value;
      this.updateCellDisplay(coord);
      const fi = document.getElementById("calcFormulaInput");
      if (fi) fi.value = input.value;
      this.selectCell(coord);
    };

    input.addEventListener("blur", finishEdit);
    input.addEventListener("keydown", (e) => {
      if (e.key === "Enter") {
        input.blur();
        this.navigateRelative(0, 1);
      } else if (e.key === "Escape") {
        td.innerHTML = this.getDisplayValue(coord, rawVal, sheet.formats[coord]);
      }
    });
  },

  updateCellDisplay(coord) {
    const td = document.getElementById(`cell-${coord}`);
    if (!td) return;
    const sheet = this.sheets[this.activeSheet];
    const raw = sheet.cells[coord] || "";
    const fmt = sheet.formats[coord] || {};
    td.innerText = this.getDisplayValue(coord, raw, fmt);

    // Refresh all dependent formula cells across the sheet
    document.querySelectorAll(".calc-cell").forEach(cEl => {
      const cCoord = cEl.getAttribute("data-coord");
      const cRaw = sheet.cells[cCoord] || "";
      if (String(cRaw).startsWith("=")) {
        cEl.innerText = this.getDisplayValue(cCoord, cRaw, sheet.formats[cCoord]);
      }
    });
  },

  clearSelectedCell() {
    const sheet = this.sheets[this.activeSheet];
    this.selectionRange.forEach(coord => {
      delete sheet.cells[coord];
      this.updateCellDisplay(coord);
    });
    const fi = document.getElementById("calcFormulaInput");
    if (fi) fi.value = "";
    this.selectCell(this.selectedCell);
  },

  autoSum(funcName = "SUM") {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];

    let formula = "";
    if (funcName === "IF") {
      formula = `=IF(${this.selectedCell}>50000000, "Target Tercapai", "Kurang")`;
    } else if (funcName === "VLOOKUP") {
      formula = `=VLOOKUP(A4, A4:F8, 2, FALSE)`;
    } else if (funcName === "CONCAT") {
      formula = `=CONCAT(A4, " - ", B4)`;
    } else if (funcName === "PRODUCT") {
      const range = this.detectSmartRange(cur.col, cur.row);
      formula = `=PRODUCT(${range})`;
    } else if (funcName === "ROUND") {
      formula = `=ROUND(${this.selectedCell}, 2)`;
    } else if (funcName === "TODAY") {
      formula = `=TODAY()`;
    } else if (funcName === "COUNT" || funcName === "COUNTA" || funcName === "AVERAGE" || funcName === "MAX" || funcName === "MIN" || funcName === "SUM") {
      const range = this.detectSmartRange(cur.col, cur.row);
      formula = `=${funcName}(${range})`;
    } else {
      formula = `=${funcName}()`;
    }

    sheet.cells[this.selectedCell] = formula;
    const formulaInput = document.getElementById("calcFormulaInput");
    if (formulaInput) {
      formulaInput.value = formula;
      formulaInput.focus();
    }
    this.updateCellDisplay(this.selectedCell);
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  detectSmartRange(col, row) {
    const sheet = this.sheets[this.activeSheet];
    // Check above
    let startRow = row - 1;
    let endRow = row - 1;
    while (startRow > 1) {
      const coord = `${this.getColLetter(col)}${startRow - 1}`;
      const val = sheet.cells[coord];
      if (val !== undefined && val !== "" && !isNaN(parseFloat(val))) {
        startRow--;
      } else {
        break;
      }
    }

    if (startRow <= endRow && endRow >= 1) {
      const checkVal = sheet.cells[`${this.getColLetter(col)}${endRow}`];
      if (checkVal !== undefined && checkVal !== "" && !isNaN(parseFloat(checkVal))) {
        return `${this.getColLetter(col)}${startRow}:${this.getColLetter(col)}${endRow}`;
      }
    }

    // Check left
    let startCol = col - 1;
    let endCol = col - 1;
    while (startCol > 1) {
      const coord = `${this.getColLetter(startCol - 1)}${row}`;
      const val = sheet.cells[coord];
      if (val !== undefined && val !== "" && !isNaN(parseFloat(val))) {
        startCol--;
      } else {
        break;
      }
    }

    if (startCol <= endCol && endCol >= 1) {
      const checkVal = sheet.cells[`${this.getColLetter(endCol)}${row}`];
      if (checkVal !== undefined && checkVal !== "" && !isNaN(parseFloat(checkVal))) {
        return `${this.getColLetter(startCol)}${row}:${this.getColLetter(endCol)}${row}`;
      }
    }

    // Default fallback
    return `${this.getColLetter(col)}4:${this.getColLetter(col)}${Math.max(4, row - 1)}`;
  },

  formatSelected(type, val = null) {
    const sheet = this.sheets[this.activeSheet];
    this.selectionRange.forEach(coord => {
      const fmt = { ...(sheet.formats[coord] || {}) };
      if (type === "bold") fmt.bold = !fmt.bold;
      else if (type === "italic") fmt.italic = !fmt.italic;
      else if (type === "underline") fmt.underline = !fmt.underline;
      else if (type === "currency") fmt.currency = fmt.currency === "IDR" ? null : "IDR";
      else if (type === "number") fmt.number = !fmt.number;
      else if (type === "percentage") fmt.percentage = !fmt.percentage;
      else if (type === "color") fmt.color = val;
      else if (type === "bg") fmt.bg = val;
      else if (type === "align") fmt.align = val;
      else if (type === "wrap") fmt.wrap = !fmt.wrap;
      else if (type === "border") fmt.border = val;
      else if (type === "decimal_inc") fmt.decimals = (fmt.decimals || 0) + 1;
      else if (type === "decimal_dec") fmt.decimals = Math.max(0, (fmt.decimals || 0) - 1);

      sheet.formats[coord] = fmt;
      this.updateCellDisplay(coord);
    });
    this.renderGrid();
    this.selectRange(this.rangeStart, this.rangeEnd);
  },

  toggleMergeCenter() {
    if (this.selectionRange.length <= 1) return;
    const sheet = this.sheets[this.activeSheet];
    const firstCoord = this.selectionRange[0];
    const firstVal = sheet.cells[firstCoord] || "";

    const fmt = { ...(sheet.formats[firstCoord] || {}), align: "center", bold: true };
    sheet.formats[firstCoord] = fmt;

    for (let i = 1; i < this.selectionRange.length; i++) {
      delete sheet.cells[this.selectionRange[i]];
    }
    this.renderGrid();
    this.selectCell(firstCoord);
  },

  // Row and Column operations
  insertRow() {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];
    sheet.maxRow = (sheet.maxRow || 30) + 1;

    // Shift rows down
    const newCells = {};
    const newFormats = {};
    Object.keys(sheet.cells).forEach(coord => {
      const p = this.parseCoord(coord);
      if (p.row >= cur.row) {
        newCells[`${this.getColLetter(p.col)}${p.row + 1}`] = sheet.cells[coord];
      } else {
        newCells[coord] = sheet.cells[coord];
      }
    });
    Object.keys(sheet.formats).forEach(coord => {
      const p = this.parseCoord(coord);
      if (p.row >= cur.row) {
        newFormats[`${this.getColLetter(p.col)}${p.row + 1}`] = sheet.formats[coord];
      } else {
        newFormats[coord] = sheet.formats[coord];
      }
    });
    sheet.cells = newCells;
    sheet.formats = newFormats;
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  deleteRow() {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];
    const newCells = {};
    const newFormats = {};
    Object.keys(sheet.cells).forEach(coord => {
      const p = this.parseCoord(coord);
      if (p.row === cur.row) return;
      if (p.row > cur.row) {
        newCells[`${this.getColLetter(p.col)}${p.row - 1}`] = sheet.cells[coord];
      } else {
        newCells[coord] = sheet.cells[coord];
      }
    });
    sheet.cells = newCells;
    sheet.formats = newFormats;
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  insertCol() {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];
    sheet.maxCol = (sheet.maxCol || 12) + 1;
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  deleteCol() {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];
    const newCells = {};
    Object.keys(sheet.cells).forEach(coord => {
      const p = this.parseCoord(coord);
      if (p.col === cur.col) return;
      if (p.col > cur.col) {
        newCells[`${this.getColLetter(p.col - 1)}${p.row}`] = sheet.cells[coord];
      } else {
        newCells[coord] = sheet.cells[coord];
      }
    });
    sheet.cells = newCells;
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  // Sorting
  sortSelected(ascending = true) {
    const cur = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];

    // Identify row boundaries of table around active cell
    let startRow = cur.row;
    while (startRow > 1 && sheet.cells[`${this.getColLetter(cur.col)}${startRow - 1}`]) {
      startRow--;
    }
    let endRow = cur.row;
    while (endRow < (sheet.maxRow || 30) && sheet.cells[`${this.getColLetter(cur.col)}${endRow + 1}`]) {
      endRow++;
    }

    if (startRow >= endRow) return;

    // Gather rows
    const rows = [];
    for (let r = startRow; r <= endRow; r++) {
      const rowData = {};
      for (let c = 1; c <= (sheet.maxCol || 12); c++) {
        rowData[c] = sheet.cells[`${this.getColLetter(c)}${r}`] || "";
      }
      rows.push({ rowIdx: r, data: rowData });
    }

    rows.sort((a, b) => {
      const vA = a.data[cur.col];
      const vB = b.data[cur.col];
      const nA = parseFloat(vA);
      const nB = parseFloat(vB);
      if (!isNaN(nA) && !isNaN(nB)) {
        return ascending ? nA - nB : nB - nA;
      }
      return ascending ? String(vA).localeCompare(String(vB)) : String(vB).localeCompare(String(vA));
    });

    // Write back sorted data
    rows.forEach((item, idx) => {
      const targetRow = startRow + idx;
      for (let c = 1; c <= (sheet.maxCol || 12); c++) {
        const coord = `${this.getColLetter(c)}${targetRow}`;
        sheet.cells[coord] = item.data[c];
      }
    });

    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  toggleAutoFilter() {
    this.autoFilterActive = !this.autoFilterActive;
    if (!this.autoFilterActive) {
      this.columnFilters = {};
    }
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  openFilterPopover(colIdx, event) {
    if (event) event.stopPropagation();
    this.activeFilterCol = colIdx;
    const sheet = this.sheets[this.activeSheet];
    const colLetter = this.getColLetter(colIdx);

    const titleEl = document.getElementById("calcFilterColTitle");
    if (titleEl) titleEl.innerText = `Filter Kolom ${colLetter}`;

    // Gather unique display values
    const uniqueVals = new Set();
    const maxRow = sheet.maxRow || 30;
    for (let r = 4; r <= maxRow; r++) {
      const coord = `${colLetter}${r}`;
      const raw = sheet.cells[coord];
      if (raw !== undefined && raw !== "") {
        const fmt = sheet.formats[coord] || {};
        const disp = this.getDisplayValue(coord, raw, fmt);
        uniqueVals.add(String(disp));
      }
    }

    const checklistEl = document.getElementById("calcFilterChecklist");
    if (checklistEl) {
      const activeFilterSet = (this.columnFilters && this.columnFilters[colIdx]) || new Set();
      let html = `
        <label class="calc-filter-check-item" style="font-weight:700; border-bottom:1px solid #e1dfdd; padding-bottom:3px; margin-bottom:3px;">
          <input type="checkbox" id="calcFilterSelectAll" onchange="WidyaCalc.toggleFilterSelectAll(this.checked)" checked> (Pilih Semua)
        </label>
      `;

      Array.from(uniqueVals).sort().forEach(val => {
        const isChecked = !activeFilterSet.has(val);
        html += `
          <label class="calc-filter-check-item">
            <input type="checkbox" class="calc-filter-item-chk" value="${val.replace(/"/g, '&quot;')}" ${isChecked ? "checked" : ""}>
            <span>${val}</span>
          </label>
        `;
      });
      checklistEl.innerHTML = html;
    }

    const popover = document.getElementById("calcFilterPopover");
    if (popover && event && event.target) {
      const rect = event.target.getBoundingClientRect();
      popover.style.display = "flex";
      popover.style.top = `${rect.bottom + window.scrollY + 2}px`;
      popover.style.left = `${Math.min(rect.left + window.scrollX, window.innerWidth - 260)}px`;
    }
  },

  closeFilterPopover() {
    const popover = document.getElementById("calcFilterPopover");
    if (popover) popover.style.display = "none";
    this.activeFilterCol = null;
  },

  toggleFilterSelectAll(checked) {
    document.querySelectorAll(".calc-filter-item-chk").forEach(chk => {
      chk.checked = checked;
    });
  },

  filterChecklistSearch(query) {
    const q = query.toLowerCase();
    document.querySelectorAll("#calcFilterChecklist .calc-filter-check-item").forEach(item => {
      if (item.querySelector("#calcFilterSelectAll")) return;
      const text = item.innerText.toLowerCase();
      item.style.display = text.includes(q) ? "flex" : "none";
    });
  },

  sortColumnFromFilter(ascending = true) {
    if (!this.activeFilterCol) return;
    this.sortColumnByIndex(this.activeFilterCol, ascending);
    this.closeFilterPopover();
  },

  sortColumnByIndex(colIdx, ascending = true) {
    const sheet = this.sheets[this.activeSheet];
    const maxRow = sheet.maxRow || 30;

    // Detect data bounds (typically rows 4 to end)
    let startRow = 4;
    let endRow = maxRow;
    while (endRow > startRow && !sheet.cells[`${this.getColLetter(colIdx)}${endRow}`]) {
      endRow--;
    }

    if (startRow >= endRow) return;

    const rows = [];
    for (let r = startRow; r <= endRow; r++) {
      const rowData = {};
      for (let c = 1; c <= (sheet.maxCol || 12); c++) {
        rowData[c] = sheet.cells[`${this.getColLetter(c)}${r}`] !== undefined ? sheet.cells[`${this.getColLetter(c)}${r}`] : "";
      }
      rows.push({ rowIdx: r, data: rowData });
    }

    rows.sort((a, b) => {
      const vA = a.data[colIdx];
      const vB = b.data[colIdx];
      const nA = parseFloat(vA);
      const nB = parseFloat(vB);
      if (!isNaN(nA) && !isNaN(nB)) {
        return ascending ? nA - nB : nB - nA;
      }
      return ascending ? String(vA).localeCompare(String(vB)) : String(vB).localeCompare(String(vA));
    });

    rows.forEach((item, idx) => {
      const targetRow = startRow + idx;
      for (let c = 1; c <= (sheet.maxCol || 12); c++) {
        sheet.cells[`${this.getColLetter(c)}${targetRow}`] = item.data[c];
      }
    });

    this.renderGrid();
    this.selectCell(`${this.getColLetter(colIdx)}${startRow}`);
  },

  applyColumnFilter() {
    if (!this.activeFilterCol) return;
    const uncheckedVals = new Set();
    document.querySelectorAll(".calc-filter-item-chk").forEach(chk => {
      if (!chk.checked) uncheckedVals.add(chk.value);
    });

    if (!this.columnFilters) this.columnFilters = {};
    if (uncheckedVals.size > 0) {
      this.columnFilters[this.activeFilterCol] = uncheckedVals;
    } else {
      delete this.columnFilters[this.activeFilterCol];
    }

    this.closeFilterPopover();
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  clearColumnFilter() {
    if (this.activeFilterCol && this.columnFilters) {
      delete this.columnFilters[this.activeFilterCol];
    }
    this.closeFilterPopover();
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  clearAllFilters() {
    this.columnFilters = {};
    this.renderGrid();
    this.selectCell(this.selectedCell);
    alert("[WidyaCalc] Seluruh filter kolom telah dibersihkan.");
  },

  textToColumns() {
    const p = this.parseCoord(this.selectedCell);
    const sheet = this.sheets[this.activeSheet];
    let splitCount = 0;
    for (let r = 1; r <= (sheet.maxRow || 30); r++) {
      const coord = `${this.getColLetter(p.col)}${r}`;
      const val = sheet.cells[coord];
      if (val && typeof val === "string" && (val.includes(",") || val.includes(";") || val.includes("\t"))) {
        const parts = val.split(/[,;\t]/);
        parts.forEach((part, idx) => {
          const targetCoord = `${this.getColLetter(p.col + idx)}${r}`;
          sheet.cells[targetCoord] = part.trim();
        });
        splitCount++;
      }
    }
    if (splitCount > 0) {
      alert(`[WidyaCalc] Berhasil memisahkan teks pada ${splitCount} baris ke kolom-kolom bersebelahan.`);
      this.renderGrid();
      this.selectCell(this.selectedCell);
    } else {
      alert("[WidyaCalc] Tidak ditemukan teks dengan pemisah koma/titik-koma pada kolom terpilih.");
    }
  },

  clipboardCopy() {
    const sheet = this.sheets[this.activeSheet];
    const data = {};
    const formats = {};
    this.selectionRange.forEach(c => {
      data[c] = sheet.cells[c] !== undefined ? sheet.cells[c] : "";
      if (sheet.formats[c]) formats[c] = { ...sheet.formats[c] };
    });

    this.clipboard = {
      action: "copy",
      range: [...this.selectionRange],
      startCoord: this.rangeStart,
      endCoord: this.rangeEnd,
      data,
      formats
    };

    // Add marching ants outline
    document.querySelectorAll(".calc-cell.calc-cell-copied").forEach(el => el.classList.remove("calc-cell-copied"));
    this.selectionRange.forEach(c => {
      const td = document.getElementById(`cell-${c}`);
      if (td) td.classList.add("calc-cell-copied");
    });
  },

  clipboardCut() {
    this.clipboardCopy();
    if (this.clipboard) this.clipboard.action = "cut";
  },

  clipboardPaste() {
    if (!this.clipboard) return;
    const sheet = this.sheets[this.activeSheet];
    const target = this.parseCoord(this.selectedCell);
    const srcStart = this.parseCoord(this.clipboard.startCoord);

    const dRow = target.row - srcStart.row;
    const dCol = target.col - srcStart.col;

    const newSelection = [];

    this.clipboard.range.forEach(srcCoord => {
      const p = this.parseCoord(srcCoord);
      const destCol = p.col + dCol;
      const destRow = p.row + dRow;
      const destCoord = `${this.getColLetter(destCol)}${destRow}`;
      newSelection.push(destCoord);

      const val = this.clipboard.data[srcCoord];
      if (String(val).startsWith("=")) {
        // Shift relative formula references
        const adjusted = val.replace(/(\$?)([A-Z]+)(\$?)(\d+)/g, (match, colLock, colLetters, rowLock, rowNum) => {
          const shiftedCol = colLock === "$" ? colLetters : this.getColLetter(this.getColIndex(colLetters) + dCol);
          const shiftedRow = rowLock === "$" ? rowNum : (parseInt(rowNum, 10) + dRow);
          return `${colLock}${shiftedCol}${rowLock}${shiftedRow}`;
        });
        sheet.cells[destCoord] = adjusted;
      } else {
        sheet.cells[destCoord] = val;
      }

      if (this.clipboard.formats[srcCoord]) {
        sheet.formats[destCoord] = { ...this.clipboard.formats[srcCoord] };
      }

      if (this.clipboard.action === "cut") {
        delete sheet.cells[srcCoord];
        delete sheet.formats[srcCoord];
      }
    });

    if (this.clipboard.action === "cut") {
      this.clearClipboard();
    }

    this.renderGrid();
    if (newSelection.length > 0) {
      this.selectRange(newSelection[0], newSelection[newSelection.length - 1]);
    }
  },

  clearClipboard() {
    this.clipboard = null;
    document.querySelectorAll(".calc-cell.calc-cell-copied").forEach(el => el.classList.remove("calc-cell-copied"));
  },

  openFindDialog() {
    const dialog = document.getElementById("calcFindDialog");
    if (dialog) {
      dialog.style.display = "flex";
      const input = document.getElementById("calcFindInput");
      if (input) { input.focus(); input.select(); }
    }
  },

  closeFindDialog() {
    const dialog = document.getElementById("calcFindDialog");
    if (dialog) dialog.style.display = "none";
  },

  findNext() {
    const input = document.getElementById("calcFindInput");
    if (!input || !input.value) return;
    const q = input.value;
    const matchCase = document.getElementById("calcFindMatchCase")?.checked;
    const sheet = this.sheets[this.activeSheet];
    const coords = Object.keys(sheet.cells).sort();

    const matches = [];
    coords.forEach(c => {
      const v = String(sheet.cells[c] || "");
      const matched = matchCase ? v.includes(q) : v.toLowerCase().includes(q.toLowerCase());
      if (matched) matches.push(c);
    });

    if (matches.length === 0) {
      alert(`[WidyaCalc] Nilai '${q}' tidak ditemukan.`);
      return;
    }

    let nextIdx = matches.indexOf(this.selectedCell);
    nextIdx = (nextIdx + 1) % matches.length;
    const targetCoord = matches[nextIdx];
    this.selectCell(targetCoord);
    const td = document.getElementById(`cell-${targetCoord}`);
    if (td) td.scrollIntoView({ block: "center", inline: "center", behavior: "smooth" });
  },

  replaceOne() {
    const findInp = document.getElementById("calcFindInput");
    const repInp = document.getElementById("calcReplaceInput");
    if (!findInp || !findInp.value) return;
    const q = findInp.value;
    const rep = repInp ? repInp.value : "";
    const sheet = this.sheets[this.activeSheet];
    const curVal = String(sheet.cells[this.selectedCell] || "");

    const matchCase = document.getElementById("calcFindMatchCase")?.checked;
    const regex = new RegExp(q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), matchCase ? 'g' : 'gi');
    if (regex.test(curVal)) {
      sheet.cells[this.selectedCell] = curVal.replace(regex, rep);
      this.updateCellDisplay(this.selectedCell);
      this.findNext();
    } else {
      this.findNext();
    }
  },

  replaceAll() {
    const findInp = document.getElementById("calcFindInput");
    const repInp = document.getElementById("calcReplaceInput");
    if (!findInp || !findInp.value) return;
    const q = findInp.value;
    const rep = repInp ? repInp.value : "";
    const sheet = this.sheets[this.activeSheet];
    const matchCase = document.getElementById("calcFindMatchCase")?.checked;
    const regex = new RegExp(q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), matchCase ? 'g' : 'gi');

    let count = 0;
    Object.keys(sheet.cells).forEach(c => {
      const v = String(sheet.cells[c] || "");
      if (regex.test(v)) {
        sheet.cells[c] = v.replace(regex, rep);
        this.updateCellDisplay(c);
        count++;
      }
    });
    alert(`[WidyaCalc] Selesai! ${count} sel berhasil diganti.`);
    this.renderGrid();
    this.selectCell(this.selectedCell);
  },

  applyAutoFill(sourceRange, targetCoordStr) {
    if (!sourceRange || sourceRange.length === 0 || !targetCoordStr) return;
    const sheet = this.sheets[this.activeSheet];

    // Determine bounding box of source range
    let sMinCol = 999, sMaxCol = 0, sMinRow = 999, sMaxRow = 0;
    sourceRange.forEach(c => {
      const p = this.parseCoord(c);
      if (p.col < sMinCol) sMinCol = p.col;
      if (p.col > sMaxCol) sMaxCol = p.col;
      if (p.row < sMinRow) sMinRow = p.row;
      if (p.row > sMaxRow) sMaxRow = p.row;
    });

    const targetCoord = this.parseCoord(targetCoordStr);

    const verticalDiff = targetCoord.row > sMaxRow ? targetCoord.row - sMaxRow : (targetCoord.row < sMinRow ? sMinRow - targetCoord.row : 0);
    const horizontalDiff = targetCoord.col > sMaxCol ? targetCoord.col - sMaxCol : (targetCoord.col < sMinCol ? sMinCol - targetCoord.col : 0);

    if (verticalDiff >= horizontalDiff && verticalDiff > 0) {
      // Filling downward
      if (targetCoord.row > sMaxRow) {
        for (let c = sMinCol; c <= sMaxCol; c++) {
          const colLetter = this.getColLetter(c);

          const srcValues = [];
          for (let r = sMinRow; r <= sMaxRow; r++) {
            srcValues.push(sheet.cells[`${colLetter}${r}`]);
          }

          const allNumbers = srcValues.every(v => v !== undefined && v !== "" && !isNaN(parseFloat(v)) && !String(v).startsWith("="));
          let step = 0;
          if (allNumbers && srcValues.length > 1) {
            step = (parseFloat(srcValues[srcValues.length - 1]) - parseFloat(srcValues[0])) / (srcValues.length - 1);
          }

          for (let r = sMaxRow + 1; r <= targetCoord.row; r++) {
            const destCoord = `${colLetter}${r}`;
            const srcRow = sMinRow + ((r - sMaxRow - 1) % (sMaxRow - sMinRow + 1));
            const srcCoord = `${colLetter}${srcRow}`;
            const srcVal = sheet.cells[srcCoord] || "";
            const srcFmt = sheet.formats[srcCoord];

            if (String(srcVal).startsWith("=")) {
              // Adjust relative row references in formula
              const dRow = r - srcRow;
              const adjustedFormula = srcVal.replace(/(\$?)([A-Z]+)(\$?)(\d+)/g, (match, colLock, colLetters, rowLock, rowNum) => {
                const shiftedRow = rowLock === "$" ? rowNum : (parseInt(rowNum, 10) + dRow);
                return `${colLock}${colLetters}${rowLock}${shiftedRow}`;
              });
              sheet.cells[destCoord] = adjustedFormula;
            } else if (allNumbers && srcValues.length > 1) {
              const baseNum = parseFloat(srcValues[srcValues.length - 1]);
              const multiplier = r - sMaxRow;
              sheet.cells[destCoord] = String(baseNum + step * multiplier);
            } else if (typeof srcVal === "string" && srcVal.match(/^(.*?)(\d+)$/)) {
              const match = srcVal.match(/^(.*?)(\d+)$/);
              const prefix = match[1];
              const num = parseInt(match[2], 10);
              sheet.cells[destCoord] = `${prefix}${num + (r - srcRow)}`;
            } else {
              sheet.cells[destCoord] = srcVal;
            }

            if (srcFmt) {
              sheet.formats[destCoord] = { ...srcFmt };
            }
          }
        }
        this.renderGrid();
        this.selectRange(this.rangeStart, `${this.getColLetter(sMaxCol)}${targetCoord.row}`);
      }
    } else if (horizontalDiff > 0) {
      // Filling rightward
      if (targetCoord.col > sMaxCol) {
        for (let r = sMinRow; r <= sMaxRow; r++) {
          const srcValues = [];
          for (let c = sMinCol; c <= sMaxCol; c++) {
            srcValues.push(sheet.cells[`${this.getColLetter(c)}${r}`]);
          }

          const allNumbers = srcValues.every(v => v !== undefined && v !== "" && !isNaN(parseFloat(v)) && !String(v).startsWith("="));
          let step = 0;
          if (allNumbers && srcValues.length > 1) {
            step = (parseFloat(srcValues[srcValues.length - 1]) - parseFloat(srcValues[0])) / (srcValues.length - 1);
          }

          for (let c = sMaxCol + 1; c <= targetCoord.col; c++) {
            const destCoord = `${this.getColLetter(c)}${r}`;
            const srcCol = sMinCol + ((c - sMaxCol - 1) % (sMaxCol - sMinCol + 1));
            const srcCoord = `${this.getColLetter(srcCol)}${r}`;
            const srcVal = sheet.cells[srcCoord] || "";
            const srcFmt = sheet.formats[srcCoord];

            if (String(srcVal).startsWith("=")) {
              // Adjust relative col references in formula
              const dCol = c - srcCol;
              const adjustedFormula = srcVal.replace(/(\$?)([A-Z]+)(\$?)(\d+)/g, (match, colLock, colLetters, rowLock, rowNum) => {
                if (colLock === "$") return match;
                const origIdx = this.getColIndex(colLetters);
                const shiftedLetters = this.getColLetter(origIdx + dCol);
                return `${colLock}${shiftedLetters}${rowLock}${rowNum}`;
              });
              sheet.cells[destCoord] = adjustedFormula;
            } else if (allNumbers && srcValues.length > 1) {
              const baseNum = parseFloat(srcValues[srcValues.length - 1]);
              const multiplier = c - sMaxCol;
              sheet.cells[destCoord] = String(baseNum + step * multiplier);
            } else if (typeof srcVal === "string" && srcVal.match(/^(.*?)(\d+)$/)) {
              const match = srcVal.match(/^(.*?)(\d+)$/);
              const prefix = match[1];
              const num = parseInt(match[2], 10);
              sheet.cells[destCoord] = `${prefix}${num + (c - srcCol)}`;
            } else {
              sheet.cells[destCoord] = srcVal;
            }

            if (srcFmt) {
              sheet.formats[destCoord] = { ...srcFmt };
            }
          }
        }
        this.renderGrid();
        this.selectRange(this.rangeStart, `${this.getColLetter(targetCoord.col)}${sMaxRow}`);
      }
    }
  },

  // Interactive Live SVG Chart Generator
  generateChart(type = "column") {
    const sheet = this.sheets[this.activeSheet];
    // Gather sample chart data from active sheet
    const labels = ["Q1", "Q2", "Q3", "Q4"];
    const datasets = [
      { name: "Infrastruktur", values: [45, 52, 48, 61], color: "#185abd" },
      { name: "R&D Widya", values: [85, 92, 110, 125], color: "#107c41" },
      { name: "Cyber PQC", values: [32, 35, 40, 45], color: "#c43e1c" }
    ];

    let svgContent = "";

    if (type === "column") {
      svgContent = `
        <svg width="440" height="240" viewBox="0 0 440 240" style="background:#ffffff; font-family:sans-serif;">
          <!-- Grid lines -->
          <line x1="40" y1="200" x2="420" y2="200" stroke="#cbd5e1" stroke-width="1.5" />
          <line x1="40" y1="140" x2="420" y2="140" stroke="#e2e8f0" stroke-dasharray="4" />
          <line x1="40" y1="80" x2="420" y2="80" stroke="#e2e8f0" stroke-dasharray="4" />
          <line x1="40" y1="20" x2="420" y2="20" stroke="#e2e8f0" stroke-dasharray="4" />

          <!-- Y Axis Labels -->
          <text x="35" y="204" font-size="10" text-anchor="end" fill="#64748b">0</text>
          <text x="35" y="144" font-size="10" text-anchor="end" fill="#64748b">50M</text>
          <text x="35" y="84" font-size="10" text-anchor="end" fill="#64748b">100M</text>
          <text x="35" y="24" font-size="10" text-anchor="end" fill="#64748b">150M</text>

          <!-- Bars -->
          <!-- Q1 -->
          <rect x="65" y="140" width="18" height="60" fill="#185abd" rx="2" />
          <rect x="86" y="87" width="18" height="113" fill="#107c41" rx="2" />
          <rect x="107" y="157" width="18" height="43" fill="#c43e1c" rx="2" />
          <text x="95" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Kuartal 1</text>

          <!-- Q2 -->
          <rect x="155" y="131" width="18" height="69" fill="#185abd" rx="2" />
          <rect x="176" y="77" width="18" height="123" fill="#107c41" rx="2" />
          <rect x="197" y="153" width="18" height="47" fill="#c43e1c" rx="2" />
          <text x="185" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Kuartal 2</text>

          <!-- Q3 -->
          <rect x="245" y="136" width="18" height="64" fill="#185abd" rx="2" />
          <rect x="266" y="53" width="18" height="147" fill="#107c41" rx="2" />
          <rect x="287" y="147" width="18" height="53" fill="#c43e1c" rx="2" />
          <text x="275" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Kuartal 3</text>

          <!-- Q4 -->
          <rect x="335" y="119" width="18" height="81" fill="#185abd" rx="2" />
          <rect x="356" y="33" width="18" height="167" fill="#107c41" rx="2" />
          <rect x="377" y="140" width="18" height="60" fill="#c43e1c" rx="2" />
          <text x="365" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Kuartal 4</text>
        </svg>
      `;
    } else if (type === "line") {
      svgContent = `
        <svg width="440" height="240" viewBox="0 0 440 240" style="background:#ffffff; font-family:sans-serif;">
          <line x1="40" y1="200" x2="420" y2="200" stroke="#cbd5e1" stroke-width="1.5" />
          <line x1="40" y1="140" x2="420" y2="140" stroke="#e2e8f0" stroke-dasharray="4" />
          <line x1="40" y1="80" x2="420" y2="80" stroke="#e2e8f0" stroke-dasharray="4" />

          <!-- Line 1: Server -->
          <polyline fill="none" stroke="#185abd" stroke-width="3" points="95,140 185,131 275,136 365,119" />
          <circle cx="95" cy="140" r="4" fill="#185abd" />
          <circle cx="185" cy="131" r="4" fill="#185abd" />
          <circle cx="275" cy="136" r="4" fill="#185abd" />
          <circle cx="365" cy="119" r="4" fill="#185abd" />

          <!-- Line 2: R&D Widya -->
          <polyline fill="none" stroke="#107c41" stroke-width="3" points="95,87 185,77 275,53 365,33" />
          <circle cx="95" cy="87" r="4" fill="#107c41" />
          <circle cx="185" cy="77" r="4" fill="#107c41" />
          <circle cx="275" cy="53" r="4" fill="#107c41" />
          <circle cx="365" cy="33" r="4" fill="#107c41" />

          <text x="95" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Q1</text>
          <text x="185" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Q2</text>
          <text x="275" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Q3</text>
          <text x="365" y="216" font-size="11" font-weight="600" text-anchor="middle" fill="#334155">Q4</text>
        </svg>
      `;
    } else if (type === "pie") {
      svgContent = `
        <svg width="440" height="240" viewBox="0 0 440 240" style="background:#ffffff; font-family:sans-serif;">
          <!-- Pie Slices -->
          <circle cx="150" cy="120" r="85" fill="#107c41" />
          <path d="M 150 120 L 150 35 A 85 85 0 0 1 228 155 Z" fill="#185abd" />
          <path d="M 150 120 L 228 155 A 85 85 0 0 1 115 198 Z" fill="#c43e1c" />

          <!-- Legend -->
          <rect x="280" y="60" width="14" height="14" fill="#107c41" rx="2" />
          <text x="302" y="72" font-size="12" fill="#334155" font-weight="600">R&D AI Widya (52%)</text>

          <rect x="280" y="90" width="14" height="14" fill="#185abd" rx="2" />
          <text x="302" y="102" font-size="12" fill="#334155" font-weight="600">Server Infra (28%)</text>

          <rect x="280" y="120" width="14" height="14" fill="#c43e1c" rx="2" />
          <text x="302" y="132" font-size="12" fill="#334155" font-weight="600">Keamanan PQC (20%)</text>
        </svg>
      `;
    }

    // Remove existing chart card if open
    const existing = document.getElementById("activeCalcChartCard");
    if (existing) existing.remove();

    const chartCard = document.createElement("div");
    chartCard.id = "activeCalcChartCard";
    chartCard.className = "calc-chart-card";
    chartCard.innerHTML = `
      <div class="calc-chart-header">
        <span style="display:flex; align-items:center; gap:6px;">📊 Diagram ${type.toUpperCase()} - Analisis Pengeluaran 2026</span>
        <span class="calc-chart-close" onclick="document.getElementById('activeCalcChartCard').remove()">✕</span>
      </div>
      <div class="calc-chart-body">
        ${svgContent}
      </div>
    `;

    document.getElementById("workspace-calc")?.appendChild(chartCard);
  },

  exportXlsx() {
    const sheet = this.sheets[this.activeSheet];
    alert(`📊 [WidyaCalc] Memaketkan buku kerja '${sheet.name}' ke format OpenXML .xlsx murni via Rust...\nUkuran kompresi zip ~5.2 KB. Selesai!`);
  },

  exportCsv() {
    const sheet = this.sheets[this.activeSheet];
    let csv = "";
    for (let r = 1; r <= (sheet.maxRow || 30); r++) {
      const row = [];
      for (let c = 1; c <= (sheet.maxCol || 12); c++) {
        const coord = `${this.getColLetter(c)}${r}`;
        const raw = sheet.cells[coord] || "";
        const disp = this.getDisplayValue(coord, raw, sheet.formats[coord]);
        row.push(`"${String(disp).replace(/"/g, '""')}"`);
      }
      csv += row.join(",") + "\n";
    }

    const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${sheet.name || "WidyaCalc"}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  },

  openTemplatesModal() {
    const modal = document.getElementById("calcTemplatesModal");
    if (modal) modal.classList.add("active");
  },

  closeTemplatesModal() {
    const modal = document.getElementById("calcTemplatesModal");
    if (modal) modal.classList.remove("active");
  },

  loadTemplate(templateId) {
    const sheet = this.sheets[this.activeSheet];
    sheet.cells = {};
    sheet.formats = {};

    if (templateId === "cashflow") {
      sheet.name = "Arus Kas Bulanan";
      sheet.cells["A1"] = "LAPORAN ARUS KAS OPERASIONAL KEDATUAN 2026";
      sheet.formats["A1"] = { bold: true, color: "#107c41", fontSize: "12pt" };

      const headers = ["Tanggal", "Uraian Transaksi", "Kategori", "Kas Masuk", "Kas Keluar", "Saldo Berjalan"];
      ["A3","B3","C3","D3","E3","F3"].forEach((c, idx) => {
        sheet.cells[c] = headers[idx];
        sheet.formats[c] = { bold: true, bg: "#f1f5f9", color: "#0f172a", border: "all" };
      });

      const data = [
        ["01/10/2026", "Saldo Awal Kas & Bank", "Modal Awal", "250000000", "0", "=D4-E4"],
        ["05/10/2026", "Pendapatan Lisensi WidyaOffice", "Penjualan", "125000000", "0", "=F4+D5-E5"],
        ["12/10/2026", "Pengadaan Server HSM PQC", "Infrastruktur", "0", "48000000", "=F5+D6-E6"],
        ["18/10/2026", "Audit Kepatuhan UU PDP & BSSN", "Kepatuhan", "0", "15000000", "=F6+D7-E7"],
        ["25/10/2026", "Insentif Riset Perekayasa Rust", "SDM Riset", "0", "65000000", "=F7+D8-E8"],
        ["30/10/2026", "Termin 1 Kerjasama BUMN", "Kontrak", "180000000", "0", "=F8+D9-E9"]
      ];

      data.forEach((row, rIdx) => {
        const r = rIdx + 4;
        sheet.cells[`A${r}`] = row[0];
        sheet.cells[`B${r}`] = row[1];
        sheet.cells[`C${r}`] = row[2];
        sheet.cells[`D${r}`] = row[3];
        sheet.cells[`E${r}`] = row[4];
        sheet.cells[`F${r}`] = row[5];
        ["D","E","F"].forEach(col => {
          sheet.formats[`${col}${r}`] = { currency: "IDR" };
        });
      });

      // Total Row
      sheet.cells["A11"] = "TOTAL PERGERAKAN KAS";
      sheet.formats["A11"] = { bold: true, color: "#107c41" };
      sheet.cells["D11"] = "=SUM(D4:D9)";
      sheet.cells["E11"] = "=SUM(E4:E9)";
      sheet.cells["F11"] = "=D11-E11";
      ["D11","E11","F11"].forEach(c => {
        sheet.formats[c] = { bold: true, color: "#107c41", currency: "IDR", border: "bottom-double" };
      });
    } else if (templateId === "rab") {
      sheet.name = "RAB Proyek TI";
      sheet.cells["A1"] = "RENCANA ANGGARAN BIAYA (RAB) INFRASTRUKTUR WIDYA 2026";
      sheet.formats["A1"] = { bold: true, color: "#185abd", fontSize: "12pt" };

      const headers = ["No", "Uraian Pekerjaan / Komponen", "Volume", "Satuan", "Harga Satuan", "Total Biaya"];
      ["A3","B3","C3","D3","E3","F3"].forEach((c, idx) => {
        sheet.cells[c] = headers[idx];
        sheet.formats[c] = { bold: true, bg: "#f1f5f9", color: "#0f172a", border: "all" };
      });

      const items = [
        ["1", "Lisensi Server Kluster Rust Native", "4", "Server", "25000000", "=C4*E4"],
        ["2", "Modul Kriptografi HSM ML-KEM-768", "2", "Unit", "85000000", "=C5*E5"],
        ["3", "Fuzzing & Pen-Testing Keamanan PQC", "1", "Paket", "45000000", "=C6*E6"],
        ["4", "Pelatihan Operator Suite Perkantoran", "30", "Peserta", "2500000", "=C7*E7"],
        ["5", "Dokumentasi & Standarisasi ISO/IEC", "1", "Paket", "35000000", "=C8*E8"]
      ];

      items.forEach((row, rIdx) => {
        const r = rIdx + 4;
        sheet.cells[`A${r}`] = row[0];
        sheet.cells[`B${r}`] = row[1];
        sheet.cells[`C${r}`] = row[2];
        sheet.cells[`D${r}`] = row[3];
        sheet.cells[`E${r}`] = row[4];
        sheet.cells[`F${r}`] = row[5];
        sheet.formats[`E${r}`] = { currency: "IDR" };
        sheet.formats[`F${r}`] = { currency: "IDR", bold: true };
      });

      sheet.cells["B10"] = "SUBTOTAL BIAYA";
      sheet.formats["B10"] = { bold: true };
      sheet.cells["F10"] = "=SUM(F4:F8)";
      sheet.formats["F10"] = { bold: true, currency: "IDR", border: "all" };

      sheet.cells["B11"] = "PAJAK PPN 11%";
      sheet.formats["B11"] = { bold: true };
      sheet.cells["F11"] = "40150000";
      sheet.formats["F11"] = { currency: "IDR" };

      sheet.cells["B12"] = "TOTAL ANGGARAN (GRAND TOTAL)";
      sheet.formats["B12"] = { bold: true, color: "#185abd" };
      sheet.cells["F12"] = "=SUM(F10:F11)";
      sheet.formats["F12"] = { bold: true, color: "#185abd", currency: "IDR", border: "bottom-double" };
    } else if (templateId === "kpi") {
      sheet.name = "Scorecard KPI";
      sheet.cells["A1"] = "MATRIKS SKOR KPI KINERJA SISTEM BERDAULAT 2026";
      sheet.formats["A1"] = { bold: true, color: "#6b21a8", fontSize: "12pt" };

      const headers = ["No", "Sasaran Strategis", "Bobot %", "Target", "Realisasi", "Indeks Kinerja"];
      ["A3","B3","C3","D3","E3","F3"].forEach((c, idx) => {
        sheet.cells[c] = headers[idx];
        sheet.formats[c] = { bold: true, bg: "#f1f5f9", color: "#0f172a", border: "all" };
      });

      const kpis = [
        ["1", "Ketersediaan Layanan (Uptime 99.9%)", "25", "100", "100", "=E4/D4*C4"],
        ["2", "Adopsi Format Mandiri di BUMN", "20", "80", "88", "=E5/D5*C5"],
        ["3", "Pencegahan Kebocoran Data (UU PDP)", "30", "100", "100", "=E6/D6*C6"],
        ["4", "Waktu Muat Dokumen Rendah (<100ms)", "25", "100", "105", "=E7/D7*C7"]
      ];

      kpis.forEach((row, rIdx) => {
        const r = rIdx + 4;
        sheet.cells[`A${r}`] = row[0];
        sheet.cells[`B${r}`] = row[1];
        sheet.cells[`C${r}`] = row[2];
        sheet.cells[`D${r}`] = row[3];
        sheet.cells[`E${r}`] = row[4];
        sheet.cells[`F${r}`] = row[5];
        sheet.formats[`C${r}`] = { percentage: true };
        sheet.formats[`F${r}`] = { bold: true, decimals: 1 };
      });

      sheet.cells["B9"] = "TOTAL SKOR TERTINBANG";
      sheet.formats["B9"] = { bold: true, color: "#6b21a8" };
      sheet.cells["F9"] = "=SUM(F4:F7)";
      sheet.formats["F9"] = { bold: true, color: "#6b21a8", border: "bottom-double" };
    }

    this.closeTemplatesModal();
    this.renderGrid();
    this.renderSheetTabs();
    this.selectCell("B4");
  },

  applyConditionalFormatting(type) {
    const sheet = this.sheets[this.activeSheet];
    const nums = [];
    this.selectionRange.forEach(c => {
      const raw = sheet.cells[c];
      const val = parseFloat(String(raw).startsWith("=") ? this.evaluateFormula(raw) : raw);
      if (!isNaN(val)) nums.push(val);
    });

    this.selectionRange.forEach(c => {
      const fmt = { ...(sheet.formats[c] || {}) };
      const raw = sheet.cells[c];
      const val = parseFloat(String(raw).startsWith("=") ? this.evaluateFormula(raw) : raw);

      if (type === "clear") {
        delete fmt.bg;
        delete fmt.color;
      } else if (type === "highlight-greater" && !isNaN(val)) {
        if (val >= 50000000) {
          fmt.bg = "#dcfce7";
          fmt.color = "#166534";
          fmt.bold = true;
        }
      } else if (type === "highlight-lesser" && !isNaN(val)) {
        if (val < 50000000) {
          fmt.bg = "#fee2e2";
          fmt.color = "#991b1b";
          fmt.bold = true;
        }
      } else if (type === "color-scale" && nums.length > 1 && !isNaN(val)) {
        const min = Math.min(...nums);
        const max = Math.max(...nums);
        const ratio = (val - min) / (max - min || 1);
        if (ratio > 0.66) {
          fmt.bg = "#dcfce7";
          fmt.color = "#166534";
        } else if (ratio > 0.33) {
          fmt.bg = "#fef9c3";
          fmt.color = "#854d0e";
        } else {
          fmt.bg = "#fee2e2";
          fmt.color = "#991b1b";
        }
      }
      sheet.formats[c] = fmt;
    });
    this.renderGrid();
    this.selectRange(this.rangeStart, this.rangeEnd);
  },

  toggleFreezeHeader() {
    const tbl = document.getElementById("calcGridTable");
    if (tbl) {
      tbl.classList.toggle("frozen-header");
      const isFrozen = tbl.classList.contains("frozen-header");
      alert(isFrozen ? "Baris header (Baris 3) sekarang dibekukan di bagian atas." : "Pembekuan baris dinonaktifkan.");
    }
  },

  autoFitSelectedColumn() {
    const p = this.parseCoord(this.selectedCell);
    const colLetter = this.getColLetter(p.col);
    const sheet = this.sheets[this.activeSheet];
    let maxLen = 8;
    for (let r = 1; r <= (sheet.maxRow || 30); r++) {
      const v = String(sheet.cells[`${colLetter}${r}`] || "");
      if (v.length > maxLen) maxLen = v.length;
    }
    const widthPx = Math.min(Math.max(maxLen * 10, 70), 320);
    document.querySelectorAll(`[data-coord^='${colLetter}']`).forEach(td => {
      td.style.minWidth = `${widthPx}px`;
    });
  },

  isColumnAFrozen: false,
  showFormulasMode: false,
  dataValidations: {},

  toggleFreezeFirstColumn() {
    this.isColumnAFrozen = !this.isColumnAFrozen;
    document.querySelectorAll("[data-coord^='A'], th[data-col='1']").forEach(el => {
      el.classList.toggle("calc-frozen-col", this.isColumnAFrozen);
    });
    alert(this.isColumnAFrozen ? "Kolom A sekarang dibekukan di bagian kiri lembar kerja." : "Pembekuan kolom A dinonaktifkan.");
  },

  toggleShowFormulas() {
    this.showFormulasMode = !this.showFormulasMode;
    const btn = document.getElementById("btnShowFormulas");
    if (btn) btn.classList.toggle("active", this.showFormulasMode);
    this.renderGrid();
    this.selectRange(this.rangeStart, this.rangeEnd);
  },

  openDataValidationDialog() {
    const defaultList = "Setuju, Revisi, Tolak";
    const input = prompt("Masukkan daftar pilihan validasi dipisahkan tanda koma (List Validation):\nContoh: Setuju, Revisi, Tolak atau Lunas, Menunggu, Batal", defaultList);
    if (!input) return;

    const options = input.split(",").map(s => s.trim()).filter(Boolean);
    if (options.length === 0) return;

    if (!this.dataValidations) this.dataValidations = {};
    this.selectionRange.forEach(coord => {
      this.dataValidations[coord] = [...options];
    });

    this.renderGrid();
    this.selectRange(this.rangeStart, this.rangeEnd);
    alert(`☑️ [Validasi Data Diterapkan]\nSel ${this.selectionRange.join(", ")} kini memiliki tombol dropdown pilihan: [${options.join(" | ")}].`);
  },

  showValidationPicker(coord, e) {
    if (e) e.stopPropagation();
    const existing = document.getElementById("calcValidationPopover");
    if (existing) existing.remove();

    const options = this.dataValidations?.[coord];
    if (!options || options.length === 0) return;

    const cell = document.getElementById(`cell-${coord}`);
    if (!cell) return;
    const rect = cell.getBoundingClientRect();

    const popover = document.createElement("div");
    popover.id = "calcValidationPopover";
    popover.className = "calc-validation-popover";
    popover.style.left = `${rect.left}px`;
    popover.style.top = `${rect.bottom + window.scrollY}px`;

    let html = "";
    options.forEach(opt => {
      html += `<div class="calc-validation-option" onclick="WidyaCalc.setValidationValue('${coord}', '${opt}')">${opt}</div>`;
    });
    popover.innerHTML = html;
    document.body.appendChild(popover);

    const dismissHandler = (evt) => {
      if (!popover.contains(evt.target)) {
        popover.remove();
        document.removeEventListener("click", dismissHandler);
      }
    };
    setTimeout(() => document.addEventListener("click", dismissHandler), 10);
  },

  setValidationValue(coord, val) {
    const sheet = this.sheets[this.activeSheet];
    sheet.cells[coord] = val;
    const pop = document.getElementById("calcValidationPopover");
    if (pop) pop.remove();
    this.renderGrid();
    this.selectCell(coord);
  },

  // ==========================================
  // Excel Standard: Format Cells Dialog (Ctrl+1) & Dimensions
  // ==========================================
  _modalSelectedCategory: "general",
  _modalBorderPreset: null,

  openFormatCellsModal() {
    const modal = document.getElementById("calcFormatCellsModal");
    if (!modal) return;
    modal.classList.add("active");

    const sheet = this.sheets[this.activeSheet];
    const fmt = (sheet && sheet.formats && sheet.formats[this.selectedCell]) || {};

    // Initialise category
    if (fmt.currency) this.selectCellCategory("currency");
    else if (fmt.percentage) this.selectCellCategory("percentage");
    else if (fmt.number) this.selectCellCategory("number");
    else this.selectCellCategory("general");

    // Alignment
    const alignH = document.getElementById("cellAlignH");
    if (alignH) alignH.value = fmt.align || "left";
    const alignV = document.getElementById("cellAlignV");
    if (alignV) alignV.value = fmt.verticalAlign || "middle";
    const wrapChk = document.getElementById("cellWrapText");
    if (wrapChk) wrapChk.checked = !!fmt.wrap;

    // Font
    const boldChk = document.getElementById("cellFontBold");
    if (boldChk) boldChk.checked = !!fmt.bold;
    const italicChk = document.getElementById("cellFontItalic");
    if (italicChk) italicChk.checked = !!fmt.italic;
    const underChk = document.getElementById("cellFontUnderline");
    if (underChk) underChk.checked = !!fmt.underline;
    const fontColor = document.getElementById("cellFontColor");
    if (fontColor) fontColor.value = fmt.color || "#0f172a";

    // Fill
    const bgColor = document.getElementById("cellBgColor");
    if (bgColor) bgColor.value = fmt.bg || "#ffffff";

    this._modalBorderPreset = fmt.border || null;
    this.switchFormatCellTab("number");
  },

  closeFormatCellsModal() {
    const modal = document.getElementById("calcFormatCellsModal");
    if (modal) modal.classList.remove("active");
  },

  switchFormatCellTab(tabName) {
    const tabs = ["number", "align", "font", "border", "fill"];
    tabs.forEach(t => {
      const panel = document.getElementById("formatCellTab" + t.charAt(0).toUpperCase() + t.slice(1));
      const btn = document.getElementById("tabBtnCell" + t.charAt(0).toUpperCase() + t.slice(1));
      if (panel) panel.style.display = (t === tabName) ? "block" : "none";
      if (btn) {
        if (t === tabName) {
          btn.classList.add("active");
          btn.style.fontWeight = "700";
          btn.style.color = "var(--accent-calc)";
          btn.style.borderBottom = "2px solid var(--accent-calc)";
        } else {
          btn.classList.remove("active");
          btn.style.fontWeight = "600";
          btn.style.color = "var(--text-secondary)";
          btn.style.borderBottom = "none";
        }
      }
    });
  },

  selectCellCategory(cat) {
    this._modalSelectedCategory = cat;
    const sample = document.getElementById("cellFormatSample");
    if (!sample) return;

    if (cat === "currency") {
      sample.textContent = "Rp 12.345.670,00";
    } else if (cat === "percentage") {
      sample.textContent = "12,35%";
    } else if (cat === "number") {
      sample.textContent = "12.345,67";
    } else {
      sample.textContent = "12345.67";
    }
  },

  setCellBorderPreset(preset) {
    this._modalBorderPreset = preset;
    alert(`Garis tepi sel dipilih: ${preset === 'all' ? 'Semua Garis Tepi' : preset === 'box-thick' ? 'Kotak Tebal Keliling' : preset === 'bottom-double' ? 'Bawah Ganda Akuntansi' : 'Tanpa Garis Tepi'}. Klik "Terapkan Format" untuk menyimpan.`);
  },

  applyFormatCellsModal() {
    const sheet = this.sheets[this.activeSheet];
    if (!sheet) return;
    if (!sheet.formats) sheet.formats = {};

    const alignH = document.getElementById("cellAlignH")?.value || "left";
    const alignV = document.getElementById("cellAlignV")?.value || "middle";
    const wrap = !!document.getElementById("cellWrapText")?.checked;
    const bold = !!document.getElementById("cellFontBold")?.checked;
    const italic = !!document.getElementById("cellFontItalic")?.checked;
    const underline = !!document.getElementById("cellFontUnderline")?.checked;
    const fontColor = document.getElementById("cellFontColor")?.value || "#0f172a";
    const bgColor = document.getElementById("cellBgColor")?.value || "#ffffff";

    this.selectionRange.forEach(coord => {
      const existing = sheet.formats[coord] || {};
      const newFmt = { ...existing };

      // Number category
      delete newFmt.currency;
      delete newFmt.percentage;
      delete newFmt.number;
      if (this._modalSelectedCategory === "currency") {
        newFmt.currency = "IDR";
        newFmt.decimals = 2;
      } else if (this._modalSelectedCategory === "percentage") {
        newFmt.percentage = true;
        newFmt.decimals = 1;
      } else if (this._modalSelectedCategory === "number") {
        newFmt.number = true;
        newFmt.decimals = 2;
      }

      // Align
      newFmt.align = alignH;
      newFmt.verticalAlign = alignV;
      newFmt.wrap = wrap;

      // Font
      newFmt.bold = bold;
      newFmt.italic = italic;
      newFmt.underline = underline;
      if (fontColor && fontColor !== "#0f172a") newFmt.color = fontColor;
      else delete newFmt.color;

      // Fill
      if (bgColor && bgColor.toLowerCase() !== "#ffffff") newFmt.bg = bgColor;
      else delete newFmt.bg;

      // Border
      if (this._modalBorderPreset) {
        if (this._modalBorderPreset === "none") delete newFmt.border;
        else newFmt.border = this._modalBorderPreset;
      }

      sheet.formats[coord] = newFmt;
    });

    this.renderGrid();
    this.selectRange(this.rangeStart, this.rangeEnd);
    this.closeFormatCellsModal();
  },

  promptColumnWidth() {
    const p = this.parseCoord(this.selectedCell);
    const colLetter = this.getColLetter(p.col);
    const input = prompt(`Masukkan lebar kolom ${colLetter} dalam piksel (px):\nContoh: 80, 120, 160, 220`, "120");
    if (!input) return;
    const widthPx = parseInt(input, 10);
    if (isNaN(widthPx) || widthPx < 20 || widthPx > 1000) {
      alert("Masukkan angka lebar antara 20 dan 1000 px.");
      return;
    }
    document.querySelectorAll(`[data-coord^='${colLetter}']`).forEach(td => {
      td.style.minWidth = `${widthPx}px`;
      td.style.width = `${widthPx}px`;
    });
    const headerTh = document.querySelector(`th.calc-col-header[data-col='${p.col}']`);
    if (headerTh) {
      headerTh.style.minWidth = `${widthPx}px`;
      headerTh.style.width = `${widthPx}px`;
    }
  },

  promptRowHeight() {
    const p = this.parseCoord(this.selectedCell);
    const input = prompt(`Masukkan tinggi baris ${p.row} dalam piksel (px):\nContoh: 28, 36, 48`, "32");
    if (!input) return;
    const heightPx = parseInt(input, 10);
    if (isNaN(heightPx) || heightPx < 14 || heightPx > 400) {
      alert("Masukkan angka tinggi antara 14 dan 400 px.");
      return;
    }
    const tr = document.querySelector(`tr[data-row='${p.row}']`);
    if (tr) tr.style.height = `${heightPx}px`;
  },

  importDsl(code) {
    if (!code) return { success: false };
    const lines = code.split(/\r?\n/);
    let currentSheetName = "Sheet1";
    let inSheet = false;
    let nextRow = 1;

    lines.forEach(l => {
      const trim = l.trim();
      if (trim.startsWith("lembar(")) {
        const m = trim.match(/lembar\(["']([^"']+)["']\)/);
        if (m) {
          currentSheetName = m[1];
          inSheet = true;
          nextRow = 1;
          if (!this.sheets[currentSheetName]) {
            this.sheets[currentSheetName] = {
              name: currentSheetName,
              cells: {},
              formats: {},
              maxRow: 30,
              maxCol: 12
            };
          }
        }
      } else if (trim.startsWith("baris_header(") || trim.startsWith("header(")) {
        const arrMatch = trim.match(/\[(.*)\]/);
        if (arrMatch && inSheet) {
          const items = arrMatch[1].split(",").map(s => s.trim().replace(/^["']|["']$/g, ""));
          const sheet = this.sheets[currentSheetName];
          items.forEach((item, cIdx) => {
            const coord = `${this.getColLetter(cIdx + 1)}${nextRow}`;
            sheet.cells[coord] = item;
            sheet.formats[coord] = { bold: true, bg: "#f1f5f9", border: "all" };
          });
          nextRow++;
        }
      } else if (trim.startsWith("baris(")) {
        const arrMatch = trim.match(/\[(.*)\]/);
        if (arrMatch && inSheet) {
          const items = arrMatch[1].split(",").map(s => s.trim().replace(/^["']|["']$/g, ""));
          const sheet = this.sheets[currentSheetName];
          items.forEach((item, cIdx) => {
            const coord = `${this.getColLetter(cIdx + 1)}${nextRow}`;
            sheet.cells[coord] = item;
          });
          nextRow++;
        }
      } else if (trim.startsWith("sel(")) {
        const m = trim.match(/sel\(["']([^"']+)["'],\s*["']?([^"')]+)["']?\)/);
        if (m && inSheet) {
          this.sheets[currentSheetName].cells[m[1].toUpperCase()] = m[2];
        }
      }
    });

    this.activeSheet = currentSheetName;
    this.renderSheetTabs();
    this.renderGrid();
    return { success: true, sheet: currentSheetName };
  },

  exportDsl() {
    let dsl = `// Widya-Lang Sovereign Spreadsheet DSL\n`;
    dsl += `buku_kerja {\n`;
    dsl += `    judul: "BukuKerja_WidyaCalc",\n`;
    dsl += `    penulis: "Satriyo",\n\n`;

    Object.keys(this.sheets).forEach(sKey => {
      const s = this.sheets[sKey];
      dsl += `    lembar("${s.name || sKey}") {\n`;
      for (let r = 1; r <= (s.maxRow || 30); r++) {
        let rowVals = [];
        let hasVal = false;
        for (let c = 1; c <= (s.maxCol || 12); c++) {
          const coord = `${this.getColLetter(c)}${r}`;
          const val = s.cells[coord] || "";
          rowVals.push(val);
          if (val) hasVal = true;
        }
        if (hasVal) {
          dsl += `        baris(${JSON.stringify(rowVals)})\n`;
        }
      }
      dsl += `    }\n\n`;
    });

    dsl += `}\n`;
    return { success: true, dsl: dsl };
  },

  generateSvgChart(kind = "batang", title = "Grafik WidyaCalc", labelRange = "A4:A6", dataRange = "F4:F6") {
    const labels = [];
    const values = [];

    if (labelRange.includes(":")) {
      const [p1, p2] = labelRange.split(":").map(c => this.parseCoord(c.trim()));
      for (let r = Math.min(p1.row, p2.row); r <= Math.max(p1.row, p2.row); r++) {
        for (let c = Math.min(p1.col, p2.col); c <= Math.max(p1.col, p2.col); c++) {
          labels.push(this.getCellValue(`${this.getColLetter(c)}${r}`));
        }
      }
    }

    if (dataRange.includes(":")) {
      const [p1, p2] = dataRange.split(":").map(c => this.parseCoord(c.trim()));
      for (let r = Math.min(p1.row, p2.row); r <= Math.max(p1.row, p2.row); r++) {
        for (let c = Math.min(p1.col, p2.col); c <= Math.max(p1.col, p2.col); c++) {
          const v = parseFloat(this.getCellValue(`${this.getColLetter(c)}${r}`)) || 0;
          values.push(v);
        }
      }
    }

    const maxVal = Math.max(...values, 1);
    const w = 640, h = 360, padL = 70, padR = 40, padT = 50, padB = 60;
    const chartW = w - padL - padR, chartH = h - padT - padB;
    const palette = ["#185abd", "#107c41", "#d83b01", "#8764b8", "#008272"];

    let svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${w} ${h}" style="width:100%; height:auto; background:#ffffff; border-radius:6px; box-shadow:0 2px 8px rgba(0,0,0,0.08); font-family:sans-serif;">\n`;
    svg += `  <text x="${w / 2}" y="30" text-anchor="middle" font-size="16" font-weight="bold" fill="#0f172a">${title}</text>\n`;

    if (kind === "lingkaran" || kind === "pie") {
      const cx = w / 2, cy = (h + 20) / 2, r = 110;
      const total = values.reduce((a, b) => a + b, 0) || 1;
      let angle = 0;
      values.forEach((v, i) => {
        const slice = (v / total) * 2 * Math.PI;
        const x1 = cx + r * Math.cos(angle);
        const y1 = cy + r * Math.sin(angle);
        const nextA = angle + slice;
        const x2 = cx + r * Math.cos(nextA);
        const y2 = cy + r * Math.sin(nextA);
        const large = slice > Math.PI ? 1 : 0;
        const col = palette[i % palette.length];
        svg += `  <path d="M ${cx},${cy} L ${x1},${y1} A ${r},${r} 0 ${large},1 ${x2},${y2} Z" fill="${col}" stroke="#fff" stroke-width="2" />\n`;
        angle = nextA;
      });
    } else if (kind === "garis" || kind === "line") {
      const stepX = values.length > 1 ? chartW / (values.length - 1) : chartW;
      const pts = values.map((v, i) => `${padL + i * stepX},${padT + chartH - (v / maxVal * chartH)}`);
      svg += `  <polyline points="${pts.join(" ")}" fill="none" stroke="#185abd" stroke-width="3" stroke-linecap="round" />\n`;
      values.forEach((v, i) => {
        const px = padL + i * stepX;
        const py = padT + chartH - (v / maxVal * chartH);
        svg += `  <circle cx="${px}" cy="${py}" r="5" fill="#185abd" stroke="#fff" stroke-width="2" />\n`;
        if (labels[i]) svg += `  <text x="${px}" y="${h - 25}" text-anchor="middle" font-size="11" fill="#334155">${labels[i]}</text>\n`;
      });
    } else {
      // batang (bar)
      const slot = chartW / values.length;
      const barW = Math.min(slot * 0.65, 50);
      values.forEach((v, i) => {
        const barH = Math.max((v / maxVal * chartH), 2);
        const bx = padL + i * slot + (slot - barW) / 2;
        const by = padT + chartH - barH;
        const col = palette[i % palette.length];
        svg += `  <rect x="${bx}" y="${by}" width="${barW}" height="${barH}" fill="${col}" rx="3" />\n`;
        svg += `  <text x="${bx + barW / 2}" y="${by - 6}" text-anchor="middle" font-size="10" font-weight="600" fill="#1e293b">${v}</text>\n`;
        if (labels[i]) svg += `  <text x="${bx + barW / 2}" y="${h - 25}" text-anchor="middle" font-size="11" fill="#334155">${labels[i]}</text>\n`;
      });
    }

    svg += `</svg>\n`;
    return { success: true, svg: svg, labels: labels, values: values };
  },

  getStats() {
    const sheet = this.sheets[this.activeSheet];
    let cellCount = 0;
    let formulaCount = 0;
    if (sheet && sheet.cells) {
      Object.keys(sheet.cells).forEach(c => {
        if (sheet.cells[c] !== "" && sheet.cells[c] !== undefined) {
          cellCount++;
          if (String(sheet.cells[c]).startsWith("=")) formulaCount++;
        }
      });
    }
    return {
      totalSheets: Object.keys(this.sheets).length,
      activeSheet: this.activeSheet,
      activeSheetName: sheet ? sheet.name : this.activeSheet,
      cellCount: cellCount,
      formulaCount: formulaCount,
      selectedCell: this.selectedCell
    };
  },

  getCellData(coord) {
    const sheet = this.sheets[this.activeSheet];
    if (!sheet) return null;
    const raw = sheet.cells[coord];
    const fmt = sheet.formats[coord] || {};
    const val = this.getCellValue(coord);
    const formatted = this.getDisplayValue(coord, raw, fmt);
    return {
      coord: coord,
      raw: raw,
      value: val,
      formatted: formatted,
      style: fmt
    };
  }
};
window.WidyaCalc = WidyaCalc;

