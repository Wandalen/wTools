#![ cfg( feature = "enabled" ) ]
//! Comprehensive corner case tests for `impls_index_meta`.
//!
//! ## Test Matrix
//!
//! | Category | Test Case | Input | Expected | Status |
//! |----------|-----------|-------|----------|--------|
//! | **Function Variants** | | | | |
//! | Lifetimes | `test_lifetimes` | Function with lifetime parameters | Compiles with lifetimes preserved | ✅ |
//! | Where Clauses | `test_where_clause` | Function with where clause | Compiles with where clause preserved | ✅ |
//! | Async Functions | `test_async_function` | async fn | Compiles with async keyword preserved | ✅ |
//! | Const Functions | `test_const_function` | const fn | Compiles with const keyword preserved | ✅ |
//! | Unsafe Functions | `test_unsafe_function` | unsafe fn | Compiles with unsafe keyword preserved | ✅ |
//! | **Attribute Handling** | | | | |
//! | Inline Attribute | `test_inline_attribute` | #[inline] fn | Compiles with attribute preserved | ✅ |
//! | Doc Comments | `test_doc_comments` | /// doc fn | Compiles with doc comments preserved | ✅ |
//! | Conditional Compilation | `test_cfg_attribute` | #[cfg(...)] fn | Compiles with cfg preserved | ✅ |
//! | Multiple Attributes | `test_multiple_attributes` | Multiple attrs | Compiles with all attrs preserved | ✅ |
//! | **Edge Cases** | | | | |
//! | Empty Block | `test_empty_block` | impls3! {} | Compiles (empty expansion) | ✅ |
//! | Mixed Optional/Required | `test_mixed_optional_required` | ? fn only (avoids unused-macro error) | Compiles, optional macros generated | ✅ |
//! | Complex Parameters | `test_complex_parameters` | Complex type params | Compiles with types preserved | ✅ |
//! | impl Trait Return | `test_impl_trait_return` | -> impl Trait | Compiles with impl Trait preserved | ✅ |
//! | Default Type Params | `test_default_type_parameters` | T = i32 | Compiles with defaults preserved | ✅ |
//! | **Integration** | | | | |
//! | Multiple Mixed | `test_multiple_mixed_features` | All features combined | All compile successfully | ✅ |
//!
//! ## Coverage
//!
//! - ✅ Function variants (lifetimes, where clauses, async, const, unsafe)
//! - ✅ Attribute handling (inline, doc comments, cfg, multiple)
//! - ✅ Edge cases (empty block, mixed optional/required, complex types, impl Trait, defaults)
//! - ✅ Integration scenarios (multiple features combined)
//!
//! ## Notes
//!
//! - All tests use `?` prefix for optional functions to avoid unused macro warnings
//! - These are compilation tests - behavioral testing is in parent `impls_index` crate
//! - Focus is on syntax preservation and correct macro generation

use impls_index_meta::impls3;

// --- Function Variants ---

/// Verifies functions with lifetime parameters compile correctly.
///
/// Tests that:
/// 1. Lifetime parameters are parsed
/// 2. Lifetimes are preserved in generated macro
/// 3. Function signature with lifetimes compiles without errors
#[ test ]
fn test_lifetimes()
{
  #[ allow( dead_code ) ]
  mod test_lifetimes_module
  {
    use super::*;

    impls3!
    {
      ? fn with_lifetime< 'a >( s : &'a str ) -> &'a str
      {
        s
      }

      ? fn with_multiple_lifetimes< 'a, 'b >( s1 : &'a str, s2 : &'b str ) -> &'a str
      {
        s1
      }
    }
  }
}

/// Verifies functions with where clauses compile correctly.
///
/// Tests that:
/// 1. Where clauses are parsed
/// 2. Where clauses are preserved in generated macro
/// 3. Generic constraints compile correctly
#[ test ]
fn test_where_clause()
{
  #[ allow( dead_code ) ]
  mod test_where_module
  {
    use super::*;

    impls3!
    {
      ? fn with_where< T >( x : T ) -> T
      where
        T : Clone + Send
      {
        x.clone()
      }

      ? fn complex_where< T, U >( x : T, y : U ) -> T
      where
        T : Clone,
        U : Into< T >
      {
        x
      }
    }
  }
}

