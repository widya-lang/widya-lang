use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn register_spatial(env: &mut Environment) {
    // WidyaGeo (Geospatial & Geodesic GIS Engine)
    register_fn(env, "TitikGeo", None, builtin_geo_titik);
    register_fn(env, "PoligonGeo", Some(1), builtin_geo_poligon);
    register_fn(env, "hitung_jarak_haversine", Some(2), builtin_geo_jarak_haversine);
    register_fn(env, "apakah_dalam_poligon", Some(2), builtin_geo_dalam_poligon);
    register_fn(env, "ekspor_geojson", Some(1), builtin_geo_ekspor_geojson);


    // WidyaSemanticSearch (Vector Embeddings & Cosine Similarity)
    register_fn(env, "buat_indeks_vektor", None, builtin_vektor_buat_indeks);
    register_fn(env, "vektor_tambah", Some(4), builtin_vektor_tambah);
    register_fn(env, "vektor_kueri_kemiripan", Some(3), builtin_vektor_kueri);
    register_fn(env, "vektor_cosine_similarity", Some(2), builtin_vektor_cosine);

    // WidyaSimHash (Locality-Sensitive Hashing & Near-Duplicate Detector)
    register_fn(env, "simhash_buat", Some(1), builtin_simhash_buat);
    register_fn(env, "simhash_jarak_hamming", Some(2), builtin_simhash_jarak_hamming);
    register_fn(env, "simhash_apakah_mirip", None, builtin_simhash_apakah_mirip);

    // WidyaVectorDB (Vector Embeddings & k-NN Semantic Search)
    register_fn(env, "BasisDataVektor", Some(1), builtin_vectordb_buat);
    register_fn(env, "vektor_sisip", Some(4), builtin_vectordb_sisip);
    register_fn(env, "vektor_kueri_knn", None, builtin_vectordb_kueri_knn);
    register_fn(env, "vektor_hitung", Some(1), builtin_vectordb_hitung);

    // WidyaLSH (MinHash Locality-Sensitive Hashing & Deduplication)
    register_fn(env, "MinHashLSH", None, builtin_lsh_buat);
    register_fn(env, "lsh_sisip", Some(3), builtin_lsh_sisip);
    register_fn(env, "lsh_kueri", Some(2), builtin_lsh_kueri);
    register_fn(env, "lsh_estimasi_jaccard", Some(3), builtin_lsh_estimasi_jaccard);

    // WidyaSpatialRTree (2D Spatial Bounding-Box Index)
    register_fn(env, "PohonR", None, builtin_rtree_buat);
    register_fn(env, "rtree_sisip", Some(7), builtin_rtree_sisip);
    register_fn(env, "rtree_kueri_kotak", Some(5), builtin_rtree_kueri_kotak);
    register_fn(env, "rtree_hitung", Some(1), builtin_rtree_hitung);

}

// =========================================================================
// WidyaGeo: Geospatial GIS & Geodesic Navigation Engine
// =========================================================================

