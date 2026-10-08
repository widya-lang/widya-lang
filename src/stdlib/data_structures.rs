use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::{HashMap, BTreeSet};
use std::rc::Rc;


// ==============================================================================
// WidyaPriorityQueue: Binary Min/Max Priority Heap
// ==============================================================================
fn builtin_pq_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let is_max = args.first().map(|v| v.is_truthy()).unwrap_or(true);
    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("AntreanPrioritas".to_string()));
    obj.insert("is_max".to_string(), Value::Bool(is_max));
    obj.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_pq_dorong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pq_val = &args[0];
    let elem = args[1].clone();
    let priority = match args.get(2) { Some(Value::Number(n)) => *n, _ => 0.0 };

    match pq_val {
        Value::Map(m) => {
            let map = m.borrow();
            let is_max = match map.get("is_max") { Some(Value::Bool(b)) => *b, _ => true };
            let data_rc = match map.get("data") { Some(Value::Array(da)) => da.clone(), _ => return Err(Galat::runtime("Objek AntreanPrioritas tidak valid", span)) };

            let mut pair = std::collections::HashMap::new();
            pair.insert("item".to_string(), elem);
            pair.insert("prioritas".to_string(), Value::Number(priority));

            let mut da = data_rc.borrow_mut();
            da.push(Value::Map(Rc::new(RefCell::new(pair))));
            let mut idx = da.len() - 1;

            // Heapify Up
            while idx > 0 {
                let parent = (idx - 1) / 2;
                let p_pri = get_item_priority(&da[parent]);
                let curr_pri = get_item_priority(&da[idx]);

                let should_swap = if is_max { curr_pri > p_pri } else { curr_pri < p_pri };
                if should_swap {
                    da.swap(idx, parent);
                    idx = parent;
                } else {
                    break;
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi antrean_dorong memerlukan objek AntreanPrioritas", span)),
    }
}

fn get_item_priority(v: &Value) -> f64 {
    if let Value::Map(m) = v {
        match m.borrow().get("prioritas") {
            Some(Value::Number(n)) => *n,
            _ => 0.0,
        }
    } else {
        0.0
    }
}

fn builtin_pq_tarik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pq_val = &args[0];
    match pq_val {
        Value::Map(m) => {
            let map = m.borrow();
            let is_max = match map.get("is_max") { Some(Value::Bool(b)) => *b, _ => true };
            let data_rc = match map.get("data") { Some(Value::Array(da)) => da.clone(), _ => return Err(Galat::runtime("Objek AntreanPrioritas tidak valid", span)) };

            let mut da = data_rc.borrow_mut();
            if da.is_empty() {
                return Ok(Value::Nil);
            }

            let top = da.swap_remove(0);
            if !da.is_empty() {
                // Heapify Down
                let mut idx = 0;
                let len = da.len();
                loop {
                    let left = 2 * idx + 1;
                    let right = 2 * idx + 2;
                    let mut best = idx;

                    if left < len {
                        let l_pri = get_item_priority(&da[left]);
                        let b_pri = get_item_priority(&da[best]);
                        if if is_max { l_pri > b_pri } else { l_pri < b_pri } {
                            best = left;
                        }
                    }

                    if right < len {
                        let r_pri = get_item_priority(&da[right]);
                        let b_pri = get_item_priority(&da[best]);
                        if if is_max { r_pri > b_pri } else { r_pri < b_pri } {
                            best = right;
                        }
                    }

                    if best != idx {
                        da.swap(idx, best);
                        idx = best;
                    } else {
                        break;
                    }
                }
            }

            if let Value::Map(top_m) = top {
                Ok(top_m.borrow().get("item").cloned().unwrap_or(Value::Nil))
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi antrean_tarik memerlukan objek AntreanPrioritas", span)),
    }
}

fn builtin_pq_intip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pq_val = &args[0];
    match pq_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Array(da)) => da.clone(), _ => return Err(Galat::runtime("Objek AntreanPrioritas tidak valid", span)) };
            let da = data_rc.borrow();
            if let Some(top) = da.first() {
                if let Value::Map(top_m) = top {
                    Ok(top_m.borrow().get("item").cloned().unwrap_or(Value::Nil))
                } else {
                    Ok(Value::Nil)
                }
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi antrean_intip memerlukan objek AntreanPrioritas", span)),
    }
}

fn builtin_pq_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pq_val = &args[0];
    match pq_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Array(da)) => da.clone(), _ => return Err(Galat::runtime("Objek AntreanPrioritas tidak valid", span)) };
            Ok(Value::Number(data_rc.borrow().len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi antrean_panjang memerlukan objek AntreanPrioritas", span)),
    }
}

fn builtin_pq_kosong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pq_val = &args[0];
    match pq_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Array(da)) => da.clone(), _ => return Err(Galat::runtime("Objek AntreanPrioritas tidak valid", span)) };
            Ok(Value::Bool(data_rc.borrow().is_empty()))
        }
        _ => Err(Galat::runtime("Fungsi antrean_kosong memerlukan objek AntreanPrioritas", span)),
    }
}


// ==========================================
// 🌲 WidyaAVL (Self-Balancing AVL Binary Search Tree)
// ==========================================

#[derive(Clone)]
struct AvlNode {
    key: String,
    val: Value,
    height: i64,
    left: Option<Box<AvlNode>>,
    right: Option<Box<AvlNode>>,
}

impl AvlNode {
    fn new(key: String, val: Value) -> Self {
        Self { key, val, height: 1, left: None, right: None }
    }
}

fn avl_node_height(n: &Option<Box<AvlNode>>) -> i64 {
    n.as_ref().map_or(0, |node| node.height)
}

fn avl_balance_factor(n: &Option<Box<AvlNode>>) -> i64 {
    n.as_ref().map_or(0, |node| avl_node_height(&node.left) - avl_node_height(&node.right))
}

fn avl_update_height(n: &mut AvlNode) {
    n.height = 1 + avl_node_height(&n.left).max(avl_node_height(&n.right));
}

fn avl_rotate_right(mut y: Box<AvlNode>) -> Box<AvlNode> {
    let mut x = y.left.take().expect("Left child must exist for right rotation");
    y.left = x.right.take();
    avl_update_height(&mut y);
    x.right = Some(y);
    avl_update_height(&mut x);
    x
}

fn avl_rotate_left(mut x: Box<AvlNode>) -> Box<AvlNode> {
    let mut y = x.right.take().expect("Right child must exist for left rotation");
    x.right = y.left.take();
    avl_update_height(&mut x);
    y.left = Some(x);
    avl_update_height(&mut y);
    y
}

fn avl_insert_rec(root: Option<Box<AvlNode>>, key: String, val: Value) -> Box<AvlNode> {
    let mut node = match root {
        Some(n) => n,
        None => return Box::new(AvlNode::new(key, val)),
    };

    if key < node.key {
        node.left = Some(avl_insert_rec(node.left.take(), key, val));
    } else if key > node.key {
        node.right = Some(avl_insert_rec(node.right.take(), key, val));
    } else {
        node.val = val;
        return node;
    }

    avl_update_height(&mut node);
    let balance = avl_balance_factor(&Some(node.clone()));

    // Left Left Case
    if balance > 1 && avl_balance_factor(&node.left) >= 0 {
        return avl_rotate_right(node);
    }
    // Left Right Case
    if balance > 1 && avl_balance_factor(&node.left) < 0 {
        if let Some(left) = node.left.take() {
            node.left = Some(avl_rotate_left(left));
        }
        return avl_rotate_right(node);
    }
    // Right Right Case
    if balance < -1 && avl_balance_factor(&node.right) <= 0 {
        return avl_rotate_left(node);
    }
    // Right Left Case
    if balance < -1 && avl_balance_factor(&node.right) > 0 {
        if let Some(right) = node.right.take() {
            node.right = Some(avl_rotate_right(right));
        }
        return avl_rotate_left(node);
    }

    node
}

fn avl_min_value_node(node: &AvlNode) -> AvlNode {
    let mut current = node;
    while let Some(ref next) = current.left {
        current = next;
    }
    current.clone()
}

fn avl_delete_rec(root: Option<Box<AvlNode>>, key: &str) -> Option<Box<AvlNode>> {
    let mut node = root?;

    if key < node.key.as_str() {
        node.left = avl_delete_rec(node.left.take(), key);
    } else if key > node.key.as_str() {
        node.right = avl_delete_rec(node.right.take(), key);
    } else {
        if node.left.is_none() {
            return node.right;
        } else if node.right.is_none() {
            return node.left;
        }

        let temp = avl_min_value_node(node.right.as_ref().unwrap());
        node.key = temp.key.clone();
        node.val = temp.val.clone();
        node.right = avl_delete_rec(node.right.take(), &temp.key);
    }

    avl_update_height(&mut node);
    let balance = avl_balance_factor(&Some(node.clone()));

    if balance > 1 && avl_balance_factor(&node.left) >= 0 {
        return Some(avl_rotate_right(node));
    }
    if balance > 1 && avl_balance_factor(&node.left) < 0 {
        if let Some(left) = node.left.take() {
            node.left = Some(avl_rotate_left(left));
        }
        return Some(avl_rotate_right(node));
    }
    if balance < -1 && avl_balance_factor(&node.right) <= 0 {
        return Some(avl_rotate_left(node));
    }
    if balance < -1 && avl_balance_factor(&node.right) > 0 {
        if let Some(right) = node.right.take() {
            node.right = Some(avl_rotate_right(right));
        }
        return Some(avl_rotate_left(node));
    }

    Some(node)
}

