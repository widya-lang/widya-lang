use crate::ast::*;
use crate::error::{Galat, Span};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum OwnershipState {
    Owned,
    Moved(Span),
    BorrowedImmutable { count: usize, span: Span, end_point: Option<usize> },
    BorrowedMutable(Span),
}

/// Region Peminjaman Non-Lexical Lifetime (NLL)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NllLoanRegion {
    pub variable: String,
    pub start_point: usize,
    pub last_used_point: usize,
    pub is_mutable: bool,
}

/// Static Ownership & Non-Lexical Lifetimes (NLL) Borrow Checker
pub struct BorrowChecker {
    variables: HashMap<String, OwnershipState>,
    field_borrows: HashMap<(String, String), OwnershipState>,
    errors: Vec<Galat>,
    loan_regions: Vec<NllLoanRegion>,
    current_cfg_point: usize,
    live_variables: HashSet<String>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            field_borrows: HashMap::new(),
            errors: Vec::new(),
            loan_regions: Vec::new(),
            current_cfg_point: 0,
            live_variables: HashSet::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Galat> {
        self.variables.clear();
        self.field_borrows.clear();
        self.errors.clear();
        self.loan_regions.clear();
        self.current_cfg_point = 0;
        self.live_variables.clear();

        for stmt in &program.statements {
            self.current_cfg_point += 1;
            self.check_stmt(stmt);
        }

        if let Some(first_err) = self.errors.first() {
            Err(first_err.clone())
        } else {
            Ok(())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, initializer, .. } => {
                if let Some(init) = initializer {
                    self.check_expr(init);
                }
                self.variables.insert(name.clone(), OwnershipState::Owned);
                self.live_variables.insert(name.clone());
            }
            Stmt::Assignment { target, value, span, .. } => {
                self.check_expr(value);
                if let AssignTarget::Variable(name) = target {
                    // Update NLL borrow termination
                    self.update_nll_regions(self.current_cfg_point);

                    if let Some(state) = self.variables.get(name) {
                        match state {
                            OwnershipState::BorrowedImmutable { end_point, .. } => {
                                if let Some(end) = end_point {
                                    if self.current_cfg_point <= *end {
                                        self.errors.push(Galat::runtime(
                                            format!("Galat Keamanan Memori: Tidak dapat memutasi '{}' karena sedang dipinjam secara imutabel", name),
                                            span,
                                        ));
                                    }
                                } else {
                                    self.errors.push(Galat::runtime(
                                        format!("Galat Keamanan Memori: Tidak dapat memutasi '{}' karena sedang dipinjam secara imutabel", name),
                                        span,
                                    ));
                                }
                            }
                            OwnershipState::Moved(_) => {
                                self.errors.push(Galat::runtime(
                                    format!("Galat Keamanan Memori: Tidak dapat menggunakan variabel '{}' yang sudah dipindahkan (moved)", name),
                                    span,
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Stmt::FunctionDecl { params, body, .. } => {
                let mut inner_checker = BorrowChecker::new();
                for p in params {
                    inner_checker.variables.insert(p.clone(), OwnershipState::Owned);
                }
                for s in body {
                    inner_checker.current_cfg_point += 1;
                    inner_checker.check_stmt(s);
                }
                self.errors.extend(inner_checker.errors);
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.current_cfg_point += 1;
                    self.check_stmt(s);
                }
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, .. } => {
                self.check_expr(condition);
                self.check_stmt(then_branch);
                for (cond, b) in elif_branches {
                    self.check_expr(cond);
                    self.check_stmt(b);
                }
                if let Some(eb) = else_branch {
                    self.check_stmt(eb);
                }
            }
            Stmt::While { condition, body, .. } => {
                self.check_expr(condition);
                self.check_stmt(body);
            }
            Stmt::ForIn { iterable, body, .. } => {
                self.check_expr(iterable);
                self.check_stmt(body);
            }
            Stmt::Expression(expr) => {
                self.check_expr(expr);
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name, span) => {
                if let Some(OwnershipState::Moved(_)) = self.variables.get(name) {
                    self.errors.push(Galat::runtime(
                        format!("Galat Keamanan Memori (Borrow Checker): Penggunaan variabel '{}' setelah dipindahkan (use-after-move)", name),
                        span,
                    ));
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::Call { callee, arguments, .. } => {
                self.check_expr(callee);
                for arg in arguments {
                    self.check_expr(arg);
                }
            }
            Expr::Array(items, _) => {
                for item in items {
                    self.check_expr(item);
                }
            }
            _ => {}
        }
    }

    /// Update status peminjaman berdasarkan titik CFG saat ini (NLL Early Expiry)
    pub fn update_nll_regions(&mut self, current_point: usize) {
        let mut expired = Vec::new();
        for region in &self.loan_regions {
            if current_point > region.last_used_point {
                expired.push(region.variable.clone());
            }
        }

        for var in expired {
            self.release_borrow(&var);
        }
    }

    /// Registrasi peminjaman NLL dengan titik akhir penggunaan (last used point)
    pub fn borrow_immutable_nll(&mut self, name: &str, span: Span, last_used_point: usize) -> Result<(), Galat> {
        let current = self.current_cfg_point;
        self.loan_regions.push(NllLoanRegion {
            variable: name.to_string(),
            start_point: current,
            last_used_point,
            is_mutable: false,
        });

        match self.variables.get(name) {
            Some(OwnershipState::BorrowedMutable(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker (Aliasing XOR Mutability): Variabel '{}' tidak dapat dipinjam imutabel saat sedang dipinjam secara mutabel (&mut)", name),
                    &span,
                ))
            }
            Some(OwnershipState::Moved(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Tidak dapat meminjam variabel '{}' yang nilainya sudah dipindahkan (moved)", name),
                    &span,
                ))
            }
            Some(OwnershipState::BorrowedImmutable { count, span: orig_span, .. }) => {
                let current_count = *count;
                let orig = orig_span.clone();
                self.variables.insert(name.to_string(), OwnershipState::BorrowedImmutable { count: current_count + 1, span: orig, end_point: Some(last_used_point) });
                Ok(())
            }
            _ => {
                self.variables.insert(name.to_string(), OwnershipState::BorrowedImmutable { count: 1, span, end_point: Some(last_used_point) });
                Ok(())
            }
        }
    }

    /// Explicitly mark a variable as moved (ownership transfer)
    pub fn mark_moved(&mut self, name: &str, span: Span) {
        self.variables.insert(name.to_string(), OwnershipState::Moved(span));
    }

    /// Borrow variable immutably (Multiple readers allowed)
    pub fn borrow_immutable(&mut self, name: &str, span: Span) -> Result<(), Galat> {
        self.borrow_immutable_nll(name, span, usize::MAX)
    }

    /// Borrow variable mutably (Exclusive single writer)
    pub fn borrow_mutable(&mut self, name: &str, span: Span) -> Result<(), Galat> {
        match self.variables.get(name) {
            Some(OwnershipState::BorrowedMutable(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Variabel '{}' tidak dapat dipinjam secara mutabel lebih dari satu kali secara bersamaan", name),
                    &span,
                ))
            }
            Some(OwnershipState::BorrowedImmutable { count, .. }) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Variabel '{}' sedang dipinjam imutabel oleh {} pembaca, tidak dapat dipinjam secara mutabel", name, count),
                    &span,
                ))
            }
            Some(OwnershipState::Moved(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Tidak dapat meminjam variabel '{}' yang nilainya sudah dipindahkan (moved)", name),
                    &span,
                ))
            }
            _ => {
                self.variables.insert(name.to_string(), OwnershipState::BorrowedMutable(span));
                Ok(())
            }
        }
    }