fn builtin_geo_titik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat = match args.first() {
        Some(Value::Number(n)) => *n,
        _ => return Err(Galat::runtime("Latitude harus berupa angka derajat (-90 s/d 90)", span)),
    };
    let lon = match args.get(1) {
        Some(Value::Number(n)) => *n,
        _ => return Err(Galat::runtime("Longitude harus berupa angka derajat (-180 s/d 180)", span)),
    };
    let alt = match args.get(2) {
        Some(Value::Number(n)) => *n,
        _ => 0.0,
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("TitikGeo".to_string()));
    map.insert("lintang".to_string(), Value::Number(lat));
    map.insert("bujur".to_string(), Value::Number(lon));
    map.insert("ketinggian".to_string(), Value::Number(alt));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geo_poligon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let vertices = match &args[0] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("PoligonGeo memerlukan daftar TitikGeo", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PoligonGeo".to_string()));
    map.insert("titik_sudut".to_string(), Value::Array(Rc::new(RefCell::new(vertices))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_geo_jarak_haversine(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (lat1, lon1) = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let lat = match map.get("lintang") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let lon = match map.get("bujur") { Some(Value::Number(n)) => *n, _ => 0.0 };
            (lat, lon)
        }
        _ => return Err(Galat::runtime("Argumen 1 harus berupa TitikGeo", span)),
    };

    let (lat2, lon2) = match &args[1] {
        Value::Map(m) => {
            let map = m.borrow();
            let lat = match map.get("lintang") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let lon = match map.get("bujur") { Some(Value::Number(n)) => *n, _ => 0.0 };
            (lat, lon)
        }
        _ => return Err(Galat::runtime("Argumen 2 harus berupa TitikGeo", span)),
    };

    let r = 6371000.0; // Jari-jari bumi dalam meter
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let jarak_m = r * c;

    Ok(Value::Number(jarak_m))
}

fn builtin_geo_dalam_poligon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (p_lat, p_lon) = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let lat = match map.get("lintang") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let lon = match map.get("bujur") { Some(Value::Number(n)) => *n, _ => 0.0 };
            (lat, lon)
        }
        _ => return Err(Galat::runtime("Argumen 1 harus berupa TitikGeo", span)),
    };

    let vertices = match &args[1] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("titik_sudut") {
                arr.borrow().clone()
            } else {
                Vec::new()
            }
        }
        _ => return Err(Galat::runtime("Argumen 2 harus berupa PoligonGeo", span)),
    };

    if vertices.len() < 3 {
        return Ok(Value::Bool(false));
    }

    // Ray casting algorithm for Point-In-Polygon
    let mut inside = false;
    let n = vertices.len();
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = match &vertices[i] {
            Value::Map(m) => {
                let map = m.borrow();
                (match map.get("bujur") { Some(Value::Number(num)) => *num, _ => 0.0 },
                 match map.get("lintang") { Some(Value::Number(num)) => *num, _ => 0.0 })
            }
            _ => (0.0, 0.0),
        };

        let (xj, yj) = match &vertices[j] {
            Value::Map(m) => {
                let map = m.borrow();
                (match map.get("bujur") { Some(Value::Number(num)) => *num, _ => 0.0 },
                 match map.get("lintang") { Some(Value::Number(num)) => *num, _ => 0.0 })
            }
            _ => (0.0, 0.0),
        };

        let intersect = ((yi > p_lat) != (yj > p_lat))
            && (p_lon < (xj - xi) * (p_lat - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }
        j = i;
    }

    Ok(Value::Bool(inside))
}