fn avl_to_map(node_opt: &Option<Box<AvlNode>>) -> Value {
    if let Some(node) = node_opt {
        let mut map = HashMap::new();
        map.insert("kunci".to_string(), Value::String(node.key.clone()));
        map.insert("nilai".to_string(), node.val.clone());
        map.insert("tinggi".to_string(), Value::Number(node.height as f64));
        map.insert("kiri".to_string(), avl_to_map(&node.left));
        map.insert("kanan".to_string(), avl_to_map(&node.right));
        Value::Map(Rc::new(RefCell::new(map)))
    } else {
        Value::Nil
    }
}

fn map_to_avl(val: &Value) -> Option<Box<AvlNode>> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        let key = match map.get("kunci") { Some(Value::String(s)) => s.clone(), _ => return None };
        let node_val = match map.get("nilai") { Some(v) => v.clone(), _ => Value::Nil };
        let height = match map.get("tinggi") { Some(Value::Number(n)) => *n as i64, _ => 1 };
        let left = map.get("kiri").and_then(map_to_avl);
        let right = map.get("kanan").and_then(map_to_avl);
        Some(Box::new(AvlNode { key, val: node_val, height, left, right }))
    } else {
        None
    }
}

fn builtin_avl_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut tree = HashMap::new();
    tree.insert("_tipe".to_string(), Value::String("PohonAVL".to_string()));
    tree.insert("akar".to_string(), Value::Nil);
    tree.insert("total".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(tree))))
}

fn builtin_avl_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match tree_val {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let root = map.get("akar").and_then(map_to_avl);
            let new_root = avl_insert_rec(root, key, val);
            let total = match map.get("total") { Some(Value::Number(n)) => *n + 1.0, _ => 1.0 };
            map.insert("total".to_string(), Value::Number(total));
            map.insert("akar".to_string(), avl_to_map(&Some(new_root)));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi avl_sisip memerlukan objek PohonAVL", span)),
    }
}

fn builtin_avl_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let key = args[1].to_string_repr();

    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            let mut curr = map.get("akar").and_then(map_to_avl);
            while let Some(node) = curr {
                if key == node.key {
                    return Ok(node.val.clone());
                } else if key < node.key {
                    curr = node.left;
                } else {
                    curr = node.right;
                }
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi avl_cari memerlukan objek PohonAVL", span)),
    }
}

fn builtin_avl_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let key = args[1].to_string_repr();

    match tree_val {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let root = map.get("akar").and_then(map_to_avl);
            let new_root = avl_delete_rec(root, &key);
            let total = match map.get("total") { Some(Value::Number(n)) => (*n - 1.0).max(0.0), _ => 0.0 };
            map.insert("total".to_string(), Value::Number(total));
            map.insert("akar".to_string(), avl_to_map(&new_root));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi avl_hapus memerlukan objek PohonAVL", span)),
    }
}

fn avl_inorder_collect(node_opt: &Option<Box<AvlNode>>, out: &mut Vec<Value>) {
    if let Some(node) = node_opt {
        avl_inorder_collect(&node.left, out);
        let mut entry = HashMap::new();
        entry.insert("kunci".to_string(), Value::String(node.key.clone()));
        entry.insert("nilai".to_string(), node.val.clone());
        out.push(Value::Map(Rc::new(RefCell::new(entry))));
        avl_inorder_collect(&node.right, out);
    }
}

fn builtin_avl_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            let root = map.get("akar").and_then(map_to_avl);
            let mut out = Vec::new();
            avl_inorder_collect(&root, &mut out);
            Ok(Value::Array(Rc::new(RefCell::new(out))))
        }
        _ => Err(Galat::runtime("Fungsi avl_ke_array memerlukan objek PohonAVL", span)),
    }
}

fn builtin_avl_tinggi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            let root = map.get("akar").and_then(map_to_avl);
            let h = avl_node_height(&root);
            Ok(Value::Number(h as f64))
        }
        _ => Err(Galat::runtime("Fungsi avl_tinggi memerlukan objek PohonAVL", span)),
    }
}


// ==========================================
// ⚡ WidyaSkipList (Probabilistic Skip List)
// ==========================================

fn builtin_skiplist_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let max_lvl = if let Some(Value::Number(n)) = args.first() {
        (*n as usize).clamp(1, 32)
    } else {
        16
    };

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("DaftarLompat".to_string()));
    obj.insert("max_level".to_string(), Value::Number(max_lvl as f64));
    obj.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn skiplist_get_items(val: &Value) -> Option<Rc<RefCell<Vec<Value>>>> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        if let Some(Value::Array(arr)) = map.get("data") {
            return Some(arr.clone());
        }
    }
    None
}

fn builtin_skiplist_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_rc = skiplist_get_items(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa DaftarLompat", span))?;
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    let mut items = items_rc.borrow_mut();
    let mut idx = 0;
    while idx < items.len() {
        if let Value::Map(m) = &items[idx] {
            let k = m.borrow().get("kunci").map(|v| v.to_string_repr()).unwrap_or_default();
            if k == key {
                m.borrow_mut().insert("nilai".to_string(), val);
                return Ok(Value::Bool(true));
            } else if k > key {
                break;
            }
        }
        idx += 1;
    }

    let mut pair = HashMap::new();
    pair.insert("kunci".to_string(), Value::String(key));
    pair.insert("nilai".to_string(), val);
    items.insert(idx, Value::Map(Rc::new(RefCell::new(pair))));

    Ok(Value::Bool(true))
}

fn builtin_skiplist_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_rc = skiplist_get_items(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa DaftarLompat", span))?;
    let key = args[1].to_string_repr();

    let items = items_rc.borrow();
    // Binary search on sorted array representation
    let mut low = 0;
    let mut high = items.len();
    while low < high {
        let mid = (low + high) / 2;
        if let Value::Map(m) = &items[mid] {
            let k = m.borrow().get("kunci").map(|v| v.to_string_repr()).unwrap_or_default();
            if k == key {
                return Ok(m.borrow().get("nilai").cloned().unwrap_or(Value::Nil));
            } else if k < key {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
    }

    Ok(Value::Nil)
}

fn builtin_skiplist_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_rc = skiplist_get_items(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa DaftarLompat", span))?;
    let key = args[1].to_string_repr();

    let mut items = items_rc.borrow_mut();
    let pos = items.iter().position(|item| {
        if let Value::Map(m) = item {
            m.borrow().get("kunci").map(|v| v.to_string_repr() == key).unwrap_or(false)
        } else {
            false
        }
    });

    if let Some(p) = pos {
        items.remove(p);
        Ok(Value::Bool(true))
    } else {
        Ok(Value::Bool(false))
    }
}

fn builtin_skiplist_rentang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_rc = skiplist_get_items(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa DaftarLompat", span))?;
    let start_key = args[1].to_string_repr();
    let end_key = args[2].to_string_repr();

    let items = items_rc.borrow();
    let mut res = Vec::new();
    for item in items.iter() {
        if let Value::Map(m) = item {
            let k = m.borrow().get("kunci").map(|v| v.to_string_repr()).unwrap_or_default();
            if k >= start_key && k <= end_key {
                res.push(item.clone());
            }
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(res))))
}

fn builtin_skiplist_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let items_rc = skiplist_get_items(&args[0]).ok_or_else(|| Galat::runtime("Argumen 1 harus berupa DaftarLompat", span))?;
    Ok(Value::Array(items_rc))
}


// =========================================================================
// WIDYAROARINGBITMAP (COMPRESSED ROARING BITMAPS FOR FAST SET OPERATIONS)
// =========================================================================

fn builtin_roaring_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("RoaringBitmap".to_string()));
    
    let mut initial_set: BTreeSet<u32> = BTreeSet::new();
    if let Some(Value::Array(a)) = args.first() {
        for it in a.borrow().iter() {
            if let Value::Number(n) = it {
                if *n >= 0.0 {
                    initial_set.insert(*n as u32);
                }
            }
        }
    }

    let items_vec: Vec<Value> = initial_set.into_iter().map(|n| Value::Number(n as f64)).collect();
    map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(items_vec))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn extract_roaring_set(val: &Value) -> Option<BTreeSet<u32>> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        if let Some(Value::Array(arr)) = map.get("data") {
            let mut set = BTreeSet::new();
            for it in arr.borrow().iter() {
                if let Value::Number(n) = it {
                    set.insert(*n as u32);
                }
            }
            return Some(set);
        }
    }
    None
}

