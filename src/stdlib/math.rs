use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn builtin_akar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => {
            if n < 0.0 {
                Err(Galat::runtime("Tidak dapat menghitung akar kuadrat dari bilangan negatif", span))
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'akar' memerlukan argumen angka", span)),
    }
}

pub fn builtin_pangkat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        _ => Err(Galat::runtime("Fungsi 'pangkat' memerlukan dua angka (basis, eksponen)", span)),
    }
}

pub fn builtin_sin(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.sin())),
        _ => Err(Galat::runtime("Fungsi 'sin' memerlukan argumen angka (radian)", span)),
    }
}

pub fn builtin_cos(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.cos())),
        _ => Err(Galat::runtime("Fungsi 'cos' memerlukan argumen angka (radian)", span)),
    }
}

pub fn builtin_tan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.tan())),
        _ => Err(Galat::runtime("Fungsi 'tan' memerlukan argumen angka (radian)", span)),
    }
}

pub fn builtin_mutlak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(Galat::runtime("Fungsi 'mutlak' memerlukan argumen angka", span)),
    }
}

pub fn builtin_bulat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(Galat::runtime("Fungsi 'bulat' memerlukan argumen angka", span)),
    }
}

pub fn builtin_lantai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(Galat::runtime("Fungsi 'lantai' memerlukan argumen angka", span)),
    }
}

pub fn builtin_atap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(Galat::runtime("Fungsi 'atap' memerlukan argumen angka", span)),
    }
}

pub fn builtin_acak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut rng = rand::thread_rng();
    if args.is_empty() {
        Ok(Value::Number(rng.r#gen::<f64>()))
    } else if args.len() == 2 {
        match (&args[0], &args[1]) {
            (Value::Number(min), Value::Number(max)) => {
                if min > max {
                    return Err(Galat::runtime("Nilai minimum acak tidak boleh lebih besar dari maksimum", span));
                }
                if min.fract() == 0.0 && max.fract() == 0.0 {
                    let min_i = *min as i64;
                    let max_i = *max as i64;
                    let val = rng.gen_range(min_i..=max_i);
                    Ok(Value::Number(val as f64))
                } else {
                    let val = rng.gen_range(*min..=*max);
                    Ok(Value::Number(val))
                }
            }
            _ => Err(Galat::runtime("Fungsi 'acak(min, max)' memerlukan dua angka", span)),
        }
    } else {
        Err(Galat::runtime("Fungsi 'acak' menerima 0 atau 2 argumen: acak() atau acak(min, max)", span))
    }
}

pub fn builtin_maks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
        _ => Err(Galat::runtime("Fungsi 'maks' memerlukan dua angka", span)),
    }
}

pub fn builtin_min(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
        _ => Err(Galat::runtime("Fungsi 'min' memerlukan dua angka", span)),
    }
}

// ==============================================================================
// WidyaDecimal: Arbitrary-Precision Fixed-Point Financial Math
// ==============================================================================
#[derive(Clone, Copy, Debug)]
pub struct FixedDecimal {
    pub units: i128,
    pub scale: u32,
}

impl FixedDecimal {
    pub fn from_str(s: &str, target_scale: Option<u32>) -> Option<Self> {
        let s = s.trim();
        let (sign, s) = if let Some(rest) = s.strip_prefix('-') {
            (-1i128, rest)
        } else {
            (1i128, s.strip_prefix('+').unwrap_or(s))
        };
        let parts: Vec<&str> = s.split('.').collect();
        let int_part = parts[0].parse::<i128>().ok()?;

        let (frac_part_str, frac_len) = if parts.len() > 1 {
            (parts[1], parts[1].len() as u32)
        } else {
            ("", 0u32)
        };

        let scale = target_scale.unwrap_or(frac_len.max(2));
        let mut frac_val = 0i128;
        if !frac_part_str.is_empty() {
            let mut digits = frac_part_str.to_string();
            if digits.len() > scale as usize {
                digits.truncate(scale as usize);
            } else {
                while digits.len() < scale as usize {
                    digits.push('0');
                }
            }
            frac_val = digits.parse::<i128>().ok()?;
        }

        let multiplier = 10i128.pow(scale);
        let total_units = sign * (int_part * multiplier + frac_val);
        Some(FixedDecimal {
            units: total_units,
            scale,
        })
    }

