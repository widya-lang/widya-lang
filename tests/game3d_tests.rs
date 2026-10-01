use widya::game3d::*;

#[test]
fn test_math3d_vector_and_matrix_operations() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let cross = v1.cross(v2);
    assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(v1.dot(v2), 0.0);

    let norm = Vec3::new(3.0, 4.0, 0.0).normalize();
    assert!((norm.length() - 1.0).abs() < 1e-5);

    let mat_trans = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    let p = mat_trans.transform_point(Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(p, Vec3::new(11.0, 22.0, 33.0));
}

#[test]
fn test_math3d_quaternion_and_slerp() {
    let q_start = Quat::IDENTITY;
    let q_end = Quat::from_euler(0.0, std::f32::consts::PI, 0.0); // 180 derajat yaw
    let q_mid = q_start.slerp(q_end, 0.5);

    assert!((q_mid.length() - 1.0).abs() < 1e-4);
    let rot_mat = q_mid.to_mat4();
    let rotated = rot_mat.transform_point(Vec3::FORWARD);
    // Di 90 derajat harus mengarah ke sumbu X/Z
    assert!(rotated.length() > 0.9);
}

#[test]
fn test_mesh3d_procedural_primitives() {
    let cube = Mesh3D::cube(2.0);
    assert_eq!(cube.vertices.len(), 24);
    assert_eq!(cube.indices.len(), 36);
    assert_eq!(cube.min_bounds, Vec3::new(-1.0, -1.0, -1.0));
    assert_eq!(cube.max_bounds, Vec3::new(1.0, 1.0, 1.0));

    let sphere = Mesh3D::sphere(1.0, 12, 12);
    assert!(!sphere.vertices.is_empty());
    assert!(!sphere.indices.is_empty());

    let plane = Mesh3D::plane(10.0, 10.0);
    assert_eq!(plane.vertices.len(), 4);
    assert_eq!(plane.indices.len(), 6);

    let capsule = Mesh3D::capsule(0.5, 2.0, 8, 8);
    assert!(!capsule.vertices.is_empty());
}

#[test]
fn test_mesh3d_obj_export_and_parse() {
    let original = Mesh3D::cube(1.0);
    let obj_text = original.export_obj();
    assert!(obj_text.contains("v -0.5"));
    assert!(obj_text.contains("f 1/1/1"));

    let parsed = Mesh3D::parse_obj(&obj_text).expect("Parsing OBJ harus berhasil");
    assert_eq!(parsed.indices.len(), original.indices.len());
}

#[test]
fn test_pbr_shading_and_tone_mapping() {
    let mat_gold = PbrMaterial::gold();
    assert_eq!(mat_gold.metallic, 1.0);

    let normal = Vec3::UP;
    let view_dir = Vec3::new(0.0, 1.0, 1.0).normalize();
    let light_dir = Vec3::new(0.0, 1.0, 0.0).normalize();
    let light_color = [1.0, 1.0, 1.0];

    let color = hitung_pbr_shading(&mat_gold, normal, view_dir, light_dir, light_color, 2.0);
    assert!(color[0] > 0.0 && color[1] > 0.0);

    let tone_mapped = tone_mapping_aces_filmic(color);
    assert!(tone_mapped[0] <= 1.0 && tone_mapped[0] >= 0.0);
}

#[test]
fn test_physics3d_raycasting() {
    let ray = Ray3D::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0));
    let sphere = SphereCollider::new(Vec3::new(0.0, 0.0, 0.0), 2.0);

    let hit = ray.intersects_sphere(&sphere);
    assert!(hit.is_some());
    let hit_info = hit.unwrap();
    assert!((hit_info.distance - 8.0).abs() < 1e-4);
    assert_eq!(hit_info.point, Vec3::new(0.0, 0.0, -2.0));

    let aabb = AABB::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
    let hit_aabb = ray.intersects_aabb(&aabb);
    assert!(hit_aabb.is_some());
    assert!((hit_aabb.unwrap().distance - 9.0).abs() < 1e-4);
}

