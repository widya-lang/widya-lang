/**
 * WidyaOffice Desktop - Math Editor & OMML Parser Controller
 * Zero External Dependencies
 */

const MathEditor = {
  currentLatex: "\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}",
  isBlock: true,

  init() {
    const input = document.getElementById("latexInput");
    if (input) {
      input.value = this.currentLatex;
      input.addEventListener("input", (e) => {
        this.currentLatex = e.target.value;
        this.updatePreview();
      });
    }

    this.bindPaletteButtons();
    this.updatePreview();
  },

  bindPaletteButtons() {
    // Template buttons
    document.querySelectorAll(".math-tpl-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        const snippet = btn.getAttribute("data-snippet");
        if (snippet) {
          this.insertSnippet(snippet);
        }
      });
    });

    // Symbol grid buttons
    document.querySelectorAll(".math-sym-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        const sym = btn.getAttribute("data-sym");
        if (sym) {
          this.insertSnippet(sym + " ");
        }
      });
    });
  },

  insertSnippet(text) {
    const input = document.getElementById("latexInput");
    if (!input) return;

    const start = input.selectionStart;
    const end = input.selectionEnd;
    const val = input.value;

    input.value = val.substring(0, start) + text + val.substring(end);
    input.focus();
    input.selectionStart = input.selectionEnd = start + text.length;

    this.currentLatex = input.value;
    this.updatePreview();
  },

  updatePreview() {
    const previewEl = document.getElementById("mathPreview");
    if (!previewEl) return;

    // Fast vector render fallback
    let rendered = this.simpleLatexToHtml(this.currentLatex);
    previewEl.innerHTML = rendered;
  },

  // Lightweight standalone LaTeX to HTML mathematical symbol renderer
  simpleLatexToHtml(latex) {
    if (!latex || !latex.trim()) return "<span style='color:#94a3b8;'>Ketik rumus atau klik palet simbol di atas...</span>";

    let html = latex
      .replace(/\\frac\{([^}]+)\}\{([^}]+)\}/g, "<span style='display:inline-flex; flex-direction:column; vertical-align:middle; text-align:center; margin:0 4px;'><span style='border-bottom:1.5px solid #1e293b; padding:0 2px;'>$1</span><span style='padding:0 2px;'>$2</span></span>")
      .replace(/\\sqrt\[([^}]+)\]\{([^}]+)\}/g, "<sup>$1</sup>√<span style='border-top:1.5px solid #1e293b; padding:0 2px;'>$2</span>")
      .replace(/\\sqrt\{([^}]+)\}/g, "√<span style='border-top:1.5px solid #1e293b; padding:0 2px;'>$1</span>")
      .replace(/\^\{([^}]+)\}/g, "<sup>$1</sup>")
      .replace(/\^([0-9a-zA-Z])/g, "<sup>$1</sup>")
      .replace(/_\{([^}]+)\}/g, "<sub>$1</sub>")
      .replace(/_([0-9a-zA-Z])/g, "<sub>$1</sub>")
      .replace(/\\pm/g, "±")
      .replace(/\\times/g, "×")
      .replace(/\\div/g, "÷")
      .replace(/\\leq/g, "≤")
      .replace(/\\geq/g, "≥")
      .replace(/\\neq/g, "≠")
      .replace(/\\approx/g, "≈")
      .replace(/\\infty/g, "∞")
      .replace(/\\int/g, "∫")
      .replace(/\\sum/g, "∑")
      .replace(/\\alpha/g, "α")
      .replace(/\\beta/g, "β")
      .replace(/\\gamma/g, "γ")
      .replace(/\\theta/g, "θ")
      .replace(/\\pi/g, "π")
      .replace(/\\lambda/g, "λ")
      .replace(/\\omega/g, "ω")
      .replace(/\\Delta/g, "Δ")
      .replace(/\\rightarrow/g, "→")
      .replace(/\\Rightarrow/g, "⇒");

    return `<span style="font-family:'Cambria Math',serif; font-size:1.25em;">${html}</span>`;
  },

  openDialog() {
    const modal = document.getElementById("mathModal");
    if (modal) {
      modal.classList.add("active");
      const input = document.getElementById("latexInput");
      if (input) input.focus();
    }
  },

  closeDialog() {
    const modal = document.getElementById("mathModal");
    if (modal) modal.classList.remove("active");
  },

  confirmInsert() {
    const isBlockCheck = document.getElementById("isBlockEquation");
    const isBlock = isBlockCheck ? isBlockCheck.checked : true;

    OfficeCore.insertEquationAtCursor(this.currentLatex, isBlock);
    this.closeDialog();
  }
};