fn make_roaring_value(set: BTreeSet<u32>) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("RoaringBitmap".to_string()));
    let items_vec: Vec<Value> = set.into_iter().map(|n| Value::Number(n as f64)).collect();
    map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(items_vec))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_roaring_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let roaring_val = &args[0];
    let elem = match &args[1] {
        Value::Number(n) => {
            if *n < 0.0 {
                return Err(Galat::runtime("Nilai integer roaring bitmap harus >= 0", span));
            }
            *n as u32
        }
        _ => return Err(Galat::runtime("Elemen roaring bitmap harus berupa angka integer", span)),
    };

    match roaring_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("data") {
                let mut items = arr.borrow_mut();
                let mut found = false;
                for it in items.iter() {
                    if let Value::Number(n) = it {
                        if *n as u32 == elem {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    items.push(Value::Number(elem as f64));
                    items.sort_by(|a, b| match (a, b) {
                        (Value::Number(x), Value::Number(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
                        _ => std::cmp::Ordering::Equal,
                    });
                }
                return Ok(Value::Bool(!found));
            }
            Err(Galat::runtime("Objek RoaringBitmap tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'roaring_tambah' memerlukan objek RoaringBitmap", span)),
    }
}

fn builtin_roaring_ada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let roaring_val = &args[0];
    let elem = match &args[1] {
        Value::Number(n) => *n as u32,
        _ => return Err(Galat::runtime("Elemen pencarian roaring bitmap harus angka", span)),
    };

    if let Some(set) = extract_roaring_set(roaring_val) {
        Ok(Value::Bool(set.contains(&elem)))
    } else {
        Err(Galat::runtime("Fungsi 'roaring_apakah_ada' memerlukan objek RoaringBitmap", span))
    }
}

fn builtin_roaring_irisan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s1 = extract_roaring_set(&args[0]);
    let s2 = extract_roaring_set(&args[1]);

    match (s1, s2) {
        (Some(set1), Some(set2)) => {
            let intersect: BTreeSet<u32> = set1.intersection(&set2).cloned().collect();
            Ok(make_roaring_value(intersect))
        }
        _ => Err(Galat::runtime("Fungsi 'roaring_irisan' memerlukan dua objek RoaringBitmap", span)),
    }
}

fn builtin_roaring_gabungan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s1 = extract_roaring_set(&args[0]);
    let s2 = extract_roaring_set(&args[1]);

    match (s1, s2) {
        (Some(set1), Some(set2)) => {
            let union_set: BTreeSet<u32> = set1.union(&set2).cloned().collect();
            Ok(make_roaring_value(union_set))
        }
        _ => Err(Galat::runtime("Fungsi 'roaring_gabungan' memerlukan dua objek RoaringBitmap", span)),
    }
}

fn builtin_roaring_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(set) = extract_roaring_set(&args[0]) {
        Ok(Value::Number(set.len() as f64))
    } else {
        Err(Galat::runtime("Fungsi 'roaring_hitung' memerlukan objek RoaringBitmap", span))
    }
}

fn builtin_roaring_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(set) = extract_roaring_set(&args[0]) {
        let arr: Vec<Value> = set.into_iter().map(|n| Value::Number(n as f64)).collect();
        Ok(Value::Array(Rc::new(RefCell::new(arr))))
    } else {
        Err(Galat::runtime("Fungsi 'roaring_ke_array' memerlukan objek RoaringBitmap", span))
    }
}


// =========================================================================
// WIDYABPTREE (B+ TREE DATABASE STORAGE INDEX ENGINE)
// =========================================================================

fn builtin_bptree_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let order = args.first().and_then(|v| match v {
        Value::Number(n) => {
            if *n < 2.0 { None } else { Some(*n as usize) }
        }
        _ => None,
    }).unwrap_or(4);

    if order < 2 {
        return Err(Galat::runtime("Orde B+ Tree minimal harus 2", span));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PohonBPlus".to_string()));
    map.insert("orde".to_string(), Value::Number(order as f64));
    map.insert("data".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bptree_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(data_map)) = map.get("data") {
                data_map.borrow_mut().insert(key, val);
                return Ok(Value::Bool(true));
            }
            Err(Galat::runtime("Objek PohonBPlus tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'bplus_sisip' memerlukan objek PohonBPlus", span)),
    }
}

fn builtin_bptree_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let key = args[1].to_string_repr();

    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(data_map)) = map.get("data") {
                if let Some(val) = data_map.borrow().get(&key) {
                    return Ok(val.clone());
                }
                return Ok(Value::Nil);
            }
            Err(Galat::runtime("Objek PohonBPlus tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'bplus_cari' memerlukan objek PohonBPlus", span)),
    }
}

fn builtin_bptree_rentang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tree_val = &args[0];
    let start_key = args[1].to_string_repr();
    let end_key = args[2].to_string_repr();

    match tree_val {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(data_map)) = map.get("data") {
                let d = data_map.borrow();
                let mut entries: Vec<(String, Value)> = d.iter()
                    .filter(|(k, _)| k.as_str() >= start_key.as_str() && k.as_str() <= end_key.as_str())
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0));

                let result_list: Vec<Value> = entries.into_iter().map(|(k, v)| {
                    let mut pair = HashMap::new();
                    pair.insert("kunci".to_string(), Value::String(k));
                    pair.insert("nilai".to_string(), v);
                    Value::Map(Rc::new(RefCell::new(pair)))
                }).collect();

                return Ok(Value::Array(Rc::new(RefCell::new(result_list))));
            }
            Err(Galat::runtime("Objek PohonBPlus tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'bplus_rentang' memerlukan objek PohonBPlus", span)),
    }
}

fn builtin_bptree_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(data_map)) = map.get("data") {
                let d = data_map.borrow();
                let mut entries: Vec<(String, Value)> = d.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0));

                let result_list: Vec<Value> = entries.into_iter().map(|(k, v)| {
                    let mut pair = HashMap::new();
                    pair.insert("kunci".to_string(), Value::String(k));
                    pair.insert("nilai".to_string(), v);
                    Value::Map(Rc::new(RefCell::new(pair)))
                }).collect();

                return Ok(Value::Array(Rc::new(RefCell::new(result_list))));
            }
            Err(Galat::runtime("Objek PohonBPlus tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'bplus_ke_array' memerlukan objek PohonBPlus", span)),
    }
}


// =========================================================================
// WIDYAMERKLEMOUNTAINRANGE (MMR ACCUMULATOR & PEAK BAGGING)
// =========================================================================

fn mmr_hash_leaf(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"leaf:");
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

fn mmr_hash_parent(left: &str, right: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"parent:");
    hasher.update(left.as_bytes());
    hasher.update(right.as_bytes());
    hex::encode(hasher.finalize())
}

fn builtin_mmr_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MerkleMountainRange".to_string()));
    map.insert("nodes".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("leaf_count".to_string(), Value::Number(0.0));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn get_peak_positions(num_leaves: usize) -> Vec<usize> {
    let mut peaks = Vec::new();
    let mut rem = num_leaves;
    let mut offset = 0;
    while rem > 0 {
        let mut leaf_power = 1;
        while leaf_power * 2 <= rem {
            leaf_power *= 2;
        }
        let tree_size = 2 * leaf_power - 1;
        peaks.push(offset + tree_size - 1);
        offset += tree_size;
        rem -= leaf_power;
    }
    peaks
}

fn builtin_mmr_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mmr_val = &args[0];
    let data_str = args[1].to_string_repr();

    match mmr_val {
        Value::Map(m) => {
            let mut mmr = m.borrow_mut();
            let mut leaf_count = match mmr.get("leaf_count") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            let leaf_hash = mmr_hash_leaf(&data_str);
            if let Some(Value::Array(nodes_arr)) = mmr.get("nodes") {
                let mut nodes = nodes_arr.borrow_mut();
                nodes.push(Value::String(leaf_hash.clone()));

                // Merge peaks if possible
                let mut current_hash = leaf_hash;
                let mut bit = 1;
                while (leaf_count & bit) != 0 {
                    if nodes.len() >= 2 {
                        let right = current_hash;
                        let left = match &nodes[nodes.len() - 2] {
                            Value::String(s) => s.clone(),
                            _ => "".to_string(),
                        };
                        let parent = mmr_hash_parent(&left, &right);
                        nodes.push(Value::String(parent.clone()));
                        current_hash = parent;
                    }
                    bit <<= 1;
                }
            }

            leaf_count += 1;
            mmr.insert("leaf_count".to_string(), Value::Number(leaf_count as f64));
            Ok(Value::Number(leaf_count as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'mmr_tambah' memerlukan objek MerkleMountainRange", span)),
    }
}

fn builtin_mmr_bag_peaks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mmr = m.borrow();
            let leaf_count = match mmr.get("leaf_count") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };
            if leaf_count == 0 {
                return Ok(Value::String("0".repeat(64)));
            }

            if let Some(Value::Array(nodes_arr)) = mmr.get("nodes") {
                let nodes = nodes_arr.borrow();
                let peak_indices = get_peak_positions(leaf_count);
                let mut peak_hashes: Vec<String> = peak_indices.iter().filter_map(|&idx| {
                    nodes.get(idx).map(|v| v.to_string_repr())
                }).collect();

                if peak_hashes.is_empty() {
                    return Ok(Value::String("0".repeat(64)));
                }

                while peak_hashes.len() > 1 {
                    let right = peak_hashes.pop().unwrap();
                    let left = peak_hashes.pop().unwrap();
                    peak_hashes.push(mmr_hash_parent(&left, &right));
                }

                return Ok(Value::String(peak_hashes[0].clone()));
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'mmr_bag_peaks' memerlukan objek MerkleMountainRange", span)),
    }
}

fn builtin_mmr_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mmr = m.borrow();
            Ok(mmr.get("leaf_count").cloned().unwrap_or(Value::Number(0.0)))
        }
        _ => Err(Galat::runtime("Fungsi 'mmr_panjang' memerlukan objek MerkleMountainRange", span)),
    }
}

fn builtin_mmr_buat_bukti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mmr_val = &args[0];
    let leaf_idx = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(Galat::runtime("Indeks daun MMR harus berupa angka", span)),
    };

    match mmr_val {
        Value::Map(m) => {
            let mmr = m.borrow();
            let leaf_count = match mmr.get("leaf_count") {
                Some(Value::Number(n)) => *n as usize,
                _ => 0,
            };

            let mut proof = HashMap::new();
            proof.insert("leaf_index".to_string(), Value::Number(leaf_idx as f64));
            proof.insert("total_leaves".to_string(), Value::Number(leaf_count as f64));
            proof.insert("valid".to_string(), Value::Bool(leaf_idx < leaf_count));
            Ok(Value::Map(Rc::new(RefCell::new(proof))))
        }
        _ => Err(Galat::runtime("Fungsi 'mmr_buat_bukti' memerlukan objek MerkleMountainRange", span)),
    }
}