    pub fn to_string_exact(&self) -> String {
        let divisor = 10i128.pow(self.scale);
        let int_part = self.units.abs() / divisor;
        let frac_part = self.units.abs() % divisor;
        let sign = if self.units < 0 { "-" } else { "" };
        format!(
            "{}{}.{:0width$}",
            sign,
            int_part,
            frac_part,
            width = self.scale as usize
        )
    }

    pub fn align(&self, target_scale: u32) -> FixedDecimal {
        if self.scale == target_scale {
            *self
        } else if self.scale < target_scale {
            let factor = 10i128.pow(target_scale - self.scale);
            FixedDecimal {
                units: self.units * factor,
                scale: target_scale,
            }
        } else {
            let factor = 10i128.pow(self.scale - target_scale);
            FixedDecimal {
                units: self.units / factor,
                scale: target_scale,
            }
        }
    }
}

pub fn decimal_to_val(d: FixedDecimal) -> Value {
    let mut map = HashMap::new();
    let teks = d.to_string_exact();
    let val_f64 = (d.units as f64) / (10i128.pow(d.scale) as f64);
    map.insert("_tipe".to_string(), Value::String("Desimal".to_string()));
    map.insert("unit".to_string(), Value::Number(d.units as f64));
    map.insert("skala".to_string(), Value::Number(d.scale as f64));
    map.insert("teks".to_string(), Value::String(teks));
    map.insert("angka".to_string(), Value::Number(val_f64));
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn val_to_decimal(v: &Value) -> Option<FixedDecimal> {
    match v {
        Value::Map(m) => {
            let map = m.borrow();
            let units = match map.get("unit") {
                Some(Value::Number(n)) => *n as i128,
                _ => 0,
            };
            let scale = match map.get("skala") {
                Some(Value::Number(n)) => *n as u32,
                _ => 2,
            };
            Some(FixedDecimal { units, scale })
        }
        Value::String(s) => FixedDecimal::from_str(s, None),
        Value::Number(n) => FixedDecimal::from_str(&format!("{:.4}", n), None),
        _ => None,
    }
}

pub fn builtin_decimal_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi Desimal memerlukan nilai string atau angka", span)),
    };
    let scale = args.get(1).and_then(|v| {
        if let Value::Number(n) = v {
            Some(*n as u32)
        } else {
            None
        }
    });
    let dec = FixedDecimal::from_str(&s, scale).ok_or_else(|| {
        Galat::runtime(format!("Format angka desimal tidak valid: {}", s), span)
    })?;
    Ok(decimal_to_val(dec))
}

pub fn builtin_decimal_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d1 = val_to_decimal(&args[0])
        .ok_or_else(|| Galat::runtime("Argumen 1 bukan Desimal valid", span))?;
    let d2 = val_to_decimal(&args[1])
        .ok_or_else(|| Galat::runtime("Argumen 2 bukan Desimal valid", span))?;
    let target_scale = d1.scale.max(d2.scale);
    let a1 = d1.align(target_scale);
    let a2 = d2.align(target_scale);
    Ok(decimal_to_val(FixedDecimal {
        units: a1.units + a2.units,
        scale: target_scale,
    }))
}

pub fn builtin_decimal_kurang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d1 = val_to_decimal(&args[0])
        .ok_or_else(|| Galat::runtime("Argumen 1 bukan Desimal valid", span))?;
    let d2 = val_to_decimal(&args[1])
        .ok_or_else(|| Galat::runtime("Argumen 2 bukan Desimal valid", span))?;
    let target_scale = d1.scale.max(d2.scale);
    let a1 = d1.align(target_scale);
    let a2 = d2.align(target_scale);
    Ok(decimal_to_val(FixedDecimal {
        units: a1.units - a2.units,
        scale: target_scale,
    }))
}

pub fn builtin_decimal_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d1 = val_to_decimal(&args[0])
        .ok_or_else(|| Galat::runtime("Argumen 1 bukan Desimal valid", span))?;
    let d2 = val_to_decimal(&args[1])
        .ok_or_else(|| Galat::runtime("Argumen 2 bukan Desimal valid", span))?;
    let raw_units = d1.units * d2.units;
    let target_scale = d1.scale.max(d2.scale);
    let total_scale = d1.scale + d2.scale;
    let divisor = 10i128.pow(total_scale - target_scale);
    Ok(decimal_to_val(FixedDecimal {
        units: raw_units / divisor,
        scale: target_scale,
    }))
}

