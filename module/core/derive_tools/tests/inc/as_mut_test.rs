//! ## Test Matrix for `AsMut`
//!
//! | ID   | Struct Type        | Implementation | Expected Behavior                                           | Test File                   |
//! |------|--------------------|----------------|-------------------------------------------------------------|-----------------------------|
//! | T2.1 | Tuple struct (1 field) | `#[derive(AsMut)]` | `.as_mut()` returns a mutable reference to the inner field. | `as_mut_test.rs`            |
//! | T2.2 | Tuple struct (1 field) | Manual `impl`  | `.as_mut()` returns a mutable reference to the inner field. | `as_mut_manual_test.rs`     |
use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, the_module::AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_mut.rs" );
