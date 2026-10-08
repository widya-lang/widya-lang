#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod wpm;
pub mod wpm_resolver;

pub use wpm::*;
pub use wpm_resolver::{PackageSpec, ResolvedGraph};
