//! Pattern matching, Destructuring, and ADT (Algebraic Data Types) Engine for Widya-Lang
//! Mendukung evaluasi pola bertingkat, dekonstruksi ADT (Hasil<T, G>, Opsi<T>), Exhaustiveness Check, dan Guard Expressions.

use crate::typesystem::{Type, Span};
use std::collections::HashMap;

/// Representasi nilai runtime untuk pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum PatternValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Tuple(Vec<PatternValue>),
    List(Vec<PatternValue>),
    Enum {
        enum_name: String,
        variant_name: String,
        payload: Vec<PatternValue>,
    },
    Record(HashMap<String, PatternValue>),
}

impl PatternValue {
    pub fn sukses(val: PatternValue) -> Self {
        PatternValue::Enum {
            enum_name: "Hasil".to_string(),
            variant_name: "Sukses".to_string(),
            payload: vec![val],
        }
    }

    pub fn galat(err: PatternValue) -> Self {
        PatternValue::Enum {
            enum_name: "Hasil".to_string(),
            variant_name: "Galat".to_string(),
            payload: vec![err],
        }
    }

    pub fn ada(val: PatternValue) -> Self {
        PatternValue::Enum {
            enum_name: "Opsi".to_string(),
            variant_name: "Ada".to_string(),
            payload: vec![val],
        }
    }

    pub fn kosong() -> Self {
        PatternValue::Enum {
            enum_name: "Opsi".to_string(),
            variant_name: "Kosong".to_string(),
            payload: vec![],
        }
    }
}

/// Pattern for pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard(Span),
    
    /// Literal pattern (42, "hello", benar, salah, nihil)
    Literal(PatternValue, Span),
    
    /// Variable binding pattern (x)
    Variable(String, Span),
    
    /// Constructor/Enum pattern (Sukses(x), Ada(val), Status::Aktif)
    Constructor {
        name: String,
        variant: Option<String>,
        subpatterns: Vec<Pattern>,
        span: Span,
    },
    
    /// Tuple pattern (x, y, z)
    Tuple(Vec<Pattern>, Span),
    
    /// List pattern [kepala, ..ekor]
    List {
        elements: Vec<Pattern>,
        rest: Option<Box<Pattern>>,
        span: Span,
    },
    
    /// Or pattern (A | B)
    Or(Vec<Pattern>, Span),
    
    /// Type annotation pattern (x: Angka)
    TypeAnnotation {
        pattern: Box<Pattern>,
        ty: Type,
        span: Span,
    },
}

/// Pattern matching arm dengan Guard
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<fn(&HashMap<String, PatternValue>) -> bool>,
    pub body_label: String,
    pub span: Span,
}

/// Hasil evaluasi pencocokan pola
#[derive(Debug, Clone, PartialEq)]
pub struct MatchResult {
    pub matched: bool,
    pub bindings: HashMap<String, PatternValue>,
}

pub struct PatternMatcher;