    /// Release borrow (Scope exit / Drop)
    pub fn release_borrow(&mut self, name: &str) {
        if let Some(state) = self.variables.get(name) {
            match state {
                OwnershipState::BorrowedImmutable { count, span, .. } => {
                    if *count > 1 {
                        let new_count = *count - 1;
                        let s = span.clone();
                        self.variables.insert(name.to_string(), OwnershipState::BorrowedImmutable { count: new_count, span: s, end_point: None });
                    } else {
                        self.variables.insert(name.to_string(), OwnershipState::Owned);
                    }
                }
                OwnershipState::BorrowedMutable(_) => {
                    self.variables.insert(name.to_string(), OwnershipState::Owned);
                }
                _ => {}
            }
        }
    }

    /// Meminjam field tertentu dari sebuah objek secara imutabel (Disjoint Field-Sensitive Borrow)
    pub fn borrow_field_immutable(&mut self, root: &str, field: &str, span: Span) -> Result<(), Galat> {
        // Cek apakah seluruh objek induk sedang dipinjam secara mutabel
        if let Some(OwnershipState::BorrowedMutable(_)) = self.variables.get(root) {
            return Err(Galat::runtime(
                format!("Galat Borrow Checker: Tidak dapat meminjam field '{}.{}' karena seluruh objek '{}' sedang dipinjam secara mutabel", root, field, root),
                &span,
            ));
        }

        let key = (root.to_string(), field.to_string());
        if let Some(OwnershipState::BorrowedMutable(_)) = self.field_borrows.get(&key) {
            return Err(Galat::runtime(
                format!("Galat Borrow Checker (Aliasing XOR Mutability): Field '{}.{}' sedang dipinjam secara mutabel (&mut)", root, field),
                &span,
            ));
        }

        let current_count = match self.field_borrows.get(&key) {
            Some(OwnershipState::BorrowedImmutable { count, .. }) => *count,
            _ => 0,
        };

        self.field_borrows.insert(key, OwnershipState::BorrowedImmutable {
            count: current_count + 1,
            span,
            end_point: None,
        });

        Ok(())
    }