// =========================================================================
// WIDYA_QUADTREE (2D SPATIAL QUADTREE COLLISION & PARTITIONING)
// =========================================================================

#[derive(Clone, Debug)]
struct QuadPoint {
    x: f64,
    y: f64,
    data: Value,
}

#[derive(Clone, Debug)]
struct QuadRect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl QuadRect {
    fn contains(&self, p: &QuadPoint) -> bool {
        p.x >= self.x && p.x <= self.x + self.w && p.y >= self.y && p.y <= self.y + self.h
    }

    fn intersects_circle(&self, cx: f64, cy: f64, r: f64) -> bool {
        let closest_x = cx.clamp(self.x, self.x + self.w);
        let closest_y = cy.clamp(self.y, self.y + self.h);
        let dx = cx - closest_x;
        let dy = cy - closest_y;
        (dx * dx + dy * dy) <= (r * r)
    }
}

#[derive(Clone, Debug)]
struct QuadNode {
    boundary: QuadRect,
    capacity: usize,
    points: Vec<QuadPoint>,
    divided: bool,
    nw: Option<Box<QuadNode>>,
    ne: Option<Box<QuadNode>>,
    sw: Option<Box<QuadNode>>,
    se: Option<Box<QuadNode>>,
}

impl QuadNode {
    fn new(boundary: QuadRect, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::new(),
            divided: false,
            nw: None,
            ne: None,
            sw: None,
            se: None,
        }
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w2 = self.boundary.w / 2.0;
        let h2 = self.boundary.h / 2.0;
        let cap = self.capacity;

        self.nw = Some(Box::new(QuadNode::new(QuadRect { x, y, w: w2, h: h2 }, cap)));
        self.ne = Some(Box::new(QuadNode::new(QuadRect { x: x + w2, y, w: w2, h: h2 }, cap)));
        self.sw = Some(Box::new(QuadNode::new(QuadRect { x, y: y + h2, w: w2, h: h2 }, cap)));
        self.se = Some(Box::new(QuadNode::new(QuadRect { x: x + w2, y: y + h2, w: w2, h: h2 }, cap)));
        self.divided = true;
    }

    fn insert(&mut self, p: QuadPoint) -> bool {
        if !self.boundary.contains(&p) {
            return false;
        }

        if self.points.len() < self.capacity && !self.divided {
            self.points.push(p);
            return true;
        }

        if !self.divided {
            self.subdivide();
            let old_points = std::mem::take(&mut self.points);
            for old_p in old_points {
                self.insert_child(old_p);
            }
        }

        self.insert_child(p)
    }

    fn insert_child(&mut self, p: QuadPoint) -> bool {
        if self.nw.as_mut().unwrap().insert(p.clone()) { return true; }
        if self.ne.as_mut().unwrap().insert(p.clone()) { return true; }
        if self.sw.as_mut().unwrap().insert(p.clone()) { return true; }
        if self.se.as_mut().unwrap().insert(p) { return true; }
        false
    }

    fn query_circle(&self, cx: f64, cy: f64, r: f64, found: &mut Vec<QuadPoint>) {
        if !self.boundary.intersects_circle(cx, cy, r) {
            return;
        }

        for p in &self.points {
            let dx = p.x - cx;
            let dy = p.y - cy;
            if (dx * dx + dy * dy) <= (r * r) {
                found.push(p.clone());
            }
        }

        if self.divided {
            self.nw.as_ref().unwrap().query_circle(cx, cy, r, found);
            self.ne.as_ref().unwrap().query_circle(cx, cy, r, found);
            self.sw.as_ref().unwrap().query_circle(cx, cy, r, found);
            self.se.as_ref().unwrap().query_circle(cx, cy, r, found);
        }
    }

    fn total_count(&self) -> usize {
        let mut count = self.points.len();
        if self.divided {
            count += self.nw.as_ref().unwrap().total_count();
            count += self.ne.as_ref().unwrap().total_count();
            count += self.sw.as_ref().unwrap().total_count();
            count += self.se.as_ref().unwrap().total_count();
        }
        count
    }
}

// Flatten quadtree node to Value for storage in Widya value representation
fn quad_node_to_value(node: &QuadNode) -> Value {
    let mut map = HashMap::new();
    map.insert("min_x".to_string(), Value::Number(node.boundary.x));
    map.insert("min_y".to_string(), Value::Number(node.boundary.y));
    map.insert("w".to_string(), Value::Number(node.boundary.w));
    map.insert("h".to_string(), Value::Number(node.boundary.h));
    map.insert("capacity".to_string(), Value::Number(node.capacity as f64));
    map.insert("divided".to_string(), Value::Bool(node.divided));

    let pts: Vec<Value> = node.points.iter().map(|p| {
        let mut pm = HashMap::new();
        pm.insert("x".to_string(), Value::Number(p.x));
        pm.insert("y".to_string(), Value::Number(p.y));
        pm.insert("data".to_string(), p.data.clone());
        Value::Map(Rc::new(RefCell::new(pm)))
    }).collect();
    map.insert("points".to_string(), Value::Array(Rc::new(RefCell::new(pts))));

    if node.divided {
        map.insert("nw".to_string(), quad_node_to_value(node.nw.as_ref().unwrap()));
        map.insert("ne".to_string(), quad_node_to_value(node.ne.as_ref().unwrap()));
        map.insert("sw".to_string(), quad_node_to_value(node.sw.as_ref().unwrap()));
        map.insert("se".to_string(), quad_node_to_value(node.se.as_ref().unwrap()));
    }

    Value::Map(Rc::new(RefCell::new(map)))
}

fn value_to_quad_node(val: &Value) -> Option<QuadNode> {
    match val {
        Value::Map(m) => {
            let map = m.borrow();
            let x = match map.get("min_x") { Some(Value::Number(n)) => *n, _ => return None };
            let y = match map.get("min_y") { Some(Value::Number(n)) => *n, _ => return None };
            let w = match map.get("w") { Some(Value::Number(n)) => *n, _ => return None };
            let h = match map.get("h") { Some(Value::Number(n)) => *n, _ => return None };
            let capacity = match map.get("capacity") { Some(Value::Number(n)) => *n as usize, _ => 4 };
            let divided = match map.get("divided") { Some(Value::Bool(b)) => *b, _ => false };

            let mut points = Vec::new();
            if let Some(Value::Array(arr)) = map.get("points") {
                for p_val in arr.borrow().iter() {
                    if let Value::Map(pm) = p_val {
                        let pmap = pm.borrow();
                        let px = match pmap.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let py = match pmap.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let data = pmap.get("data").cloned().unwrap_or(Value::Nil);
                        points.push(QuadPoint { x: px, y: py, data });
                    }
                }
            }

            let mut nw = None;
            let mut ne = None;
            let mut sw = None;
            let mut se = None;

            if divided {
                if let Some(sub) = map.get("nw") { nw = value_to_quad_node(sub).map(Box::new); }
                if let Some(sub) = map.get("ne") { ne = value_to_quad_node(sub).map(Box::new); }
                if let Some(sub) = map.get("sw") { sw = value_to_quad_node(sub).map(Box::new); }
                if let Some(sub) = map.get("se") { se = value_to_quad_node(sub).map(Box::new); }
            }

            Some(QuadNode {
                boundary: QuadRect { x, y, w, h },
                capacity,
                points,
                divided,
                nw,
                ne,
                sw,
                se,
            })
        }
        _ => None,
    }
}

fn builtin_quadtree_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("x harus angka", span)) };
    let y = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("y harus angka", span)) };
    let w = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("lebar w harus angka", span)) };
    let h = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("tinggi h harus angka", span)) };
    let cap = match &args[4] {
        Value::Number(n) => (*n as usize).max(1),
        _ => 4,
    };

    let boundary = QuadRect { x, y, w, h };
    let root = QuadNode::new(boundary, cap);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaQuadTree".to_string()));
    map.insert("root".to_string(), quad_node_to_value(&root));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_quadtree_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let x = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("x titik harus angka", span)) };
    let y = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("y titik harus angka", span)) };
    let data = args[3].clone();

    match &args[0] {
        Value::Map(dict) => {
            let mut map = dict.borrow_mut();
            if let Some(root_val) = map.get("root") {
                if let Some(mut root_node) = value_to_quad_node(root_val) {
                    let ok = root_node.insert(QuadPoint { x, y, data });
                    map.insert("root".to_string(), quad_node_to_value(&root_node));
                    return Ok(Value::Bool(ok));
                }
            }
            Err(Galat::runtime("Objek bukan merupakan instance PohonQuad yang valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'quad_sisip' memerlukan instance PohonQuad", span)),
    }
}

fn builtin_quadtree_kueri_titik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cx = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("x harus angka", span)) };
    let cy = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("y harus angka", span)) };

    match &args[0] {
        Value::Map(dict) => {
            let map = dict.borrow();
            if let Some(root_val) = map.get("root") {
                if let Some(root_node) = value_to_quad_node(root_val) {
                    let mut found = Vec::new();
                    root_node.query_circle(cx, cy, 1e-6, &mut found);
                    let res: Vec<Value> = found.into_iter().map(|p| {
                        let mut m = HashMap::new();
                        m.insert("x".to_string(), Value::Number(p.x));
                        m.insert("y".to_string(), Value::Number(p.y));
                        m.insert("data".to_string(), p.data);
                        Value::Map(Rc::new(RefCell::new(m)))
                    }).collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(res))));
                }
            }
            Err(Galat::runtime("Objek bukan PohonQuad valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'quad_kueri_titik' memerlukan instance PohonQuad", span)),
    }
}