impl PatternMatcher {
    /// Evaluasi apakah `value` cocok dengan `pattern`, dan menghasilkan variabel yang diikat (*variable bindings*).
    pub fn match_value(value: &PatternValue, pattern: &Pattern) -> Option<HashMap<String, PatternValue>> {
        let mut bindings = HashMap::new();
        if Self::match_internal(value, pattern, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    fn match_internal(value: &PatternValue, pattern: &Pattern, bindings: &mut HashMap<String, PatternValue>) -> bool {
        match (value, pattern) {
            (_, Pattern::Wildcard(_)) => true,

            (val, Pattern::Variable(name, _)) => {
                bindings.insert(name.clone(), val.clone());
                true
            }

            (PatternValue::Number(a), Pattern::Literal(PatternValue::Number(b), _)) => {
                (a - b).abs() < f64::EPSILON
            }
            (PatternValue::String(a), Pattern::Literal(PatternValue::String(b), _)) => a == b,
            (PatternValue::Boolean(a), Pattern::Literal(PatternValue::Boolean(b), _)) => a == b,
            (PatternValue::Nil, Pattern::Literal(PatternValue::Nil, _)) => true,

            (PatternValue::Tuple(val_elements), Pattern::Tuple(pat_elements, _)) => {
                if val_elements.len() != pat_elements.len() {
                    return false;
                }
                for (v, p) in val_elements.iter().zip(pat_elements.iter()) {
                    if !Self::match_internal(v, p, bindings) {
                        return false;
                    }
                }
                true
            }

            (PatternValue::List(val_items), Pattern::List { elements, rest, .. }) => {
                if rest.is_none() && val_items.len() != elements.len() {
                    return false;
                }
                if val_items.len() < elements.len() {
                    return false;
                }

                for (v, p) in val_items.iter().take(elements.len()).zip(elements.iter()) {
                    if !Self::match_internal(v, p, bindings) {
                        return false;
                    }
                }

                if let Some(rest_pat) = rest {
                    let remaining = PatternValue::List(val_items[elements.len()..].to_vec());
                    if !Self::match_internal(&remaining, rest_pat, bindings) {
                        return false;
                    }
                }
                true
            }

            (
                PatternValue::Enum { enum_name: val_enum, variant_name: val_var, payload },
                Pattern::Constructor { name: pat_name, variant: pat_var, subpatterns, .. }
            ) => {
                let name_matches = match pat_var {
                    Some(v) => (val_enum == pat_name || pat_name == "_") && val_var == v,
                    None => val_var == pat_name || val_enum == pat_name,
                };

                if !name_matches {
                    return false;
                }

                if payload.len() != subpatterns.len() {
                    return false;
                }

                for (val, pat) in payload.iter().zip(subpatterns.iter()) {
                    if !Self::match_internal(val, pat, bindings) {
                        return false;
                    }
                }
                true
            }

            (val, Pattern::Or(alternatives, _)) => {
                for alt in alternatives {
                    let mut temp_bindings = bindings.clone();
                    if Self::match_internal(val, alt, &mut temp_bindings) {
                        *bindings = temp_bindings;
                        return true;
                    }
                }
                false
            }

            (val, Pattern::TypeAnnotation { pattern, .. }) => {
                Self::match_internal(val, pattern, bindings)
            }

            _ => false,
        }
    }

    /// Mengecek kesesuaian list arm dan mengeksekusi guard condition
    pub fn evaluate_arms<'a>(value: &PatternValue, arms: &'a [MatchArm]) -> Option<(&'a MatchArm, HashMap<String, PatternValue>)> {
        for arm in arms {
            if let Some(bindings) = Self::match_value(value, &arm.pattern) {
                if let Some(guard_fn) = &arm.guard {
                    if !guard_fn(&bindings) {
                        continue;
                    }
                }
                return Some((arm, bindings));
            }
        }
        None
    }

    /// Validasi kelengkapan (*Exhaustiveness Checking*) untuk tipe ADT standar
    pub fn check_exhaustiveness(arms: &[MatchArm], adt_type: &Type) -> Result<bool, Vec<String>> {
        let mut missing_variants = Vec::new();

        // Jika ada Wildcard atau Variable pattern murni di top level tanpa guard, pasti exhaustive
        for arm in arms {
            if arm.guard.is_none() {
                if matches!(arm.pattern, Pattern::Wildcard(_) | Pattern::Variable(_, _)) {
                    return Ok(true);
                }
            }
        }

        match adt_type {
            Type::Result(_, _) => {
                let mut has_sukses = false;
                let mut has_galat = false;
                for arm in arms {
                    if let Pattern::Constructor { name, variant, .. } = &arm.pattern {
                        let var_name = variant.as_ref().unwrap_or(name);
                        if var_name == "Sukses" { has_sukses = true; }
                        if var_name == "Galat" { has_galat = true; }
                    }
                }
                if !has_sukses { missing_variants.push("Sukses(_)".to_string()); }
                if !has_galat { missing_variants.push("Galat(_)".to_string()); }
            }
            Type::Option(_) => {
                let mut has_ada = false;
                let mut has_kosong = false;
                for arm in arms {
                    if let Pattern::Constructor { name, variant, .. } = &arm.pattern {
                        let var_name = variant.as_ref().unwrap_or(name);
                        if var_name == "Ada" { has_ada = true; }
                        if var_name == "Kosong" { has_kosong = true; }
                    }
                }
                if !has_ada { missing_variants.push("Ada(_) / Some(_) ".to_string()); }
                if !has_kosong { missing_variants.push("Kosong / None".to_string()); }
            }
            Type::Boolean => {
                let mut has_true = false;
                let mut has_false = false;
                for arm in arms {
                    if let Pattern::Literal(PatternValue::Boolean(b), _) = &arm.pattern {
                        if *b { has_true = true; } else { has_false = true; }
                    }
                }
                if !has_true { missing_variants.push("benar".to_string()); }
                if !has_false { missing_variants.push("salah".to_string()); }
            }
            _ => return Ok(true),
        }

        if missing_variants.is_empty() {
            Ok(true)
        } else {
            Err(missing_variants)
        }
    }
}

/// ADT Utilities
pub struct ADTUtils;

impl ADTUtils {
    pub fn is_irrefutable(pattern: &Pattern) -> bool {
        matches!(pattern, Pattern::Wildcard(_) | Pattern::Variable(_, _))
    }

