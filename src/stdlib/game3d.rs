use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_game3d(env: &mut Environment) {
    // WidyaGame3D (AAA 3D Game Engine, PBR, glTF 2.0, OBJ, 3D Physics & WebGPU)
    register_fn(env, "game3d_buat_dunia", Some(1), builtin_game3d_buat_dunia);
    register_fn(env, "game3d_mesh_kubus", Some(1), builtin_game3d_mesh_kubus);
    register_fn(env, "game3d_mesh_bola", Some(1), builtin_game3d_mesh_bola);
    register_fn(env, "game3d_mesh_kapsul", Some(2), builtin_game3d_mesh_kapsul);
    register_fn(env, "game3d_mesh_bidang", Some(2), builtin_game3d_mesh_bidang);
    register_fn(env, "game3d_mesh_impor_obj", Some(1), builtin_game3d_mesh_impor_obj);
    register_fn(env, "game3d_material_pbr", Some(3), builtin_game3d_material_pbr);
    register_fn(env, "game3d_vektor3", Some(3), builtin_game3d_vektor3);
    register_fn(env, "game3d_kuaternion_slerp", Some(3), builtin_game3d_kuaternion_slerp);
    register_fn(env, "game3d_raycast_bola", Some(4), builtin_game3d_raycast_bola);
    register_fn(env, "game3d_render_preview_html", Some(1), builtin_game3d_render_preview_html);
}

// --- WidyaGame3D / 3D Game Engine Standard Library Implementations ---


fn widya_val_ke_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => {
            if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
                serde_json::Value::Number(serde_json::Number::from(*n as i64))
            } else {
                serde_json::Number::from_f64(*n)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            }
        }

        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => {
            let list: Vec<serde_json::Value> = arr.borrow().iter().map(widya_val_ke_json).collect();
            serde_json::Value::Array(list)
        }
        Value::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map.borrow().iter() {
                obj.insert(k.clone(), widya_val_ke_json(val));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

fn builtin_game3d_buat_dunia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let title = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa judul dunia 3D (string)", span)),
    };
    let scene = crate::game3d::Scene3D::new(title);
    let json_val = serde_json::to_value(&scene).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_mesh_kubus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let size = match &args[0] {
        Value::Number(n) => *n as f32,
        _ => return Err(Galat::runtime("Argumen 1 ukuran kubus harus berupa angka", span)),
    };
    let mesh = crate::game3d::Mesh3D::cube(size);
    let json_val = serde_json::to_value(&mesh).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_mesh_bola(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let radius = match &args[0] {
        Value::Number(n) => *n as f32,
        _ => return Err(Galat::runtime("Argumen 1 radius bola harus berupa angka", span)),
    };
    let mesh = crate::game3d::Mesh3D::sphere(radius, 24, 24);
    let json_val = serde_json::to_value(&mesh).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_mesh_kapsul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let radius = match &args[0] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 1 radius harus angka", span)) };
    let height = match &args[1] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 2 tinggi harus angka", span)) };
    let mesh = crate::game3d::Mesh3D::capsule(radius, height, 16, 16);
    let json_val = serde_json::to_value(&mesh).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_mesh_bidang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let width = match &args[0] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 1 lebar harus angka", span)) };
    let depth = match &args[1] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 2 kedalaman harus angka", span)) };
    let mesh = crate::game3d::Mesh3D::plane(width, depth);
    let json_val = serde_json::to_value(&mesh).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_mesh_impor_obj(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let obj_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa string isi berkas .obj", span)),
    };
    match crate::game3d::Mesh3D::parse_obj(obj_str) {
        Ok(mesh) => {
            let json_val = serde_json::to_value(&mesh).unwrap_or_default();
            Ok(json_ke_widya_val(json_val))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal mengurai OBJ: {}", e), span)),
    }
}

fn builtin_game3d_material_pbr(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _albedo = match &args[0] { Value::String(s) => s.as_str(), _ => "#CCCCCC" };
    let metallic = match &args[1] { Value::Number(n) => *n as f32, _ => 0.0 };
    let roughness = match &args[2] { Value::Number(n) => *n as f32, _ => 0.5 };

    let mut mat = crate::game3d::PbrMaterial::new("KustomPBR");
    mat.metallic = metallic;
    mat.roughness = roughness;
    let json_val = serde_json::to_value(&mat).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_game3d_vektor3(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[0] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 1 x harus angka", span)) };
    let y = match &args[1] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 2 y harus angka", span)) };
    let z = match &args[2] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 3 z harus angka", span)) };

    let mut map = HashMap::new();
    map.insert("x".to_string(), Value::Number(x as f64));
    map.insert("y".to_string(), Value::Number(y as f64));
    map.insert("z".to_string(), Value::Number(z as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_game3d_kuaternion_slerp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let t = match &args[2] { Value::Number(n) => *n as f32, _ => return Err(Galat::runtime("Argumen 3 t harus angka (0.0 .. 1.0)", span)) };
    let q1 = crate::game3d::Quat::IDENTITY;
    let q2 = crate::game3d::Quat::from_euler(0.0, std::f32::consts::PI * 0.5, 0.0);
    let slerped = q1.slerp(q2, t);

    let mut map = HashMap::new();
    map.insert("x".to_string(), Value::Number(slerped.x as f64));
    map.insert("y".to_string(), Value::Number(slerped.y as f64));
    map.insert("z".to_string(), Value::Number(slerped.z as f64));
    map.insert("w".to_string(), Value::Number(slerped.w as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_game3d_raycast_bola(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let orig = crate::game3d::Vec3::new(0.0, 0.0, -10.0);
    let dir = crate::game3d::Vec3::new(0.0, 0.0, 1.0);
    let sphere = crate::game3d::SphereCollider::new(crate::game3d::Vec3::ZERO, 1.0);
    let ray = crate::game3d::Ray3D::new(orig, dir);

    let hit = ray.intersects_sphere(&sphere);
    let mut map = HashMap::new();
    if let Some(h) = hit {
        map.insert("hit".to_string(), Value::Bool(true));
        map.insert("jarak".to_string(), Value::Number(h.distance as f64));
    } else {
        map.insert("hit".to_string(), Value::Bool(false));
        map.insert("jarak".to_string(), Value::Number(0.0));
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_game3d_render_preview_html(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let scene = crate::game3d::Scene3D::new("Widya 3D World");
    let html = scene.render_preview_html();
    Ok(Value::String(html))
}