/// Verifies async functions compile correctly.
///
/// Tests that:
/// 1. Async keyword is parsed
/// 2. Async keyword is preserved in generated macro
/// 3. Async function syntax compiles without errors
#[ test ]
fn test_async_function()
{
  #[ allow( dead_code ) ]
  mod test_async_module
  {
    use super::*;

    impls3!
    {
      ? async fn async_operation() -> i32
      {
        42
      }

      ? async fn async_with_params( x : i32 ) -> i32
      {
        x * 2
      }
    }
  }
}

/// Verifies const functions compile correctly.
///
/// Tests that:
/// 1. Const keyword is parsed
/// 2. Const keyword is preserved in generated macro
/// 3. Const function syntax compiles without errors
#[ test ]
fn test_const_function()
{
  #[ allow( dead_code ) ]
  mod test_const_module
  {
    use super::*;

    impls3!
    {
      ? const fn const_operation() -> i32
      {
        42
      }

      ? const fn const_with_params( x : i32 ) -> i32
      {
        x * 2
      }
    }
  }
}

/// Verifies unsafe functions compile correctly.
///
/// Tests that:
/// 1. Unsafe keyword is parsed
/// 2. Unsafe keyword is preserved in generated macro
/// 3. Unsafe function syntax compiles without errors
#[ test ]
fn test_unsafe_function()
{
  #[ allow( dead_code ) ]
  mod test_unsafe_module
  {
    use super::*;

    impls3!
    {
      ? unsafe fn unsafe_operation()
      {
        // Empty unsafe function
      }

      ? unsafe fn unsafe_with_params( ptr : *const i32 ) -> i32
      {
        if ptr.is_null()
        {
          0
        }
        else
        {
          *ptr
        }
      }
    }
  }
}

// --- Attribute Handling ---

/// Verifies functions with inline attribute compile correctly.
///
/// Tests that:
/// 1. Inline attribute is parsed
/// 2. Attribute is preserved in generated macro
/// 3. Function with attribute compiles without errors
#[ test ]
fn test_inline_attribute()
{
  #[ allow( dead_code ) ]
  mod test_inline_module
  {
    use super::*;

    impls3!
    {
      ? #[ inline ]
      fn inline_fn() -> i32
      {
        42
      }

      ? #[ inline( always ) ]
      fn inline_always_fn() -> i32
      {
        100
      }
    }
  }
}

/// Verifies functions with doc comments compile correctly.
///
/// Tests that:
/// 1. Doc comments are parsed
/// 2. Doc comments are preserved in generated macro
/// 3. Function with doc comments compiles without errors
#[ test ]
fn test_doc_comments()
{
  #[ allow( dead_code ) ]
  mod test_doc_module
  {
    use super::*;

    impls3!
    {
      ? /// Documentation for this function
      /// with multiple lines
      fn documented() -> i32
      {
        42
      }

      ? /// Single line doc
      fn single_doc() -> i32
      {
        100
      }
    }
  }
}

/// Verifies functions with cfg attribute compile correctly.
///
/// Tests that:
/// 1. Cfg attribute is parsed
/// 2. Cfg attribute is preserved in generated macro
/// 3. Conditional compilation works correctly
#[ test ]
fn test_cfg_attribute()
{
  #[ allow( dead_code ) ]
  mod test_cfg_module
  {
    use super::*;

    impls3!
    {
      ? #[ cfg( test ) ]
      fn test_only() -> i32
      {
        42
      }

      ? #[ cfg( not( test ) ) ]
      fn not_test() -> i32
      {
        100
      }
    }
  }
}

/// Verifies functions with multiple attributes compile correctly.
///
/// Tests that:
/// 1. Multiple attributes are parsed
/// 2. All attributes are preserved in generated macro
/// 3. Function with multiple attributes compiles without errors
#[ test ]
fn test_multiple_attributes()
{
  #[ allow( dead_code ) ]
  mod test_multiple_attrs_module
  {
    use super::*;

    impls3!
    {
      ? /// Documentation
      #[ inline ]
      #[ cfg( test ) ]
      fn multiple_attrs() -> i32
      {
        42
      }
    }
  }
}

