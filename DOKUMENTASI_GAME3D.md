# 🎮 DOKUMENTASI RESMI WIDYAGAME3D & MESIN GAME 3D (v1.8.0)

Modul **WidyaGame3D** (`src/game3d/`) adalah mesin game 3D modern (*3D Game Engine*) skala industri yang terintegrasi langsung di dalam **Widya-Lang**. Engine ini dirancang untuk pembuatan game 3D berkualitas tinggi dengan arsitektur **Physically Based Rendering (PBR)**, simulasi fisika benda tegar (*Rigid Body Dynamics*), pipeline aset **glTF 2.0 / OBJ**, optimasi ruang **Octree Frustum Culling**, dan target rendering **WebGPU / WebGL2 / Native 3D**.

---

## 🌟 Arsitektur Utama Engine 3D

```
                     ┌────────────────────────────────────────┐
                     │        WIDYA 3D GAME ENGINE CORE       │
                     └───────────────────┬────────────────────┘
                                         │
    ┌──────────────────────┬─────────────┴────────┬──────────────────────┐
    │                      │                      │                      │
┌───▼─────────────┐ ┌──────▼────────────┐ ┌───────▼───────────┐ ┌────────▼──────────┐
│ 1. PBR GRAPHICS │ │ 2. ASSET PIPELINE │ │ 3. 3D PHYSICS     │ │ 4. SPATIAL & SCENE│
│ - Cook-Torrance │ │ - glTF 2.0 JSON   │ │ - Rigid Body 3D   │ │ - Scene Graph Tree│
│ - GGX Normal    │ │ - Wavefront .OBJ  │ │ - GJK / Overlap   │ │ - Octree Culling  │
│ - ACES Filmic   │ │ - Skeletal Bones  │ │ - Raycasting      │ │ - Perspective Cam │
│ - Dynamic Light │ │ - Slerp Animation │ │ - AABB / Capsule  │ │ - LookAt Matrix   │
└───┬─────────────┘ └──────┬────────────┘ └───────┬───────────┘ └────────┬──────────┘
    │                      │                      │                      │
    └──────────────────────┴──────────────┬───────┴──────────────────────┘
                                          │
                    ┌─────────────────────▼────────────────────┐
                    │    5. 3D RUNTIME & WEBGL2/WEBGPU PREVIEW │
                    │   - Fixed-timestep Physics Tick (60/120) │
                    │   - Standalone Interactive 3D Canvas     │
                    │   - Orbit Controls & Camera Projection   │
                    └──────────────────────────────────────────┘
```

---

## 🛠️ 1. Perintah CLI `widya game3d`

Widya-Lang menyediakan antarmuka baris perintah (CLI) untuk pembuatan aset, scaffolding dunia 3D, dan audit engine:

```bash
# 1. Menghasilkan Adegan Game 3D Baru Interaktif (HTML5 WebGL2/WebGPU)
widya game3d buat --nama "Petualangan_Nusantara" --output "petualangan.html"

# 2. Menghasilkan Mesh 3D Primitif (Kubus, Bola, Kapsul, Bidang) ke format .obj
widya game3d primitif --tipe kubus --ukuran 2.0 --output kubus.obj
widya game3d primitif --tipe bola --ukuran 1.5 --output bola.obj
widya game3d primitif --tipe kapsul --ukuran 1.0 --output karakter_kapsul.obj

# 3. Memeriksa Status Kesiapan Engine Game 3D
widya game3d status
```

---

## 💻 2. Contoh Pembuatan Game 3D di Widya-Lang

### A. Inisialisasi Objek Geometris & Material PBR
```widya
// Buat objek geometris 3D
biar kubus = game3d_mesh_kubus(2.0)
biar bola = game3d_mesh_bola(1.0)
biar kapsul_karakter = game3d_mesh_kapsul(0.5, 2.0)

// Buat material PBR dengan albedo emas, metallic 1.0, dan roughness 0.2
biar material_emas = game3d_material_pbr("#FFD700", 1.0, 0.2)
```

### B. Perhitungan Transformasi Vektor & Kuaternion Slerp
```widya
// Koordinat 3D
biar pos = game3d_vektor3(10.0, 5.0, -15.0)

// Interpolasi rotasi animasi skeletal mulus bebas Gimbal Lock
biar rotasi_tengah = game3d_kuaternion_slerp(0, 0, 0.5)
```

### C. Raycasting & Deteksi Tabrakan 3D (Tembakan Senjata / Hitscan)
```widya
// Tembakkan ray dari kamera ke depan
biar hasil_tembakan = game3d_raycast_bola(0, 0, 0, 1)

jika hasil_tembakan["hit"] {
    cetak("Musuh Terkena Tembakan pada Jarak:", hasil_tembakan["jarak"])
}
```

### D. Generator Primitif Wavefront OBJ
```widya
// Muat model 3D OBJ eksternal
biar mesh_impor = game3d_mesh_impor_obj("v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3")
```

---

## 🔬 3. Spesifikasi Teknis Engine

| Modul | Implementasi | Karakteristik Teknis |
| :--- | :--- | :--- |
| **PBR Shading** | `src/game3d/pbr.rs` | Model Cook-Torrance (GGX Normal Distribution, Smith Schlick-GGX, Fresnel-Schlick) + ACES Filmic Tone Mapping. |
| **Physics 3D** | `src/game3d/physics3d.rs` | Benda tegar (Rigid Body), gravitasi, pantulan elastis, gesekan, raycasting AABB slab method & sphere-sphere impulse resolution. |
| **Spatial Culling** | `src/game3d/camera.rs` | Pohon ruang 3D Octree dengan subdivisi otomatis + kamera perspektif & frustum culling. |
| **Asset Pipeline** | `src/game3d/gltf_loader.rs` | Parser format glTF 2.0 JSON/GLB & evaluator animasi skeletal berbasis Slerp Quaternion. |

---

## 🧪 4. Status Pengujian
Seluruh modul telah diverifikasi dengan **11 unit test terpadu (`tests/game3d_tests.rs`)** dengan tingkat kelulusan **100% (Passed)**.
