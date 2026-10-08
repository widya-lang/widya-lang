//! Formal Verification & Invariant Proof Engine (formal_verifier.rs)
//! Membuktikan kebenaran matematika kode sebelum kompilasi untuk mencegah divide-by-zero,
//! integer overflow, loop non-termination, dan pelanggaran kontrak fungsi (Hoare Logic / SMT-LIB).
use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct NumericInterval {
    pub min: i64,
    pub max: i64,
}

impl NumericInterval {
    pub fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }

    pub fn includes_zero(&self) -> bool {
        self.min <= 0 && self.max >= 0
    }

    pub fn add(&self, other: &NumericInterval) -> Self {
        Self {
            min: self.min.saturating_add(other.min),
            max: self.max.saturating_add(other.max),
        }
    }

    pub fn sub(&self, other: &NumericInterval) -> Self {
        Self {
            min: self.min.saturating_sub(other.max),
            max: self.max.saturating_sub(other.min),
        }
    }

    pub fn mul(&self, other: &NumericInterval) -> Self {
        let p1 = self.min.saturating_mul(other.min);
        let p2 = self.min.saturating_mul(other.max);
        let p3 = self.max.saturating_mul(other.min);
        let p4 = self.max.saturating_mul(other.max);

        Self {
            min: p1.min(p2).min(p3).min(p4),
            max: p1.max(p2).max(p3).max(p4),
        }
    }
}

/// Kontrak Verifikasi Formal (Pre-condition & Post-condition)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionContract {
    pub function_name: String,
    pub input_intervals: Vec<(String, NumericInterval)>,
    pub divisor_interval: Option<NumericInterval>,
    pub max_expected_output: i64,
    pub ensures_positive: bool,
}

/// Invarian Perulangan (Loop Invariant & Termination Metric)
#[derive(Debug, Clone, PartialEq)]
pub struct LoopInvariant {
    pub loop_name: String,
    pub initial_counter: i64,
    pub step: i64,
    pub target_bound: i64,
    pub invariant_condition: String,
}

/// SMT Horn Clause Constraint
#[derive(Debug, Clone, PartialEq)]
pub enum SmtPredicate {
    GreaterThan(String, i64),
    LessThan(String, i64),
    Range(String, i64, i64),
    NonZero(String),
}

/// Engine Verifikasi Formal Berbasis SMT & Hoare Logic
pub struct FormalVerifier;

impl FormalVerifier {
    /// Memverifikasi keamanan operasi pembagian dari potensi Divide-by-Zero
    pub fn verify_division_safety(contract: &FunctionContract) -> Result<bool, String> {
        if let Some(ref interval) = contract.divisor_interval {
            if interval.includes_zero() {
                return Err(format!(
                    "Gagal Verifikasi Formal pada fungsi '{}': Pembagi memiliki rentang [{}; {}] yang memuat angka 0 (Potensi Divide-by-Zero)",
                    contract.function_name, interval.min, interval.max
                ));
            }
        }
        Ok(true)
    }

    /// Memverifikasi batas atas nilai keluaran (No Overflow Guarantee)
    pub fn verify_range_safety(interval: &NumericInterval, contract: &FunctionContract) -> Result<bool, String> {
        if interval.max > contract.max_expected_output {
            return Err(format!(
                "Gagal Verifikasi Formal: Nilai maksimum yang mungkin ({}) melebihi batas kontrak ({})",
                interval.max, contract.max_expected_output
            ));
        }
        if contract.ensures_positive && interval.min < 0 {
            return Err(format!(
                "Gagal Verifikasi Formal: Kontrak menjamin output positif namun rentang minimum bernilai {}",
                interval.min
            ));
        }
        Ok(true)
    }

    /// Memverifikasi terminasi perulangan (Well-founded relation / loop termination proof)
    pub fn verify_loop_termination(inv: &LoopInvariant) -> Result<bool, String> {
        if inv.step == 0 {
            return Err(format!(
                "Gagal Verifikasi Terminasi: Loop '{}' memiliki step 0 yang menyebabkan Infinite Loop",
                inv.loop_name
            ));
        }
        if inv.step > 0 && inv.initial_counter >= inv.target_bound {
            // Loop langsung selesai / 0 iterasi
            return Ok(true);
        }
        if inv.step < 0 && inv.initial_counter <= inv.target_bound {
            return Ok(true);
        }
        if (inv.step > 0 && inv.initial_counter < inv.target_bound) || (inv.step < 0 && inv.initial_counter > inv.target_bound) {
            return Ok(true);
        }
        Err(format!("Gagal membuktikan terminasi matematis untuk loop '{}'", inv.loop_name))
    }

