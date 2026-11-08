//! Industrial MIR-Based Borrow Checker for Widya-Lang
//! Menegakkan kepemilikan statis, Non-Lexical Lifetimes (NLL), dan teorema Aliasing XOR Mutability
//! langsung di atas Control Flow Graph (CFG) MIR selevel rustc_borrowck.

use crate::mir::*;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Titik instruksi presisi dalam CFG MIR (Blok dan Indeks Pernyataan)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MirLocation {
    pub block: usize,
    pub statement_index: usize,
}

impl fmt::Display for MirLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}[{}]", self.block, self.statement_index)
    }
}

/// Identitas pinjaman aktif (Loan)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loan {
    pub id: usize,
    pub holder: Place,
    pub place: Place,
    pub kind: BorrowKind,
    pub issued_at: MirLocation,
    pub active_until: Option<MirLocation>,
}

/// Kode Galat Borrow Checker Standar Industri
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirBorrowckErrorKind {
    /// E0501: Cannot borrow as mutable more than once at a time
    DoubleMutBorrow {
        place: String,
        first_issued: MirLocation,
        conflicting: MirLocation,
    },
    /// E0502: Cannot borrow as mutable because it is also borrowed as immutable
    MutBorrowWhileShared {
        place: String,
        shared_issued: MirLocation,
        mut_attempt: MirLocation,
    },
    /// E0503: Cannot use or mutate place while borrowed
    UseWhileBorrowed {
        place: String,
        loan_issued: MirLocation,
        use_point: MirLocation,
    },
    /// E0382: Use of moved value
    UseOfMovedValue {
        place: String,
        moved_at: MirLocation,
        used_at: MirLocation,
    },
    /// E0505: Cannot move out of place because it is borrowed
    MoveWhileBorrowed {
        place: String,
        loan_issued: MirLocation,
        move_point: MirLocation,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirBorrowckError {
    pub code: &'static str,
    pub message: String,
    pub location: MirLocation,
    pub kind: MirBorrowckErrorKind,
}

impl fmt::Display for MirBorrowckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Galat [{}]: {} pada {}", self.code, self.message, self.location)
    }
}

/// Hasil Audit Keselamatan Memori MIR
#[derive(Debug, Clone)]
pub struct MirBorrowckReport {
    pub function_name: String,
    pub total_loans_analyzed: usize,
    pub total_cfg_points: usize,
    pub active_loans: Vec<Loan>,
    pub errors: Vec<MirBorrowckError>,
}

impl MirBorrowckReport {
    pub fn is_safe(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Penganalisis Keselamatan Memori MIR (MIR Borrow Checker)
pub struct MirBorrowChecker<'a> {
    body: &'a MirBody,
    next_loan_id: usize,
    loans: Vec<Loan>,
    errors: Vec<MirBorrowckError>,
    /// Status pemindahan nilai (Moved Places per Local Name)
    moved_places: HashMap<String, MirLocation>,
}

impl<'a> MirBorrowChecker<'a> {
    pub fn new(body: &'a MirBody) -> Self {
        Self {
            body,
            next_loan_id: 1,
            loans: Vec::new(),
            errors: Vec::new(),
            moved_places: HashMap::new(),
        }
    }

    /// Menjalankan verifikasi keselamatan memori penuh
    pub fn check(mut self) -> MirBorrowckReport {
        let mut total_points = 0;

        // Hitung liveness sederhana untuk Non-Lexical Lifetimes (NLL)
        let last_use_map = self.compute_last_uses();

        // Iterasi teratur sepanjang graf alur kendali MIR
        for bb in &self.body.basic_blocks {
            for (idx, stmt) in bb.statements.iter().enumerate() {
                let loc = MirLocation {
                    block: bb.id,
                    statement_index: idx,
                };
                total_points += 1;
                self.check_statement(stmt, loc, &last_use_map);
            }

            if let Some(term) = &bb.terminator {
                let term_loc = MirLocation {
                    block: bb.id,
                    statement_index: bb.statements.len(),
                };
                total_points += 1;
                self.check_terminator(term, term_loc, &last_use_map);
            }
        }

        MirBorrowckReport {
            function_name: self.body.name.clone(),
            total_loans_analyzed: self.loans.len(),
            total_cfg_points: total_points,
            active_loans: self.loans,
            errors: self.errors,
        }
    }

    /// Menghitung titik penggunaan terakhir (last-use point) untuk setiap lokal
    /// sebagai dasar penghentian pinjaman NLL (Non-Lexical Lifetimes)
    fn compute_last_uses(&self) -> HashMap<String, MirLocation> {
        let mut last_uses = HashMap::new();

        for bb in &self.body.basic_blocks {
            for (idx, stmt) in bb.statements.iter().enumerate() {
                let loc = MirLocation {
                    block: bb.id,
                    statement_index: idx,
                };
                match stmt {
                    MirStatement::Assign(_, rvalue) => {
                        self.collect_rvalue_uses(rvalue, loc, &mut last_uses);
                    }
                    _ => {}
                }
            }

            if let Some(term) = &bb.terminator {
                let loc = MirLocation {
                    block: bb.id,
                    statement_index: bb.statements.len(),
                };
                match term {
                    MirTerminator::SwitchInt { discr, .. } => {
                        if let Some(p) = discr.place() {
                            last_uses.insert(p.base_name().to_string(), loc);
                        }
                    }
                    MirTerminator::Call { args, .. } => {
                        for arg in args {
                            if let Some(p) = arg.place() {
                                last_uses.insert(p.base_name().to_string(), loc);
                            }
                        }
                    }
                    MirTerminator::Drop { place, .. } => {
                        last_uses.insert(place.base_name().to_string(), loc);
                    }
                    _ => {}
                }
            }
        }

        last_uses
    }