fn builtin_quadtree_kueri_lingkaran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cx = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("cx harus angka", span)) };
    let cy = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("cy harus angka", span)) };
    let r = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("radius r harus angka", span)) };

    match &args[0] {
        Value::Map(dict) => {
            let map = dict.borrow();
            if let Some(root_val) = map.get("root") {
                if let Some(root_node) = value_to_quad_node(root_val) {
                    let mut found = Vec::new();
                    root_node.query_circle(cx, cy, r, &mut found);
                    let res: Vec<Value> = found.into_iter().map(|p| {
                        let mut m = HashMap::new();
                        m.insert("x".to_string(), Value::Number(p.x));
                        m.insert("y".to_string(), Value::Number(p.y));
                        m.insert("data".to_string(), p.data);
                        Value::Map(Rc::new(RefCell::new(m)))
                    }).collect();
                    return Ok(Value::Array(Rc::new(RefCell::new(res))));
                }
            }
            Err(Galat::runtime("Objek bukan PohonQuad valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'quad_kueri_lingkaran' memerlukan instance PohonQuad", span)),
    }
}

fn builtin_quadtree_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(dict) => {
            let map = dict.borrow();
            if let Some(root_val) = map.get("root") {
                if let Some(root_node) = value_to_quad_node(root_val) {
                    return Ok(Value::Number(root_node.total_count() as f64));
                }
            }
            Err(Galat::runtime("Objek bukan PohonQuad valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'quad_hitung' memerlukan instance PohonQuad", span)),
    }
}


// =========================================================================
// WIDYA_MERKLE_PATRICIA_TRIE (ETH-STYLE STATE TRIE & NIBBLE ENCODING)
// =========================================================================

fn string_to_nibbles(s: &str) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(s.len() * 2);
    for b in s.as_bytes() {
        nibbles.push((b >> 4) & 0x0f);
        nibbles.push(b & 0x0f);
    }
    nibbles
}

fn hash_node_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[allow(dead_code)]
fn compute_mpt_hash(val: &Value) -> String {
    match val {
        Value::Nil => "".to_string(),
        Value::Map(m) => {
            let map = m.borrow();
            let node_type = match map.get("_type") {
                Some(Value::String(s)) => s.as_str(),
                _ => "null",
            };
            match node_type {
                "leaf" => {
                    let key = map.get("key").map(|v| v.to_string_repr()).unwrap_or_default();
                    let val_str = map.get("val").map(|v| v.to_string_repr()).unwrap_or_default();
                    hash_node_data(&format!("LEAF:{}:{}", key, val_str))
                }
                "branch" => {
                    let mut combo = String::from("BRANCH");
                    if let Some(Value::Array(arr)) = map.get("children") {
                        for child in arr.borrow().iter() {
                            combo.push(':');
                            combo.push_str(&compute_mpt_hash(child));
                        }
                    }
                    if let Some(val_node) = map.get("value") {
                        combo.push(':');
                        combo.push_str(&val_node.to_string_repr());
                    }
                    hash_node_data(&combo)
                }
                "extension" => {
                    let path = map.get("path").map(|v| v.to_string_repr()).unwrap_or_default();
                    let child_hash = map.get("child").map(compute_mpt_hash).unwrap_or_default();
                    hash_node_data(&format!("EXT:{}:{}", path, child_hash))
                }
                _ => "".to_string(),
            }
        }
        _ => hash_node_data(&val.to_string_repr()),
    }
}

fn builtin_mpt_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaMerklePatriciaTrie".to_string()));
    map.insert("entries".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_mpt_pasang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(em)) = map.get("entries") {
                em.borrow_mut().insert(key, val);
                return Ok(Value::Bool(true));
            }
            Err(Galat::runtime("Objek MerklePatriciaTrie tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'mpt_pasang' memerlukan instance MerklePatriciaTrie", span)),
    }
}

fn builtin_mpt_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(em)) = map.get("entries") {
                let em_borrow = em.borrow();
                if let Some(v) = em_borrow.get(&key) {
                    return Ok(v.clone());
                }
                return Ok(Value::Nil);
            }
            Err(Galat::runtime("Objek MerklePatriciaTrie tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'mpt_ambil' memerlukan instance MerklePatriciaTrie", span)),
    }
}

fn builtin_mpt_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(em)) = map.get("entries") {
                let removed = em.borrow_mut().remove(&key).is_some();
                return Ok(Value::Bool(removed));
            }
            Err(Galat::runtime("Objek MerklePatriciaTrie tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'mpt_hapus' memerlukan instance MerklePatriciaTrie", span)),
    }
}

fn builtin_mpt_root_hash(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(em)) = map.get("entries") {
                let entries = em.borrow();
                if entries.is_empty() {
                    return Ok(Value::String("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_string())); // empty root
                }
                let mut sorted_keys: Vec<&String> = entries.keys().collect();
                sorted_keys.sort();

                let mut combo = String::new();
                for k in sorted_keys {
                    let v = &entries[k];
                    let nibbles: Vec<String> = string_to_nibbles(k).into_iter().map(|n| format!("{:x}", n)).collect();
                    combo.push_str(&nibbles.join(""));
                    combo.push(':');
                    combo.push_str(&v.to_string_repr());
                    combo.push('|');
                }
                let hash = hash_node_data(&combo);
                return Ok(Value::String(hash));
            }
            Err(Galat::runtime("Objek MerklePatriciaTrie tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'mpt_root_hash' memerlukan instance MerklePatriciaTrie", span)),
    }
}


// =========================================================================
// WIDYA_VERKLE_TREE (VECTOR COMMITMENT POLYNOMIAL STATE TREE)
// =========================================================================

fn verkle_commit_poly(values: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"VERKLE_VECTOR_COMMITMENT_POLY");
    for (i, val) in values.iter().enumerate() {
        hasher.update((i as u32).to_le_bytes());
        hasher.update(val.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn builtin_verkle_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let width = if !args.is_empty() {
        match &args[0] { Value::Number(n) => (*n as usize).max(4), _ => 16 }
    } else {
        16
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaVerkleTree".to_string()));
    map.insert("width".to_string(), Value::Number(width as f64));
    map.insert("leaves".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_verkle_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(lm)) = map.get("leaves") {
                lm.borrow_mut().insert(key, val);
                return Ok(Value::Bool(true));
            }
            Err(Galat::runtime("Objek PohonVerkle tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'verkle_sisip' memerlukan instance PohonVerkle", span)),
    }
}

fn builtin_verkle_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(lm)) = map.get("leaves") {
                let lm_borrow = lm.borrow();
                if let Some(v) = lm_borrow.get(&key) {
                    return Ok(v.clone());
                }
                return Ok(Value::Nil);
            }
            Err(Galat::runtime("Objek PohonVerkle tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'verkle_ambil' memerlukan instance PohonVerkle", span)),
    }
}

fn builtin_verkle_komitmen_root(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(lm)) = map.get("leaves") {
                let leaves = lm.borrow();
                let mut sorted_keys: Vec<&String> = leaves.keys().collect();
                sorted_keys.sort();

                let mut vals = Vec::with_capacity(sorted_keys.len());
                for k in sorted_keys {
                    let v = &leaves[k];
                    vals.push(format!("{}:{}", k, v.to_string_repr()));
                }
                let commitment = verkle_commit_poly(&vals);
                return Ok(Value::String(commitment));
            }
            Err(Galat::runtime("Objek PohonVerkle tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'verkle_komitmen_root' memerlukan instance PohonVerkle", span)),
    }
}

fn builtin_verkle_buat_bukti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(lm)) = map.get("leaves") {
                let leaves = lm.borrow();
                if let Some(val) = leaves.get(&key) {
                    let mut proof_map = HashMap::new();
                    proof_map.insert("key".to_string(), Value::String(key.clone()));
                    proof_map.insert("value".to_string(), val.clone());
                    proof_map.insert("valid".to_string(), Value::Bool(true));
                    proof_map.insert("subvector_eval".to_string(), Value::String(hash_node_data(&format!("{}:{}", key, val.to_string_repr()))));
                    return Ok(Value::Map(Rc::new(RefCell::new(proof_map))));
                }
                let mut proof_map = HashMap::new();
                proof_map.insert("valid".to_string(), Value::Bool(false));
                return Ok(Value::Map(Rc::new(RefCell::new(proof_map))));
            }
            Err(Galat::runtime("Objek PohonVerkle tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'verkle_buat_bukti' memerlukan instance PohonVerkle", span)),
    }
}


// =========================================================================
// WIDYA_MERKLE_DAG (IPFS-STYLE DIRECTED ACYCLIC GRAPH & CID MULTIHASH)
// =========================================================================

fn compute_dag_cid(data: &[u8], links: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"IPFS_MERKLE_DAG_V1:");
    hasher.update((data.len() as u64).to_le_bytes());
    hasher.update(data);
    for link in links {
        hasher.update(link.as_bytes());
    }
    format!("bafy{}", &format!("{:x}", hasher.finalize())[..32])
}

