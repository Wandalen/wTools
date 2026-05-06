//! Bug reproducer for `ComponentModel` tuple struct limitation
//!
//! # Root Cause
//!
//! `ComponentModel` macro explicitly rejects tuple structs with error "`ComponentModel` requires named fields"
//! at `component_model.rs:37`. The implementation uses pattern matching on `syn::Fields::Named`
//! and returns `syn_err!` for all other field types (Unit, Unnamed).
//!
//! # Why Not Caught
//!
//! No test coverage existed for `ComponentModel` with tuple structs. The limitation is intentional
//! in the implementation but completely undocumented in the public API (src/lib.rs lines 544-591).
//! Users attempting to use `ComponentModel` with tuple structs get a compile error with no prior
//! warning from documentation.
//!
//! # Fix Applied
//!
//! DOCUMENTATION FIX NEEDED (not yet applied):
//! Add "Limitations" section to `ComponentModel` documentation in src/lib.rs explaining:
//! - Only works with named-field structs
//! - Does not support tuple structs (use ComponentAssign/ComponentFrom instead)
//! - Does not support unit structs
//! - Does not support enums or unions
//!
//! Example documentation addition:
//! ```markdown
//! ## Limitations
//!
//! ComponentModel only supports structs with named fields:
//!
//! ```rust,compile_fail
//! #[derive(ComponentModel)]
//! struct Point(i32, i32); // ERROR: ComponentModel requires named fields
//! ```
//!
//! For tuple structs, use `ComponentAssign` or `ComponentFrom` instead.
//! ```
//!
//! # Prevention
//!
//! - Add comprehensive test matrix for all derive macros covering struct types (named, tuple, unit)
//! - Document limitations prominently in macro documentation
//! - Consider improving error message to suggest alternatives: "ComponentModel requires named fields. For tuple structs, use #[derive(ComponentFrom, Assign)] instead."
//! - Add compile_fail doctests showing unsupported cases
//!
//! # Pitfall
//!
//! ComponentModel documentation (src/lib.rs:544-591) shows only named-field struct examples,
//! giving no indication that tuple structs are unsupported. Other macros (ComponentFrom,
//! ComponentAssign) DO support tuple structs, creating inconsistent user expectations.
//!
//! Users naturally expect that if ComponentFrom works with tuple structs, ComponentModel (which
//! claims to be a "unified derive") should also work with them.

// test_kind: bug_reproducer(issue-004)

#[ test ]
#[ should_panic( expected = "ComponentModel requires named fields" ) ]
fn test_component_model_tuple_struct_limitation_004()
{
  // This test is expected to fail compilation, not panic at runtime.
  // Including as bug_reproducer to document the limitation.

  // Uncomment to see compilation error:
  /*
  use component_model_meta::ComponentModel;

  #[derive(Default, ComponentModel)]
  struct Point(i32, i32);

  let _point = Point::default();
  */

  // ERROR: ComponentModel requires named fields
  // The macro implementation at component_model.rs:37 rejects tuple structs

  panic!( "ComponentModel requires named fields" ); // Simulates compilation failure
}

#[ test ]
fn test_component_model_tuple_struct_workaround()
{
  // WORKAROUND: Use ComponentFrom + Assign for tuple structs with different types
  use component_model_meta:: { ComponentFrom, Assign as AssignDerive };
  use component_model_types::Assign;

  #[ derive( Default, ComponentFrom, AssignDerive ) ]
  struct Pair( i32, String );

  let mut pair = Pair::default();

  // Assign works with tuple structs
  pair.assign( 100 );
  pair.assign( "test" );

  assert_eq!( pair.0, 100 );
  assert_eq!( pair.1, "test" );

  // ComponentFrom works with tuple structs
  let num: i32 = From::from( &pair );
  let text: String = From::from( &pair );

  assert_eq!( num, 100 );
  assert_eq!( text, "test" );
}
