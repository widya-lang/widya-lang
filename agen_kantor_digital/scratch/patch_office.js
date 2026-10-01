const fs = require('fs');
let code = fs.readFileSync('web_3d/office3d.js', 'utf8');

const startMarker = '  // ================= HUMAN AVATAR REALISTIK DENGAN GENDER =================';
const endMarker = '  // ================= 3D SPEECH BUBBLE DI ATAS MEJA AGEN =================';

const startIndex = code.indexOf(startMarker);
const endIndex = code.indexOf(endMarker);

if (startIndex === -1 || endIndex === -1) {
  console.error('Markers not found! startIndex:', startIndex, 'endIndex:', endIndex);
  process.exit(1);
}

const replacement = `  // ================= HUMAN AVATAR REALISTIK (SKULPTUR 3D FISIK ASLI) =================
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
      agen_humas: 0xd59b72
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
`;

code = code.substring(0, startIndex) + replacement + code.substring(endIndex);

// Also update handL and handR baseY in animate method
code = code.replace(
  'agent.handL.position.y = 0.59 + Math.sin(tIdle * 5.0) * 0.003;',
  'agent.handL.position.y = 1.06 + Math.sin(tIdle * 5.0) * 0.003;'
);
code = code.replace(
  'agent.handR.position.y = 0.59 + Math.cos(tIdle * 4.5) * 0.003;',
  'agent.handR.position.y = 1.06 + Math.cos(tIdle * 4.5) * 0.003;'
);

fs.writeFileSync('web_3d/office3d.js', code, 'utf8');
console.log('SUCCESS! Updated web_3d/office3d.js successfully.');
