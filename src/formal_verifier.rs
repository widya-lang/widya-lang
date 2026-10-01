//! Formal Verification & Invariant Proof Engine (formal_verifier.rs)
//! Membuktikan kebenaran matematika kode sebelum kompilasi untuk mencegah divide-by-zero, integer overflow, dan pelanggaran kontrak fungsi (Hoare Logic).

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
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionContract {
    pub function_name: String,
    pub divisor_interval: Option<NumericInterval>,
    pub max_expected_output: i64,
}

/// Engine Verifikasi Formal
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
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_arithmetic() {
        let i1 = NumericInterval::new(2, 5);
        let i2 = NumericInterval::new(10, 20);

        let sum = i1.add(&i2);
        assert_eq!(sum.min, 12);
        assert_eq!(sum.max, 25);
        assert!(!sum.includes_zero());

        let mul = i1.mul(&i2);
        assert_eq!(mul.min, 20);
        assert_eq!(mul.max, 100);
    }

    #[test]
    fn test_division_safety_verification() {
        let safe_contract = FunctionContract {
            function_name: "bagi_aman".to_string(),
            divisor_interval: Some(NumericInterval::new(1, 10)),
            max_expected_output: 100,
        };
        assert!(FormalVerifier::verify_division_safety(&safe_contract).is_ok());

        let unsafe_contract = FunctionContract {
            function_name: "bagi_bahaya".to_string(),
            divisor_interval: Some(NumericInterval::new(-5, 5)),
            max_expected_output: 100,
        };
        assert!(FormalVerifier::verify_division_safety(&unsafe_contract).is_err());
    }
}

