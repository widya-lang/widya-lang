/**
 * office3d.js - Mesin Grafika 3D Kantor Digital Widya (Three.js) v1.2.0-GA
 * Lingkungan Kantor Profesional Skala Industri — 10 AI Agent Berdaulat:
 * - Tata Letak Pod Cluster Realistik (Bukan Melingkar):
 *     1. Executive Suite (Arya Wicaksana, PMP - Mandor Utama)
 *     2. Engineering Pod 2x2 Benching (Rizky Firmansyah & Bambang Prasetyo, Siti Rahmawati & Fajar Nugroho)
 *     3. Business, Governance & Comms Pod 2x2 (Dian Kartika & Hendra Gunawan, Ratna Sari & Bayu Pratama)
 *     4. Analytics Hub (Dewi Lestari, M.Sc. - Kepala Intelijen Data & KPI)
 *     5. Dedicated Glass Boardroom (Ruang Sidang Partisi Kaca dengan Meja Oval & Layar Presentasi 4K)
 *     6. Data Center DevOps 42U dengan Animasi LED Berkedip
 *     7. Pantry Bar & Executive Coffee Lounge (Sofa L, Mesin Espresso)
 * - Pencahayaan Ekstra Terang & Bersih (Ambient 1.4x, Hemi bounce, Key Sun 2.0x, Fill light, Downlights)
 * - Avatar Humanoid Realistik Disesuaikan Gender (Pria / Wanita) dengan Rambut, Blazer/Kemeja, ID Badge
 * - 3D Floating Speech Bubbles di atas Meja Kerja Agen untuk Memantau Percakapan Live
 */

class Office3D {
  constructor(containerId) {
    this.container = document.getElementById(containerId);
    this.agents3D = new Map(); // id -> { mesh, charGroup, deskPos, meetPos, isMeeting }
    this.isNight = false;
    this.camMode = 'iso'; // 'iso', 'top', 'focus'
    this.selectedAgentId = null;
    this.serverLeds = []; // Array of mesh LEDs for animated blinking
    this.speechBubbles = new Map(); // agentId -> sprite mesh
    this.labelsContainer = document.getElementById('labels-container');
    this.showTags = true;

    this.initThree();
    this.setupLighting();
    this.buildOfficeEnvironment();
    this.buildPanoramicWindowsAndSkyline();
    this.buildAcousticWoodWallAndLogo();
    this.buildExecutiveBoardroomDetails();
    this.buildConferenceTable();
    this.buildPantryAndCoffeeLounge();
    this.buildDataCenterAndServerRoom();
    this.buildBiophilicGreenery();
    this.setupRaycaster();
    this.startIdleWalkEngine();
    this.animate();

    window.addEventListener('resize', () => this.onWindowResize());
  }

  initThree() {
    this.scene = new THREE.Scene();
    this.scene.background = new THREE.Color(0x0a0f1d);
    this.scene.fog = new THREE.FogExp2(0x0a0f1d, 0.012); // Fog lembut agar ruangan luas dan jernih

    this.camera = new THREE.PerspectiveCamera(
      42,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    this.camera.position.set(24, 21, 24);

    this.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.45; // Pencahayaan studio terang benderang

    this.container.appendChild(this.renderer.domElement);

    this.controls = new THREE.OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.dampingFactor = 0.05;
    this.controls.maxPolarAngle = Math.PI / 2 - 0.04;
    this.controls.minDistance = 6;
    this.controls.maxDistance = 55;
    this.controls.target.set(0, 1.2, 0);
    this.controls.update();
  }

  setupLighting() {
    // 1. Ambient Light Terang, Bersih, Menjangkau Seluruh Sudut
    this.ambientLight = new THREE.AmbientLight(0xffffff, 1.4);
    this.scene.add(this.ambientLight);

    // 2. Hemisphere Light: Pantulan Alami Langit Biru Lembut & Lantai Hangat
    this.hemiLight = new THREE.HemisphereLight(0xe0f2fe, 0x64748b, 0.95);
    this.hemiLight.position.set(0, 32, 0);
    this.scene.add(this.hemiLight);

    // 3. Directional Sun Key Light yang Kuat & Bersih
    this.sunLight = new THREE.DirectionalLight(0xffffff, 1.95);
    this.sunLight.position.set(28, 38, 22);
    this.sunLight.castShadow = true;
    this.sunLight.shadow.mapSize.width = 2048;
    this.sunLight.shadow.mapSize.height = 2048;
    this.sunLight.shadow.camera.near = 0.5;
    this.sunLight.shadow.camera.far = 95;
    const d = 26;
    this.sunLight.shadow.camera.left = -d;
    this.sunLight.shadow.camera.right = d;
    this.sunLight.shadow.camera.top = d;
    this.sunLight.shadow.camera.bottom = -d;
    this.sunLight.shadow.bias = -0.0003;
    this.scene.add(this.sunLight);

    // 4. Fill Directional Light Menghilangkan Bayangan Gelap di Sisi Kiri
    this.fillLight = new THREE.DirectionalLight(0xdbeafe, 1.15);
    this.fillLight.position.set(-25, 26, -20);
    this.scene.add(this.fillLight);

    // 5. Lampu Aksen Ruang Sidang Kaca
    this.meetingLight = new THREE.PointLight(0x38bdf8, 0.6, 20);
    this.meetingLight.position.set(-8.5, 4.8, 0);
    this.scene.add(this.meetingLight);

    // 6. Lampu Hangat Lounge & Coffee Bar
    this.loungeLamp = new THREE.PointLight(0xf59e0b, 1.0, 15);
    this.loungeLamp.position.set(-10.5, 3.8, 8.5);
    this.scene.add(this.loungeLamp);

    // 7. Lampu Cyber Data Center
    this.serverLamp = new THREE.PointLight(0x06b6d4, 1.2, 14);
    this.serverLamp.position.set(13.5, 3.8, -8.0);
    this.scene.add(this.serverLamp);

    // 8. Ceiling Recessed Downlights di Berbagai Kuadran Kantor
    this.downlights = [];
    const downlightCoords = [
      { x: 6.8, z: -8.0 },  // Engineering Cluster
      { x: 6.8, z: 6.0 },   // Business Cluster
      { x: 6.8, z: -1.0 },  // Executive Suite
      { x: -8.5, z: 0 },    // Boardroom
      { x: -10.5, z: 8.5 }  // Pantry Lounge
    ];
    downlightCoords.forEach(pos => {
      const pl = new THREE.PointLight(0xfffbeb, 0.7, 14);
      pl.position.set(pos.x, 5.2, pos.z);
      this.scene.add(pl);
      this.downlights.push(pl);
    });
  }

  setBrightness(level) {
    if (level === 'bright') {
      this.renderer.toneMappingExposure = 1.6;
      this.ambientLight.intensity = 1.6;
      this.hemiLight.intensity = 1.1;
      this.sunLight.intensity = 2.2;
      this.fillLight.intensity = 1.3;
    } else if (level === 'normal') {
      this.renderer.toneMappingExposure = 1.45;
      this.ambientLight.intensity = 1.4;
      this.hemiLight.intensity = 0.95;
      this.sunLight.intensity = 1.95;
      this.fillLight.intensity = 1.15;
    } else if (level === 'dim') {
      this.renderer.toneMappingExposure = 1.15;
      this.ambientLight.intensity = 0.8;
      this.hemiLight.intensity = 0.5;
      this.sunLight.intensity = 1.0;
      this.fillLight.intensity = 0.6;
    }
  }

  setCameraView(mode, targetPos = null) {
    this.camMode = mode;
    if (!this.controls) return;
    const hasTween = typeof TWEEN !== 'undefined';

    if (mode === 'iso') {
      if (hasTween) {
        new TWEEN.Tween(this.camera.position)
          .to({ x: 24, y: 21, z: 24 }, 1000)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
        new TWEEN.Tween(this.controls.target)
          .to({ x: 0, y: 1.2, z: 0 }, 1000)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
      } else {
        this.camera.position.set(24, 21, 24);
        this.controls.target.set(0, 1.2, 0);
      }
    } else if (mode === 'top') {
      if (hasTween) {
        new TWEEN.Tween(this.camera.position)
          .to({ x: 0.1, y: 36, z: 0.1 }, 1000)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
        new TWEEN.Tween(this.controls.target)
          .to({ x: 0, y: 0, z: 0 }, 1000)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
      } else {
        this.camera.position.set(0.1, 36, 0.1);
        this.controls.target.set(0, 0, 0);
      }
    } else if (mode === 'focus' && targetPos) {
      if (hasTween) {
        new TWEEN.Tween(this.camera.position)
          .to({ x: targetPos.x + 4.5, y: targetPos.y + 3.5, z: targetPos.z + 4.5 }, 800)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
        new TWEEN.Tween(this.controls.target)
          .to({ x: targetPos.x, y: targetPos.y + 1, z: targetPos.z }, 800)
          .easing(TWEEN.Easing.Cubic.Out)
          .start();
      } else {
        this.camera.position.set(targetPos.x + 4.5, targetPos.y + 3.5, targetPos.z + 4.5);
        this.controls.target.set(targetPos.x, targetPos.y + 1, targetPos.z);
      }
    }
  }

  toggleDayNight(isNight) {
    this.isNight = isNight;
    const duration = 1200;

    if (typeof TWEEN !== 'undefined') {
      if (isNight) {
        if (this.ambientLight) new TWEEN.Tween(this.ambientLight).to({ intensity: 0.38 }, duration).start();
        if (this.hemiLight) new TWEEN.Tween(this.hemiLight).to({ intensity: 0.28 }, duration).start();
        if (this.sunLight) new TWEEN.Tween(this.sunLight).to({ intensity: 0.2 }, duration).start();
        if (this.fillLight) new TWEEN.Tween(this.fillLight).to({ intensity: 0.15 }, duration).start();
        if (this.scene && this.scene.fog) new TWEEN.Tween(this.scene.fog).to({ density: 0.025 }, duration).start();
        if (this.scene) this.scene.background.set(0x04060c);
        if (this.skylineMesh) this.skylineMesh.children.forEach(b => b.material.color.set(0x1e293b));
      } else {
        if (this.ambientLight) new TWEEN.Tween(this.ambientLight).to({ intensity: 1.4 }, duration).start();
        if (this.hemiLight) new TWEEN.Tween(this.hemiLight).to({ intensity: 0.95 }, duration).start();
        if (this.sunLight) new TWEEN.Tween(this.sunLight).to({ intensity: 1.95 }, duration).start();
        if (this.fillLight) new TWEEN.Tween(this.fillLight).to({ intensity: 1.15 }, duration).start();
        if (this.scene && this.scene.fog) new TWEEN.Tween(this.scene.fog).to({ density: 0.012 }, duration).start();
        if (this.scene) this.scene.background.set(0x0a0f1d);
        if (this.skylineMesh) this.skylineMesh.children.forEach(b => b.material.color.set(0x475569));
      }
    } else {
      if (this.ambientLight) this.ambientLight.intensity = isNight ? 0.38 : 1.4;
      if (this.hemiLight) this.hemiLight.intensity = isNight ? 0.28 : 0.95;
      if (this.sunLight) this.sunLight.intensity = isNight ? 0.2 : 1.95;
      if (this.fillLight) this.fillLight.intensity = isNight ? 0.15 : 1.15;
      if (this.scene) this.scene.background.set(isNight ? 0x04060c : 0x0a0f1d);
    }
  }

  buildOfficeEnvironment() {
    // 1. Lantai Parquet Kayu Hangat & Terang
    const floorGeo = new THREE.BoxGeometry(36, 0.4, 28);
    const canvas = document.createElement('canvas');
    canvas.width = 512;
    canvas.height = 512;
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = '#2d2218'; // Warm natural oak
    ctx.fillRect(0, 0, 512, 512);
    ctx.strokeStyle = '#1a130c';
    ctx.lineWidth = 3;
    for (let x = 0; x <= 512; x += 64) {
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, 512); ctx.stroke();
    }
    for (let y = 0; y <= 512; y += 32) {
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(512, y); ctx.stroke();
    }
    const floorTex = new THREE.CanvasTexture(canvas);
    floorTex.wrapS = THREE.RepeatWrapping;
    floorTex.wrapT = THREE.RepeatWrapping;
    floorTex.repeat.set(6, 4);

    const floorMat = new THREE.MeshStandardMaterial({
      map: floorTex,
      roughness: 0.38,
      metalness: 0.08
    });
    const floor = new THREE.Mesh(floorGeo, floorMat);
    floor.position.y = -0.2;
    floor.receiveShadow = true;
    this.scene.add(floor);

    // 2. Karpet Koridor / Aisle Tengah
    const aisleRugGeo = new THREE.BoxGeometry(3.2, 0.015, 20);
    const aisleRugMat = new THREE.MeshStandardMaterial({ color: 0x161e2e, roughness: 0.9 });
    const aisleRug = new THREE.Mesh(aisleRugGeo, aisleRugMat);
    aisleRug.position.set(0, 0.01, 0);
    aisleRug.receiveShadow = true;
    this.scene.add(aisleRug);

    // 3. Karpet Area Engineering Cluster
    const engRugGeo = new THREE.BoxGeometry(8.5, 0.015, 7.5);
    const engRugMat = new THREE.MeshStandardMaterial({ color: 0x111827, roughness: 0.95 });
    const engRug = new THREE.Mesh(engRugGeo, engRugMat);
    engRug.position.set(6.8, 0.01, -8.0);
    engRug.receiveShadow = true;
    this.scene.add(engRug);

    // 4. Karpet Area Business Cluster
    const bizRugGeo = new THREE.BoxGeometry(8.5, 0.015, 7.5);
    const bizRug = new THREE.Mesh(bizRugGeo, engRugMat);
    bizRug.position.set(6.8, 0.01, 6.0);
    bizRug.receiveShadow = true;
    this.scene.add(bizRug);
  }