    fn collect_rvalue_uses(
        &self,
        rvalue: &Rvalue,
        loc: MirLocation,
        uses: &mut HashMap<String, MirLocation>,
    ) {
        match rvalue {
            Rvalue::Use(op) => {
                if let Some(p) = op.place() {
                    uses.insert(p.base_name().to_string(), loc);
                }
            }
            Rvalue::Ref(_, place) => {
                uses.insert(place.base_name().to_string(), loc);
            }
            Rvalue::BinaryOp(_, lhs, rhs) => {
                if let Some(p) = lhs.place() {
                    uses.insert(p.base_name().to_string(), loc);
                }
                if let Some(p) = rhs.place() {
                    uses.insert(p.base_name().to_string(), loc);
                }
            }
            Rvalue::UnaryNot(op) | Rvalue::UnaryNeg(op) => {
                if let Some(p) = op.place() {
                    uses.insert(p.base_name().to_string(), loc);
                }
            }
            Rvalue::Discriminant(p) => {
                uses.insert(p.base_name().to_string(), loc);
            }
            Rvalue::Aggregate(_, ops) => {
                for op in ops {
                    if let Some(p) = op.place() {
                        uses.insert(p.base_name().to_string(), loc);
                    }
                }
            }
        }
    }

    fn check_statement(
        &mut self,
        stmt: &MirStatement,
        loc: MirLocation,
        last_uses: &HashMap<String, MirLocation>,
    ) {
        match stmt {
            MirStatement::Assign(dest_place, rvalue) => {
                // Periksa pembacaan operand pada rvalue
                self.check_rvalue(rvalue, loc, last_uses);

                // Periksa penulisan ke dest_place: Tidak boleh menulis jika sedang dipinjam
                let dest_base = dest_place.base_name();

                // Bersihkan pinjaman NLL yang telah kedaluwarsa sebelum evaluasi penulisan
                self.expire_dead_loans(loc, last_uses);

                for loan in &self.loans {
                    if loan.place.overlaps_with(dest_place) {
                        self.errors.push(MirBorrowckError {
                            code: "E0503",
                            message: format!(
                                "Tidak dapat mengubah atau menetapkan '{}' karena sedang dipinjam oleh {}",
                                dest_place, loan.issued_at
                            ),
                            location: loc,
                            kind: MirBorrowckErrorKind::UseWhileBorrowed {
                                place: dest_place.to_string(),
                                loan_issued: loan.issued_at,
                                use_point: loc,
                            },
                        });
                        break;
                    }
                }

                // Jika dest_place sebelumnya telah dipindahkan (moved), penugasan ulang menginisialisasi kembali
                self.moved_places.remove(dest_base);

                // Jika rvalue adalah Ref, catat pinjaman baru
                if let Rvalue::Ref(kind, place) = rvalue {
                    self.register_loan(dest_place.clone(), place.clone(), *kind, loc);
                }
            }
            MirStatement::StorageDead(local) => {
                // Saat variabel mati, batalkan seluruh pinjaman terkait
                self.loans.retain(|l| l.place.base_name() != local.name && l.holder.base_name() != local.name);
                self.moved_places.remove(&local.name);
            }
            _ => {}
        }
    }

    fn check_rvalue(
        &mut self,
        rvalue: &Rvalue,
        loc: MirLocation,
        last_uses: &HashMap<String, MirLocation>,
    ) {
        match rvalue {
            Rvalue::Use(op) => self.check_operand(op, loc),
            Rvalue::BinaryOp(_, lhs, rhs) => {
                self.check_operand(lhs, loc);
                self.check_operand(rhs, loc);
            }
            Rvalue::UnaryNot(op) | Rvalue::UnaryNeg(op) => {
                self.check_operand(op, loc);
            }
            Rvalue::Aggregate(_, ops) => {
                for op in ops {
                    self.check_operand(op, loc);
                }
            }
            Rvalue::Discriminant(p) => {
                self.check_use_of_place(p, loc);
            }
            Rvalue::Ref(kind, place) => {
                self.check_new_borrow_conflicts(place, *kind, loc);
            }
        }
    }