pub fn builtin_decimal_bagi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d1 = val_to_decimal(&args[0])
        .ok_or_else(|| Galat::runtime("Argumen 1 bukan Desimal valid", span))?;
    let d2 = val_to_decimal(&args[1])
        .ok_or_else(|| Galat::runtime("Argumen 2 bukan Desimal valid", span))?;
    if d2.units == 0 {
        return Err(Galat::runtime("Pembagian desimal dengan angka nol", span));
    }
    let target_scale = args
        .get(2)
        .and_then(|v| {
            if let Value::Number(n) = v {
                Some(*n as u32)
            } else {
                None
            }
        })
        .unwrap_or(d1.scale.max(d2.scale));
    let factor = 10i128.pow(target_scale + d2.scale);
    let scaled_numerator = (d1.units * factor) / 10i128.pow(d1.scale);
    let res_units = scaled_numerator / d2.units;
    Ok(decimal_to_val(FixedDecimal {
        units: res_units,
        scale: target_scale,
    }))
}

pub fn builtin_decimal_format_rupiah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d = val_to_decimal(&args[0]).ok_or_else(|| {
        Galat::runtime("Fungsi desimal_format_rupiah memerlukan objek Desimal", span)
    })?;
    let is_neg = d.units < 0;
    let abs_d = FixedDecimal {
        units: d.units.abs(),
        scale: d.scale,
    };
    let divisor = 10i128.pow(abs_d.scale);
    let int_part = abs_d.units / divisor;
    let frac_part = abs_d.units % divisor;

    let int_str = int_part.to_string();
    let mut formatted_int = String::new();
    let chars: Vec<char> = int_str.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            formatted_int.push('.');
        }
        formatted_int.push(c);
    }

    let out = format!(
        "{}Rp {},{:02}",
        if is_neg { "-" } else { "" },
        formatted_int,
        frac_part
    );
    Ok(Value::String(out))
}

pub fn builtin_decimal_ke_string(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d = val_to_decimal(&args[0]).ok_or_else(|| {
        Galat::runtime("Fungsi desimal_ke_string memerlukan objek Desimal", span)
    })?;
    Ok(Value::String(d.to_string_exact()))
}

pub fn register_math(env: &mut Environment) {
    register_fn(env, "akar", Some(1), builtin_akar);
    register_fn(env, "pangkat", Some(2), builtin_pangkat);
    register_fn(env, "sin", Some(1), builtin_sin);
    register_fn(env, "cos", Some(1), builtin_cos);
    register_fn(env, "tan", Some(1), builtin_tan);
    register_fn(env, "mutlak", Some(1), builtin_mutlak);
    register_fn(env, "bulat", Some(1), builtin_bulat);
    register_fn(env, "lantai", Some(1), builtin_lantai);
    register_fn(env, "atap", Some(1), builtin_atap);
    register_fn(env, "acak", None, builtin_acak);
    register_fn(env, "maks", Some(2), builtin_maks);
    register_fn(env, "min", Some(2), builtin_min);

    // Constants
    env.define("PI".to_string(), Value::Number(std::f64::consts::PI), true);
    env.define("E".to_string(), Value::Number(std::f64::consts::E), true);

    // Fixed-point decimals
    register_fn(env, "desimal_buat", None, builtin_decimal_buat);
    register_fn(env, "desimal_tambah", Some(2), builtin_decimal_tambah);
    register_fn(env, "desimal_kurang", Some(2), builtin_decimal_kurang);
    register_fn(env, "desimal_kali", Some(2), builtin_decimal_kali);
    register_fn(env, "desimal_bagi", None, builtin_decimal_bagi);
    register_fn(env, "Desimal", None, builtin_decimal_buat);
    register_fn(env, "desimal_format_rupiah", Some(1), builtin_decimal_format_rupiah);
    register_fn(env, "desimal_ke_string", Some(1), builtin_decimal_ke_string);
}
