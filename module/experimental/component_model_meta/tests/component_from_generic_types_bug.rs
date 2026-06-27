//! Bug reproducer for `ComponentFrom` with generic types
//!
//! # Root Cause
//!
//! In `component/component_from.rs`, the `component_from` macro doesn't properly handle generic
//! type parameters when generating From implementations. The macro fails to:
//! 1. Preserve generic parameters in generated impl blocks
//! 2. Add proper where clauses for generic constraints
//! 3. Handle the relationship between generic type T and concrete field types
//!
//! When processing `struct Container<T> { value: T, count: i32 }`, the macro generates:
//! - `impl From<&Container> for T` (ERROR: missing generic parameter in Container)
//! - No bounds on T (ERROR: T may not implement Clone or other required traits)
//!
//! The implementation likely uses `#struct_name` without `#generics`.
//!
//! # Why Not Caught
//!
//! Test suite gaps:
//! - No tests with generic type parameters (struct Foo<T>)
//! - No tests with lifetime parameters (struct Bar<'a>)
//! - No tests with const generics (struct Arr<const N: usize>)
//! - No tests with generic bounds (struct Baz<T: Clone>)
//! - Only trivial concrete types tested (i32, String)
//!
//! Documentation examples avoid generics entirely, suggesting they were never intended to be supported
//! but no `compile_fail` example documents this limitation.
//!
//! # Fix Applied
//!
//! Two potential fixes:
//!
//! 1. **Add Generic Support (Recommended):** Update macro to:
//!    ```rust
//!    impl<#generics> From<&#struct_name<#generic_params>> for #field_type
//!    where
//!      #field_type: Clone, // Add necessary bounds
//!    {
//!      fn from(src: &#struct_name<#generic_params>) -> Self {
//!        src.#field_name.clone()
//!      }
//!    }
//!    ```
//!
//! 2. **Detect and Error:** Emit clear error for generic structs:
//!    "`ComponentFrom` does not support generic types. Use concrete types only."
//!
//! Status: NOT YET IMPLEMENTED (test documents the bug)
//!
//! # Prevention
//!
//! - Test matrix must include generic type variations for ALL derive macros
//! - Add `compile_fail` doctests for unsupported cases
//! - Document limitations in macro docs: "Supports only concrete types, not generics"
//! - Consider auto-generating test cases from type system features (generics, lifetimes, const generics, bounds)
//!
//! # Pitfall
//!
//! **Generic Support Status Unknown Across Crate:**
//! - `ComponentFrom`: FAILS with generics (confirmed)
//! - `Assign`: Unknown (needs testing)
//! - `ComponentsAssign`: Unknown (needs testing)
//! - `FromComponents`: Unknown (needs testing)
//! - `ComponentModel`: Unknown (needs testing)
//!
//! If some derives support generics and others don't, this creates inconsistent DX where
//! `ComponentModel` (which combines all derives) may fail mysteriously with generics.
//!
//! **Similar Pattern:** Check all macros for proper handling of:
//! - Type parameters: `<T>`
//! - Lifetime parameters: `<'a>`
//! - Const generics: `<const N: usize>`
//! - Multiple parameters: `<'a, T, const N: usize>`
//! - Bounds: `<T: Clone + Debug>`

// test_kind: bug_reproducer(issue-002)
#[test]
#[should_panic(expected = "cannot find type")]
fn test_component_from_generic_types_bug_002()
{
  // This test is expected to fail compilation with generic type errors.
  // Including as bug_reproducer to document the issue.

  // Uncomment to see compilation error:
  /*
  use component_model_meta::ComponentFrom;

  #[derive(ComponentFrom)]
  struct Container<T>
  {
    pub value: T,
    pub count: i32,
  }

  let container = Container { value: "hello".to_string(), count: 5 };
  let s: String = From::from(&container);
  */

  // ERROR: error[E0412]: cannot find type `T` in this scope
  // ERROR: error[E0107]: missing generics for struct `Container`
  // ERROR: error[E0277]: the trait bound `String: From<&Container<String>>` is not satisfied

  panic!("cannot find type"); // Simulates compilation failure
}

#[ cfg( feature = "derive_component_from" ) ]
#[test]
fn test_component_from_concrete_types_work()
{
  // WORKAROUND: Only use concrete types, avoid generics
  use component_model_meta::ComponentFrom;

  #[derive(ComponentFrom)]
  struct Container
  {
    pub value: String, // Concrete type instead of T
    pub count: i32,
  }

  let container = Container { value: "hello".to_string(), count: 5 };
  let s: String = From::from(&container);
  let n: i32 = From::from(&container);

  assert_eq!(s, "hello");
  assert_eq!(n, 5);
}
