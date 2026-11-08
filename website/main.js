// ==========================================================================
// Widya-Lang Portal Main Scripts
// Theme Toggle, Navigation, Copy Snippets & Interactions
// ==========================================================================

// 1. Theme Management (Dark / Light Kalem)
(function initTheme() {
  const saved = localStorage.getItem('widya-theme');
  if (saved === 'light') {
    document.documentElement.classList.add('light');
  } else {
    document.documentElement.classList.remove('light');
  }
})();

function toggleTheme() {
  const isLight = document.documentElement.classList.toggle('light');
  localStorage.setItem('widya-theme', isLight ? 'light' : 'dark');
  updateThemeIcons();
}

function updateThemeIcons() {
  const isLight = document.documentElement.classList.contains('light');
  document.querySelectorAll('.theme-toggle').forEach(btn => {
    btn.innerHTML = isLight ? '🌙' : '☀️';
    btn.setAttribute('title', isLight ? 'Beralih ke Tema Gelap Kalem' : 'Beralih ke Tema Terang Kalem');
  });
}

document.addEventListener('DOMContentLoaded', () => {
  updateThemeIcons();

  // Mobile navigation toggle
  const mobileBtn = document.querySelector('.mobile-toggle');
  const navMenu = document.querySelector('.nav-menu');
  if (mobileBtn && navMenu) {
    mobileBtn.addEventListener('click', () => {
      const isVisible = navMenu.style.display === 'flex';
      navMenu.style.display = isVisible ? 'none' : 'flex';
      if (!isVisible) {
        navMenu.style.flexDirection = 'column';
        navMenu.style.position = 'absolute';
        navMenu.style.top = '100%';
        navMenu.style.left = '0';
        navMenu.style.width = '100%';
        navMenu.style.background = 'var(--bg-card)';
        navMenu.style.padding = '1rem 1.5rem';
        navMenu.style.borderBottom = '1px solid var(--border-subtle)';
      }
    });
  }
});

// 2. Clipboard Helper
function copyText(elementId, btnElement) {
  const el = document.getElementById(elementId);
  if (!el) return;
  const text = el.innerText || el.textContent;
  navigator.clipboard.writeText(text.trim()).then(() => {
    if (btnElement) {
      const original = btnElement.innerText;
      btnElement.innerText = 'Tersalin! ✓';
      btnElement.style.background = 'var(--primary)';
      btnElement.style.color = '#ffffff';
      setTimeout(() => {
        btnElement.innerText = original;
        btnElement.style.background = '';
        btnElement.style.color = '';
      }, 2000);
    }
  });
}
