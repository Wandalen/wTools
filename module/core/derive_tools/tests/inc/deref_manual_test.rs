//! ## Test Matrix for `Deref`
//!
//! | ID   | Struct Type        | Inner Type | Implementation | Expected Behavior                                       | Test File                   |
//! |------|--------------------|------------|----------------|---------------------------------------------------------|-----------------------------|
//! | T5.1 | Tuple struct (1 field) | `i32`      | `#[derive(Deref)]` | Dereferencing returns a reference to the inner `i32`. | `deref_test.rs`             |
//! | T5.2 | Tuple struct (1 field) | `i32`      | Manual `impl`  | Dereferencing returns a reference to the inner `i32`. | `deref_manual_test.rs`      |
//! | T5.3 | Named struct (1 field) | `String`   | `#[derive(Deref)]` | Dereferencing returns a reference to the inner `String`. | `deref_test.rs`             |
//! | T5.4 | Named struct (1 field) | `String`   | Manual `impl`  | Dereferencing returns a reference to the inner `String`. | `deref_manual_test.rs`      |
include!( "./only_test/deref.rs" );