/**
 * WidyaBlok - Studio UI Controller
 * Bridges the user interactions, block editor workspace, sprite manager, and runtime engine.
 */

document.addEventListener('DOMContentLoaded', () => {
  // Initialize Engine
  const engine = new WidyaBlokStageEngine();

  // UI State
  let currentCategory = 'motion';
  let activeTab = 'blocks';
  let zoomLevel = 1.0;

  // DOM Elements
  const paletteList = document.getElementById('palette-blocks-list');
  const paletteCatTitle = document.getElementById('palette-cat-title');
  const catButtons = document.querySelectorAll('.cat-btn');
  const workspaceTabs = document.querySelectorAll('.workspace-tab');
  const workspaceContents = document.querySelectorAll('.workspace-content');
  const stackContainer = document.getElementById('blocks-stack-container');
  const workspaceHint = document.getElementById('workspace-hint');
  const liveWidyaCode = document.getElementById('live-widya-code');
  const terminalLogs = document.getElementById('terminal-logs');

  // Nav buttons
  const btnGreenFlag = document.getElementById('btn-green-flag');
  const btnStopAll = document.getElementById('btn-stop-all');
  const btnTurboMode = document.getElementById('btn-turbo-mode');
  const btnExportCode = document.getElementById('btn-export-code');
  const btnSaveProject = document.getElementById('btn-save-project');
  const btnLoadProject = document.getElementById('btn-load-project');
  const fileProjectLoader = document.getElementById('file-project-loader');
  const examplesDropdown = document.querySelector('.dropdown');
  const examplesMenu = document.getElementById('examples-menu');
  const projectTitleInput = document.getElementById('project-title-input');

  // Sprite Props
  const propName = document.getElementById('prop-name');
  const propX = document.getElementById('prop-x');
  const propY = document.getElementById('prop-y');
  const propSize = document.getElementById('prop-size');
  const propDir = document.getElementById('prop-dir');
  const btnToggleVis = document.getElementById('btn-toggle-vis');
  const spritesGrid = document.getElementById('sprites-grid');
  const btnAddSprite = document.getElementById('btn-add-sprite');
  const spriteLibraryMenu = document.getElementById('sprite-library-menu');
  const selectBackdrop = document.getElementById('select-backdrop');
  const backdropNameLabel = document.getElementById('backdrop-name-label');

  // Stage Tools
  const btnToggleGrid = document.getElementById('btn-toggle-grid');
  const btnStageFullscreen = document.getElementById('btn-stage-fullscreen');
  const canvasViewport = document.getElementById('canvas-viewport');

  // Modal
  const modalExport = document.getElementById('modal-export');
  const btnCloseModal = document.getElementById('btn-close-modal');
  const modalCodeTextarea = document.getElementById('modal-code-textarea');
  const btnModalCopy = document.getElementById('btn-modal-copy');
  const btnModalDownload = document.getElementById('btn-modal-download');
  const btnCopyWidya = document.getElementById('btn-copy-widya');
  const btnDownloadWya = document.getElementById('btn-download-wya');
  const btnClearTerminal = document.getElementById('btn-clear-terminal');
  const btnClearWorkspace = document.getElementById('btn-clear-workspace');
  const btnZoomIn = document.getElementById('btn-zoom-in');
  const btnZoomReset = document.getElementById('btn-zoom-reset');
  const btnZoomOut = document.getElementById('btn-zoom-out');

  // Engine callbacks
  engine.logCallback = (text, type) => {
    const entry = document.createElement('div');
    entry.className = `log-entry ${type}`;
    entry.textContent = text;
    terminalLogs.appendChild(entry);
    terminalLogs.scrollTop = terminalLogs.scrollHeight;
  };

  engine.stepHighlightCallback = (blockId, isHighlight) => {
    if (!blockId) return;
    const el = document.querySelector(`[data-block-id="${blockId}"]`);
    if (el) {
      if (isHighlight) el.classList.add('executing');
      else el.classList.remove('executing');
    }
  };

  window.onEngineStopped = () => {
    btnGreenFlag.classList.remove('running');
    document.querySelectorAll('.block.executing').forEach(el => el.classList.remove('executing'));
  };

  // =========================================================================
  // 1. Initial Load (Load First Example Project)
  // =========================================================================
  loadProject(PREBUILT_EXAMPLES.animasi_kucing);

  // =========================================================================
  // 2. Palette & Category Switcher
  // =========================================================================
  function renderPalette(categoryKey) {
    currentCategory = categoryKey;
    paletteList.innerHTML = '';

    const catNameMap = {
      motion: 'Gerakan',
      looks: 'Tampilan',
      sound: 'Suara & Musik',
      events: 'Kejadian',
      control: 'Kontrol & Alur',
      sensing: 'Sensor & Interaksi',
      operators: 'Operator & Logika',
      variables: 'Variabel & Data',
      pen: 'Pena Seni Grafika',
      widya: 'Widya Khusus',
    };
    paletteCatTitle.textContent = catNameMap[categoryKey] || 'Blok';

    Object.entries(BLOCK_DEFINITIONS).forEach(([opcode, def]) => {
      if (def.category !== categoryKey) return;

      const blockEl = createBlockDOM(opcode, def.defaultArgs, true);
      paletteList.appendChild(blockEl);
    });
  }

  catButtons.forEach(btn => {
    btn.addEventListener('click', () => {
      catButtons.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      renderPalette(btn.dataset.cat);
    });
  });

  renderPalette('motion');

  // =========================================================================
  // 3. Block DOM Construction & Drag/Click Handling
  // =========================================================================
  function createBlockDOM(opcode, args = {}, isPalette = false, blockId = null) {
    const def = BLOCK_DEFINITIONS[opcode];
    if (!def) return document.createElement('div');

    const id = blockId || 'blk_' + Math.random().toString(36).substring(2, 9);
    const blockEl = document.createElement('div');
    blockEl.className = `block ${def.type === 'hat' ? 'hat-block' : ''}`;
    blockEl.dataset.opcode = opcode;
    blockEl.dataset.category = def.category;
    blockEl.dataset.blockId = id;
    blockEl.draggable = true;

    // Build inner HTML with current arguments
    let html = def.label;
    for (const [argKey, argVal] of Object.entries(args)) {
      html = html.replace(new RegExp(`data-arg="${argKey}" value="[^"]*"`, 'g'), `data-arg="${argKey}" value="${argVal}"`);
    }

    // Set select selected options
    for (const [argKey, argVal] of Object.entries(args)) {
      html = html.replace(new RegExp(`(<select[^>]*data-arg="${argKey}"[^>]*>[\\s\\S]*?<option value="${argVal}")`, 'g'), `$1 selected`);
    }

    blockEl.innerHTML = html;

    // Attach Input Change Listeners for live sync
    blockEl.querySelectorAll('input, select').forEach(inp => {
      inp.addEventListener('input', () => {
        syncBlockArgs(id);
        updateLiveCode();
      });
      inp.addEventListener('change', () => {
        syncBlockArgs(id);
        updateLiveCode();
      });
      // Prevent drag initiation on inputs
      inp.addEventListener('mousedown', (e) => e.stopPropagation());
    });

    // If on workspace, add delete icon
    if (!isPalette) {
      const delBtn = document.createElement('button');
      delBtn.className = 'block-del-btn';
      delBtn.title = 'Hapus Blok';
      delBtn.innerHTML = '✕';
      delBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        deleteBlock(id);
      });
      blockEl.appendChild(delBtn);
    }

    // Drag and Drop Events
    blockEl.addEventListener('dragstart', (e) => {
      e.dataTransfer.setData('text/plain', JSON.stringify({
        opcode,
        args: extractArgsFromDOM(blockEl),
        isPalette,
        sourceBlockId: isPalette ? null : id,
      }));
      e.dataTransfer.effectAllowed = 'copyMove';
    });

    // If clicked on palette, instantly append to active sprite workspace
    if (isPalette) {
      blockEl.addEventListener('click', () => {
        addBlockToActiveSprite(opcode, extractArgsFromDOM(blockEl));
      });
    }

    // If C-Block, wrap inside structure
    if (def.type === 'c_block') {
      const wrapper = document.createElement('div');
      wrapper.className = 'c-block-wrapper';
      wrapper.dataset.wrapperId = id;

      wrapper.appendChild(blockEl);

      const body = document.createElement('div');
      body.className = 'c-block-body';
      body.dataset.cbodyId = id;
      wrapper.appendChild(body);

      const endCap = document.createElement('div');
      endCap.className = 'c-block-end';
      wrapper.appendChild(endCap);

      return wrapper;
    }

    return blockEl;
  }

  function extractArgsFromDOM(el) {
    const args = {};
    el.querySelectorAll('[data-arg]').forEach(inp => {
      args[inp.dataset.arg] = inp.value;
    });
    return args;
  }

  function syncBlockArgs(blockId) {
    const sprite = engine.getActiveSprite();
    if (!sprite) return;

    function searchAndSync(list) {
      for (const item of list) {
        if (item.id === blockId) {
          const dom = document.querySelector(`[data-block-id="${blockId}"]`);
          if (dom) item.args = extractArgsFromDOM(dom);
          return true;
        }
        if (item.children && searchAndSync(item.children)) return true;
      }
      return false;
    }

    (sprite.stacks || []).forEach(stack => searchAndSync(stack));
  }

  function addBlockToActiveSprite(opcode, args) {
    const sprite = engine.getActiveSprite();
    if (!sprite) return;
    if (!sprite.stacks) sprite.stacks = [];

    const def = BLOCK_DEFINITIONS[opcode];
    const newBlock = {
      id: 'blk_' + Math.random().toString(36).substring(2, 9),
      opcode,
      args: { ...(def.defaultArgs || {}), ...args },
      children: def.type === 'c_block' ? [] : undefined,
    };

    // If it's a Hat block or stacks are empty, start new stack
    if (def.type === 'hat' || sprite.stacks.length === 0) {
      sprite.stacks.push([newBlock]);
    } else {
      // Append to the last stack
      sprite.stacks[sprite.stacks.length - 1].push(newBlock);
    }

    renderWorkspaceBlocks();
    updateLiveCode();
    engine.audio.playSound('pop');
  }

  function deleteBlock(blockId) {
    const sprite = engine.getActiveSprite();
    if (!sprite) return;

    function removeRecursive(list) {
      const idx = list.findIndex(b => b.id === blockId);
      if (idx !== -1) {
        list.splice(idx, 1);
        return true;
      }
      for (const item of list) {
        if (item.children && removeRecursive(item.children)) return true;
      }
      return false;
    }

    sprite.stacks = sprite.stacks.filter(stack => {
      removeRecursive(stack);
      return stack.length > 0;
    });

    renderWorkspaceBlocks();
    updateLiveCode();
  }

  // Drop onto workspace canvas
  const canvasWorkspace = document.getElementById('workspace-canvas');
  canvasWorkspace.addEventListener('dragover', (e) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  });

  canvasWorkspace.addEventListener('drop', (e) => {
    e.preventDefault();
    try {
      const data = JSON.parse(e.dataTransfer.getData('text/plain'));
      if (data && data.opcode) {
        if (data.isPalette) {
          addBlockToActiveSprite(data.opcode, data.args);
        }
      }
    } catch (err) {
      console.error(err);
    }
  });

  // =========================================================================
  // 4. Render Workspace Blocks from Active Sprite Model
  // =========================================================================
  function renderWorkspaceBlocks() {
    stackContainer.innerHTML = '';
    const sprite = engine.getActiveSprite();
    if (!sprite || !sprite.stacks || sprite.stacks.length === 0) {
      workspaceHint.classList.remove('hidden');
      return;
    }
    workspaceHint.classList.add('hidden');

    sprite.stacks.forEach(stack => {
      const stackDiv = document.createElement('div');
      stackDiv.className = 'block-stack';

      stack.forEach(block => {
        const dom = renderBlockTree(block);
        stackDiv.appendChild(dom);
      });

      stackContainer.appendChild(stackDiv);
    });
  }

  function renderBlockTree(block) {
    const def = BLOCK_DEFINITIONS[block.opcode];
    const dom = createBlockDOM(block.opcode, block.args, false, block.id);

    if (def && def.type === 'c_block' && block.children) {
      const cbody = dom.querySelector('.c-block-body');
      if (cbody) {
        block.children.forEach(child => {
          cbody.appendChild(renderBlockTree(child));
        });
      }
    }
    return dom;
  }

  function updateLiveCode() {
    const sprite = engine.getActiveSprite();
    const code = transpileStacksToWidya(sprite ? sprite.stacks : []);
    liveWidyaCode.textContent = code;
  }

  // =========================================================================
  // 5. Sprite Management
  // =========================================================================
  function updateSpritesUI() {
    spritesGrid.innerHTML = '';
    engine.sprites.forEach(sprite => {
      const card = document.createElement('div');
      card.className = `sprite-card ${sprite.id === engine.activeSpriteId ? 'active' : ''}`;
      
      const icons = {
        garuda: '🦅', kucing: '🐱', burung: '🦜', apel: '🍎',
        bintang: '⭐', roket: '🚀', robot: '🤖'
      };
      
      card.innerHTML = `
        <span class="sprite-icon">${icons[sprite.type] || '👾'}</span>
        <span class="sprite-label">${sprite.name}</span>
        ${engine.sprites.length > 1 ? `<button class="sprite-card-del" title="Hapus">✕</button>` : ''}
      `;

      card.addEventListener('click', () => {
        engine.activeSpriteId = sprite.id;
        syncPropsFromActiveSprite();
        updateSpritesUI();
        renderWorkspaceBlocks();
        updateLiveCode();
      });

      const del = card.querySelector('.sprite-card-del');
      if (del) {
        del.addEventListener('click', (e) => {
          e.stopPropagation();
          deleteSprite(sprite.id);
        });
      }

      spritesGrid.appendChild(card);
    });

    syncPropsFromActiveSprite();
  }

  function syncPropsFromActiveSprite() {
    const sprite = engine.getActiveSprite();
    if (!sprite) return;
    propName.value = sprite.name;
    propX.value = Math.round(sprite.x);
    propY.value = Math.round(sprite.y);
    propSize.value = sprite.size;
    propDir.value = sprite.direction;
    btnToggleVis.textContent = sprite.visible ? '👁️' : '🙈';
  }

  // Inputs for sprite props
  [propX, propY, propSize, propDir].forEach(inp => {
    inp.addEventListener('input', () => {
      const sprite = engine.getActiveSprite();
      if (!sprite) return;
      sprite.x = Number(propX.value) || 0;
      sprite.y = Number(propY.value) || 0;
      sprite.size = Number(propSize.value) || 100;
      sprite.direction = Number(propDir.value) || 90;
    });
  });

  btnToggleVis.addEventListener('click', () => {
    const sprite = engine.getActiveSprite();
    if (!sprite) return;
    sprite.visible = !sprite.visible;
    btnToggleVis.textContent = sprite.visible ? '👁️' : '🙈';
  });

  // Add Sprite dropdown
  btnAddSprite.addEventListener('click', (e) => {
    e.stopPropagation();
    document.querySelector('.add-sprite-dropdown').classList.toggle('open');
  });

  document.querySelectorAll('.menu-choice').forEach(choice => {
    choice.addEventListener('click', () => {
      const type = choice.dataset.sprite;
      const count = engine.sprites.length + 1;
      const newSprite = {
        id: 'sprite-' + Math.random().toString(36).substring(2, 7),
        name: type.charAt(0).toUpperCase() + type.slice(1) + count,
        type: type,
        x: Math.floor(Math.random() * 100 - 50),
        y: Math.floor(Math.random() * 100 - 50),
        size: 100,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Halo!', SECS: 2 } }
          ]
        ]
      };
      engine.sprites.push(newSprite);
      engine.activeSpriteId = newSprite.id;
      document.querySelector('.add-sprite-dropdown').classList.remove('open');
      updateSpritesUI();
      renderWorkspaceBlocks();
      updateLiveCode();
    });
  });

  function deleteSprite(spriteId) {
    engine.sprites = engine.sprites.filter(s => s.id !== spriteId);
    if (engine.activeSpriteId === spriteId && engine.sprites.length > 0) {
      engine.activeSpriteId = engine.sprites[0].id;
    }
    updateSpritesUI();
    renderWorkspaceBlocks();
    updateLiveCode();
  }

  // Backdrop select
  selectBackdrop.addEventListener('change', () => {
    engine.backdrop = selectBackdrop.value;
    backdropNameLabel.textContent = selectBackdrop.options[selectBackdrop.selectedIndex].text;
  });

  // =========================================================================
  // 6. Navigation Tabs
  // =========================================================================
  workspaceTabs.forEach(tab => {
    tab.addEventListener('click', () => {
      workspaceTabs.forEach(t => t.classList.remove('active'));
      workspaceContents.forEach(c => c.classList.remove('active'));

      tab.classList.add('active');
      activeTab = tab.dataset.tab;
      document.getElementById(`tab-content-${activeTab}`).classList.add('active');

      if (activeTab === 'widya-code') {
        updateLiveCode();
      }
    });
  });

  // =========================================================================
  // 7. Green Flag & Controls
  // =========================================================================
  btnGreenFlag.addEventListener('click', () => {
    btnGreenFlag.classList.add('running');
    engine.startGreenFlag();
  });

  btnStopAll.addEventListener('click', () => {
    btnGreenFlag.classList.remove('running');
    engine.stopAll();
  });

  btnTurboMode.addEventListener('click', () => {
    engine.turboMode = !engine.turboMode;
    btnTurboMode.classList.toggle('active', engine.turboMode);
  });

  btnToggleGrid.addEventListener('click', () => {
    engine.showGrid = !engine.showGrid;
    btnToggleGrid.style.background = engine.showGrid ? '#3b82f6' : '';
    btnToggleGrid.style.color = engine.showGrid ? '#fff' : '';
  });

  btnStageFullscreen.addEventListener('click', () => {
    if (!document.fullscreenElement) {
      canvasViewport.requestFullscreen().catch(err => alert(err.message));
    } else {
      document.exitFullscreen();
    }
  });

  // =========================================================================
  // 8. Projects & Templates
  // =========================================================================
  function loadProject(proj) {
    projectTitleInput.value = proj.title || 'Proyek WidyaBlok';
    selectBackdrop.value = proj.backdrop || 'grid';
    backdropNameLabel.textContent = selectBackdrop.options[selectBackdrop.selectedIndex].text;
    engine.loadProjectState(proj);
    updateSpritesUI();
    renderWorkspaceBlocks();
    updateLiveCode();
  }

  // Example Projects Dropdown
  document.querySelector('.dropdown-toggle').addEventListener('click', (e) => {
    e.stopPropagation();
    examplesDropdown.classList.toggle('open');
  });

  document.addEventListener('click', () => {
    examplesDropdown.classList.remove('open');
    document.querySelector('.add-sprite-dropdown').classList.remove('open');
  });

  document.querySelectorAll('#examples-menu .dropdown-item').forEach(item => {
    item.addEventListener('click', () => {
      const key = item.dataset.example;
      if (PREBUILT_EXAMPLES[key]) {
        loadProject(PREBUILT_EXAMPLES[key]);
        engine.audio.playSound('koin');
      }
    });
  });

  // Save Project (.json)
  btnSaveProject.addEventListener('click', () => {
    const projectData = {
      title: projectTitleInput.value,
      backdrop: engine.backdrop,
      sprites: engine.sprites,
      version: '1.0.0',
    };
    const blob = new Blob([JSON.stringify(projectData, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${projectTitleInput.value.toLowerCase().replace(/\s+/g, '_')}.json`;
    a.click();
    URL.revokeObjectURL(url);
  });

  // Load Project (.json)
  btnLoadProject.addEventListener('click', () => {
    fileProjectLoader.click();
  });

  fileProjectLoader.addEventListener('change', (e) => {
    const file = e.target.files[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      try {
        const data = JSON.parse(ev.target.result);
        loadProject(data);
      } catch (err) {
        alert('Gagal memuat berkas proyek: ' + err.message);
      }
    };
    reader.readAsText(file);
  });

  // Export Code Modal & Download
  function openExportModal() {
    const code = liveWidyaCode.textContent;
    modalCodeTextarea.value = code;
    modalExport.style.display = 'flex';
  }

  btnExportCode.addEventListener('click', openExportModal);
  btnCloseModal.addEventListener('click', () => { modalExport.style.display = 'none'; });

  function copyCodeToClipboard(text) {
    navigator.clipboard.writeText(text).then(() => {
      alert('Kode Widya-Lang berhasil disalin ke clipboard! 📋');
    });
  }

  btnModalCopy.addEventListener('click', () => copyCodeToClipboard(modalCodeTextarea.value));
  btnCopyWidya.addEventListener('click', () => copyCodeToClipboard(liveWidyaCode.textContent));

  function downloadWyaFile(code) {
    const blob = new Blob([code], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${projectTitleInput.value.toLowerCase().replace(/\s+/g, '_')}.wya`;
    a.click();
    URL.revokeObjectURL(url);
  }

  btnModalDownload.addEventListener('click', () => downloadWyaFile(modalCodeTextarea.value));
  btnDownloadWya.addEventListener('click', () => downloadWyaFile(liveWidyaCode.textContent));

  btnClearTerminal.addEventListener('click', () => {
    terminalLogs.innerHTML = '<div class="log-entry system">[Konsol Dibersihkan]</div>';
  });

  btnClearWorkspace.addEventListener('click', () => {
    if (confirm('Kosongkan semua blok dari sprite yang aktif?')) {
      const sprite = engine.getActiveSprite();
      if (sprite) sprite.stacks = [];
      renderWorkspaceBlocks();
      updateLiveCode();
    }
  });

  // Zoom Controls
  btnZoomIn.addEventListener('click', () => {
    zoomLevel = Math.min(1.6, zoomLevel + 0.1);
    stackContainer.style.transform = `scale(${zoomLevel})`;
    stackContainer.style.transformOrigin = 'top left';
  });
  btnZoomReset.addEventListener('click', () => {
    zoomLevel = 1.0;
    stackContainer.style.transform = 'scale(1)';
  });
  btnZoomOut.addEventListener('click', () => {
    zoomLevel = Math.max(0.6, zoomLevel - 0.1);
    stackContainer.style.transform = `scale(${zoomLevel})`;
    stackContainer.style.transformOrigin = 'top left';
  });
});