    /// Meminjam field tertentu dari sebuah objek secara mutabel (Disjoint Field Mutation)
    pub fn borrow_field_mutable(&mut self, root: &str, field: &str, span: Span) -> Result<(), Galat> {
        // Cek apakah seluruh objek induk sedang dipinjam
        if let Some(state) = self.variables.get(root) {
            match state {
                OwnershipState::BorrowedMutable(_) | OwnershipState::BorrowedImmutable { .. } => {
                    return Err(Galat::runtime(
                        format!("Galat Borrow Checker: Tidak dapat meminjam mutabel field '{}.{}' karena seluruh objek '{}' sedang dipinjam", root, field, root),
                        &span,
                    ));
                }
                OwnershipState::Moved(_) => {
                    return Err(Galat::runtime(
                        format!("Galat Borrow Checker: Tidak dapat meminjam field dari objek '{}' yang sudah dipindahkan", root),
                        &span,
                    ));
                }
                _ => {}
            }
        }

        let key = (root.to_string(), field.to_string());
        if let Some(state) = self.field_borrows.get(&key) {
            match state {
                OwnershipState::BorrowedMutable(_) => {
                    return Err(Galat::runtime(
                        format!("Galat Borrow Checker: Field '{}.{}' sudah dipinjam secara mutabel", root, field),
                        &span,
                    ));
                }
                OwnershipState::BorrowedImmutable { .. } => {
                    return Err(Galat::runtime(
                        format!("Galat Borrow Checker: Field '{}.{}' tidak dapat dipinjam mutabel saat sedang dipinjam imutabel", root, field),
                        &span,
                    ));
                }
                _ => {}
            }
        }

        self.field_borrows.insert(key, OwnershipState::BorrowedMutable(span));
        Ok(())
    }

    /// Melepas peminjaman field tertentu
    pub fn release_field_borrow(&mut self, root: &str, field: &str) {
        let key = (root.to_string(), field.to_string());
        if let Some(OwnershipState::BorrowedImmutable { count, span, end_point }) = self.field_borrows.get(&key).cloned() {
            if count > 1 {
                self.field_borrows.insert(key, OwnershipState::BorrowedImmutable { count: count - 1, span, end_point });
            } else {
                self.field_borrows.remove(&key);
            }
        } else {
            self.field_borrows.remove(&key);
        }
    }

    pub fn is_field_borrowed(&self, root: &str, field: &str) -> bool {
        self.field_borrows.contains_key(&(root.to_string(), field.to_string()))
    }

    /// Check that all functions with #[profil("X")] attribute are compatible with active_profile
    pub fn check_profile_context(&mut self, program: &Program, active_profile: Option<&str>) -> Result<(), Galat> {
        let active = match active_profile {
            Some(p) => p,
            None => return Ok(()),
        };

        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, attributes, span, .. } = stmt {
                for attr in attributes {
                    if attr.name == "profil" {
                        for required_profile in &attr.arguments {
                            if required_profile != active {
                                return Err(Galat::runtime(
                                    format!(
                                        "GALAT: Fungsi '{}' hanya tersedia di profil '{}', profil aktif '{}'",
                                        name, required_profile, active
                                    ),
                                    span,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
