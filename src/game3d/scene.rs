use crate::game3d::camera::Camera3D;
use crate::game3d::math3d::{Mat4, Quat, Vec3};
use crate::game3d::mesh::Mesh3D;
use crate::game3d::pbr::{DirectionalLight, PbrMaterial, PointLight};
use crate::game3d::physics3d::{langkah_simulasi_fisika_3d, RigidBody3D};
use serde::{Deserialize, Serialize};

/// Node Hirarki dalam Pohon Scene Graph 3D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNode3D {
    pub id: u32,
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mesh: Option<Mesh3D>,
    pub material: Option<PbrMaterial>,
    pub rigid_body: Option<RigidBody3D>,
    pub children: Vec<SceneNode3D>,
}

impl SceneNode3D {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            mesh: None,
            material: None,
            rigid_body: None,
            children: Vec::new(),
        }
    }

    pub fn with_mesh(mut self, mesh: Mesh3D, material: PbrMaterial) -> Self {
        self.mesh = Some(mesh);
        self.material = Some(material);
        self
    }

    pub fn with_rigidbody(mut self, rb: RigidBody3D) -> Self {
        self.rigid_body = Some(rb);
        self
    }

    pub fn local_transform_matrix(&self) -> Mat4 {
        let t = Mat4::from_translation(self.position);
        let r = self.rotation.to_mat4();
        let s = Mat4::from_scale(self.scale);
        s.multiply(&r).multiply(&t)
    }
}

/// Dunia Scene 3D Lengkap dengan Fisika, Kamera, dan Pencahayaan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene3D {
    pub title: String,
    pub root_nodes: Vec<SceneNode3D>,
    pub camera: Camera3D,
    pub sun_light: DirectionalLight,
    pub point_lights: Vec<PointLight>,
    pub gravity: Vec3,
}

impl Scene3D {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            root_nodes: Vec::new(),
            camera: Camera3D::default(),
            sun_light: DirectionalLight::default(),
            point_lights: Vec::new(),
            gravity: Vec3::new(0.0, -9.81, 0.0),
        }
    }

    pub fn add_node(&mut self, node: SceneNode3D) {
        self.root_nodes.push(node);
    }

    pub fn add_point_light(&mut self, light: PointLight) {
        self.point_lights.push(light);
    }

    /// Melakukan 1 langkah simulasi fisika pada seluruh objek ber-rigidbody
    pub fn update_physics(&mut self, dt: f32) {
        let mut bodies: Vec<RigidBody3D> = Vec::new();
        for node in &self.root_nodes {
            if let Some(rb) = &node.rigid_body {
                bodies.push(rb.clone());
            }
        }

        langkah_simulasi_fisika_3d(&mut bodies, dt, self.gravity);

        let mut body_idx = 0;
        for node in &mut self.root_nodes {
            if node.rigid_body.is_some() {
                if let Some(updated_rb) = bodies.get(body_idx) {
                    node.position = updated_rb.position;
                    node.rigid_body = Some(updated_rb.clone());
                }
                body_idx += 1;
            }
        }
    }

    /// Menghasilkan Halaman HTML5 WebGL2 / Three.js Preview Interaktif Standalone
    pub fn render_preview_html(&self) -> String {
        let scene_json = serde_json::to_string(self).unwrap_or_default();
        format!(r##"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Widya 3D Game Engine - {}</title>
    <style>
        body, html {{ margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; background: #0b0f19; font-family: sans-serif; }}
        #canvas3d {{ width: 100%; height: 100%; display: block; }}
        #hud {{
            position: absolute; top: 16px; left: 16px; color: #fff;
            background: rgba(15, 23, 42, 0.85); padding: 12px 20px; border-radius: 12px;
            backdrop-filter: blur(8px); border: 1px solid rgba(255,255,255,0.1);
            box-shadow: 0 8px 32px rgba(0,0,0,0.5); pointer-events: none;
        }}
        .badge {{ background: #059669; color: white; padding: 2px 8px; border-radius: 6px; font-size: 11px; font-weight: bold; }}
    </style>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
</head>
<body>
    <div id="hud">
        <h3 style="margin:0 0 4px 0;">🎮 {}</h3>
        <p style="margin:0; font-size: 13px; color: #94a3b8;">Widya 3D PBR Game Engine <span class="badge">60 FPS</span></p>
    </div>
    <canvas id="canvas3d"></canvas>

    <script>
        const sceneData = {};
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x0b0f19);
        scene.fog = new THREE.FogExp2(0x0b0f19, 0.015);

        const camera = new THREE.PerspectiveCamera(60, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.set(0, 8, 16);

        const renderer = new THREE.WebGLRenderer({{ canvas: document.getElementById('canvas3d'), antialias: true }});
        renderer.setSize(window.innerWidth, window.innerHeight);
        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.shadowMap.enabled = true;
        renderer.shadowMap.type = THREE.PCFSoftShadowMap;

        const controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.05;

        // Pencahayaan PBR
        const hemiLight = new THREE.HemisphereLight(0xffffff, 0x1e293b, 0.6);
        scene.add(hemiLight);

        const dirLight = new THREE.DirectionalLight(0xfff4e0, 1.5);
        dirLight.position.set(10, 20, 10);
        dirLight.castShadow = true;
        dirLight.shadow.mapSize.width = 2048;
        dirLight.shadow.mapSize.height = 2048;
        scene.add(dirLight);

        // Ground Grid
        const gridHelper = new THREE.GridHelper(50, 50, 0x3b82f6, 0x1e293b);
        scene.add(gridHelper);

        const planeGeo = new THREE.PlaneGeometry(100, 100);
        const planeMat = new THREE.MeshStandardMaterial({{ color: 0x0f172a, roughness: 0.8, metalness: 0.2 }});
        const plane = new THREE.Mesh(planeGeo, planeMat);
        plane.rotation.x = -Math.PI / 2;
        plane.receiveShadow = true;
        scene.add(plane);

        // Render nodes
        if (sceneData.root_nodes) {{
            sceneData.root_nodes.forEach(node => {{
                let geo = new THREE.BoxGeometry(1, 1, 1);
                let mat = new THREE.MeshStandardMaterial({{ color: 0x10b981, roughness: 0.3, metalness: 0.8 }});
                if (node.mesh && node.mesh.name === "Sphere") {{
                    geo = new THREE.SphereGeometry(0.5, 32, 32);
                }} else if (node.mesh && node.mesh.name === "Capsule") {{
                    geo = new THREE.CylinderGeometry(0.5, 0.5, 1.5, 24);
                }}
                const mesh = new THREE.Mesh(geo, mat);
                mesh.position.set(node.position.x, node.position.y, node.position.z);
                mesh.castShadow = true;
                mesh.receiveShadow = true;
                scene.add(mesh);
            }});
        }}

        function animate() {{
            requestAnimationFrame(animate);
            controls.update();
            renderer.render(scene, camera);
        }}
        animate();

        window.addEventListener('resize', () => {{
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        }});
    </script>
</body>
</html>"##, self.title, self.title, scene_json)
    }
}