    fn check_operand(&mut self, op: &Operand, loc: MirLocation) {
        match op {
            Operand::Copy(place) => {
                self.check_use_of_place(place, loc);
            }
            Operand::Move(place) => {
                // Periksa apakah sudah dipindahkan sebelumnya
                self.check_use_of_place(place, loc);

                // Periksa apakah sedang dipinjam saat akan dipindahkan (Move while borrowed)
                for loan in &self.loans {
                    if loan.place.overlaps_with(place) {
                        self.errors.push(MirBorrowckError {
                            code: "E0505",
                            message: format!(
                                "Tidak dapat memindahkan '{}' ke variabel lain karena sedang dipinjam oleh {}",
                                place, loan.issued_at
                            ),
                            location: loc,
                            kind: MirBorrowckErrorKind::MoveWhileBorrowed {
                                place: place.to_string(),
                                loan_issued: loan.issued_at,
                                move_point: loc,
                            },
                        });
                        break;
                    }
                }

                // Catat kepemilikan telah berpindah
                self.moved_places.insert(place.base_name().to_string(), loc);
            }
            Operand::Constant(_) => {}
        }
    }

    fn check_use_of_place(&mut self, place: &Place, loc: MirLocation) {
        let base = place.base_name();
        if let Some(moved_at) = self.moved_places.get(base) {
            self.errors.push(MirBorrowckError {
                code: "E0382",
                message: format!(
                    "Penggunaan nilai yang telah dipindahkan (moved): '{}' telah dipindahkan pada {}",
                    place, moved_at
                ),
                location: loc,
                kind: MirBorrowckErrorKind::UseOfMovedValue {
                    place: place.to_string(),
                    moved_at: *moved_at,
                    used_at: loc,
                },
            });
        }
    }

    fn check_new_borrow_conflicts(&mut self, place: &Place, kind: BorrowKind, loc: MirLocation) {
        // Cek apakah variabel sumber sudah pernah dipindahkan
        self.check_use_of_place(place, loc);

        for loan in &self.loans {
            if loan.place.overlaps_with(place) {
                match (loan.kind, kind) {
                    (BorrowKind::Mut, BorrowKind::Mut) => {
                        self.errors.push(MirBorrowckError {
                            code: "E0501",
                            message: format!(
                                "Tidak dapat meminjam '{}' sebagai mutabel lebih dari sekali pada waktu yang sama. Pinjaman pertama terjadi pada {}",
                                place, loan.issued_at
                            ),
                            location: loc,
                            kind: MirBorrowckErrorKind::DoubleMutBorrow {
                                place: place.to_string(),
                                first_issued: loan.issued_at,
                                conflicting: loc,
                            },
                        });
                    }
                    (BorrowKind::Shared, BorrowKind::Mut) => {
                        self.errors.push(MirBorrowckError {
                            code: "E0502",
                            message: format!(
                                "Tidak dapat meminjam '{}' sebagai mutabel karena saat ini juga sedang dipinjam sebagai imutabel oleh {}",
                                place, loan.issued_at
                            ),
                            location: loc,
                            kind: MirBorrowckErrorKind::MutBorrowWhileShared {
                                place: place.to_string(),
                                shared_issued: loan.issued_at,
                                mut_attempt: loc,
                            },
                        });
                    }
                    (BorrowKind::Mut, BorrowKind::Shared) => {
                        self.errors.push(MirBorrowckError {
                            code: "E0502",
                            message: format!(
                                "Tidak dapat meminjam '{}' sebagai imutabel karena saat ini sedang dipinjam secara eksklusif (mutabel) oleh {}",
                                place, loan.issued_at
                            ),
                            location: loc,
                            kind: MirBorrowckErrorKind::MutBorrowWhileShared {
                                place: place.to_string(),
                                shared_issued: loan.issued_at,
                                mut_attempt: loc,
                            },
                        });
                    }
                    (BorrowKind::Shared, BorrowKind::Shared) => {
                        // Diizinkan: Aliasing XOR Mutability (bisa banyak pembaca imutabel sekaligus)
                    }
                }
            }
        }
    }

    fn register_loan(&mut self, holder: Place, target: Place, kind: BorrowKind, loc: MirLocation) {
        let loan = Loan {
            id: self.next_loan_id,
            holder,
            place: target,
            kind,
            issued_at: loc,
            active_until: None,
        };
        self.next_loan_id += 1;
        self.loans.push(loan);
    }

    fn expire_dead_loans(&mut self, current_loc: MirLocation, last_uses: &HashMap<String, MirLocation>) {
        // NLL: Pinjaman aktif selama pemegang referensi (holder) masih akan digunakan
        self.loans.retain(|l| {
            if let Some(last_use) = last_uses.get(l.holder.base_name()) {
                last_use >= &current_loc
            } else {
                false
            }
        });
    }

    fn check_terminator(
        &mut self,
        term: &MirTerminator,
        loc: MirLocation,
        last_uses: &HashMap<String, MirLocation>,
    ) {
        match term {
            MirTerminator::SwitchInt { discr, .. } => {
                self.check_operand(discr, loc);
            }
            MirTerminator::Call { args, destination, .. } => {
                for arg in args {
                    self.check_operand(arg, loc);
                }
                self.moved_places.remove(destination.base_name());
            }
            MirTerminator::Drop { place, .. } => {
                self.check_use_of_place(place, loc);
            }
            _ => {}
        }
    }
}
