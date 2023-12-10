//! Low-level bindings for the [Bitwuzla] SMT solver.
//!
//! Please see the Bitwuzla [C API documentation] for function descriptions.
//!
//! [Bitwuzla]: https://bitwuzla.github.io/
//! [C API documentation]: https://bitwuzla.github.io/docs/c/api.html

#![allow(non_upper_case_globals)]

#[cfg(feature = "vendor")]
extern crate link_cplusplus;

include!("../src-generated/bindings.rs");