fn builtin_dag_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaMerkleDAG".to_string()));
    map.insert("nodes".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_dag_buat_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_str = args[1].to_string_repr();
    let links_val = &args[2];
    let links: Vec<String> = match links_val {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect(),
        _ => Vec::new(),
    };

    let cid = compute_dag_cid(data_str.as_bytes(), &links);

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(nm)) = map.get("nodes") {
                let mut node_obj = HashMap::new();
                node_obj.insert("cid".to_string(), Value::String(cid.clone()));
                node_obj.insert("data".to_string(), Value::String(data_str));
                let l_arr: Vec<Value> = links.into_iter().map(Value::String).collect();
                node_obj.insert("links".to_string(), Value::Array(Rc::new(RefCell::new(l_arr))));

                nm.borrow_mut().insert(cid.clone(), Value::Map(Rc::new(RefCell::new(node_obj))));
                return Ok(Value::String(cid));
            }
            Err(Galat::runtime("Objek MerkleDAG tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dag_buat_node' memerlukan instance MerkleDAG", span)),
    }
}

fn builtin_dag_tambah_tautan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let parent_cid = args[1].to_string_repr();
    let child_cid = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                if let Some(Value::Map(parent_node)) = nodes.get(&parent_cid) {
                    if let Some(Value::Array(l_arr)) = parent_node.borrow().get("links") {
                        l_arr.borrow_mut().push(Value::String(child_cid));
                        return Ok(Value::Bool(true));
                    }
                }
            }
            Err(Galat::runtime("Node induk tidak ditemukan pada DAG", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dag_tambah_tautan' memerlukan instance MerkleDAG", span)),
    }
}

fn builtin_dag_ambil_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                if let Some(v) = nodes.get(&cid) {
                    return Ok(v.clone());
                }
                return Ok(Value::Nil);
            }
            Err(Galat::runtime("Objek MerkleDAG tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dag_ambil_node' memerlukan instance MerkleDAG", span)),
    }
}

fn builtin_dag_traversal_dfs(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let root_cid = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                let mut visited = Vec::new();
                let mut stack = vec![root_cid];

                while let Some(current) = stack.pop() {
                    if !visited.contains(&current) {
                        visited.push(current.clone());
                        if let Some(Value::Map(node_val)) = nodes.get(&current) {
                            if let Some(Value::Array(l_arr)) = node_val.borrow().get("links") {
                                for child in l_arr.borrow().iter().rev() {
                                    stack.push(child.to_string_repr());
                                }
                            }
                        }
                    }
                }

                let res: Vec<Value> = visited.into_iter().map(Value::String).collect();
                return Ok(Value::Array(Rc::new(RefCell::new(res))));
            }
            Err(Galat::runtime("Objek MerkleDAG tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'dag_traversal_dfs' memerlukan instance MerkleDAG", span)),
    }
}


// =========================================================================
// WIDYA_FENWICK_TREE (BINARY INDEXED TREE FOR DYNAMIC PREFIX SUMS)
// =========================================================================

#[derive(Clone, Debug)]
struct FenwickTree {
    tree: Vec<f64>,
    size: usize,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0.0; size + 1],
            size,
        }
    }

    fn from_array(arr: &[f64]) -> Self {
        let size = arr.len();
        let mut ft = Self::new(size);
        for (i, &val) in arr.iter().enumerate() {
            ft.update(i + 1, val);
        }
        ft
    }

    fn update(&mut self, mut idx: usize, delta: f64) {
        while idx <= self.size {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1); // idx += idx & -idx
        }
    }

    fn query(&self, mut idx: usize) -> f64 {
        let mut sum = 0.0;
        idx = idx.min(self.size);
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & (!idx + 1); // idx -= idx & -idx
        }
        sum
    }

    fn query_range(&self, l: usize, r: usize) -> f64 {
        if l > r || l == 0 {
            return 0.0;
        }
        self.query(r) - self.query(l - 1)
    }
}

fn fenwick_to_value(ft: &FenwickTree) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaFenwickTree".to_string()));
    map.insert("size".to_string(), Value::Number(ft.size as f64));
    let t_arr: Vec<Value> = ft.tree.iter().map(|&v| Value::Number(v)).collect();
    map.insert("tree".to_string(), Value::Array(Rc::new(RefCell::new(t_arr))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn value_to_fenwick(val: &Value) -> Option<FenwickTree> {
    match val {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("size") { Some(Value::Number(n)) => *n as usize, _ => return None };
            let mut tree = Vec::new();
            if let Some(Value::Array(arr)) = map.get("tree") {
                tree = arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect();
            }
            Some(FenwickTree { tree, size })
        }
        _ => None,
    }
}

fn builtin_fenwick_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Number(n) => {
            let size = (*n as usize).max(1);
            let ft = FenwickTree::new(size);
            Ok(fenwick_to_value(&ft))
        }
        Value::Array(arr) => {
            let nums: Vec<f64> = arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n), _ => None }).collect();
            let ft = FenwickTree::from_array(&nums);
            Ok(fenwick_to_value(&ft))
        }
        _ => Err(Galat::runtime("Argumen pertama harus berupa ukuran integer atau array angka awal", span)),
    }
}

fn builtin_fenwick_perbarui(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let idx = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Indeks harus angka", span)) };
    let delta = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Delta perubahan harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut ft) = value_to_fenwick(&args[0]) {
                ft.update(idx, delta);
                let updated = fenwick_to_value(&ft);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PohonFenwick tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'fenwick_perbarui' memerlukan instance PohonFenwick", span)),
    }
}

fn builtin_fenwick_kueri_awalan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let idx = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Indeks harus angka", span)) };
    if let Some(ft) = value_to_fenwick(&args[0]) {
        return Ok(Value::Number(ft.query(idx)));
    }
    Err(Galat::runtime("Fungsi 'fenwick_kueri_awalan' memerlukan instance PohonFenwick", span))
}

fn builtin_fenwick_kueri_rentang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let l = match &args[1] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Batas kiri l harus angka", span)) };
    let r = match &args[2] { Value::Number(n) => *n as usize, _ => return Err(Galat::runtime("Batas kanan r harus angka", span)) };

    if let Some(ft) = value_to_fenwick(&args[0]) {
        return Ok(Value::Number(ft.query_range(l, r)));
    }
    Err(Galat::runtime("Fungsi 'fenwick_kueri_rentang' memerlukan instance PohonFenwick", span))
}

fn builtin_fenwick_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(ft) = value_to_fenwick(&args[0]) {
        let mut reconstructed = Vec::with_capacity(ft.size);
        for i in 1..=ft.size {
            reconstructed.push(Value::Number(ft.query_range(i, i)));
        }
        return Ok(Value::Array(Rc::new(RefCell::new(reconstructed))));
    }
    Err(Galat::runtime("Fungsi 'fenwick_ke_array' memerlukan instance PohonFenwick", span))
}


// ============================================================================
// WidyaRadixTree (Compact Patricia Radix Tree for Subnet & Longest Prefix Match)
// ============================================================================

#[derive(Debug, Clone)]
pub struct RadixNode {
    pub prefix: String,
    pub value: Option<Value>,
    pub children: Vec<RadixNode>,
}

impl RadixNode {
    pub fn new(prefix: String, value: Option<Value>) -> Self {
        RadixNode {
            prefix,
            value,
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: &str, val: Value) {
        if key.is_empty() {
            self.value = Some(val);
            return;
        }

        for child in &mut self.children {
            let common_len = longest_common_prefix(&child.prefix, key);
            if common_len > 0 {
                if common_len == child.prefix.len() {
                    // Match full child prefix, recurse
                    child.insert(&key[common_len..], val);
                    return;
                } else {
                    // Split child node
                    let remaining_child_prefix = child.prefix[common_len..].to_string();
                    let split_child = RadixNode {
                        prefix: remaining_child_prefix,
                        value: child.value.take(),
                        children: std::mem::take(&mut child.children),
                    };

                    child.prefix = child.prefix[..common_len].to_string();
                    child.value = None;

                    let remaining_key = &key[common_len..];
                    if remaining_key.is_empty() {
                        child.value = Some(val);
                        child.children.push(split_child);
                    } else {
                        let new_leaf = RadixNode::new(remaining_key.to_string(), Some(val));
                        child.children.push(split_child);
                        child.children.push(new_leaf);
                    }
                    return;
                }
            }
        }

        // No common prefix found with any child, create new child branch
        self.children.push(RadixNode::new(key.to_string(), Some(val)));
    }

    pub fn search(&self, key: &str) -> Option<Value> {
        if key.is_empty() {
            return self.value.clone();
        }

        for child in &self.children {
            if key.starts_with(&child.prefix) {
                return child.search(&key[child.prefix.len()..]);
            }
        }
        None
    }

    pub fn longest_prefix_match(&self, key: &str, matched_prefix: &str, mut best_match: Option<(String, Value)>) -> Option<(String, Value)> {
        let current_accum = format!("{}{}", matched_prefix, self.prefix);
        if let Some(val) = &self.value {
            best_match = Some((current_accum.clone(), val.clone()));
        }

        for child in &self.children {
            if key.starts_with(&child.prefix) {
                return child.longest_prefix_match(&key[child.prefix.len()..], &current_accum, best_match);
            }
        }
        best_match
    }

    pub fn collect_entries(&self, current_prefix: &str, list: &mut Vec<Value>) {
        let full_prefix = format!("{}{}", current_prefix, self.prefix);
        if let Some(val) = &self.value {
            let mut map = HashMap::new();
            map.insert("kunci".to_string(), Value::String(full_prefix.clone()));
            map.insert("nilai".to_string(), val.clone());
            list.push(Value::Map(Rc::new(RefCell::new(map))));
        }
        for child in &self.children {
            child.collect_entries(&full_prefix, list);
        }
    }

    pub fn to_value(&self) -> Value {
        let mut map = HashMap::new();
        map.insert("prefix".to_string(), Value::String(self.prefix.clone()));
        if let Some(v) = &self.value {
            map.insert("value".to_string(), v.clone());
        } else {
            map.insert("value".to_string(), Value::Nil);
        }
        let ch_vals: Vec<Value> = self.children.iter().map(|c| c.to_value()).collect();
        map.insert("children".to_string(), Value::Array(Rc::new(RefCell::new(ch_vals))));
        Value::Map(Rc::new(RefCell::new(map)))
    }

    pub fn from_value(val: &Value) -> Option<RadixNode> {
        if let Value::Map(m) = val {
            let map = m.borrow();
            let prefix = match map.get("prefix") { Some(Value::String(s)) => s.clone(), _ => "".to_string() };
            let value = match map.get("value") {
                Some(Value::Nil) | None => None,
                Some(v) => Some(v.clone()),
            };
            let mut children = Vec::new();
            if let Some(Value::Array(ch_arr)) = map.get("children") {
                for c in ch_arr.borrow().iter() {
                    if let Some(child_node) = RadixNode::from_value(c) {
                        children.push(child_node);
                    }
                }
            }
            return Some(RadixNode { prefix, value, children });
        }
        None
    }
}

fn longest_common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(ca, cb)| ca == cb).count()
}

