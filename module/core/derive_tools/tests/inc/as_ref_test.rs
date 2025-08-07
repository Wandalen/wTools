//! ## Test Matrix for `AsRef`
//!
//! | ID   | Struct Type        | Implementation | Expected Behavior                                       | Test File                   |
//! |------|--------------------|----------------|---------------------------------------------------------|-----------------------------|
//! | T3.1 | Tuple struct (1 field) | `#[derive(AsRef)]` | `.as_ref()` returns a reference to the inner field. | `as_ref_test.rs`            |
//! | T3.2 | Tuple struct (1 field) | Manual `impl`  | `.as_ref()` returns a reference to the inner field. | `as_ref_manual_test.rs`     |
use test_tools::a_id;
use crate::the_module;
use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[derive(Debug, Clone, Copy, PartialEq, the_module::AsRef)]
pub struct IsTransparent(bool);

include!("./only_test/as_ref.rs");