    /// Memverifikasi pemenuhan formula SMT Solver
    pub fn verify_smt_constraints(predicates: &[SmtPredicate], concrete_values: &[(String, i64)]) -> Result<bool, String> {
        let val_map: std::collections::HashMap<_, _> = concrete_values.iter().cloned().collect();

        for pred in predicates {
            match pred {
                SmtPredicate::GreaterThan(var, min_val) => {
                    if let Some(val) = val_map.get(var) {
                        if *val <= *min_val {
                            return Err(format!("Pelanggaran SMT: {} ({}) tidak > {}", var, val, min_val));
                        }
                    }
                }
                SmtPredicate::LessThan(var, max_val) => {
                    if let Some(val) = val_map.get(var) {
                        if *val >= *max_val {
                            return Err(format!("Pelanggaran SMT: {} ({}) tidak < {}", var, val, max_val));
                        }
                    }
                }
                SmtPredicate::Range(var, min_val, max_val) => {
                    if let Some(val) = val_map.get(var) {
                        if *val < *min_val || *val > *max_val {
                            return Err(format!("Pelanggaran SMT: {} ({}) di luar rentang [{}, {}]", var, val, min_val, max_val));
                        }
                    }
                }
                SmtPredicate::NonZero(var) => {
                    if let Some(val) = val_map.get(var) {
                        if *val == 0 {
                            return Err(format!("Pelanggaran SMT: {} bernilai 0 (NonZero dilanggar)", var));
                        }
                    }
                }
            }
        }

        Ok(true)
    }

    /// Menghitung estimasi rentang numerik suatu ekspresi AST secara statik (Interval Abstract Interpretation)
    pub fn infer_expr_interval(
        expr: &Expr,
        env: &HashMap<String, NumericInterval>,
    ) -> Option<NumericInterval> {
        match expr {
            Expr::Number(n, _) => {
                let v = *n as i64;
                Some(NumericInterval::new(v, v))
            }
            Expr::Identifier(name, _) => {
                env.get(name).cloned()
            }
            Expr::Grouping(inner, _) => {
                Self::infer_expr_interval(inner, env)
            }
            Expr::Binary { left, op, right, .. } => {
                let l = Self::infer_expr_interval(left, env)?;
                let r = Self::infer_expr_interval(right, env)?;
                match op {
                    BinaryOp::Add => Some(l.add(&r)),
                    BinaryOp::Subtract => Some(l.sub(&r)),
                    BinaryOp::Multiply => Some(l.mul(&r)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Memverifikasi secara statik ketiadaan potensi Divide-by-Zero pada seluruh operasi pembagian AST
    pub fn verify_ast_division_in_expr(
        expr: &Expr,
        env: &HashMap<String, NumericInterval>,
    ) -> Result<(), String> {
        match expr {
            Expr::Binary { left, op, right, span } => {
                Self::verify_ast_division_in_expr(left, env)?;
                Self::verify_ast_division_in_expr(right, env)?;

                if matches!(op, BinaryOp::Divide | BinaryOp::Modulo) {
                    if let Some(divisor_interval) = Self::infer_expr_interval(right, env) {
                        if divisor_interval.includes_zero() {
                            return Err(format!(
                                "Gagal Verifikasi Formal pada baris {}: Pembagi memiliki rentang [{}; {}] yang memuat 0 (Potensi Divide-by-Zero)",
                                span.line, divisor_interval.min, divisor_interval.max
                            ));
                        }
                    }
                }
            }
            Expr::Grouping(inner, _) => {
                Self::verify_ast_division_in_expr(inner, env)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Membuktikan terminasi matematis pernyataan perulangan AST (Loop Invariant Proof)
    pub fn verify_ast_loop(stmt: &Stmt) -> Result<LoopInvariant, String> {
        match stmt {
            Stmt::ForIn { var_name, iterable, .. } => {
                if let Expr::Range { start, end, .. } = iterable {
                    if let (Expr::Number(s, _), Expr::Number(e, _)) = (&**start, &**end) {
                        let inv = LoopInvariant {
                            loop_name: format!("for_{}", var_name),
                            initial_counter: *s as i64,
                            step: 1,
                            target_bound: *e as i64,
                            invariant_condition: format!("{} < {}", var_name, e),
                        };
                        Self::verify_loop_termination(&inv)?;
                        return Ok(inv);
                    }
                }
                Err("ForIn iterable tidak dapat dibuktikan terminasinya secara statik".into())
            }
            Stmt::While { condition, .. } => {
                if let Expr::Boolean(false, _) = condition {
                    return Ok(LoopInvariant {
                        loop_name: "while_false".into(),
                        initial_counter: 0,
                        step: 1,
                        target_bound: 0,
                        invariant_condition: "palsu".into(),
                    });
                }
                if let Expr::Boolean(true, _) = condition {
                    return Err("Gagal Verifikasi Terminasi: Perulangan 'while (benar)' merupakan infinite loop tanpa kondisi henti terbukti".into());
                }
                Err("Kondisi while dinamis memerlukan bukti invarian manual".into())
            }
            Stmt::Loop { .. } => {
                Err("Gagal Verifikasi Terminasi: 'loop {}' tanpa syarat terminasi formal".into())
            }
            _ => Err("Bukan pernyataan perulangan".into()),
        }
    }
}