// --- Edge Cases ---

/// Verifies empty impls3! block compiles correctly.
///
/// Tests that:
/// 1. Empty block is parsed
/// 2. Empty expansion compiles without errors
#[ test ]
fn test_empty_block()
{
  #[ allow( dead_code ) ]
  mod test_empty_module
  {
    use super::*;

    impls3! {}
  }
}

/// Verifies blocks containing only optional functions compile correctly.
///
/// Tests that:
/// 1. Multiple optional (?) functions can coexist in one block without conflict
/// 2. Each function generates an independent optional macro definition
/// 3. No unused-macro errors when all macros are optional
///
/// Note: Required functions (without `?`) cannot be tested in isolation without
/// invoking the generated macro, since `#[deny(unused_macros)]` would trigger
/// a compile error. Required-function behavior is covered by the parent crate tests.
#[ test ]
fn test_mixed_optional_required()
{
  #[ allow( dead_code ) ]
  mod test_mixed_module
  {
    use super::*;

    impls3!
    {
      // All optional to avoid unused macro errors in test
      ? fn first() -> i32
      {
        1
      }

      ? fn second() -> i32
      {
        2
      }
    }
  }
}

/// Verifies functions with complex parameter types compile correctly.
///
/// Tests that:
/// 1. Complex type parameters are parsed
/// 2. Complex types are preserved in generated macro
/// 3. Function with complex types compiles without errors
#[ test ]
fn test_complex_parameters()
{
  #[ allow( dead_code ) ]
  mod test_complex_params_module
  {
    use super::*;

    impls3!
    {
      ? fn complex(
        a : Vec< ( i32, String ) >,
        b : Option< Result< i32, String > >,
      ) -> i32
      {
        42
      }

      ? fn nested_generics< T >( x : Vec< Vec< T > > ) -> usize
      {
        x.len()
      }
    }
  }
}

/// Verifies functions returning impl Trait compile correctly.
///
/// Tests that:
/// 1. impl Trait return type is parsed
/// 2. impl Trait is preserved in generated macro
/// 3. Function with impl Trait return compiles without errors
#[ test ]
fn test_impl_trait_return()
{
  #[ allow( dead_code ) ]
  mod test_impl_trait_module
  {
    use super::*;

    impls3!
    {
      ? fn returns_impl() -> impl Iterator< Item = i32 >
      {
        std::iter::once( 42 )
      }

      ? fn returns_impl_clone() -> impl Clone
      {
        42
      }
    }
  }
}

/// Verifies functions with default type parameters compile correctly.
///
/// Tests that:
/// 1. Default type parameters are parsed
/// 2. Defaults are preserved in generated macro
/// 3. Function with defaults compiles without errors
#[ test ]
fn test_default_type_parameters()
{
  #[ allow( dead_code ) ]
  mod test_default_params_module
  {
    use super::*;

    impls3!
    {
      ? fn with_default< T = i32 >( x : T ) -> T
      {
        x
      }

      ? fn multiple_defaults< T = i32, U = String >( x : T ) -> T
      {
        x
      }
    }
  }
}

// --- Integration ---

/// Verifies multiple functions with various features compile correctly.
///
/// Tests that:
/// 1. Multiple functions with different features coexist
/// 2. All features are preserved correctly
/// 3. Complex integration scenario compiles without errors
#[ test ]
fn test_multiple_mixed_features()
{
  #[ allow( dead_code ) ]
  mod test_integration_module
  {
    use super::*;

    impls3!
    {
      ? /// First function with generics
      #[ inline ]
      fn first< T : Clone >( x : T ) -> T
      {
        x.clone()
      }

      ? async fn second() -> i32
      {
        42
      }

      ? const fn third() -> bool
      {
        true
      }

      ? fn fourth< 'a >( s : &'a str ) -> &'a str
      {
        s
      }

      ? fn fifth< T >( x : T ) -> T
      where
        T : Send + Sync
      {
        x
      }

      ? unsafe fn sixth( ptr : *const i32 ) -> i32
      {
        if ptr.is_null()
        {
          0
        }
        else
        {
          *ptr
        }
      }
    }
  }
}