    pub fn result_type(ok_type: String, err_type: String) -> String {
        format!("Hasil[{}, {}]", ok_type, err_type)
    }

    pub fn option_type(inner_type: String) -> String {
        format!("Opsi[{}]", inner_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_matching() {
        let span = Span { start: 0, end: 0, line: 1, column: 1 };
        let val = PatternValue::Number(42.0);
        let pat = Pattern::Literal(PatternValue::Number(42.0), span.clone());
        let pat_wrong = Pattern::Literal(PatternValue::Number(99.0), span.clone());

        assert!(PatternMatcher::match_value(&val, &pat).is_some());
        assert!(PatternMatcher::match_value(&val, &pat_wrong).is_none());
    }

    #[test]
    fn test_variable_and_wildcard_matching() {
        let span = Span { start: 0, end: 0, line: 1, column: 1 };
        let val = PatternValue::String("Widya".to_string());
        
        let wildcard = Pattern::Wildcard(span.clone());
        assert!(PatternMatcher::match_value(&val, &wildcard).is_some());

        let var_pat = Pattern::Variable("nama".to_string(), span.clone());
        let bindings = PatternMatcher::match_value(&val, &var_pat).unwrap();
        assert_eq!(bindings.get("nama"), Some(&PatternValue::String("Widya".to_string())));
    }

    #[test]
    fn test_adt_matching() {
        let span = Span { start: 0, end: 0, line: 1, column: 1 };
        let val_sukses = PatternValue::sukses(PatternValue::Number(100.0));
        let pat_sukses = Pattern::Constructor {
            name: "Hasil".to_string(),
            variant: Some("Sukses".to_string()),
            subpatterns: vec![Pattern::Variable("nilai".to_string(), span.clone())],
            span: span.clone(),
        };

        let bindings = PatternMatcher::match_value(&val_sukses, &pat_sukses).unwrap();
        assert_eq!(bindings.get("nilai"), Some(&PatternValue::Number(100.0)));
    }

    #[test]
    fn test_exhaustiveness_checking() {
        let span = Span { start: 0, end: 0, line: 1, column: 1 };
        let arms_complete = vec![
            MatchArm {
                pattern: Pattern::Constructor {
                    name: "Hasil".to_string(),
                    variant: Some("Sukses".to_string()),
                    subpatterns: vec![Pattern::Wildcard(span.clone())],
                    span: span.clone(),
                },
                guard: None,
                body_label: "ok".to_string(),
                span: span.clone(),
            },
            MatchArm {
                pattern: Pattern::Constructor {
                    name: "Hasil".to_string(),
                    variant: Some("Galat".to_string()),
                    subpatterns: vec![Pattern::Wildcard(span.clone())],
                    span: span.clone(),
                },
                guard: None,
                body_label: "err".to_string(),
                span: span.clone(),
            },
        ];

        let result_type = Type::Result(Box::new(Type::Number), Box::new(Type::String));
        assert!(PatternMatcher::check_exhaustiveness(&arms_complete, &result_type).is_ok());

        let arms_incomplete = vec![arms_complete[0].clone()];
        let exhaust_res = PatternMatcher::check_exhaustiveness(&arms_incomplete, &result_type);
        assert!(exhaust_res.is_err());
        assert_eq!(exhaust_res.unwrap_err(), vec!["Galat(_)".to_string()]);
    }
}
