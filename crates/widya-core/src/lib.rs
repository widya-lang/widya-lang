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