#[test]
fn test_physics3d_rigidbody_simulation_step() {
    let mut bodies = vec![
        RigidBody3D::new_dynamic(Vec3::new(0.0, 10.0, 0.0), 1.0, 0.5),
        RigidBody3D::new_static(Vec3::new(0.0, 0.0, 0.0), 1.0),
    ];

    let gravity = Vec3::new(0.0, -9.81, 0.0);
    for _ in 0..60 {
        langkah_simulasi_fisika_3d(&mut bodies, 1.0 / 60.0, gravity);
    }

    // Objek harus jatuh mendekati tanah dan tidak tembus di bawah Y=0.5
    assert!(bodies[0].position.y < 10.0);
    assert!(bodies[0].position.y >= 0.5);
}

#[test]
fn test_camera3d_and_frustum_culling() {
    let camera = Camera3D::new_perspective(
        Vec3::new(0.0, 5.0, -10.0),
        Vec3::ZERO,
        60.0,
        16.0 / 9.0,
    );

    let vp = camera.view_projection_matrix();
    assert_ne!(vp, Mat4::IDENTITY);

    let inside_sphere = SphereCollider::new(Vec3::ZERO, 1.0);
    assert!(camera.is_sphere_in_frustum(&inside_sphere));

    let outside_sphere = SphereCollider::new(Vec3::new(0.0, 0.0, -50.0), 1.0); // Di belakang kamera
    assert!(!camera.is_sphere_in_frustum(&outside_sphere));
}

#[test]
fn test_spatial_octree_insertion() {
    let mut octree = OctreeNode::new(AABB::new(Vec3::new(-100.0, -100.0, -100.0), Vec3::new(100.0, 100.0, 100.0)));
    for i in 0..20 {
        let pos = Vec3::new(i as f32 * 2.0, i as f32 * 1.5, i as f32 * 0.5);
        octree.insert(pos, format!("Objek_{}", i));
    }
    assert!(octree.children.is_some(), "Octree harus melakukan subdivisi otomatis");
}

#[test]
fn test_gltf_json_parser_and_animation() {
    let gltf_json = r#"{
        "nodes": [
            { "name": "Hero", "translation": [0, 1, 0], "mesh": 0 }
        ],
        "meshes": [
            { "name": "HeroMesh" }
        ]
    }"#;

    let scene_data = parse_gltf_json(gltf_json).expect("Parsing glTF JSON harus berhasil");
    assert_eq!(scene_data.nodes.len(), 1);
    assert_eq!(scene_data.nodes[0].name, "Hero");
    assert_eq!(scene_data.meshes.len(), 1);

    let channel = GltfAnimationChannel {
        target_node: 0,
        property: "rotation".to_string(),
        timestamps: vec![0.0, 1.0],
        keyframe_data: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.7071, 0.0, 0.7071],
    };

    let (_, rot, _) = evaluasi_animasi_skeletal(&channel, 0.5);
    assert!(rot.length() > 0.9);
}

#[test]
fn test_widya_script_game3d_execution() {
    let script = r##"
        biar dunia = game3d_buat_dunia("Petualangan Nusantara 3D")
        biar kubus = game3d_mesh_kubus(2.0)
        biar bola = game3d_mesh_bola(1.0)
        biar mat = game3d_material_pbr("#FFD700", 0.9, 0.2)
        biar v = game3d_vektor3(10.0, 20.0, 30.0)
        biar ray = game3d_raycast_bola(0, 0, 0, 1)

        pastikan_sama(v["x"], 10.0)
        pastikan_sama(v["y"], 20.0)
        pastikan_sama(v["z"], 30.0)
        pastikan_benar(ray["hit"])
    "##;

    let res = widya::jalankan(script);
    assert!(res.is_ok(), "Eksekusi skrip Game 3D gagal: {:?}", res.err());
}
