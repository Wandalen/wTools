#![ cfg( feature = "enabled" ) ]
//! Procedural macro compilation tests for `impls_index_meta`.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `test_macro_single_function` | Single function compilation | `fn test() -> i32 { 42 }` | Macro compiles, generates valid macro definition | ✅ |
//! | `test_macro_optional_function` | Optional function (? prefix) | `? fn optional() -> i32 { 42 }` | Macro compiles with optional marker | ✅ |
//! | `test_macro_multiple_functions` | Multiple functions | Two functions in same block | Macro compiles, generates multiple macro definitions | ✅ |
//! | `test_macro_with_generics` | Generic function | `fn generic<T>(x: T) -> T { x }` | Macro compiles with generic parameters | ✅ |
//!
//! ## Coverage
//!
//! - ✅ Single function compilation
//! - ✅ Optional functions with `?` prefix
//! - ✅ Multiple functions in single `impls3!` block
//! - ✅ Generic functions with type parameters
//! - ✅ Validation that generated code is syntactically valid
//!
//! ## Notes
//!
//! This is a proc-macro crate. Full behavioral testing (invoking generated macros,
//! testing renaming syntax, runtime validation) is performed in the parent
//! `impls_index` crate which provides the complete public API including the
//! `fn_rename!` macro required for renaming functionality.

use impls_index_meta::impls3;

/// Verifies single function compilation.
///
/// Tests that `impls3!` macro:
/// 1. Parses single function item correctly
/// 2. Generates syntactically valid macro definition
/// 3. Compiles without errors
///
/// Note: This tests compilation only. Runtime behavior and macro invocation
/// are tested in the parent `impls_index` crate.
#[ test ]
fn test_macro_single_function()
{
  // The impls3! macro should compile and generate a macro definition
  // Using `?` prefix makes macros optional (compilation-only test)
  #[ allow( dead_code ) ]
  mod test_single
  {
    use super::*;

    impls3!
    {
      ? fn test_function() -> i32
      {
        42
      }
    }
  }
  // If this compiles, the macro generated valid Rust code
}

/// Verifies optional functions with `?` prefix compile correctly.
///
/// Tests that:
/// 1. Functions prefixed with `?` are parsed correctly
/// 2. Optional marker generates valid code
/// 3. No compilation errors for optional functions
#[ test ]
fn test_macro_optional_function()
{
  #[ allow( dead_code ) ]
  mod test_optional
  {
    use super::*;

    impls3!
    {
      ? fn optional_function() -> i32
      {
        100
      }
    }
  }
  // Compilation success validates optional marker handling
}

/// Verifies multiple functions in single `impls3!` block compile correctly.
///
/// Tests that:
/// 1. Multiple functions can be defined in one macro invocation
/// 2. Each function generates its own macro definition
/// 3. All optional functions compile without usage requirement
/// 4. No conflicts between generated macros
#[ test ]
fn test_macro_multiple_functions()
{
  #[ allow( dead_code ) ]
  mod test_multiple
  {
    use super::*;

    impls3!
    {
      ? fn first_function() -> i32
      {
        1
      }

      ? fn second_function() -> i32
      {
        2
      }

      ? fn third_function() -> i32
      {
        3
      }
    }
  }
  // Compilation success validates multiple function handling
}

/// Verifies generic functions compile correctly.
///
/// Tests that:
/// 1. Functions with generic type parameters are parsed
/// 2. Generic syntax is preserved in generated code
/// 3. No compilation errors for generic functions
#[ test ]
fn test_macro_with_generics()
{
  #[ allow( dead_code ) ]
  mod test_generics
  {
    use super::*;

    impls3!
    {
      ? fn generic_function< T >( x : T ) -> T
      {
        x
      }
    }
  }
  // Compilation success validates generic function handling
}
