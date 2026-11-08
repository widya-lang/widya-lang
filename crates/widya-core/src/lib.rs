#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod ast;
pub mod token;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod typesystem;
pub mod patternmatching;
pub mod typeclasses;
pub mod typeparser;
pub mod borrow_checker;
pub mod value;
pub mod environment;
pub mod bare_metal;

// Re-export key types at crate root for convenience
pub use ast::*;
pub use token::{Token, TokenType};
pub use lexer::Lexer;
pub use parser::Parser;
pub use error::{Galat, Span};
pub use typesystem::Type;
pub use borrow_checker::BorrowChecker;
pub use value::{Value, WidyaFunction, BuiltinFunction, WidyaStruct};
pub use environment::Environment;
pub use bare_metal::{
    AlokatorTumpukanStatik, AntarmukaAlokator, Irisan, Kotak, LarikDinamis, LayoutMemori,
    PenunjukMentah, TanganKepanikan,
};

#[cfg(test)]
mod bare_metal_tests {
    use super::*;

    #[test]
    fn test_freestanding_raw_pointer_volatile_access() {
        let mut hardware_reg: u32 = 0xDEADBEEF;
        let ptr = PenunjukMentah { ptr: &mut hardware_reg as *mut u32 };

        assert_eq!(ptr.is_null(), false);
        unsafe {
            let val = ptr.baca_volatil();
            assert_eq!(val, 0xDEADBEEF);

            ptr.tulis_volatil(0xCAFEBABE);
            assert_eq!(ptr.baca_volatil(), 0xCAFEBABE);
        }
        assert_eq!(hardware_reg, 0xCAFEBABE);
    }

    #[test]
    fn test_freestanding_slice_zero_allocation() {
        let array = [10i64, 20, 30, 40, 50];
        let ptr = PenunjukMentah { ptr: array.as_ptr() as *mut i64 };
        let slice = Irisan::dari_pointer_mentah(ptr, array.len());

        assert_eq!(slice.len(), 5);
        assert_eq!(slice.is_empty(), false);
        assert_eq!(slice.ambil(0), Some(&10));
        assert_eq!(slice.ambil(4), Some(&50));
        assert_eq!(slice.ambil(5), None);
    }

    #[test]
    fn test_freestanding_static_bump_allocator() {
        let mut allocator = AlokatorTumpukanStatik::<1024>::baru();
        assert_eq!(allocator.kapasitas(), 1024);
        assert_eq!(allocator.terpakai(), 0);

        let layout_i32 = LayoutMemori::dari_tipe::<i32>();
        unsafe {
            let p1 = allocator.alokasi(layout_i32).expect("Alokasi 1 harus berhasil");
            assert_eq!(allocator.terpakai() >= 4, true);

            let layout_i64 = LayoutMemori::dari_tipe::<i64>();
            let p2 = allocator.alokasi(layout_i64).expect("Alokasi 2 harus berhasil");
            // Harus diselaraskan ke batas 8 byte
            assert_eq!(p2.alamat() % 8, 0);

            let val_ptr = p2.ptr as *mut i64;
            std::ptr::write(val_ptr, 999_888_777i64);
            assert_eq!(*val_ptr, 999_888_777i64);
        }

        allocator.reset();
        assert_eq!(allocator.terpakai(), 0);
    }

    #[test]
    fn test_freestanding_box_and_vec_with_custom_allocator() {
        let mut allocator = AlokatorTumpukanStatik::<4096>::baru();

        // 1. Kotak (Box)
        {
            let mut k = Kotak::baru_in(123456789i64, &mut allocator).expect("Alokasi Kotak");
            assert_eq!(*k.nilai(), 123456789i64);
            *k.nilai_mut() = 987654321i64;
            assert_eq!(*k.nilai(), 987654321i64);
        }

        // 2. LarikDinamis (Vec)
        {
            let mut vec = LarikDinamis::<u32, _>::baru_in(&mut allocator, 8).expect("Alokasi Vec");
            assert_eq!(vec.is_empty(), true);

            vec.push(100).unwrap();
            vec.push(200).unwrap();
            vec.push(300).unwrap();

            assert_eq!(vec.len(), 3);
            assert_eq!(vec.get(0), Some(&100));
            assert_eq!(vec.get(1), Some(&200));
            assert_eq!(vec.get(2), Some(&300));
            assert_eq!(vec.get(3), None);
        }
    }
}