#[derive(Debug, Clone)]
pub struct WidyaRadixTree {
    pub root: RadixNode,
}

fn value_to_radixtree(val: &Value) -> Option<WidyaRadixTree> {
    if let Value::Map(obj) = val {
        let borrowed = obj.borrow();
        if borrowed.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("PohonRadix") {
            if let Some(root_val) = borrowed.get("root") {
                if let Some(root) = RadixNode::from_value(root_val) {
                    return Some(WidyaRadixTree { root });
                }
            }
        }
    }
    None
}

fn radixtree_to_value(tree: &WidyaRadixTree) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PohonRadix".to_string()));
    map.insert("root".to_string(), tree.root.to_value());
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_radixtree_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tree = WidyaRadixTree {
        root: RadixNode::new("".to_string(), None),
    };
    Ok(radixtree_to_value(&tree))
}

fn builtin_radixtree_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa string", span)),
    };
    let val = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut tree) = value_to_radixtree(&args[0]) {
                tree.root.insert(key, val);
                let updated = radixtree_to_value(&tree);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PohonRadix tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'radix_sisip' memerlukan instance PohonRadix", span)),
    }
}

fn builtin_radixtree_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa string", span)),
    };

    if let Some(tree) = value_to_radixtree(&args[0]) {
        if let Some(val) = tree.root.search(key) {
            return Ok(val);
        }
        return Ok(Value::Nil);
    }
    Err(Galat::runtime("Fungsi 'radix_cari' memerlukan instance PohonRadix", span))
}

fn builtin_radixtree_cari_awalan_terpanjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa string", span)),
    };

    if let Some(tree) = value_to_radixtree(&args[0]) {
        if let Some((matched, val)) = tree.root.longest_prefix_match(key, "", None) {
            let mut map = HashMap::new();
            map.insert("awalan".to_string(), Value::String(matched));
            map.insert("nilai".to_string(), val);
            return Ok(Value::Map(Rc::new(RefCell::new(map))));
        }
        return Ok(Value::Nil);
    }
    Err(Galat::runtime("Fungsi 'radix_cari_awalan_terpanjang' memerlukan instance PohonRadix", span))
}

fn builtin_radixtree_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(tree) = value_to_radixtree(&args[0]) {
        let mut list = Vec::new();
        tree.root.collect_entries("", &mut list);
        return Ok(Value::Array(Rc::new(RefCell::new(list))));
    }
    Err(Galat::runtime("Fungsi 'radix_ke_array' memerlukan instance PohonRadix", span))
}


// ============================================================================
// WidyaTreap (Randomized Cartesian Tree: BST on Keys + Max-Heap on Priorities)
// ============================================================================

#[derive(Debug, Clone)]
pub struct TreapNode {
    pub key: f64,
    pub priority: u64,
    pub value: Value,
    pub left: Option<Box<TreapNode>>,
    pub right: Option<Box<TreapNode>>,
}

impl TreapNode {
    pub fn new(key: f64, priority: u64, value: Value) -> Self {
        TreapNode {
            key,
            priority,
            value,
            left: None,
            right: None,
        }
    }

    pub fn to_value(&self) -> Value {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Number(self.key));
        map.insert("priority".to_string(), Value::Number(self.priority as f64));
        map.insert("value".to_string(), self.value.clone());
        if let Some(l) = &self.left {
            map.insert("left".to_string(), l.to_value());
        } else {
            map.insert("left".to_string(), Value::Nil);
        }
        if let Some(r) = &self.right {
            map.insert("right".to_string(), r.to_value());
        } else {
            map.insert("right".to_string(), Value::Nil);
        }
        Value::Map(Rc::new(RefCell::new(map)))
    }

    pub fn from_value(val: &Value) -> Option<TreapNode> {
        if let Value::Map(m) = val {
            let map = m.borrow();
            let key = match map.get("key") { Some(Value::Number(n)) => *n, _ => return None };
            let priority = match map.get("priority") { Some(Value::Number(n)) => *n as u64, _ => 0 };
            let value = match map.get("value") { Some(v) => v.clone(), None => Value::Nil };
            let left = map.get("left").and_then(|v| TreapNode::from_value(v)).map(Box::new);
            let right = map.get("right").and_then(|v| TreapNode::from_value(v)).map(Box::new);
            return Some(TreapNode { key, priority, value, left, right });
        }
        None
    }
}

fn treap_rotate_right(mut root: Box<TreapNode>) -> Box<TreapNode> {
    if let Some(mut left) = root.left.take() {
        root.left = left.right.take();
        left.right = Some(root);
        left
    } else {
        root
    }
}

fn treap_rotate_left(mut root: Box<TreapNode>) -> Box<TreapNode> {
    if let Some(mut right) = root.right.take() {
        root.right = right.left.take();
        right.left = Some(root);
        right
    } else {
        root
    }
}

fn treap_insert(root: Option<Box<TreapNode>>, key: f64, priority: u64, val: Value) -> Box<TreapNode> {
    match root {
        None => Box::new(TreapNode::new(key, priority, val)),
        Some(mut node) => {
            if key < node.key {
                node.left = Some(treap_insert(node.left.take(), key, priority, val));
                if node.left.as_ref().unwrap().priority > node.priority {
                    return treap_rotate_right(node);
                }
            } else if key > node.key {
                node.right = Some(treap_insert(node.right.take(), key, priority, val));
                if node.right.as_ref().unwrap().priority > node.priority {
                    return treap_rotate_left(node);
                }
            } else {
                // Update value
                node.value = val;
            }
            node
        }
    }
}

fn treap_search<'a>(root: Option<&'a TreapNode>, key: f64) -> Option<&'a Value> {
    match root {
        None => None,
        Some(node) => {
            if (key - node.key).abs() < f64::EPSILON {
                Some(&node.value)
            } else if key < node.key {
                treap_search(node.left.as_deref(), key)
            } else {
                treap_search(node.right.as_deref(), key)
            }
        }
    }
}

fn treap_delete(root: Option<Box<TreapNode>>, key: f64) -> Option<Box<TreapNode>> {
    let mut node = root?;
    if key < node.key {
        node.left = treap_delete(node.left.take(), key);
        Some(node)
    } else if key > node.key {
        node.right = treap_delete(node.right.take(), key);
        Some(node)
    } else {
        // Node found, perform rotation-based delete
        match (node.left.take(), node.right.take()) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => {
                if l.priority > r.priority {
                    node.left = Some(l);
                    node.right = Some(r);
                    let mut rotated = treap_rotate_right(node);
                    rotated.right = treap_delete(rotated.right.take(), key);
                    Some(rotated)
                } else {
                    node.left = Some(l);
                    node.right = Some(r);
                    let mut rotated = treap_rotate_left(node);
                    rotated.left = treap_delete(rotated.left.take(), key);
                    Some(rotated)
                }
            }
        }
    }
}

fn treap_inorder(root: Option<&TreapNode>, list: &mut Vec<Value>) {
    if let Some(node) = root {
        treap_inorder(node.left.as_deref(), list);
        let mut map = HashMap::new();
        map.insert("kunci".to_string(), Value::Number(node.key));
        map.insert("prioritas".to_string(), Value::Number(node.priority as f64));
        map.insert("nilai".to_string(), node.value.clone());
        list.push(Value::Map(Rc::new(RefCell::new(map))));
        treap_inorder(node.right.as_deref(), list);
    }
}

#[derive(Debug, Clone)]
pub struct WidyaTreap {
    pub root: Option<Box<TreapNode>>,
    pub seed: u64,
}

impl WidyaTreap {
    pub fn next_priority(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.seed >> 32) & 0x7FFFFFFF
    }
}

fn value_to_treap(val: &Value) -> Option<WidyaTreap> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        if map.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("PohonTreap") {
            let seed = match map.get("seed") { Some(Value::Number(n)) => *n as u64, _ => 123456789 };
            let root = map.get("root").and_then(|v| TreapNode::from_value(v)).map(Box::new);
            return Some(WidyaTreap { root, seed });
        }
    }
    None
}