fn builtin_geo_ekspor_geojson(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let tipe = map.get("_tipe").map(|v| v.to_string_repr()).unwrap_or_default();
            if tipe == "TitikGeo" {
                let lat = match map.get("lintang") { Some(Value::Number(n)) => *n, _ => 0.0 };
                let lon = match map.get("bujur") { Some(Value::Number(n)) => *n, _ => 0.0 };
                let geojson = format!(r#"{{"type":"Feature","geometry":{{"type":"Point","coordinates":[{:.6},{:.6}]}},"properties":{{}}}}"#, lon, lat);
                Ok(Value::String(geojson))
            } else if tipe == "PoligonGeo" {
                let vertices = match map.get("titik_sudut") {
                    Some(Value::Array(a)) => a.borrow().clone(),
                    _ => Vec::new(),
                };
                let mut coords = Vec::new();
                for v in vertices {
                    if let Value::Map(mv) = v {
                        let vm = mv.borrow();
                        let lat = match vm.get("lintang") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let lon = match vm.get("bujur") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        coords.push(format!("[{:.6},{:.6}]", lon, lat));
                    }
                }
                let geojson = format!(r#"{{"type":"Feature","geometry":{{"type":"Polygon","coordinates":[[{}]]}},"properties":{{}}}}"#, coords.join(","));
                Ok(Value::String(geojson))
            } else {
                Err(Galat::runtime("Objek bukan berupa TitikGeo atau PoligonGeo", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek geometri WidyaGeo", span)),
    }
}

// ==============================================================================
// WidyaSemanticSearch: Vector Embeddings & Cosine Similarity Engine
// ==============================================================================
fn cosine_similarity_calc(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

fn builtin_vektor_cosine(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let extract_vec = |v: &Value| -> Result<Vec<f64>, Galat> {
        match v {
            Value::Array(a) => {
                let items = a.borrow();
                items.iter().map(|item| match item {
                    Value::Number(n) => Ok(*n),
                    _ => Err(Galat::runtime("Vektor harus berisi angka", span)),
                }).collect()
            }
            _ => Err(Galat::runtime("Diharapkan array vektor angka", span)),
        }
    };

    let v1 = extract_vec(&args[0])?;
    let v2 = extract_vec(&args[1])?;

    Ok(Value::Number(cosine_similarity_calc(&v1, &v2)))
}

fn builtin_vektor_buat_indeks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let dim = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(0);

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("IndeksVektor".to_string()));
    obj.insert("dimensi".to_string(), Value::Number(dim as f64));
    obj.insert("dokumen".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_vektor_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let index_val = &args[0];
    let doc_id = args[1].to_string_repr();
    let embedding = match &args[2] {
        Value::Array(a) => a.clone(),
        _ => return Err(Galat::runtime("Embedding harus berupa array angka", span)),
    };
    let metadata = args[3].clone();

    match index_val {
        Value::Map(m) => {
            let map = m.borrow();
            let docs_rc = match map.get("dokumen") {
                Some(Value::Array(a)) => a.clone(),
                _ => return Err(Galat::runtime("IndeksVektor tidak valid", span)),
            };

            let mut doc = HashMap::new();
            doc.insert("id".to_string(), Value::String(doc_id));
            doc.insert("vektor".to_string(), Value::Array(embedding));
            doc.insert("metadata".to_string(), metadata);

            docs_rc.borrow_mut().push(Value::Map(Rc::new(RefCell::new(doc))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi vektor_tambah memerlukan objek IndeksVektor", span)),
    }
}

fn builtin_vektor_kueri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let index_val = &args[0];
    let query_embedding: Vec<f64> = match &args[1] {
        Value::Array(a) => {
            a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect()
        }
        _ => return Err(Galat::runtime("Kueri vektor harus berupa array angka", span)),
    };
    let top_k = match args.get(2) {
        Some(Value::Number(n)) => n.max(1.0) as usize,
        _ => 5,
    };

    match index_val {
        Value::Map(m) => {
            let map = m.borrow();
            let docs_rc = match map.get("dokumen") {
                Some(Value::Array(a)) => a.clone(),
                _ => return Err(Galat::runtime("IndeksVektor tidak valid", span)),
            };

            let mut scored: Vec<(f64, Value)> = Vec::new();
            for doc_val in docs_rc.borrow().iter() {
                if let Value::Map(dm) = doc_val {
                    let dmap = dm.borrow();
                    let doc_vec: Vec<f64> = match dmap.get("vektor") {
                        Some(Value::Array(va)) => va.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
                        _ => vec![],
                    };
                    let sim = cosine_similarity_calc(&query_embedding, &doc_vec);

                    let mut res_entry = HashMap::new();
                    res_entry.insert("id".to_string(), dmap.get("id").cloned().unwrap_or(Value::Nil));
                    res_entry.insert("kemiripan".to_string(), Value::Number(sim));
                    res_entry.insert("metadata".to_string(), dmap.get("metadata").cloned().unwrap_or(Value::Nil));

                    scored.push((sim, Value::Map(Rc::new(RefCell::new(res_entry)))));
                }
            }

            scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let top_results: Vec<Value> = scored.into_iter().take(top_k).map(|(_, v)| v).collect();

            Ok(Value::Array(Rc::new(RefCell::new(top_results))))
        }
        _ => Err(Galat::runtime("Fungsi vektor_kueri_kemiripan memerlukan objek IndeksVektor", span)),
    }
}

// ==============================================================================
// WidyaSimHash: Locality-Sensitive Hashing & Near-Duplicate Detector
// ==============================================================================
fn compute_simhash64(text: &str) -> u64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() { return 0; }

    let mut v = [0i32; 64];
    use sha2::{Digest, Sha256};

    for word in words {
        let mut hasher = Sha256::new();
        hasher.update(word.as_bytes());
        let hash_bytes = hasher.finalize();
        let h = u64::from_le_bytes(hash_bytes[0..8].try_into().unwrap());

        for i in 0..64 {
            if (h & (1u64 << i)) != 0 {
                v[i] += 1;
            } else {
                v[i] -= 1;
            }
        }
    }

    let mut fingerprint = 0u64;
    for i in 0..64 {
        if v[i] > 0 {
            fingerprint |= 1u64 << i;
        }
    }
    fingerprint
}

fn builtin_simhash_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args[0].to_string_repr();
    let hash_val = compute_simhash64(&text);
    Ok(Value::String(format!("{:016x}", hash_val)))
}

fn builtin_simhash_jarak_hamming(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let h1_str = args[0].to_string_repr();
    let h2_str = args[1].to_string_repr();
    let h1 = u64::from_str_radix(h1_str.trim_start_matches("0x"), 16).map_err(|_| Galat::runtime("SimHash 1 tidak valid", span))?;
    let h2 = u64::from_str_radix(h2_str.trim_start_matches("0x"), 16).map_err(|_| Galat::runtime("SimHash 2 tidak valid", span))?;

    let dist = (h1 ^ h2).count_ones();
    Ok(Value::Number(dist as f64))
}

fn builtin_simhash_apakah_mirip(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let t1 = args[0].to_string_repr();
    let t2 = args[1].to_string_repr();
    let threshold = match args.get(2) { Some(Value::Number(n)) => *n as u32, _ => 3 };

    let h1 = compute_simhash64(&t1);
    let h2 = compute_simhash64(&t2);
    let dist = (h1 ^ h2).count_ones();
    Ok(Value::Bool(dist <= threshold))
}

// =========================================================================
// WIDYAVECTORDB (IN-MEMORY VECTOR EMBEDDINGS & k-NN SEMANTIC SEARCH)
// =========================================================================

fn builtin_vectordb_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dim = match &args[0] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(Galat::runtime("Dimensi vektor harus > 0", span));
            }
            *n as usize
        }
        _ => return Err(Galat::runtime("Fungsi 'BasisDataVektor' memerlukan argumen angka dimensi", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BasisDataVektor".to_string()));
    map.insert("dimensi".to_string(), Value::Number(dim as f64));
    map.insert("vektor".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_vectordb_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let db_val = &args[0];
    let id_str = args[1].to_string_repr();
    let vec_val = &args[2];
    let metadata_val = args.get(3).cloned().unwrap_or(Value::Nil);

    match db_val {
        Value::Map(m) => {
            let map = m.borrow();
            let dim = match map.get("dimensi") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            let vec_floats: Vec<f64> = match vec_val {
                Value::Array(arr) => {
                    let items = arr.borrow();
                    if items.len() != dim {
                        return Err(Galat::runtime(format!("Ukuran vektor ({}) tidak sesuai dengan dimensi basis data ({})", items.len(), dim), span));
                    }
                    items.iter().map(|v| match v {
                        Value::Number(n) => *n,
                        _ => 0.0,
                    }).collect()
                }
                _ => return Err(Galat::runtime("Argumen vektor harus berupa array angka float", span)),
            };

            let mut entri = HashMap::new();
            entri.insert("id".to_string(), Value::String(id_str));
            let val_floats: Vec<Value> = vec_floats.into_iter().map(Value::Number).collect();
            entri.insert("vektor".to_string(), Value::Array(Rc::new(RefCell::new(val_floats))));
            entri.insert("metadata".to_string(), metadata_val);

            if let Some(Value::Array(items_arr)) = map.get("vektor") {
                items_arr.borrow_mut().push(Value::Map(Rc::new(RefCell::new(entri))));
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'vektor_sisip' memerlukan objek BasisDataVektor", span)),
    }
}

fn cosine_similarity(v1: &[f64], v2: &[f64]) -> f64 {
    let mut dot = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    for (a, b) in v1.iter().zip(v2.iter()) {
        dot += a * b;
        norm1 += a * a;
        norm2 += b * b;
    }
    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot / (norm1.sqrt() * norm2.sqrt())
    }
}

fn builtin_vectordb_kueri_knn(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'vektor_kueri_knn' menerima minimal (BasisDataVektor, vektor_kueri, [k])", span));
    }
    let db_val = &args[0];
    let query_vec_val = &args[1];
    let k = args.get(2).and_then(|v| match v {
        Value::Number(n) => Some(*n as usize),
        _ => None,
    }).unwrap_or(5);

    match db_val {
        Value::Map(m) => {
            let map = m.borrow();
            let dim = match map.get("dimensi") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            let q_floats: Vec<f64> = match query_vec_val {
                Value::Array(arr) => {
                    let items = arr.borrow();
                    if items.len() != dim {
                        return Err(Galat::runtime(format!("Ukuran vektor kueri ({}) tidak cocok dengan dimensi database ({})", items.len(), dim), span));
                    }
                    items.iter().map(|v| match v {
                        Value::Number(n) => *n,
                        _ => 0.0,
                    }).collect()
                }
                _ => return Err(Galat::runtime("Vektor kueri harus berupa array angka", span)),
            };

            let mut scores: Vec<(f64, Value)> = Vec::new();
            if let Some(Value::Array(items_arr)) = map.get("vektor") {
                for item_val in items_arr.borrow().iter() {
                    if let Value::Map(im) = item_val {
                        let item_map = im.borrow();
                        if let Some(Value::Array(v_arr)) = item_map.get("vektor") {
                            let floats: Vec<f64> = v_arr.borrow().iter().map(|v| match v {
                                Value::Number(n) => *n,
                                _ => 0.0,
                            }).collect();
                            let sim = cosine_similarity(&q_floats, &floats);
                            let mut res_item = item_map.clone();
                            res_item.insert("kemiripan".to_string(), Value::Number(sim));
                            scores.push((sim, Value::Map(Rc::new(RefCell::new(res_item)))));
                        }
                    }
                }
            }

            scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            let top_k: Vec<Value> = scores.into_iter().take(k).map(|(_, v)| v).collect();
            Ok(Value::Array(Rc::new(RefCell::new(top_k))))
        }
        _ => Err(Galat::runtime("Fungsi 'vektor_kueri_knn' memerlukan objek BasisDataVektor", span)),
    }
}

fn builtin_vectordb_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("vektor") {
                Ok(Value::Number(arr.borrow().len() as f64))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'vektor_hitung' memerlukan objek BasisDataVektor", span)),
    }
}

// =========================================================================
// WIDYALSH (MINHASH LOCALITY-SENSITIVE HASHING & DEDUPLICATION)
// =========================================================================

fn minhash_hash(token: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in token.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn compute_minhash_signature(tokens: &[String], num_perm: usize) -> Vec<u64> {
    if tokens.is_empty() {
        return vec![0; num_perm];
    }
    let mut sig = vec![u64::MAX; num_perm];
    let prime = 4294967311u64; // Large 32-bit prime

    for token in tokens {
        let raw_h = minhash_hash(token);
        for i in 0..num_perm {
            let a = ((i as u64) * 2654435761u64).wrapping_add(1);
            let b = ((i as u64) * 805447001u64).wrapping_add(13);
            let perm_h = (a.wrapping_mul(raw_h).wrapping_add(b)) % prime;
            if perm_h < sig[i] {
                sig[i] = perm_h;
            }
        }
    }
    sig
}

fn builtin_lsh_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let num_perm = args.first().and_then(|v| match v {
        Value::Number(n) => Some((*n as usize).max(4)),
        _ => None,
    }).unwrap_or(16);

    let threshold = args.get(1).and_then(|v| match v {
        Value::Number(n) => Some(*n),
        _ => None,
    }).unwrap_or(0.5);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MinHashLSH".to_string()));
    map.insert("num_perm".to_string(), Value::Number(num_perm as f64));
    map.insert("threshold".to_string(), Value::Number(threshold));
    map.insert("dokumen".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("signatures".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn extract_token_list(val: &Value) -> Vec<String> {
    match val {
        Value::Array(a) => a.borrow().iter().map(|v| v.to_string_repr()).collect(),
        Value::String(s) => s.split_whitespace().map(|w| w.to_string()).collect(),
        other => vec![other.to_string_repr()],
    }
}

fn builtin_lsh_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lsh_val = &args[0];
    let doc_id = args[1].to_string_repr();
    let tokens = extract_token_list(&args[2]);

    match lsh_val {
        Value::Map(m) => {
            let map = m.borrow();
            let num_perm = match map.get("num_perm") {
                Some(Value::Number(n)) => *n as usize,
                _ => 16,
            };

            let sig = compute_minhash_signature(&tokens, num_perm);
            if let Some(Value::Map(sig_map)) = map.get("signatures") {
                let sig_vals: Vec<Value> = sig.into_iter().map(|n| Value::Number(n as f64)).collect();
                sig_map.borrow_mut().insert(doc_id.clone(), Value::Array(Rc::new(RefCell::new(sig_vals))));
            }

            if let Some(Value::Map(doc_map)) = map.get("dokumen") {
                let tok_vals: Vec<Value> = tokens.into_iter().map(Value::String).collect();
                doc_map.borrow_mut().insert(doc_id, Value::Array(Rc::new(RefCell::new(tok_vals))));
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'lsh_sisip' memerlukan objek MinHashLSH", span)),
    }
}

fn estimate_jaccard_from_sigs(sig1: &[u64], sig2: &[u64]) -> f64 {
    if sig1.is_empty() || sig2.is_empty() || sig1.len() != sig2.len() {
        return 0.0;
    }
    let mut matches = 0;
    for (a, b) in sig1.iter().zip(sig2.iter()) {
        if a == b {
            matches += 1;
        }
    }
    matches as f64 / sig1.len() as f64
}

fn builtin_lsh_kueri(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lsh_val = &args[0];
    let tokens = extract_token_list(&args[1]);

    match lsh_val {
        Value::Map(m) => {
            let map = m.borrow();
            let num_perm = match map.get("num_perm") {
                Some(Value::Number(n)) => *n as usize,
                _ => 16,
            };
            let threshold = match map.get("threshold") {
                Some(Value::Number(n)) => *n,
                _ => 0.5,
            };

            let q_sig = compute_minhash_signature(&tokens, num_perm);
            let mut matches: Vec<Value> = Vec::new();

            if let Some(Value::Map(sig_map)) = map.get("signatures") {
                for (doc_id, sig_val) in sig_map.borrow().iter() {
                    if let Value::Array(sig_arr) = sig_val {
                        let sig: Vec<u64> = sig_arr.borrow().iter().map(|v| match v {
                            Value::Number(n) => *n as u64,
                            _ => 0,
                        }).collect();

                        let est = estimate_jaccard_from_sigs(&q_sig, &sig);
                        if est >= threshold {
                            let mut item = HashMap::new();
                            item.insert("id".to_string(), Value::String(doc_id.clone()));
                            item.insert("estimasi_jaccard".to_string(), Value::Number(est));
                            matches.push(Value::Map(Rc::new(RefCell::new(item))));
                        }
                    }
                }
            }

            matches.sort_by(|a, b| {
                let est_a = match a { Value::Map(m) => m.borrow().get("estimasi_jaccard").cloned(), _ => None };
                let est_b = match b { Value::Map(m) => m.borrow().get("estimasi_jaccard").cloned(), _ => None };
                match (est_a, est_b) {
                    (Some(Value::Number(x)), Some(Value::Number(y))) => y.partial_cmp(&x).unwrap_or(std::cmp::Ordering::Equal),
                    _ => std::cmp::Ordering::Equal,
                }
            });

            Ok(Value::Array(Rc::new(RefCell::new(matches))))
        }
        _ => Err(Galat::runtime("Fungsi 'lsh_kueri' memerlukan objek MinHashLSH", span)),
    }
}

fn builtin_lsh_estimasi_jaccard(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let t1 = extract_token_list(&args[1]);
    let t2 = extract_token_list(&args[2]);

    let s1: std::collections::HashSet<String> = t1.into_iter().collect();
    let s2: std::collections::HashSet<String> = t2.into_iter().collect();

    let intersection = s1.intersection(&s2).count();
    let union = s1.union(&s2).count();

    if union == 0 {
        Ok(Value::Number(1.0))
    } else {
        Ok(Value::Number(intersection as f64 / union as f64))
    }
}

// =========================================================================
// WIDYASPATIALRTREE (2D SPATIAL BOUNDING-BOX INDEX)
// =========================================================================

pub fn builtin_rtree_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let capacity = args.first().and_then(|v| match v {
        Value::Number(n) => Some((*n as usize).max(2)),
        _ => None,
    }).unwrap_or(8);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PohonR".to_string()));
    map.insert("kapasitas".to_string(), Value::Number(capacity as f64));
    map.insert("elemen".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_rtree_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let rtree_val = &args[0];
    let id_str = args[1].to_string_repr();
    let min_x = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("min_x harus angka", span)) };
    let min_y = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("min_y harus angka", span)) };
    let max_x = match &args[4] { Value::Number(n) => *n, _ => return Err(Galat::runtime("max_x harus angka", span)) };
    let max_y = match &args[5] { Value::Number(n) => *n, _ => return Err(Galat::runtime("max_y harus angka", span)) };
    let data_val = args.get(6).cloned().unwrap_or(Value::Nil);

    match rtree_val {
        Value::Map(m) => {
            let map = m.borrow();
            let mut box_obj = HashMap::new();
            box_obj.insert("id".to_string(), Value::String(id_str));
            box_obj.insert("min_x".to_string(), Value::Number(min_x.min(max_x)));
            box_obj.insert("min_y".to_string(), Value::Number(min_y.min(max_y)));
            box_obj.insert("max_x".to_string(), Value::Number(min_x.max(max_x)));
            box_obj.insert("max_y".to_string(), Value::Number(min_y.max(max_y)));
            box_obj.insert("data".to_string(), data_val);

            if let Some(Value::Array(items)) = map.get("elemen") {
                items.borrow_mut().push(Value::Map(Rc::new(RefCell::new(box_obj))));
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'rtree_sisip' memerlukan objek PohonR", span)),
    }
}

fn boxes_intersect(b_min_x: f64, b_min_y: f64, b_max_x: f64, b_max_y: f64, q_min_x: f64, q_min_y: f64, q_max_x: f64, q_max_y: f64) -> bool {
    b_min_x <= q_max_x && b_max_x >= q_min_x && b_min_y <= q_max_y && b_max_y >= q_min_y
}

pub fn builtin_rtree_kueri_kotak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let rtree_val = &args[0];
    let q_min_x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("q_min_x harus angka", span)) };
    let q_min_y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("q_min_y harus angka", span)) };
    let q_max_x = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("q_max_x harus angka", span)) };
    let q_max_y = match &args[4] { Value::Number(n) => *n, _ => return Err(Galat::runtime("q_max_y harus angka", span)) };

    match rtree_val {
        Value::Map(m) => {
            let map = m.borrow();
            let mut results = Vec::new();

            if let Some(Value::Array(items)) = map.get("elemen") {
                for item in items.borrow().iter() {
                    if let Value::Map(im) = item {
                        let b = im.borrow();
                        let min_x = match b.get("min_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let min_y = match b.get("min_y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let max_x = match b.get("max_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let max_y = match b.get("max_y") { Some(Value::Number(n)) => *n, _ => 0.0 };

                        if boxes_intersect(min_x, min_y, max_x, max_y, q_min_x.min(q_max_x), q_min_y.min(q_max_y), q_min_x.max(q_max_x), q_min_y.max(q_max_y)) {
                            results.push(item.clone());
                        }
                    }
                }
            }

            Ok(Value::Array(Rc::new(RefCell::new(results))))
        }
        _ => Err(Galat::runtime("Fungsi 'rtree_kueri_kotak' memerlukan objek PohonR", span)),
    }
}

fn builtin_rtree_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("elemen") {
                Ok(Value::Number(arr.borrow().len() as f64))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'rtree_hitung' memerlukan objek PohonR", span)),
    }
}