  buildPanoramicWindowsAndSkyline() {
    const windowGroup = new THREE.Group();
    windowGroup.position.set(-17.8, 3.2, 0);

    const paneCount = 6;
    const paneWidth = 4.2;
    for (let i = 0; i < paneCount; i++) {
      const zPos = -10.5 + i * paneWidth;
      const glassGeo = new THREE.BoxGeometry(0.12, 6.2, paneWidth - 0.2);
      const glassMat = new THREE.MeshPhysicalMaterial({
        color: 0x93c5fd,
        transparent: true,
        opacity: 0.2,
        roughness: 0.05,
        transmission: 0.92,
        ior: 1.52
      });
      const glass = new THREE.Mesh(glassGeo, glassMat);
      glass.position.set(0, 0, zPos);
      windowGroup.add(glass);

      const mullionGeo = new THREE.BoxGeometry(0.2, 6.4, 0.16);
      const mullionMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, metalness: 0.85 });
      const mullion = new THREE.Mesh(mullionGeo, mullionMat);
      mullion.position.set(0, 0, zPos + (paneWidth - 0.2) / 2);
      windowGroup.add(mullion);
    }

    this.scene.add(windowGroup);

    // Skyline Gedung Pencakar Langit di Luar Jendela
    const skylineGroup = new THREE.Group();
    skylineGroup.position.set(-26, 0, 0);
    const buildingsData = [
      { w: 3.5, h: 18, d: 3.5, z: -10, c: 0x334155 },
      { w: 4.2, h: 24, d: 4.2, z: -5,  c: 0x475569 },
      { w: 3.8, h: 16, d: 3.8, z: 0,   c: 0x334155 },
      { w: 4.8, h: 28, d: 4.8, z: 6,   c: 0x1e293b },
      { w: 3.2, h: 20, d: 3.2, z: 11.5, c: 0x475569 }
    ];

    buildingsData.forEach(b => {
      const bGeo = new THREE.BoxGeometry(b.w, b.h, b.d);
      const bMat = new THREE.MeshStandardMaterial({ color: b.c, roughness: 0.7 });
      const bMesh = new THREE.Mesh(bGeo, bMat);
      bMesh.position.set(0, b.h / 2 - 2, b.z);
      skylineGroup.add(bMesh);
    });
    this.skylineMesh = skylineGroup;
    this.scene.add(skylineGroup);
  }

  buildAcousticWoodWallAndLogo() {
    const wallGroup = new THREE.Group();
    wallGroup.position.set(0, 3.2, -13.8);

    const backGeo = new THREE.BoxGeometry(35.6, 6.4, 0.2);
    const backMat = new THREE.MeshStandardMaterial({ color: 0x090d16, roughness: 0.7 });
    const backPanel = new THREE.Mesh(backGeo, backMat);
    wallGroup.add(backPanel);

    // Bilah Kayu Akustik Vertikal
    const slatGeo = new THREE.BoxGeometry(0.12, 6.2, 0.08);
    const slatMat = new THREE.MeshStandardMaterial({ color: 0x92400e, roughness: 0.45 });
    for (let x = -17; x <= 17; x += 0.5) {
      if (x > -4.8 && x < 4.8) continue; // Area logo di tengah
      const slat = new THREE.Mesh(slatGeo, slatMat);
      slat.position.set(x, 0, 0.14);
      wallGroup.add(slat);
    }

    // Plakat Logo 3D Elegan
    const plaqueGeo = new THREE.BoxGeometry(9.2, 2.6, 0.15);
    const plaqueMat = new THREE.MeshStandardMaterial({ color: 0x0f172a, roughness: 0.2, metalness: 0.7 });
    const plaque = new THREE.Mesh(plaqueGeo, plaqueMat);
    plaque.position.set(0, 0.4, 0.18);
    wallGroup.add(plaque);

    const glowBorderGeo = new THREE.BoxGeometry(9.4, 2.8, 0.05);
    const glowBorderMat = new THREE.MeshBasicMaterial({ color: 0x06b6d4 });
    const glowBorder = new THREE.Mesh(glowBorderGeo, glowBorderMat);
    glowBorder.position.set(0, 0.4, 0.1);
    wallGroup.add(glowBorder);

    const logoCanvas = document.createElement('canvas');
    logoCanvas.width = 1024;
    logoCanvas.height = 320;
    const lctx = logoCanvas.getContext('2d');
    lctx.fillStyle = '#0b1120';
    lctx.fillRect(0, 0, 1024, 320);

    lctx.fillStyle = '#06b6d4';
    lctx.font = 'bold 56px "Plus Jakarta Sans", sans-serif';
    lctx.textAlign = 'center';
    lctx.fillText('⚡ WIDYA ENTERPRISE LABS', 512, 105);

    lctx.fillStyle = '#f8fafc';
    lctx.font = '600 32px "Plus Jakarta Sans", sans-serif';
    lctx.fillText('KANTOR DIGITAL 10 AI AGENT BERDAULAT', 512, 175);

    lctx.fillStyle = '#10b981';
    lctx.font = 'bold 24px "JetBrains Mono", monospace';
    lctx.fillText('WIDYABASE NATIVE DB • UU PDP NO. 27/2022 COMPLIANT', 512, 235);

    const logoTex = new THREE.CanvasTexture(logoCanvas);
    const textScreenGeo = new THREE.PlaneGeometry(9.0, 2.4);
    const textScreenMat = new THREE.MeshBasicMaterial({ map: logoTex });
    const textScreen = new THREE.Mesh(textScreenGeo, textScreenMat);
    textScreen.position.set(0, 0.4, 0.27);
    wallGroup.add(textScreen);

    this.scene.add(wallGroup);
  }

  // Ruang Sidang Kaca Tertutup Tersendiri (Dedicated Boardroom)
  buildExecutiveBoardroomDetails() {
    const boardGroup = new THREE.Group();
    boardGroup.position.set(-8.5, 0, 0);

    // Karpet Mewah Khusus Boardroom
    const bRugGeo = new THREE.BoxGeometry(10.5, 0.02, 8.5);
    const bRugMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, roughness: 0.95 });
    const bRug = new THREE.Mesh(bRugGeo, bRugMat);
    bRug.position.set(0, 0.01, 0);
    bRug.receiveShadow = true;
    boardGroup.add(bRug);

    // Dinding Partisi Kaca Minimalis Mengelilingi Boardroom
    const glassMat = new THREE.MeshPhysicalMaterial({
      color: 0x38bdf8,
      transparent: true,
      opacity: 0.22,
      roughness: 0.08,
      transmission: 0.88,
      ior: 1.5
    });
    const frameMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, metalness: 0.85 });

    // Dinding Kaca Belakang Boardroom (z = -4.2)
    const wallBackGeo = new THREE.BoxGeometry(10.5, 3.4, 0.1);
    const wallBack = new THREE.Mesh(wallBackGeo, glassMat);
    wallBack.position.set(0, 1.7, -4.2);
    boardGroup.add(wallBack);

    // Dinding Kaca Depan Boardroom (z = 4.2)
    const wallFrontGeo = new THREE.BoxGeometry(10.5, 3.4, 0.1);
    const wallFront = new THREE.Mesh(wallFrontGeo, glassMat);
    wallFront.position.set(0, 1.7, 4.2);
    boardGroup.add(wallFront);

    // Dinding Kaca Sisi Timur/Lorong dengan Bukaan Pintu Terbuka (x = 5.25)
    // Bagian Atas Pintu & Bagian Samping
    const wallDoorLGeo = new THREE.BoxGeometry(0.1, 3.4, 2.8);
    const wallDoorL = new THREE.Mesh(wallDoorLGeo, glassMat);
    wallDoorL.position.set(5.25, 1.7, -2.8);
    boardGroup.add(wallDoorL);

    const wallDoorRGeo = new THREE.BoxGeometry(0.1, 3.4, 2.8);
    const wallDoorR = new THREE.Mesh(wallDoorRGeo, glassMat);
    wallDoorR.position.set(5.25, 1.7, 2.8);
    boardGroup.add(wallDoorR);

    // Kosen Pintu Atas
    const doorTopGeo = new THREE.BoxGeometry(0.12, 0.4, 2.9);
    const doorTop = new THREE.Mesh(doorTopGeo, frameMat);
    doorTop.position.set(5.25, 3.2, 0);
    boardGroup.add(doorTop);

    // Layar Presentasi 4K Ultra-Wide di Dinding Barat Boardroom (x = -5.2)
    const screenGeo = new THREE.BoxGeometry(0.08, 2.5, 4.8);
    const screenMat = new THREE.MeshStandardMaterial({ color: 0x0f172a, metalness: 0.9 });
    const screen = new THREE.Mesh(screenGeo, screenMat);
    screen.position.set(-5.15, 2.4, 0);
    boardGroup.add(screen);

    const scCanvas = document.createElement('canvas');
    scCanvas.width = 1024;
    scCanvas.height = 512;
    const sctx = scCanvas.getContext('2d');
    sctx.fillStyle = '#070b14';
    sctx.fillRect(0, 0, 1024, 512);

    sctx.fillStyle = '#06b6d4';
    sctx.font = 'bold 36px "Plus Jakarta Sans", sans-serif';
    sctx.fillText('📊 EXECUTIVE BOARDROOM • WIDYABASE TELEMETRY', 50, 65);

    sctx.fillStyle = '#94a3b8';
    sctx.font = '24px "JetBrains Mono", monospace';
    sctx.fillText('Portofolio: 10 Proyek Industri | 10 AI Agents On Duty | SLA 100%', 50, 110);

    const colors = ['#10b981', '#3b82f6', '#f59e0b', '#a855f7', '#06b6d4'];
    const labels = ['Auth', 'Schema', 'SSE Test', 'Backup', 'Audit'];
    for (let i = 0; i < 5; i++) {
      sctx.fillStyle = colors[i];
      const barH = 120 + i * 35;
      sctx.fillRect(100 + i * 170, 420 - barH, 110, barH);
      sctx.fillStyle = '#f8fafc';
      sctx.font = '22px sans-serif';
      sctx.fillText(labels[i], 115 + i * 170, 460);
    }

    const scTex = new THREE.CanvasTexture(scCanvas);
    const scPlaneGeo = new THREE.PlaneGeometry(4.7, 2.4);
    const scPlaneMat = new THREE.MeshBasicMaterial({ map: scTex });
    const scPlane = new THREE.Mesh(scPlaneGeo, scPlaneMat);
    scPlane.position.set(-5.08, 2.4, 0);
    scPlane.rotation.y = Math.PI / 2;
    boardGroup.add(scPlane);

    // Lampu Gantung Linier LED Modern di Atas Meja Konferensi
    const pendGeo = new THREE.BoxGeometry(5.6, 0.08, 0.4);
    const pendMat = new THREE.MeshBasicMaterial({ color: 0x38bdf8 });
    const pend = new THREE.Mesh(pendGeo, pendMat);
    pend.position.set(0, 4.4, 0);
    boardGroup.add(pend);

    this.scene.add(boardGroup);
  }

  // Meja Konferensi Oval Panjang di dalam Boardroom
  buildConferenceTable() {
    this.confGroup = new THREE.Group();
    this.confGroup.position.set(-8.5, 0, 0);

    // Meja Konferensi Panjang Oval (Black Walnut & Marble Trim)
    const tableTopGeo = new THREE.BoxGeometry(6.6, 0.22, 2.4);
    const tableMat = new THREE.MeshStandardMaterial({
      color: 0x0f172a,
      roughness: 0.25,
      metalness: 0.65
    });
    const tableTop = new THREE.Mesh(tableTopGeo, tableMat);
    tableTop.position.y = 1.1;
    tableTop.castShadow = true;
    tableTop.receiveShadow = true;
    this.confGroup.add(tableTop);

    // Kaki Meja Silinder Logam
    const legGeo = new THREE.CylinderGeometry(0.35, 0.45, 1.0, 16);
    const legMat = new THREE.MeshStandardMaterial({ color: 0x0284c7, metalness: 0.85 });
    const leg1 = new THREE.Mesh(legGeo, legMat);
    leg1.position.set(-2.0, 0.5, 0);
    this.confGroup.add(leg1);
    const leg2 = new THREE.Mesh(legGeo, legMat);
    leg2.position.set(2.0, 0.5, 0);
    this.confGroup.add(leg2);

    // Cincin Emitor Hologram di Tengah Meja
    const ringGeo = new THREE.RingGeometry(0.45, 0.75, 32);
    this.holoRingMat = new THREE.MeshBasicMaterial({
      color: 0x06b6d4,
      side: THREE.DoubleSide,
      transparent: true,
      opacity: 0.7
    });
    const holoRing = new THREE.Mesh(ringGeo, this.holoRingMat);
    holoRing.rotation.x = -Math.PI / 2;
    holoRing.position.y = 1.23;
    this.confGroup.add(holoRing);

    // Kubus Hologram Melayang
    const holoCubeGeo = new THREE.BoxGeometry(0.35, 0.35, 0.35);
    this.holoCoreMat = new THREE.MeshBasicMaterial({
      color: 0x38bdf8,
      wireframe: true,
      transparent: true,
      opacity: 0.85
    });
    this.holoCore = new THREE.Mesh(holoCubeGeo, this.holoCoreMat);
    this.holoCore.position.y = 1.8;
    this.confGroup.add(this.holoCore);

    // Mikrofon Konferensi Meja Bundar (Puck Mics)
    const micGeo = new THREE.CylinderGeometry(0.08, 0.09, 0.025, 16);
    const micMat = new THREE.MeshStandardMaterial({ color: 0x475569, metalness: 0.9 });
    for (let m = -2; m <= 2; m += 1.3) {
      const mic = new THREE.Mesh(micGeo, micMat);
      mic.position.set(m, 1.23, 0);
      this.confGroup.add(mic);
    }

    // 10 Kursi Konferensi Eksekutif di Sekeliling Meja Boardroom (Semua 10 AI Agent Mendapat Kursi Sendiri)
    const chairConfigs = [
      { x: 3.7, z: 0, rotY: Math.PI / 2, color: 0x3b82f6 },      // 1. Mandor Utama (Arya) - Ujung Meja Timur
      { x: -3.7, z: 0, rotY: -Math.PI / 2, color: 0xf97316 },    // 2. Analis Data & KPI (Dewi) - Ujung Meja Barat
      { x: -2.1, z: -1.35, rotY: Math.PI, color: 0x06b6d4 },     // 3. Analis Bisnis (Dian) - Sisi Utara
      { x: -0.7, z: -1.35, rotY: Math.PI, color: 0x6366f1 },     // 4. Arsitek Sistem (Bambang) - Sisi Utara
      { x: 0.7, z: -1.35, rotY: Math.PI, color: 0x10b981 },      // 5. Lead Coder (Rizky) - Sisi Utara
      { x: 2.1, z: -1.35, rotY: Math.PI, color: 0xec4899 },      // 6. QA Tester (Siti) - Sisi Utara
      { x: -2.1, z: 1.35, rotY: 0, color: 0xf59e0b },            // 7. DevOps SRE (Fajar) - Sisi Selatan
      { x: -0.7, z: 1.35, rotY: 0, color: 0xef4444 },            // 8. Auditor Siber (Hendra) - Sisi Selatan
      { x: 0.7, z: 1.35, rotY: 0, color: 0xa855f7 },             // 9. Notulis Vault (Ratna) - Sisi Selatan
      { x: 2.1, z: 1.35, rotY: 0, color: 0x0284c7 }              // 10. Humas Telegram (Bayu) - Sisi Selatan
    ];

    chairConfigs.forEach((c) => {
      const cChair = this.createOfficeChair(c.color);
      cChair.position.set(c.x, 0, c.z);
      cChair.rotation.y = c.rotY;
      this.confGroup.add(cChair);
    });

    this.scene.add(this.confGroup);
  }

  buildPantryAndCoffeeLounge() {
    const pantryGroup = new THREE.Group();
    pantryGroup.position.set(-10.5, 0, 8.5);

    const loungeRugGeo = new THREE.BoxGeometry(8.5, 0.02, 6.5);
    const loungeRugMat = new THREE.MeshStandardMaterial({ color: 0x1e2230, roughness: 0.95 });
    const loungeRug = new THREE.Mesh(loungeRugGeo, loungeRugMat);
    loungeRug.position.set(0, 0.01, 0);
    pantryGroup.add(loungeRug);

    // Meja Counter Bar Marmer Hitam
    const barGeo = new THREE.BoxGeometry(3.6, 1.1, 1.1);
    const barMat = new THREE.MeshStandardMaterial({ color: 0x0f172a, roughness: 0.2, metalness: 0.6 });
    const bar = new THREE.Mesh(barGeo, barMat);
    bar.position.set(1.5, 0.55, -2.2);
    pantryGroup.add(bar);

    const topBarGeo = new THREE.BoxGeometry(3.8, 0.08, 1.25);
    const topBarMat = new THREE.MeshStandardMaterial({ color: 0xe2e8f0, roughness: 0.25 });
    const topBar = new THREE.Mesh(topBarGeo, topBarMat);
    topBar.position.set(1.5, 1.14, -2.2);
    pantryGroup.add(topBar);

    // Mesin Kopi Espresso Komersial
    const coffeeGeo = new THREE.BoxGeometry(0.9, 0.65, 0.6);
    const coffeeMat = new THREE.MeshStandardMaterial({ color: 0xd97706, metalness: 0.85, roughness: 0.2 });
    const coffee = new THREE.Mesh(coffeeGeo, coffeeMat);
    coffee.position.set(0.6, 1.5, -2.2);
    pantryGroup.add(coffee);

    // Cangkir Porselen
    const cupGeo = new THREE.CylinderGeometry(0.06, 0.05, 0.09, 12);
    const cupMat = new THREE.MeshStandardMaterial({ color: 0xffffff });
    for (let c = 0; c < 3; c++) {
      const cup = new THREE.Mesh(cupGeo, cupMat);
      cup.position.set(1.4 + c * 0.2, 1.22, -2.1);
      pantryGroup.add(cup);
    }

    // Dispenser Air Minum Galon
    const dispGeo = new THREE.BoxGeometry(0.45, 1.25, 0.45);
    const dispMat = new THREE.MeshStandardMaterial({ color: 0xf1f5f9 });
    const disp = new THREE.Mesh(dispGeo, dispMat);
    disp.position.set(3.1, 0.65, -2.2);
    pantryGroup.add(disp);

    const jugGeo = new THREE.CylinderGeometry(0.18, 0.18, 0.45, 16);
    const jugMat = new THREE.MeshPhysicalMaterial({ color: 0x38bdf8, transparent: true, opacity: 0.65 });
    const jug = new THREE.Mesh(jugGeo, jugMat);
    jug.position.set(3.1, 1.48, -2.2);
    pantryGroup.add(jug);

    // Sofa Kulit Eksekutif Bentuk L
    const sofaMat = new THREE.MeshStandardMaterial({ color: 0x1f2937, roughness: 0.6 });
    const sofaMain = new THREE.Mesh(new THREE.BoxGeometry(3.6, 0.65, 1.1), sofaMat);
    sofaMain.position.set(-0.8, 0.35, 1.2);
    pantryGroup.add(sofaMain);

    const sofaBack = new THREE.Mesh(new THREE.BoxGeometry(3.6, 0.55, 0.28), sofaMat);
    sofaBack.position.set(-0.8, 0.85, 1.6);
    pantryGroup.add(sofaBack);

    const sofaL = new THREE.Mesh(new THREE.BoxGeometry(1.1, 0.65, 1.8), sofaMat);
    sofaL.position.set(1.45, 0.35, 0.7);
    pantryGroup.add(sofaL);

    // Meja Kopi Scandinavian
    const cTable = new THREE.Mesh(new THREE.CylinderGeometry(0.75, 0.75, 0.08, 24), new THREE.MeshStandardMaterial({ color: 0x92400e, roughness: 0.4 }));
    cTable.position.set(-0.6, 0.42, -0.2);
    pantryGroup.add(cTable);

    this.scene.add(pantryGroup);
  }

  buildDataCenterAndServerRoom() {
    const dataGroup = new THREE.Group();
    dataGroup.position.set(13.5, 0, -8.0);

    const dcMat = new THREE.Mesh(new THREE.BoxGeometry(4.8, 0.02, 5.2), new THREE.MeshStandardMaterial({ color: 0x0b1329, roughness: 0.9 }));
    dcMat.position.set(0, 0.01, 0);
    dataGroup.add(dcMat);

    // 2 Unit Lemari Rak Server 42U
    for (let r = 0; r < 2; r++) {
      const xOffset = (r === 0) ? -1.0 : 1.0;
      const rackGroup = new THREE.Group();
      rackGroup.position.set(xOffset, 0, 0);

      const rack = new THREE.Mesh(new THREE.BoxGeometry(1.6, 3.4, 1.2), new THREE.MeshStandardMaterial({ color: 0x090d16, metalness: 0.85, roughness: 0.3 }));
      rack.position.y = 1.7;
      rackGroup.add(rack);

      const door = new THREE.Mesh(new THREE.BoxGeometry(1.48, 3.2, 0.05), new THREE.MeshPhysicalMaterial({ color: 0x06b6d4, transparent: true, opacity: 0.35 }));
      door.position.set(0, 1.7, 0.62);
      rackGroup.add(door);

      const bladeGeo = new THREE.BoxGeometry(1.35, 0.16, 1.0);
      const bladeMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, metalness: 0.7 });
      const ledGeo = new THREE.BoxGeometry(0.05, 0.05, 0.02);
      const ledColors = [0x10b981, 0x06b6d4, 0xf59e0b];

      for (let s = 0; s < 12; s++) {
        const yPos = 0.5 + s * 0.22;
        const blade = new THREE.Mesh(bladeGeo, bladeMat);
        blade.position.set(0, yPos, 0.05);
        rackGroup.add(blade);

        for (let l = 0; l < 3; l++) {
          const lMat = new THREE.MeshBasicMaterial({ color: ledColors[l % 3], transparent: true, opacity: 0.95 });
          const led = new THREE.Mesh(ledGeo, lMat);
          led.position.set(-0.45 + l * 0.45, yPos, 0.58);
          rackGroup.add(led);
          this.serverLeds.push(led);
        }
      }

      rackGroup.userData = { isServerRack: true };
      rackGroup.traverse(c => { if (c.isMesh) c.userData = { isServerRack: true }; });
      dataGroup.add(rackGroup);
    }

    // Monitor Diagnostik Pedestal
    const monScreen = new THREE.Mesh(new THREE.BoxGeometry(1.2, 0.7, 0.06), new THREE.MeshStandardMaterial({ color: 0x0f172a }));
    const monCanvas = document.createElement('canvas');
    monCanvas.width = 512; monCanvas.height = 256;
    const mctx = monCanvas.getContext('2d');
    mctx.fillStyle = '#020617'; mctx.fillRect(0, 0, 512, 256);
    mctx.fillStyle = '#10b981'; mctx.font = 'bold 28px monospace';
    mctx.fillText('🗄️ WIDYABASE v1.0.0', 30, 45);
    mctx.fillStyle = '#38bdf8'; mctx.font = '20px monospace';
    mctx.fillText('STATUS   : SEHAT (BERSIH)', 30, 95);
    mctx.fillText('STORAGE  : Slotted-Page 4KB', 30, 135);
    mctx.fillText('BUFFER   : 512 Pages (2.0MB)', 30, 175);
    mctx.fillText('WAL LOG  : ARIES Checkpoint', 30, 215);

    const monTex = new THREE.CanvasTexture(monCanvas);
    const monScreenMat = new THREE.MeshBasicMaterial({ map: monTex });
    const screenMesh = new THREE.Mesh(new THREE.BoxGeometry(1.2, 0.7, 0.06), monScreenMat);
    screenMesh.position.set(0, 1.35, 1.5);
    dataGroup.add(screenMesh);

    this.scene.add(dataGroup);
  }

  buildBiophilicGreenery() {
    const plantsGroup = new THREE.Group();
    this.createPottedPlant(plantsGroup, -13.5, 0.0, 5.5, 'monstera');
    this.createPottedPlant(plantsGroup, -2.8, 0.0, -3.8, 'ficus');
    this.createPottedPlant(plantsGroup, 11.5, 0.0, -11.5, 'ficus');
    this.createPottedPlant(plantsGroup, 11.5, 0.0, 9.5, 'monstera');
    this.scene.add(plantsGroup);
  }

  createPottedPlant(parentGroup, x, y, z, type) {
    const pGroup = new THREE.Group();
    pGroup.position.set(x, y, z);
    const potColor = (type === 'monstera') ? 0xf8fafc : 0xb45309;
    const pot = new THREE.Mesh(new THREE.CylinderGeometry(0.38, 0.28, 0.75, 16), new THREE.MeshStandardMaterial({ color: potColor, roughness: 0.3 }));
    pot.position.y = 0.38;
    pGroup.add(pot);

    const leafMat = new THREE.MeshStandardMaterial({ color: 0x15803d, roughness: 0.4 });
    const leafGeo = (type === 'monstera') ? new THREE.SphereGeometry(0.35, 8, 8) : new THREE.CylinderGeometry(0.15, 0.45, 1.4, 8);

    if (type === 'monstera') {
      for (let i = 0; i < 5; i++) {
        const leaf = new THREE.Mesh(leafGeo, leafMat);
        const a = (i / 5) * Math.PI * 2;
        leaf.scale.set(1.4, 0.15, 0.9);
        leaf.position.set(Math.cos(a) * 0.4, 0.9 + (i % 2) * 0.2, Math.sin(a) * 0.4);
        pGroup.add(leaf);
      }
    } else {
      const trunk = new THREE.Mesh(new THREE.CylinderGeometry(0.06, 0.08, 1.8, 8), new THREE.MeshStandardMaterial({ color: 0x78350f, roughness: 0.8 }));
      trunk.position.y = 1.3;
      pGroup.add(trunk);
      for (let f = 0; f < 6; f++) {
        const fLeaf = new THREE.Mesh(leafGeo, leafMat);
        fLeaf.scale.set(0.6, 0.4, 0.6);
        fLeaf.position.set(0, 1.4 + f * 0.2, 0);
        fLeaf.rotation.y = f * 0.9;
        pGroup.add(fLeaf);
      }
    }
    parentGroup.add(pGroup);
  }

  // ================= TATA LETAK WORKSTATION CLUSTER REALISTIK =================
  setupWorkstations(agentsList) {
    /**
     * ZONASI TATA LETAK BARU (BUKAN MELINGKAR):
     * 1. Executive Suite (Mandor Utama / Arya Wicaksana) -> Mandiri di tengah strategis
     * 2. Engineering Pod 2x2 Benching (Lead Coder, Arsitek, QA, DevOps)
     * 3. Business & Comms Pod 2x2 Benching (Analis, Auditor Keamanan, Notulis, Humas)
     * 4. Dedicated Boardroom (-8.5, 0) untuk Rapat Sidang Bersama
     */
    if (this.labelsContainer) {
      this.labelsContainer.innerHTML = '';
    }

    const deskConfigs = [
      // 1. Executive Suite
      {
        id: 'agen_orkestrator',
        nama: 'Arya Wicaksana, PMP',
        shortName: 'Arya',
        roleLabel: 'MANDOR UTAMA',
        avatar: '👨‍💼',
        gender: 'pria',
        role: 'pm',
        color: 0x3b82f6,
        deskX: 6.8, deskZ: -1.0, rotY: 0,
        meetX: -4.8, meetZ: 0, meetRotY: -Math.PI / 2
      },
      // 2. Engineering Cluster
      {
        id: 'agen_coder',
        nama: 'Rizky Firmansyah',
        shortName: 'Rizky',
        roleLabel: 'LEAD CODER',
        avatar: '👨‍💻',
        gender: 'pria',
        role: 'coder',
        color: 0x10b981,
        deskX: 4.5, deskZ: -9.5, rotY: 0,
        meetX: -7.8, meetZ: -1.35, meetRotY: 0
      },
      {
        id: 'agen_arsitek',
        nama: 'Bambang Prasetyo, M.T.',
        shortName: 'Bambang',
        roleLabel: 'ARSITEK SISTEM',
        avatar: '👨‍💻',
        gender: 'pria',
        role: 'arch',
        color: 0x6366f1,
        deskX: 4.5, deskZ: -6.5, rotY: Math.PI,
        meetX: -9.2, meetZ: -1.35, meetRotY: 0
      },
      {
        id: 'agen_qa',
        nama: 'Siti Rahmawati, S.Kom.',
        shortName: 'Siti',
        roleLabel: 'QA TESTER',
        avatar: '👩‍🔬',
        gender: 'wanita',
        role: 'qa',
        color: 0xec4899,
        deskX: 9.0, deskZ: -9.5, rotY: 0,
        meetX: -6.4, meetZ: -1.35, meetRotY: 0
      },
      {
        id: 'agen_devops',
        nama: 'Fajar Nugroho',
        shortName: 'Fajar',
        roleLabel: 'DEVOPS SRE',
        avatar: '👨‍🔧',
        gender: 'pria',
        role: 'devops',
        color: 0xf59e0b,
        deskX: 9.0, deskZ: -6.5, rotY: Math.PI,
        meetX: -10.6, meetZ: 1.35, meetRotY: Math.PI
      },
      // 3. Business & Governance Cluster
      {
        id: 'agen_analis',
        nama: 'Dian Kartika, MBA',
        shortName: 'Dian',
        roleLabel: 'ANALIS BISNIS',
        avatar: '👩‍💼',
        gender: 'wanita',
        role: 'ba',
        color: 0x06b6d4,
        deskX: 4.5, deskZ: 4.5, rotY: 0,
        meetX: -10.6, meetZ: -1.35, meetRotY: 0
      },
      {
        id: 'agen_keamanan',
        nama: 'Hendra Gunawan, CISA',
        shortName: 'Hendra',
        roleLabel: 'AUDITOR SIBER',
        avatar: '👮‍♂️',
        gender: 'pria',
        role: 'security',
        color: 0xef4444,
        deskX: 4.5, deskZ: 7.5, rotY: Math.PI,
        meetX: -9.2, meetZ: 1.35, meetRotY: Math.PI
      },
      {
        id: 'agen_notulis',
        nama: 'Ratna Sari',
        shortName: 'Ratna',
        roleLabel: 'NOTULIS VAULT',
        avatar: '👩‍🏫',
        gender: 'wanita',
        role: 'scribe',
        color: 0xa855f7,
        deskX: 9.0, deskZ: 4.5, rotY: 0,
        meetX: -7.8, meetZ: 1.35, meetRotY: Math.PI
      },
      {
        id: 'agen_humas',
        nama: 'Bayu Pratama',
        shortName: 'Bayu',
        roleLabel: 'HUMAS TELEGRAM',
        avatar: '👩‍💼',
        gender: 'wanita',
        role: 'pr',
        color: 0x0284c7,
        deskX: 9.0, deskZ: 7.5, rotY: Math.PI,
        meetX: -6.4, meetZ: 1.35, meetRotY: Math.PI
      },
      // 4. Analytics Hub
      {
        id: 'agen_analis_data',
        nama: 'Dewi Lestari, M.Sc.',
        shortName: 'Dewi',
        roleLabel: 'ANALIS DATA & KPI',
        avatar: '👨‍💻',
        gender: 'pria',
        role: 'analytics',
        color: 0xf97316,
        deskX: 13.5, deskZ: -1.5, rotY: Math.PI,
        meetX: -12.2, meetZ: 0, meetRotY: Math.PI / 2
      }
    ];

    deskConfigs.forEach((cfg) => {
      // 1. Bangun Meja Pod Khusus
      this.createDeskPod(cfg);

      // 2. Bangun Karakter Humanoid Realistik Gender Pria / Wanita
      const charGroup = this.createHumanoidAvatar(cfg);
      charGroup.userData = { agentId: cfg.id };
      charGroup.traverse((child) => {
        if (child.isMesh) child.userData = { agentId: cfg.id };
      });

      // Posisi duduk nyata di dudukan kursi (y = 0.0) menghadap tepat ke arah meja kerja
      const chairDist = 0.68;
      const charWorldX = cfg.deskX + Math.sin(cfg.rotY) * chairDist;
      const charWorldY = 0.0;
      const charWorldZ = cfg.deskZ + Math.cos(cfg.rotY) * chairDist;
      const charWorldRotY = cfg.rotY + Math.PI;

      charGroup.position.set(charWorldX, charWorldY, charWorldZ);
      charGroup.rotation.y = charWorldRotY;
      this.scene.add(charGroup);

      // 3. Tag Visual HTML Melayang di Atas Agen (Satu Kata, Bersih, Tidak Bertumpuk)
      let tagEl = null;
      if (this.labelsContainer) {
        tagEl = document.createElement('div');
        tagEl.className = 'agent-floating-tag';
        tagEl.dataset.agentId = cfg.id;

        const colorHex = '#' + cfg.color.toString(16).padStart(6, '0');
        const r = (cfg.color >> 16) & 255;
        const g = (cfg.color >> 8) & 255;
        const b = cfg.color & 255;
        tagEl.style.setProperty('--agent-color', colorHex);
        tagEl.style.setProperty('--agent-glow', `rgba(${r}, ${g}, ${b}, 0.45)`);
        tagEl.style.setProperty('--agent-badge-bg', `rgba(${r}, ${g}, ${b}, 0.16)`);
        tagEl.style.setProperty('--agent-badge-border', `rgba(${r}, ${g}, ${b}, 0.4)`);

        const avatarGraphic = (typeof getAgentAvatarSvg === 'function')
          ? getAgentAvatarSvg(cfg.id, 20)
          : cfg.avatar;

        tagEl.innerHTML = `
          <div class="agent-tag-avatar">${avatarGraphic}</div>
          <span class="agent-tag-name-single">${cfg.shortName}</span>
          <div class="agent-tag-tail"></div>
        `;

        tagEl.addEventListener('click', (e) => {
          e.stopPropagation();
          window.dispatchEvent(new CustomEvent('agent-selected', { detail: { agentId: cfg.id } }));
          const aRec = this.agents3D.get(cfg.id);
          if (aRec) {
            this.setCameraView('focus', aRec.isMeeting ? aRec.meetPos : aRec.deskPos);
          }
        });

        this.labelsContainer.appendChild(tagEl);
      }

      this.agents3D.set(cfg.id, {
        id: cfg.id,
        nama: cfg.nama,
        gender: cfg.gender,
        roleLabel: cfg.roleLabel,
        avatar: cfg.avatar,
        color: cfg.color,
        charGroup: charGroup,
        torso: charGroup.userData.torso,
        headGroup: charGroup.userData.headGroup,
        handL: charGroup.userData.handL,
        handR: charGroup.userData.handR,
        tagEl: tagEl,
        deskPos: { x: charWorldX, y: 0.0, z: charWorldZ, rotY: charWorldRotY },
        meetPos: { x: cfg.meetX, y: 0.0, z: cfg.meetZ, rotY: cfg.meetRotY },
        isMeeting: false
      });
    });
  }

  createDeskPod(cfg) {
    const pod = new THREE.Group();
    pod.position.set(cfg.deskX, 0, cfg.deskZ);
    pod.rotation.y = cfg.rotY;

    // Daun Meja Kayu Walnut
    const topGeo = new THREE.BoxGeometry(2.3, 0.1, 1.15);
    const topMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, roughness: 0.35 });
    const top = new THREE.Mesh(topGeo, topMat);
    top.position.y = 1.0;
    top.castShadow = true;
    top.receiveShadow = true;
    pod.add(top);

    // Sekat Partisi Akustik (Acoustic Divider Screen) di Sisi Depan Meja
    const dividerGeo = new THREE.BoxGeometry(2.3, 0.45, 0.05);
    const dividerMat = new THREE.MeshStandardMaterial({ color: 0x334155, roughness: 0.8 });
    const divider = new THREE.Mesh(dividerGeo, dividerMat);
    divider.position.set(0, 1.25, -0.55);
    pod.add(divider);

    // Kaki Meja Logam
    const legGeo = new THREE.BoxGeometry(0.08, 0.95, 1.05);
    const legMat = new THREE.MeshStandardMaterial({ color: 0x334155, metalness: 0.85 });
    const legL = new THREE.Mesh(legGeo, legMat);
    legL.position.set(-1.05, 0.48, 0);
    pod.add(legL);
    const legR = new THREE.Mesh(legGeo, legMat);
    legR.position.set(1.05, 0.48, 0);
    pod.add(legR);

    // Desk Mat (Mousepad Kulit Lebar)
    const matGeo = new THREE.BoxGeometry(1.6, 0.015, 0.7);
    const matMat = new THREE.MeshStandardMaterial({ color: 0x090d16, roughness: 0.8 });
    const dMat = new THREE.Mesh(matGeo, matMat);
    dMat.position.set(0, 1.055, 0.05);
    pod.add(dMat);

    // Keyboard & Mouse
    const kbGeo = new THREE.BoxGeometry(0.65, 0.025, 0.2);
    const kbMat = new THREE.MeshStandardMaterial({ color: 0x1e293b, metalness: 0.4 });
    const kb = new THREE.Mesh(kbGeo, kbMat);
    kb.position.set(-0.1, 1.07, 0.15);
    pod.add(kb);

    const mouseGeo = new THREE.BoxGeometry(0.09, 0.03, 0.14);
    const mouse = new THREE.Mesh(mouseGeo, kbMat);
    mouse.position.set(0.45, 1.07, 0.15);
    pod.add(mouse);

    // Monitor Setup: Dual Monitor untuk Coder, Arsitek, QA, DevOps; Single untuk lainnya
    const isDual = ['coder', 'arch', 'qa', 'devops'].includes(cfg.role);
    const monGeo = new THREE.BoxGeometry(1.0, 0.6, 0.04);
    const monMat = new THREE.MeshStandardMaterial({ color: 0x0f172a, metalness: 0.9 });
    const screenGeo = new THREE.PlaneGeometry(0.96, 0.56);
    const screenMat = new THREE.MeshBasicMaterial({ color: cfg.color });

    if (isDual) {
      const monL = new THREE.Mesh(monGeo, monMat);
      monL.position.set(-0.52, 1.45, -0.15);
      monL.rotation.y = 0.18;
      const scL = new THREE.Mesh(screenGeo, screenMat);
      scL.position.set(-0.52, 1.45, -0.125);
      scL.rotation.y = 0.18;
      pod.add(monL); pod.add(scL);

      const monR = new THREE.Mesh(monGeo, monMat);
      monR.position.set(0.52, 1.45, -0.15);
      monR.rotation.y = -0.18;
      const scR = new THREE.Mesh(screenGeo, screenMat);
      scR.position.set(0.52, 1.45, -0.125);
      scR.rotation.y = -0.18;
      pod.add(monR); pod.add(scR);
    } else {
      const mon = new THREE.Mesh(monGeo, monMat);
      mon.position.set(0, 1.45, -0.2);
      const sc = new THREE.Mesh(screenGeo, screenMat);
      sc.position.set(0, 1.45, -0.17);
      pod.add(mon); pod.add(sc);
    }

    // Lampu Meja Kerja Minimalis LED (Warm Glow)
    const lampArm = new THREE.Mesh(new THREE.CylinderGeometry(0.02, 0.02, 0.55), new THREE.MeshStandardMaterial({ color: 0x475569 }));
    lampArm.position.set(-0.95, 1.3, -0.25);
    lampArm.rotation.z = -0.3;
    pod.add(lampArm);

    const lampHead = new THREE.Mesh(new THREE.BoxGeometry(0.25, 0.04, 0.1), new THREE.MeshBasicMaterial({ color: 0xfef08a }));
    lampHead.position.set(-0.8, 1.55, -0.25);
    pod.add(lampHead);

    // Kursi Kerja Ergonomis
    const chair = this.createOfficeChair(cfg.color);
    chair.position.set(0, 0, 0.7);
    pod.add(chair);

    pod.userData = { agentId: cfg.id };
    pod.traverse((child) => {
      if (child.isMesh) child.userData = { agentId: cfg.id };
    });
    this.scene.add(pod);
  }

  createOfficeChair(colorHex) {
    const chair = new THREE.Group();
    const seat = new THREE.Mesh(new THREE.BoxGeometry(0.7, 0.1, 0.7), new THREE.MeshStandardMaterial({ color: 0x1e293b, roughness: 0.6 }));
    seat.position.y = 0.6;
    chair.add(seat);

    const back = new THREE.Mesh(new THREE.BoxGeometry(0.65, 0.7, 0.08), new THREE.MeshStandardMaterial({ color: colorHex, roughness: 0.5 }));
    back.position.set(0, 1.0, 0.32);
    chair.add(back);

    const pole = new THREE.Mesh(new THREE.CylinderGeometry(0.06, 0.06, 0.55, 8), new THREE.MeshStandardMaterial({ color: 0x64748b, metalness: 0.9 }));
    pole.position.y = 0.3;
    chair.add(pole);

    return chair;
  }

  // ================= HUMAN AVATAR REALISTIK (SKULPTUR 3D FISIK ASLI) =================
  // Fitur Wajah 3D Penuh: Bola Mata 3D (Sclera, Iris, Pupil, Glint), Kelopak, Alis, Hidung 3D (Batang, Cuping, Lubang), Bibir 3D & Kacamata
  buildSculpted3DFace(headGroup, cfg, skinMat, isFemale) {
    const faceGroup = new THREE.Group();

    // 1. Dagu & Rahang Berstruktur 3D (Defined Jaw & Chin)
    const jawGeo = new THREE.BoxGeometry(isFemale ? 0.12 : 0.15, 0.09, 0.14);
    const jawMesh = new THREE.Mesh(jawGeo, skinMat);
    jawMesh.position.set(0, -0.12, 0.08);
    jawMesh.castShadow = true;
    faceGroup.add(jawMesh);

    // 2. Tonjolan Tulang Pipi (Cheekbones)
    const cheekGeo = new THREE.SphereGeometry(0.065, 12, 12);
    cheekGeo.scale(1.0, 0.8, 1.2);
    const cheekL = new THREE.Mesh(cheekGeo, skinMat);
    cheekL.position.set(-0.095, -0.02, 0.11);
    faceGroup.add(cheekL);

    const cheekR = new THREE.Mesh(cheekGeo, skinMat);
    cheekR.position.set(0.095, -0.02, 0.11);
    faceGroup.add(cheekR);

    // 3. Bola Mata 3D Fisik (Sclera Putih Glossy, Iris Cokelat Hazel Indonesia, Pupil Hitam, Cornea Catchlight)
    const scleraMat = new THREE.MeshStandardMaterial({
      color: 0xf8fafc,
      roughness: 0.12,
      metalness: 0.08
    });

    const irisColor = (cfg.id === 'agen_analis' || cfg.id === 'agen_humas') ? 0x452310 : (cfg.id === 'agen_keamanan' ? 0x24140a : 0x381e0b);
    const irisMat = new THREE.MeshStandardMaterial({
      color: irisColor,
      roughness: 0.25,
      metalness: 0.05
    });

    const pupilMat = new THREE.MeshBasicMaterial({ color: 0x050505 });
    const glintMat = new THREE.MeshBasicMaterial({ color: 0xffffff });

    const eyeGeo = new THREE.SphereGeometry(0.025, 16, 16);
    const irisGeo = new THREE.SphereGeometry(0.014, 16, 16);
    irisGeo.scale(1.0, 1.0, 0.35);
    const pupilGeo = new THREE.SphereGeometry(0.0075, 12, 12);
    pupilGeo.scale(1.0, 1.0, 0.3);
    const glintGeo = new THREE.SphereGeometry(0.0028, 8, 8);

    [-0.062, 0.062].forEach((eyeX, idx) => {
      const eyeSubGroup = new THREE.Group();
      eyeSubGroup.position.set(eyeX, 0.025, 0.168);

      // Bola Mata
      const eyeball = new THREE.Mesh(eyeGeo, scleraMat);
      eyeSubGroup.add(eyeball);

      // Iris Cokelat Alami
      const iris = new THREE.Mesh(irisGeo, irisMat);
      iris.position.set(0, 0, 0.017);
      eyeSubGroup.add(iris);

      // Pupil Hitam Pekat
      const pupil = new THREE.Mesh(pupilGeo, pupilMat);
      pupil.position.set(0, 0, 0.021);
      eyeSubGroup.add(pupil);

      // Pantulan Cahaya Hidup (Catchlight)
      const glint = new THREE.Mesh(glintGeo, glintMat);
      glint.position.set(0.004, 0.004, 0.023);
      eyeSubGroup.add(glint);

      faceGroup.add(eyeSubGroup);

      // Lipatan Kelopak Mata Atas 3D
      const lidGeo = new THREE.BoxGeometry(0.058, 0.012, 0.022);
      const eyelid = new THREE.Mesh(lidGeo, skinMat);
      eyelid.position.set(eyeX, 0.044, 0.178);
      eyelid.rotation.z = (idx === 0) ? -0.08 : 0.08;
      faceGroup.add(eyelid);

      // Alis 3D
      const browMat = new THREE.MeshStandardMaterial({ color: 0x181411, roughness: 0.9 });
      const browGeo = new THREE.BoxGeometry(0.068, 0.015, 0.02);
      const brow = new THREE.Mesh(browGeo, browMat);
      brow.position.set(eyeX, 0.068, 0.174);
      brow.rotation.z = (idx === 0) ? -0.12 : 0.12;
      faceGroup.add(brow);
    });

    // 4. Hidung Manusia 3D Fisik (Batang Hidung, Cuping & Lubang Hidung)
    const noseMat = skinMat.clone();
    noseMat.color = new THREE.Color(skinMat.color).multiplyScalar(0.97);

    // Batang Hidung (Nasal Bridge)
    const bridgeGeo = new THREE.CylinderGeometry(0.012, 0.020, 0.075, 8);
    const bridge = new THREE.Mesh(bridgeGeo, skinMat);
    bridge.position.set(0, -0.01, 0.194);
    bridge.rotation.x = 0.26;
    faceGroup.add(bridge);

    // Ujung Cuping Hidung (Nose Tip)
    const tipGeo = new THREE.SphereGeometry(0.019, 12, 12);
    const noseTip = new THREE.Mesh(tipGeo, noseMat);
    noseTip.position.set(0, -0.042, 0.210);
    faceGroup.add(noseTip);

    // Sayap Hidung Kiri & Kanan (Nostril Wings)
    const wingGeo = new THREE.SphereGeometry(0.011, 8, 8);
    const wingL = new THREE.Mesh(wingGeo, noseMat);
    wingL.position.set(-0.020, -0.046, 0.198);
    faceGroup.add(wingL);

    const wingR = new THREE.Mesh(wingGeo, noseMat);
    wingR.position.set(0.020, -0.046, 0.198);
    faceGroup.add(wingR);

    // Bayangan Bawah Cuping (Nostril Cavity)
    const nostrilCavity = new THREE.Mesh(new THREE.BoxGeometry(0.024, 0.005, 0.015), new THREE.MeshBasicMaterial({ color: 0x1f110a }));
    nostrilCavity.position.set(0, -0.053, 0.200);
    faceGroup.add(nostrilCavity);

    // 5. Bibir & Mulut Manusia 3D Realistis (Cupid's Bow & Lower Lip Plump)
    const lipHex = isFemale
      ? (cfg.id === 'agen_humas' ? 0xd06159 : 0xc25b52)
      : 0x9e574f;
    const lipMat = new THREE.MeshStandardMaterial({
      color: lipHex,
      roughness: 0.55,
      metalness: 0.05
    });

    // Bibir Atas dengan Lekukan Cupid's Bow
    const upperLipGeo = new THREE.BoxGeometry(isFemale ? 0.066 : 0.074, 0.015, 0.022);
    const upperLip = new THREE.Mesh(upperLipGeo, lipMat);
    upperLip.position.set(0, -0.090, 0.188);
    faceGroup.add(upperLip);

    // Bibir Bawah Lembut & Berisi
    const lowerLipGeo = new THREE.BoxGeometry(isFemale ? 0.062 : 0.070, 0.019, 0.024);
    const lowerLip = new THREE.Mesh(lowerLipGeo, lipMat);
    lowerLip.position.set(0, -0.108, 0.185);
    faceGroup.add(lowerLip);

    // Garis Rekahan Mulut (Smile Line)
    const mouthLine = new THREE.Mesh(new THREE.BoxGeometry(isFemale ? 0.066 : 0.074, 0.004, 0.015), new THREE.MeshBasicMaterial({ color: 0x2c1310 }));
    mouthLine.position.set(0, -0.098, 0.187);
    faceGroup.add(mouthLine);

    // 6. Kacamata Titanium Eksekutif 3D untuk Coder, Arsitek, Auditor & Analis
    if (['agen_arsitek', 'agen_coder', 'agen_keamanan', 'agen_analis'].includes(cfg.id)) {
      const frameColor = cfg.id === 'agen_analis' ? 0x06b6d4 : (cfg.id === 'agen_keamanan' ? 0x475569 : 0x1e293b);
      const frameMat = new THREE.MeshStandardMaterial({ color: frameColor, metalness: 0.85, roughness: 0.2 });
      const rimGeo = new THREE.TorusGeometry(0.028, 0.0035, 8, 20);

      // Bingkai Kacamata Kiri
      const rimL = new THREE.Mesh(rimGeo, frameMat);
      rimL.position.set(-0.062, 0.025, 0.194);
      faceGroup.add(rimL);

      // Bingkai Kacamata Kanan
      const rimR = new THREE.Mesh(rimGeo, frameMat);
      rimR.position.set(0.062, 0.025, 0.194);
      faceGroup.add(rimR);

      // Jembatan Hidung Kacamata (Bridge)
      const gBridge = new THREE.Mesh(new THREE.BoxGeometry(0.038, 0.004, 0.006), frameMat);
      gBridge.position.set(0, 0.030, 0.197);
      faceGroup.add(gBridge);

      // Gagang Kacamata ke Telinga
      const armGeo = new THREE.BoxGeometry(0.004, 0.004, 0.19);
      const armL = new THREE.Mesh(armGeo, frameMat);
      armL.position.set(-0.090, 0.025, 0.10);
      armL.rotation.y = 0.04;
      faceGroup.add(armL);

      const armR = new THREE.Mesh(armGeo, frameMat);
      armR.position.set(0.090, 0.025, 0.10);
      armR.rotation.y = -0.04;
      faceGroup.add(armR);
    }

    headGroup.add(faceGroup);
  }

  // ================= HUMAN AVATAR REALISTIK (DUDUK MENEMPEL DI KURSI) =================
  createHumanoidAvatar(cfg) {
    const char = new THREE.Group();
    const isFemale = cfg.gender === 'wanita';

    // 1. Skin Tone Alami Indonesia
    const skinPalette = {
      agen_orkestrator: 0xc48c62,
      agen_analis: 0xdfa67f,
      agen_arsitek: 0xc68d60,
      agen_coder: 0xd89e74,
      agen_qa: 0xe2b28c,
      agen_devops: 0xbf855b,
      agen_keamanan: 0xb87f54,
      agen_notulis: 0xdea37b,
      agen_humas: 0xd59b72,
      agen_analis_data: 0xc88b5e
    };
    const skinHex = skinPalette[cfg.id] || (isFemale ? 0xdfa67f : 0xc48c62);
    const skinMat = new THREE.MeshStandardMaterial({
      color: skinHex,
      roughness: 0.72,
      metalness: 0.0
    });

    // 2. Celana Formal & Posisi Duduk Alami Tepat di Bantalan Kursi
    const pantsMat = new THREE.MeshStandardMaterial({
      color: 0x0f172a,
      roughness: 0.85,
      metalness: 0.05
    });

    // Paha Horizontal Menempel Sempurna di Atas Dudukan Kursi (y = 0.62)
    const thighGeo = new THREE.BoxGeometry(0.18, 0.14, 0.40);
    const thighL = new THREE.Mesh(thighGeo, pantsMat);
    thighL.position.set(-0.13, 0.62, 0.18);
    char.add(thighL);

    const thighR = new THREE.Mesh(thighGeo, pantsMat);
    thighR.position.set(0.13, 0.62, 0.18);
    char.add(thighR);

    // Lutut Halus
    const kneeGeo = new THREE.CylinderGeometry(0.07, 0.07, 0.17, 12);
    const kneeL = new THREE.Mesh(kneeGeo, pantsMat);
    kneeL.rotation.z = Math.PI / 2;
    kneeL.position.set(-0.13, 0.61, 0.36);
    char.add(kneeL);

    const kneeR = new THREE.Mesh(kneeGeo, pantsMat);
    kneeR.rotation.z = Math.PI / 2;
    kneeR.position.set(0.13, 0.61, 0.36);
    char.add(kneeR);

    // Betis Vertikal Turun Tegak Lurus Menuju Lantai (y = 0.08 s/d 0.61)
    const shinGeo = new THREE.BoxGeometry(0.15, 0.52, 0.14);
    const shinL = new THREE.Mesh(shinGeo, pantsMat);
    shinL.position.set(-0.13, 0.34, 0.36);
    char.add(shinL);

    const shinR = new THREE.Mesh(shinGeo, pantsMat);
    shinR.position.set(0.13, 0.34, 0.36);
    char.add(shinR);

    // Sepatu Kulit Oxford Menempel Rata di Atas Lantai (y = 0.00 s/d 0.09)
    const shoeMat = new THREE.MeshStandardMaterial({
      color: 0x09090b,
      roughness: 0.28,
      metalness: 0.3
    });
    const shoeGeo = new THREE.BoxGeometry(0.16, 0.085, 0.25);
    const shoeL = new THREE.Mesh(shoeGeo, shoeMat);
    shoeL.position.set(-0.13, 0.045, 0.41);
    char.add(shoeL);

    const shoeR = new THREE.Mesh(shoeGeo, shoeMat);
    shoeR.position.set(0.13, 0.045, 0.41);
    char.add(shoeR);

    // Sol Sepatu Menempel di Lantai
    const soleGeo = new THREE.BoxGeometry(0.165, 0.015, 0.26);
    const soleMat = new THREE.MeshBasicMaterial({ color: 0x020202 });
    const soleL = new THREE.Mesh(soleGeo, soleMat);
    soleL.position.set(-0.13, 0.008, 0.41);
    char.add(soleL);
    const soleR = new THREE.Mesh(soleGeo, soleMat);
    soleR.position.set(0.13, 0.008, 0.41);
    char.add(soleR);

    // 3. Jas / Blazer Bahan Wol Korporat
    const suitPalette = {
      agen_orkestrator: 0x1e3a8a,
      agen_analis: 0x0e7490,
      agen_arsitek: 0x312e81,
      agen_coder: 0x1e293b,
      agen_qa: 0x9d174d,
      agen_devops: 0x334155,
      agen_keamanan: 0x0f172a,
      agen_notulis: 0x581c87,
      agen_humas: 0x0284c7 // Humas Telegram Sky Blue
    };
    const suitHex = suitPalette[cfg.id] || 0x1e293b;
    const suitMat = new THREE.MeshStandardMaterial({
      color: suitHex,
      roughness: 0.85,
      metalness: 0.05
    });

    const torsoW = isFemale ? 0.46 : 0.54;
    const torsoD = isFemale ? 0.28 : 0.33;
    const torsoGeo = new THREE.BoxGeometry(torsoW, 0.52, torsoD);
    const torso = new THREE.Mesh(torsoGeo, suitMat);
    torso.position.set(0, 0.88, 0);
    torso.castShadow = true;
    char.add(torso);

    // Kemeja Berkerah Putih
    const shirtMat = new THREE.MeshStandardMaterial({ color: 0xf8fafc, roughness: 0.7 });
    const shirtInner = new THREE.Mesh(new THREE.BoxGeometry(0.22, 0.38, 0.05), shirtMat);
    shirtInner.position.set(0, 0.93, 0.16);
    char.add(shirtInner);

    const collarL = new THREE.Mesh(new THREE.BoxGeometry(0.1, 0.08, 0.04), shirtMat);
    collarL.position.set(-0.08, 1.11, 0.17);
    collarL.rotation.z = -0.3;
    char.add(collarL);

    const collarR = new THREE.Mesh(new THREE.BoxGeometry(0.1, 0.08, 0.04), shirtMat);
    collarR.position.set(0.08, 1.11, 0.17);
    collarR.rotation.z = 0.3;
    char.add(collarR);

    // Dasi Sutra Pria / Kerah Lapel Wanita
    if (!isFemale) {
      const tieMat = new THREE.MeshStandardMaterial({ color: 0x0284c7, roughness: 0.4 });
      const tie = new THREE.Mesh(new THREE.BoxGeometry(0.07, 0.35, 0.04), tieMat);
      tie.position.set(0, 0.90, 0.18);
      char.add(tie);

      // Gold Tie Clip
      const clipMat = new THREE.MeshStandardMaterial({ color: 0xd4af37, metalness: 0.9, roughness: 0.2 });
      const tieClip = new THREE.Mesh(new THREE.BoxGeometry(0.08, 0.02, 0.02), clipMat);
      tieClip.position.set(0, 0.94, 0.205);
      char.add(tieClip);
    } else {
      const lapelMat = new THREE.MeshStandardMaterial({ color: suitHex, roughness: 0.8 });
      const lapelL = new THREE.Mesh(new THREE.BoxGeometry(0.12, 0.32, 0.05), lapelMat);
      lapelL.position.set(-0.14, 0.94, 0.17);
      lapelL.rotation.z = 0.22;
      char.add(lapelL);

      const lapelR = new THREE.Mesh(new THREE.BoxGeometry(0.12, 0.32, 0.05), lapelMat);
      lapelR.position.set(0.14, 0.94, 0.17);
      lapelR.rotation.z = -0.22;
      char.add(lapelR);
    }

    // Name Badge Lanyard
    const badgeBg = new THREE.Mesh(new THREE.BoxGeometry(0.08, 0.11, 0.02), new THREE.MeshStandardMaterial({ color: 0xffffff, roughness: 0.3 }));
    badgeBg.position.set(0.16, 0.99, 0.18);
    char.add(badgeBg);

    const badgeStrip = new THREE.Mesh(new THREE.BoxGeometry(0.08, 0.03, 0.021), new THREE.MeshStandardMaterial({ color: cfg.color }));
    badgeStrip.position.set(0.16, 1.025, 0.18);
    char.add(badgeStrip);

    // 4. Leher & Kepala Realistis
    const neck = new THREE.Mesh(new THREE.CylinderGeometry(0.075, 0.088, 0.12, 16), skinMat);
    neck.position.set(0, 1.18, 0.01);
    char.add(neck);

    const headGroup = new THREE.Group();
    headGroup.position.set(0, 1.34, 0.03);

    const headGeo = new THREE.SphereGeometry(0.185, 24, 24);
    headGeo.scale(1.0, 1.12, 1.05);
    const headMesh = new THREE.Mesh(headGeo, skinMat);
    headMesh.castShadow = true;
    headGroup.add(headMesh);

    // BANGUN FITUR WAJAH 3D FISIK ASLI (NO FLAT CANVAS STICKER!)
    this.buildSculpted3DFace(headGroup, cfg, skinMat, isFemale);

    // Telinga Manusia 3D
    if (cfg.id !== 'agen_qa') {
      const earGeo = new THREE.CylinderGeometry(0.02, 0.035, 0.07, 8);
      const earL = new THREE.Mesh(earGeo, skinMat);
      earL.position.set(-0.19, 0.01, 0.01);
      earL.rotation.z = Math.PI / 2;
      headGroup.add(earL);

      const earR = new THREE.Mesh(earGeo, skinMat);
      earR.position.set(0.19, 0.01, 0.01);
      earR.rotation.z = -Math.PI / 2;
      headGroup.add(earR);
    }

    // 5. Rambut & Hijab Realistis
    const hairMat = new THREE.MeshStandardMaterial({
      color: 0x181616,
      roughness: 0.88
    });

    if (cfg.id === 'agen_qa') {
      // Siti Nurhaliza: Hijab Anggun Sutra Dusty-Rose Menutup Kepala & Dada
      const hijabMat = new THREE.MeshStandardMaterial({ color: 0x831843, roughness: 0.65 });
      const hijabCap = new THREE.Mesh(new THREE.SphereGeometry(0.21, 20, 20, 0, Math.PI * 2, 0, Math.PI / 1.7), hijabMat);
      hijabCap.position.set(0, 0.03, 0);
      headGroup.add(hijabCap);

      const drape = new THREE.Mesh(new THREE.CylinderGeometry(0.12, 0.24, 0.25, 16), hijabMat);
      drape.position.set(0, -0.12, 0.02);
      headGroup.add(drape);
    } else if (isFemale) {
      if (cfg.id === 'agen_analis') {
        const hairTop = new THREE.Mesh(new THREE.SphereGeometry(0.21, 18, 18, 0, Math.PI * 2, 0, Math.PI / 1.9), hairMat);
        hairTop.position.set(0, 0.03, 0);
        headGroup.add(hairTop);

        const bobL = new THREE.Mesh(new THREE.BoxGeometry(0.07, 0.26, 0.22), hairMat);
        bobL.position.set(-0.18, -0.06, 0.02);
        headGroup.add(bobL);

        const bobR = new THREE.Mesh(new THREE.BoxGeometry(0.07, 0.26, 0.22), hairMat);
        bobR.position.set(0.18, -0.06, 0.02);
        headGroup.add(bobR);
      } else {
        const hairTop = new THREE.Mesh(new THREE.SphereGeometry(0.21, 18, 18, 0, Math.PI * 2, 0, Math.PI / 1.9), hairMat);
        hairTop.position.set(0, 0.03, 0);
        headGroup.add(hairTop);

        const hairBack = new THREE.Mesh(new THREE.BoxGeometry(0.36, 0.34, 0.12), hairMat);
        hairBack.position.set(0, -0.1, -0.14);
        headGroup.add(hairBack);
      }
    } else {
      const hairTop = new THREE.Mesh(new THREE.SphereGeometry(0.205, 18, 18, 0, Math.PI * 2, 0, Math.PI / 2.05), hairMat);
      hairTop.position.set(0, 0.03, 0);
      headGroup.add(hairTop);

      const pomp = new THREE.Mesh(new THREE.BoxGeometry(0.24, 0.07, 0.22), hairMat);
      pomp.position.set(0, 0.18, 0.02);
      headGroup.add(pomp);

      const sbL = new THREE.Mesh(new THREE.BoxGeometry(0.04, 0.12, 0.08), hairMat);
      sbL.position.set(-0.18, -0.01, 0.03);
      headGroup.add(sbL);

      const sbR = new THREE.Mesh(new THREE.BoxGeometry(0.04, 0.12, 0.08), hairMat);
      sbR.position.set(0.18, -0.01, 0.03);
      headGroup.add(sbR);
    }

    char.add(headGroup);

    // 6. Lengan & Tangan Mengetik Tepat di Atas Meja Kerja (Keyboard & Mouse)
    const armMat = suitMat;
    const armUpperGeo = new THREE.CylinderGeometry(0.065, 0.06, 0.28, 12);
    const armForeGeo = new THREE.CylinderGeometry(0.055, 0.05, 0.28, 12);

    // Lengan Kiri (Mengetik Keyboard)
    const upperArmL = new THREE.Mesh(armUpperGeo, armMat);
    upperArmL.position.set(-0.29, 1.02, 0.10);
    upperArmL.rotation.x = 0.65;
    upperArmL.rotation.z = -0.12;
    char.add(upperArmL);

    const foreArmL = new THREE.Mesh(armForeGeo, armMat);
    foreArmL.position.set(-0.23, 1.03, 0.36);
    foreArmL.rotation.x = 1.45;
    foreArmL.rotation.y = -0.22;
    char.add(foreArmL);

    const handL = new THREE.Mesh(new THREE.BoxGeometry(0.09, 0.035, 0.11), skinMat);
    handL.position.set(-0.14, 1.06, 0.52);
    char.add(handL);

    // Lengan Kanan (Memegang Mouse / Mengetik)
    const upperArmR = new THREE.Mesh(armUpperGeo, armMat);
    upperArmR.position.set(0.29, 1.02, 0.10);
    upperArmR.rotation.x = 0.65;
    upperArmR.rotation.z = 0.12;
    char.add(upperArmR);

    const foreArmR = new THREE.Mesh(armForeGeo, armMat);
    foreArmR.position.set(0.24, 1.03, 0.36);
    foreArmR.rotation.x = 1.45;
    foreArmR.rotation.y = 0.22;
    char.add(foreArmR);

    const handR = new THREE.Mesh(new THREE.BoxGeometry(0.09, 0.035, 0.11), skinMat);
    handR.position.set(0.24, 1.06, 0.52);
    char.add(handR);

    char.userData = {
      agentId: cfg.id,
      torso: torso,
      headGroup: headGroup,
      handL: handL,
      handR: handR
    };

    return char;
  }
  // ================= 3D SPEECH BUBBLE DI ATAS MEJA AGEN =================
  showSpeechBubble(agentId, text, duration = 5000) {
    const agent = this.agents3D.get(agentId);
    if (!agent) return;

    // Hapus bubble lama jika ada
    if (this.speechBubbles.has(agentId)) {
      const oldBubble = this.speechBubbles.get(agentId);
      this.scene.remove(oldBubble);
      this.speechBubbles.delete(agentId);
    }

    // Buat kanvas bubble teks beresolusi tinggi
    const canvas = document.createElement('canvas');
    canvas.width = 512;
    canvas.height = 192;
    const bctx = canvas.getContext('2d');

    // Kotak Balon Ucapan
    bctx.fillStyle = 'rgba(15, 23, 42, 0.92)';
    bctx.strokeStyle = '#06b6d4';
    bctx.lineWidth = 4;
    bctx.beginPath();
    bctx.roundRect(10, 10, 492, 140, 20);
    bctx.fill();
    bctx.stroke();

    // Segitiga Ekor Balon
    bctx.fillStyle = 'rgba(15, 23, 42, 0.92)';
    bctx.beginPath();
    bctx.moveTo(236, 150);
    bctx.lineTo(256, 180);
    bctx.lineTo(276, 150);
    bctx.closePath();
    bctx.fill();

    // Teks Pengirim
    bctx.fillStyle = '#38bdf8';
    bctx.font = 'bold 24px "Plus Jakarta Sans", sans-serif';
    bctx.fillText(`💬 ${agent.nama}`, 30, 48);

    // Teks Pesan (Auto wrap)
    bctx.fillStyle = '#f8fafc';
    bctx.font = '20px "Plus Jakarta Sans", sans-serif';
    const words = text.split(' ');
    let line1 = '', line2 = '';
    for (let w of words) {
      if ((line1 + w).length < 34) {
        line1 += w + ' ';
      } else {
        line2 += w + ' ';
      }
    }
    bctx.fillText(line1.trim(), 30, 88);
    if (line2) bctx.fillText(line2.trim().substring(0, 36) + (line2.length > 36 ? '...' : ''), 30, 122);

    const texture = new THREE.CanvasTexture(canvas);
    const spriteMat = new THREE.SpriteMaterial({ map: texture, transparent: true, opacity: 0 });
    const sprite = new THREE.Sprite(spriteMat);

    // Posisikan di atas kepala dan tag visual agen
    const pos = agent.charGroup.position;
    sprite.position.set(pos.x, pos.y + 2.95, pos.z);
    sprite.scale.set(3.4, 1.25, 1);
    this.scene.add(sprite);
    this.speechBubbles.set(agentId, sprite);

    // Fade in
    new TWEEN.Tween(spriteMat).to({ opacity: 1 }, 300).start();

    // Fade out & hapus setelah durasi
    setTimeout(() => {
      new TWEEN.Tween(spriteMat)
        .to({ opacity: 0 }, 500)
        .onComplete(() => {
          this.scene.remove(sprite);
          this.speechBubbles.delete(agentId);
        })
        .start();
    }, duration);
  }

  // Mengumpulkan seluruh agen ke Ruang Sidang Boardroom Kaca
  gatherAgentsForMeeting() {
    this.agents3D.forEach((agent) => {
      agent.isMeeting = true;
      if (typeof TWEEN !== 'undefined') {
        new TWEEN.Tween(agent.charGroup.position)
          .to(agent.meetPos, 1400)
          .easing(TWEEN.Easing.Cubic.InOut)
          .start();

        new TWEEN.Tween(agent.charGroup.rotation)
          .to({ y: agent.meetPos.rotY }, 1400)
          .easing(TWEEN.Easing.Cubic.InOut)
          .start();
      } else {
        agent.charGroup.position.set(agent.meetPos.x, agent.meetPos.y, agent.meetPos.z);
        agent.charGroup.rotation.y = agent.meetPos.rotY;
      }
    });

    if (typeof TWEEN !== 'undefined') {
      if (this.meetingLight) new TWEEN.Tween(this.meetingLight).to({ intensity: 2.2 }, 1000).start();
      if (this.holoRingMat) new TWEEN.Tween(this.holoRingMat).to({ opacity: 1.0 }, 1000).start();

      if (this.controls) {
        // Dekatkan kamera ke Ruang Boardroom Kaca (-8.5, 0)
        new TWEEN.Tween(this.camera.position).to({ x: -2, y: 7.5, z: 9 }, 1200).easing(TWEEN.Easing.Cubic.Out).start();
        new TWEEN.Tween(this.controls.target).to({ x: -8.5, y: 1.4, z: 0 }, 1200).easing(TWEEN.Easing.Cubic.Out).start();
      }
    }
  }

  // Membubarkan rapat dan mengembalikan agen ke meja masing-masing di pod kerja
  dismissMeetingToDesks() {
    this.agents3D.forEach((agent) => {
      agent.isMeeting = false;
      if (typeof TWEEN !== 'undefined') {
        new TWEEN.Tween(agent.charGroup.position)
          .to(agent.deskPos, 1400)
          .easing(TWEEN.Easing.Cubic.InOut)
          .start();

        new TWEEN.Tween(agent.charGroup.rotation)
          .to({ y: agent.deskPos.rotY }, 1400)
          .easing(TWEEN.Easing.Cubic.InOut)
          .start();
      } else {
        agent.charGroup.position.set(agent.deskPos.x, agent.deskPos.y, agent.deskPos.z);
        agent.charGroup.rotation.y = agent.deskPos.rotY;
      }
    });

    if (typeof TWEEN !== 'undefined') {
      if (this.meetingLight) new TWEEN.Tween(this.meetingLight).to({ intensity: 0.6 }, 1000).start();
      if (this.holoRingMat) new TWEEN.Tween(this.holoRingMat).to({ opacity: 0.3 }, 1000).start();
    }

    this.setCameraView('iso');
  }

  // ============================================================
  // SISTEM IDLE WALK — Agen Bebas Bergerak saat Beban Kerja = 0
  // ============================================================

  get IDLE_WAYPOINTS() {
    return [
      // Pantry & Coffee Lounge
      { x: -10.5, z: 7.5,  rotY: 0,              label: 'Ngopi di Pantry',         icon: '\u2615' },
      { x: -11.5, z: 9.0,  rotY: Math.PI / 2,    label: 'Duduk Sofa Lounge',       icon: '\ud83d\udecc' },
      { x: -9.8,  z: 8.2,  rotY: -Math.PI / 4,   label: 'Ambil Minuman',            icon: '\ud83e\udd64' },
      // Area Jendela Panoramik
      { x: 0,     z: -12.2, rotY: Math.PI,        label: 'Melihat Kota',             icon: '\ud83c\udf06' },
      { x: 5.0,   z: -12.2, rotY: Math.PI,        label: 'Istirahat di Jendela',     icon: '\ud83e\uddd8' },
      { x: -5.0,  z: -12.2, rotY: Math.PI,        label: 'Meregangkan Badan',         icon: '\ud83e\udd38' },
      // Water Cooler / Aisle Tengah
      { x: 0,     z: -2.5,  rotY: 0,              label: 'Minum Air Putih',           icon: '\ud83d\udca7' },
      { x: 0,     z: 3.0,   rotY: Math.PI,        label: 'Obrolan Santai',            icon: '\ud83d\udcac' },
      // Tanaman Biophilic
      { x: -13.0, z: 5.0,   rotY: Math.PI / 2,   label: 'Merawat Tanaman',            icon: '\ud83c\udf3f' },
      { x: 11.0,  z: 9.0,   rotY: -Math.PI / 2,  label: 'Berdiri Santai',             icon: '\ud83e\uddcd' }
    ];
  }

  startIdleWalkEngine() {
    this._idleWalkState = new Map();
    this.agents3D.forEach((agent, id) => {
      this._idleWalkState.set(id, {
        state: 'desk',             // 'desk' | 'walking' | 'idle_at_waypoint'
        waypointIdx: -1,
        timer: 5000 + Math.random() * 15000 // stagger awal 5-20s
      });
    });

    this._idleWalkTimer = setInterval(() => { this._tickIdleWalk(); }, 1000);
  }

  _tickIdleWalk() {
    const workloadMap = window.AGENT_WORKLOAD || {};

    this.agents3D.forEach((agent, id) => {
      if (agent.isMeeting) return;

      const beban = (workloadMap[id] !== undefined) ? Number(workloadMap[id]) : -1;
      const ws = this._idleWalkState ? this._idleWalkState.get(id) : null;
      if (!ws) return;

      ws.timer -= 1000;

      // Jika ada tugas aktif — paksa kembali ke meja
      if (beban > 0) {
        if (ws.state !== 'desk') {
          this._walkAgentTo(agent, agent.deskPos);
          ws.state = 'desk';
          this._setAgentFloatingActivity(id, null);
        }
        return;
      }

      // Beban = 0 — agen bebas jalan-jalan
      if (ws.state === 'desk' && ws.timer <= 0) {
        const wps = this.IDLE_WAYPOINTS;
        const wpIdx = Math.floor(Math.random() * wps.length);
        const wp = wps[wpIdx];
        ws.waypointIdx = wpIdx;
        ws.state = 'walking';
        this._setAgentFloatingActivity(id, `${wp.icon} ${wp.label}`);
        this._walkAgentTo(agent, { x: wp.x, y: 0, z: wp.z, rotY: wp.rotY }, () => {
          ws.state = 'idle_at_waypoint';
          ws.timer = 15000 + Math.random() * 25000; // istirahat 15-40s
        });

      } else if (ws.state === 'idle_at_waypoint' && ws.timer <= 0) {
        ws.state = 'walking';
        this._setAgentFloatingActivity(id, '\ud83d\udeb6 Kembali ke Meja');
        this._walkAgentTo(agent, agent.deskPos, () => {
          ws.state = 'desk';
          this._setAgentFloatingActivity(id, null);
          ws.timer = 20000 + Math.random() * 30000; // tunggu 20-50s
        });

      } else if (ws.state === 'desk' && ws.timer <= 0) {
        ws.timer = 10000 + Math.random() * 15000;
      }
    });
  }

  _walkAgentTo(agent, target, onDone) {
    if (!agent.charGroup) { if (onDone) onDone(); return; }
    if (typeof TWEEN === 'undefined') {
      agent.charGroup.position.set(target.x, target.y || 0, target.z);
      if (target.rotY !== undefined) agent.charGroup.rotation.y = target.rotY;
      if (onDone) onDone();
      return;
    }
    const dur = 2000 + Math.random() * 1500;
    new TWEEN.Tween(agent.charGroup.position)
      .to({ x: target.x, y: target.y || 0, z: target.z }, dur)
      .easing(TWEEN.Easing.Quadratic.InOut)
      .onComplete(() => { if (onDone) onDone(); })
      .start();
    if (target.rotY !== undefined) {
      new TWEEN.Tween(agent.charGroup.rotation)
        .to({ y: target.rotY }, dur * 0.35)
        .easing(TWEEN.Easing.Quadratic.Out)
        .start();
    }
  }

  _setAgentFloatingActivity(agentId, activity) {
    const agent = this.agents3D.get(agentId);
    if (!agent) return;
    // Update tag HTML
    if (agent.tagEl) {
      const roleEl = agent.tagEl.querySelector('.agent-tag-role-badge');
      if (roleEl) {
        roleEl.textContent = activity || agent.roleLabel;
        roleEl.style.color = activity ? '#fbbf24' : '';
        roleEl.style.fontStyle = activity ? 'italic' : '';
      }
    }
    // Update HUD workload jika panel dossier terbuka untuk agen ini
    if (window.AGENT_WORKLOAD && window.AGENT_WORKLOAD[agentId] === 0) {
      const statusEl = document.getElementById('dossier-status');
      const taskEl = document.getElementById('dossier-task');
      const loadEl = document.getElementById('dossier-load-val');
      const fillEl = document.getElementById('dossier-load-fill');
      const openId = document.getElementById('agent-dossier');
      if (openId && openId.style.display !== 'none') {
        const curSop = document.getElementById('dossier-sop');
        const curAgent = Array.from(this.agents3D.values()).find(a => a.id === agentId);
        if (curSop && curAgent && curAgent.sop === (document.getElementById('dossier-sop') || {}).textContent) {
          if (statusEl) statusEl.textContent = activity ? `\ud83d\udfe1 ${activity}` : '\ud83d\udfe2 IDLE — Siap Tugas Baru';
          if (taskEl) taskEl.textContent = activity ? activity : '\u2014 Tidak ada tugas aktif saat ini \u2014';
          if (loadEl) loadEl.textContent = '0%';
          if (fillEl) fillEl.style.width = '0%';
        }
      }
    }
  }

  setupRaycaster() {
    this.raycaster = new THREE.Raycaster();
    this.mouse = new THREE.Vector2();

    window.addEventListener('click', (e) => {
      if (e.target.closest('.hud-top') || e.target.closest('.hud-left-roster') || 
          e.target.closest('.hud-agent-card') || e.target.closest('.hud-bottom-bar') ||
          e.target.closest('.modal-backdrop') || e.target.closest('.hud-chatter-dock')) {
        return;
      }

      this.mouse.x = (e.clientX / window.innerWidth) * 2 - 1;
      this.mouse.y = -(e.clientY / window.innerHeight) * 2 + 1;

      this.raycaster.setFromCamera(this.mouse, this.camera);
      const intersects = this.raycaster.intersectObjects(this.scene.children, true);

      for (let hit of intersects) {
        let curr = hit.object;
        while (curr) {
          if (curr.userData && curr.userData.isServerRack) {
            window.dispatchEvent(new CustomEvent('open-widyabase-modal'));
            return;
          }
          if (curr.userData && curr.userData.agentId) {
            window.dispatchEvent(new CustomEvent('agent-selected', { detail: { agentId: curr.userData.agentId } }));
            return;
          }
          curr = curr.parent;
        }
      }
    });
  }

  onWindowResize() {
    this.camera.aspect = window.innerWidth / window.innerHeight;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(window.innerWidth, window.innerHeight);
  }

  animate(time) {
    requestAnimationFrame((t) => this.animate(t));
    if (typeof TWEEN !== 'undefined') TWEEN.update();
    if (this.controls) this.controls.update();

    if (this.holoCore) {
      this.holoCore.rotation.x += 0.015;
      this.holoCore.rotation.y += 0.02;
    }

    const tSec = (time || 0) * 0.003;
    if (this.serverLeds && this.serverLeds.length > 0) {
      this.serverLeds.forEach((led, idx) => {
        const pulse = Math.sin(tSec * 5 + idx * 1.3);
        led.material.opacity = pulse > 0.15 ? 0.95 : 0.2;
      });
    }

    // Animasi per-agen berdasarkan state (desk / walking / idle_at_waypoint)
    this.agents3D.forEach((agent) => {
      if (!agent.isMeeting && agent.charGroup) {
        const ws = this._idleWalkState ? this._idleWalkState.get(agent.id) : null;
        const isWalking = ws && ws.state === 'walking';
        const isAtWaypoint = ws && ws.state === 'idle_at_waypoint';
        const tIdle = tSec * 1.5 + (agent.deskPos ? agent.deskPos.x : 0);

        if (!isWalking) {
          // Pernapasan alami
          if (agent.torso) {
            agent.torso.scale.y = 1 + Math.sin(tIdle * 1.8) * 0.012;
            agent.torso.scale.z = 1 + Math.sin(tIdle * 1.8) * 0.015;
          }
          if (agent.handL && agent.handR) {
            if (isAtWaypoint) {
              // Memegang gelas / santai di waypoint
              agent.handL.position.y = 1.10 + Math.sin(tIdle * 1.2) * 0.025;
              agent.handR.position.y = 1.08 + Math.cos(tIdle * 0.9) * 0.020;
            } else {
              // Mengetik di meja
              agent.handL.position.y = 1.06 + Math.sin(tIdle * 5.0) * 0.003;
              agent.handR.position.y = 1.06 + Math.cos(tIdle * 4.5) * 0.003;
            }
          }
          if (agent.headGroup) {
            const headAmpl = isAtWaypoint ? 0.08 : 0.03;
            const headFreq = isAtWaypoint ? 0.3 : 0.5;
            agent.headGroup.rotation.y = Math.sin(tIdle * headFreq) * headAmpl;
          }
        }
      }
    });

    // Perbarui proyeksi tag HTML 2D di atas kepala masing-masing agen
    this.updateFloatingTags();

    this.renderer.render(this.scene, this.camera);
  }

  // Menghitung proyeksi 3D ke 2D koordinat layar untuk seluruh Tag Agen
  updateFloatingTags() {
    if (!this.labelsContainer) return;
    if (!this.showTags) {
      this.agents3D.forEach((agent) => {
        if (agent.tagEl) agent.tagEl.style.display = 'none';
      });
      return;
    }

    const tempV = new THREE.Vector3();
    const halfW = window.innerWidth * 0.5;
    const halfH = window.innerHeight * 0.5;

    this.agents3D.forEach((agent) => {
      if (!agent.tagEl || !agent.charGroup) return;

      // Ambil posisi dunia di atas kepala agen
      agent.charGroup.getWorldPosition(tempV);
      tempV.y += 1.85;

      // Proyeksikan ke ruang layar kamera
      const screenV = tempV.clone().project(this.camera);

      // Sembunyikan jika berada di belakang kamera atau di luar jangkauan dekat
      if (screenV.z > 1.0 || screenV.z < -1.0) {
        agent.tagEl.style.display = 'none';
        return;
      }

      // Konversi NDC (-1 s/d 1) ke koordinat pixel
      const x = screenV.x * halfW + halfW;
      const y = -screenV.y * halfH + halfH;

      // Cek apakah posisi berada di dalam batas viewport layar
      if (x < -100 || x > window.innerWidth + 100 || y < -80 || y > window.innerHeight + 80) {
        agent.tagEl.style.display = 'none';
        return;
      }

      // Hitung skala berdasarkan jarak kamera agar perspektif terasa alami
      const dist = this.camera.position.distanceTo(tempV);
      const scale = Math.max(0.68, Math.min(1.05, 24 / dist));

      agent.tagEl.style.display = 'flex';
      agent.tagEl.style.transform = `translate(-50%, -100%) translate3d(${Math.round(x)}px, ${Math.round(y)}px, 0) scale(${scale.toFixed(2)})`;
      agent.tagEl.style.zIndex = Math.round(1500 - dist * 10);
    });
  }

  // Mengaktifkan / Menonaktifkan Tag Visual Agen
  toggleTags(visible) {
    this.showTags = (visible !== undefined) ? visible : !this.showTags;
    if (this.labelsContainer) {
      this.agents3D.forEach((agent) => {
        if (agent.tagEl) {
          agent.tagEl.style.display = this.showTags ? 'flex' : 'none';
        }
      });
    }
    return this.showTags;
  }
}