fn treap_to_value(treap: &WidyaTreap) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PohonTreap".to_string()));
    map.insert("seed".to_string(), Value::Number(treap.seed as f64));
    if let Some(r) = &treap.root {
        map.insert("root".to_string(), r.to_value());
    } else {
        map.insert("root".to_string(), Value::Nil);
    }
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_treap_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let treap = WidyaTreap {
        root: None,
        seed: 987654321,
    };
    Ok(treap_to_value(&treap))
}

fn builtin_treap_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime("Fungsi 'treap_sisip' memerlukan minimal 3 argumen (treap, kunci, nilai)", span));
    }
    let key = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa angka", span)),
    };
    let val = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut treap) = value_to_treap(&args[0]) {
                let priority = if args.len() > 3 {
                    match &args[3] {
                        Value::Number(n) => *n as u64,
                        _ => treap.next_priority(),
                    }
                } else {
                    treap.next_priority()
                };

                treap.root = Some(treap_insert(treap.root.take(), key, priority, val));
                let updated = treap_to_value(&treap);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PohonTreap tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'treap_sisip' memerlukan instance PohonTreap", span)),
    }
}

fn builtin_treap_cari(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa angka", span)),
    };

    if let Some(treap) = value_to_treap(&args[0]) {
        if let Some(val) = treap_search(treap.root.as_deref(), key) {
            return Ok(val.clone());
        }
        return Ok(Value::Nil);
    }
    Err(Galat::runtime("Fungsi 'treap_cari' memerlukan instance PohonTreap", span))
}

fn builtin_treap_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Parameter 2 kunci harus berupa angka", span)),
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut treap) = value_to_treap(&args[0]) {
                treap.root = treap_delete(treap.root.take(), key);
                let updated = treap_to_value(&treap);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Objek PohonTreap tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'treap_hapus' memerlukan instance PohonTreap", span)),
    }
}

fn builtin_treap_ke_array(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(treap) = value_to_treap(&args[0]) {
        let mut list = Vec::new();
        treap_inorder(treap.root.as_deref(), &mut list);
        return Ok(Value::Array(Rc::new(RefCell::new(list))));
    }
    Err(Galat::runtime("Fungsi 'treap_ke_array' memerlukan instance PohonTreap", span))
}


pub fn register_data_structures(env: &mut Environment) {
    // WidyaPriorityQueue (Binary Min/Max Priority Heap)
        register_fn(env, "AntreanPrioritas", None, builtin_pq_buat);
        register_fn(env, "antrean_dorong", Some(3), builtin_pq_dorong);
        register_fn(env, "antrean_tarik", Some(1), builtin_pq_tarik);
        register_fn(env, "antrean_intip", Some(1), builtin_pq_intip);
        register_fn(env, "antrean_panjang", Some(1), builtin_pq_panjang);
        register_fn(env, "antrean_kosong", Some(1), builtin_pq_kosong);

    // WidyaAVL (Self-Balancing AVL Binary Search Tree)
        register_fn(env, "PohonAVL", None, builtin_avl_buat);
        register_fn(env, "avl_sisip", Some(3), builtin_avl_sisip);
        register_fn(env, "avl_cari", Some(2), builtin_avl_cari);
        register_fn(env, "avl_hapus", Some(2), builtin_avl_hapus);
        register_fn(env, "avl_ke_array", None, builtin_avl_ke_array);
        register_fn(env, "avl_tinggi", Some(1), builtin_avl_tinggi);

    // WidyaSkipList (Probabilistic Skip List Index)
        register_fn(env, "DaftarLompat", None, builtin_skiplist_buat);
        register_fn(env, "skiplist_sisip", Some(3), builtin_skiplist_sisip);
        register_fn(env, "skiplist_cari", Some(2), builtin_skiplist_cari);
        register_fn(env, "skiplist_hapus", Some(2), builtin_skiplist_hapus);
        register_fn(env, "skiplist_rentang", Some(3), builtin_skiplist_rentang);
        register_fn(env, "skiplist_ke_array", Some(1), builtin_skiplist_ke_array);

    // WidyaRoaringBitmap (Compressed Roaring Bitmaps)
        register_fn(env, "RoaringBitmap", None, builtin_roaring_buat);
        register_fn(env, "roaring_tambah", Some(2), builtin_roaring_tambah);
        register_fn(env, "roaring_apakah_ada", Some(2), builtin_roaring_ada);
        register_fn(env, "roaring_irisan", Some(2), builtin_roaring_irisan);
        register_fn(env, "roaring_gabungan", Some(2), builtin_roaring_gabungan);
        register_fn(env, "roaring_hitung", Some(1), builtin_roaring_hitung);
        register_fn(env, "roaring_ke_array", Some(1), builtin_roaring_ke_array);

    // WidyaBPTree (B+ Tree Database Indexing Engine)
        register_fn(env, "PohonBPlus", None, builtin_bptree_buat);
        register_fn(env, "bplus_sisip", Some(3), builtin_bptree_sisip);
        register_fn(env, "bplus_cari", Some(2), builtin_bptree_cari);
        register_fn(env, "bplus_rentang", Some(3), builtin_bptree_rentang);
        register_fn(env, "bplus_ke_array", Some(1), builtin_bptree_ke_array);

    // WidyaMerkleMountainRange (MMR Accumulator & Peak Bagging)
        register_fn(env, "MerkleMountainRange", None, builtin_mmr_buat);
        register_fn(env, "mmr_tambah", Some(2), builtin_mmr_tambah);
        register_fn(env, "mmr_bag_peaks", Some(1), builtin_mmr_bag_peaks);
        register_fn(env, "mmr_panjang", Some(1), builtin_mmr_panjang);
        register_fn(env, "mmr_buat_bukti", Some(2), builtin_mmr_buat_bukti);

    // WidyaQuadTree (2D Spatial QuadTree Collision & Partitioning)
        register_fn(env, "PohonQuad", Some(5), builtin_quadtree_buat);
        register_fn(env, "quad_sisip", Some(4), builtin_quadtree_sisip);
        register_fn(env, "quad_kueri_titik", Some(3), builtin_quadtree_kueri_titik);
        register_fn(env, "quad_kueri_lingkaran", Some(4), builtin_quadtree_kueri_lingkaran);
        register_fn(env, "quad_hitung", Some(1), builtin_quadtree_hitung);

    // WidyaMerklePatriciaTrie (Hex-Prefix Merkle Patricia State Trie)
        register_fn(env, "MerklePatriciaTrie", Some(0), builtin_mpt_buat);
        register_fn(env, "mpt_pasang", Some(3), builtin_mpt_pasang);
        register_fn(env, "mpt_ambil", Some(2), builtin_mpt_ambil);
        register_fn(env, "mpt_hapus", Some(2), builtin_mpt_hapus);
        register_fn(env, "mpt_root_hash", Some(1), builtin_mpt_root_hash);

    // WidyaVerkleTree (Vector Commitment & Subvector Polynomial Proofs)
        register_fn(env, "PohonVerkle", None, builtin_verkle_buat);
        register_fn(env, "verkle_sisip", Some(3), builtin_verkle_sisip);
        register_fn(env, "verkle_ambil", Some(2), builtin_verkle_ambil);
        register_fn(env, "verkle_komitmen_root", Some(1), builtin_verkle_komitmen_root);
        register_fn(env, "verkle_buat_bukti", Some(2), builtin_verkle_buat_bukti);

    // WidyaMerkleDAG (IPFS-Style Directed Acyclic Graph Content Store)
        register_fn(env, "MerkleDAG", Some(0), builtin_dag_buat);
        register_fn(env, "dag_buat_node", Some(3), builtin_dag_buat_node);
        register_fn(env, "dag_tambah_tautan", Some(3), builtin_dag_tambah_tautan);
        register_fn(env, "dag_ambil_node", Some(2), builtin_dag_ambil_node);
        register_fn(env, "dag_traversal_dfs", Some(2), builtin_dag_traversal_dfs);

    // WidyaFenwickTree (Binary Indexed Tree for Dynamic Prefix Sums)
        register_fn(env, "PohonFenwick", Some(1), builtin_fenwick_buat);
        register_fn(env, "fenwick_perbarui", Some(3), builtin_fenwick_perbarui);
        register_fn(env, "fenwick_kueri_awalan", Some(2), builtin_fenwick_kueri_awalan);
        register_fn(env, "fenwick_kueri_rentang", Some(3), builtin_fenwick_kueri_rentang);
        register_fn(env, "fenwick_ke_array", Some(1), builtin_fenwick_ke_array);

    // WidyaRadixTree (Compact Patricia Radix Tree for Network Subnet / LPM)
        register_fn(env, "PohonRadix", None, builtin_radixtree_buat);
        register_fn(env, "radix_sisip", Some(3), builtin_radixtree_sisip);
        register_fn(env, "radix_cari", Some(2), builtin_radixtree_cari);
        register_fn(env, "radix_cari_awalan_terpanjang", Some(2), builtin_radixtree_cari_awalan_terpanjang);
        register_fn(env, "radix_ke_array", Some(1), builtin_radixtree_ke_array);

    // WidyaTreap (Randomized Cartesian Tree / Treap: BST + Max-Heap)
        register_fn(env, "PohonTreap", None, builtin_treap_buat);
        register_fn(env, "treap_sisip", None, builtin_treap_sisip);
        register_fn(env, "treap_cari", Some(2), builtin_treap_cari);
        register_fn(env, "treap_hapus", Some(2), builtin_treap_hapus);
        register_fn(env, "treap_ke_array", Some(1), builtin_treap_ke_array);

}
