//! # Test Module Structure and Coverage Outline
//!
//! This module aggregates various test suites for the `former` crate and its associated derive macros.
//! Below is an outline of the features tested and their corresponding test modules within this directory.
//!
//! ## Feature Coverage Outline:
//!
//! - **Former Derive for Structs**
//!   - **Basic Functionality:**
//!     - Simple struct definition and forming
//!     - Primitive types
//!     - Optional types
//!     - Tuple structs
//!     - User-defined types (with Default, without Default, without Debug)
//!     - Unsigned primitive types
//!   - **Collections Handling:**
//!     - Basic scalar setters for collections
//!     - Standard collections (Vec, HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, BinaryHeap)
//!     - Collection interface traits
//!   - **Subform Setters:**
//!     - `#[subform_collection]` (implicit, explicit definition, named, custom, setter on/off)
//!     - `#[subform_entry]` (implicit, manual, named, setter on/off, HashMap specific)
//!     - `#[subform_scalar]` (implicit, manual, named)
//!     - Combinations of subform attributes on a single field
//!   - **Attributes:**
//!     - **Struct-level:**
//!       - `#[storage_fields]`
//!       - `#[mutator(custom)]`
//!       - `#[perform]`
//!     - **Field-level:**
//!       - `#[former(default = ...)]`
//!       - `#[scalar(name = ..., setter = ..., debug)]`
//!       - `#[subform_collection(name = ..., setter = ..., debug, definition = ...)]`
//!       - `#[subform_entry(name = ..., setter = ..., debug)]`
//!       - `#[subform_scalar(name = ..., setter = ..., debug)]`
//!       - Multiple attributes on one field
//!       - Feature-gated fields (`#[cfg(...)]`)
//!   - **Generics & Lifetimes:**
//!     - Parametrized struct
//!     - Parametrized field
//!     - Slice lifetimes
//!     - Dyn traits
//!   - **Edge Cases:**
//!     - Keyword identifiers for fields
//!     - Keyword identifiers for subform setters
//!     - Name collisions (with std types, keywords, etc.)
//!     - Visibility (public/private structs and fields)
//!   - **Compile-time Failures:** Tests ensuring incorrect usage results in compile errors.

use super::*;
use test_tools::exposed::*;

use super::*;

// = basic

// Test re-enabled to verify proper fix
#[cfg(any(feature = "use_alloc", not(feature = "no_std")))]
mod a_basic;
#[cfg(any(feature = "use_alloc", not(feature = "no_std")))]
mod a_basic_manual;
// Test re-enabled to verify proper fix
mod a_primitives;
mod a_primitives_manual;
mod tuple_struct;
mod debug_e0223_minimal;
mod debug_e0223_manual;

// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_collection_basic;  // BLOCKED: Complex collection type mismatch issues
// #[cfg(any(feature = "use_alloc", not(feature = "no_std")))]
// mod subform_collection_basic_manual;  // BLOCKED: FormerBegin lifetime parameter in manual code
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[cfg(any(feature = "use_alloc", not(feature = "no_std")))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection_basic_scalar;

// = attribute

// Test re-enabled to verify proper fix
mod attribute_alias;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod attribute_default_collection;
mod attribute_default_conflict;
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod attribute_default_primitive;
mod attribute_feature;
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod attribute_multiple;
mod attribute_perform;
mod attribute_setter;
mod attribute_storage_with_end;
mod attribute_storage_with_mutator;

// = name collision

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose  
mod keyword_field_derive;
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod keyword_subform_derive;
mod name_collision_former_hashmap_without_parameter;
mod name_collision_former_vector_without_parameter;
// mod name_collisions;  // BLOCKED: Name collision with std types causes E0308 type conflicts

// = parametrization

mod parametrized_dyn_manual; // xxx2 : qqq2 : fix the issue

// mod parametrized_field;  // BLOCKED: E0726 implicit elided lifetime + complex generic bounds
mod test_lifetime_only;
mod test_lifetime_minimal;
mod minimal_lifetime;
mod debug_lifetime_minimal;
mod debug_simple_lifetime;
// mod parametrized_field_where;  // BLOCKED: E0726 implicit elided lifetime not allowed here
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod parametrized_struct_imm;  // BLOCKED: E0277 Hash/Eq trait bound issues with Definition
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod parametrized_struct_manual;  // BLOCKED: E0106 missing lifetime specifier for FormerBegin in manual code
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod parametrized_struct_where;  // BLOCKED: E0277 Hash/Eq trait bound issues with Definition

mod parametrized_slice;
mod parametrized_slice_manual;

// = etc

// Test re-enabled to verify proper fix
mod default_user_type;
mod unsigned_primitive_types;
mod user_type_no_debug;
mod user_type_no_default;
mod visibility;

// = collection former

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_binary_heap;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod collection_former_btree_map;  // BLOCKED: Complex collection type mismatch issues with subform
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_btree_set;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_common;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod collection_former_hashmap;  // BLOCKED: Complex collection type mismatch issues with subform
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_hashset;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_linked_list;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_vec;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod collection_former_vec_deque;

// = subform collection

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection_custom;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection_implicit;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_collection_manual;  // BLOCKED: FormerBegin lifetime parameter in manual code
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection_named;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_collection_playground;  // BLOCKED: E0277 Hash/Eq trait bound issues with Definition
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_collection_setter_off;

// = subform scalar

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_scalar;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_scalar_manual;  // BLOCKED: FormerBegin lifetime parameter in manual code
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_scalar_name;

// = subform entry

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_entry;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_entry_manual;  // BLOCKED: FormerBegin lifetime parameter in manual code
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_entry_named;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_entry_named_manual;  // BLOCKED: FormerBegin lifetime parameter in manual code
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_entry_setter_off;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_entry_setter_on;

// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_entry_hashmap;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_entry_hashmap_custom;  // BLOCKED: FormerBegin lifetime parameter in manual code

// = subform all : scalar, subform_scalar, subform_entry, subform_collection

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_all;
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
// mod subform_all_parametrized;  // BLOCKED: E0726 implicit elided lifetime not allowed here + E0277 FormerDefinition trait issues
// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
mod subform_all_private;

// = standalone constructor

// mod standalone_constructor_derive;  // BLOCKED: Requires standalone_constructors attribute implementation
mod standalone_constructor_manual;

// = compile-time

only_for_terminal_module! {

  // stable have different information about error
  // that's why these tests are active only for nightly

  #[ cfg( feature = "derive_former" ) ]
  #[ test_tools::nightly ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/struct_tests/compiletime/field_attr_bad.rs" );
    t.compile_fail( "tests/inc/struct_tests/compiletime/struct_attr_bad.rs" );
    t.pass( "tests/inc/struct_tests/compiletime/hashmap_without_parameter.rs" );
    t.pass( "tests/inc/struct_tests/compiletime/vector_without_parameter.rs" );
    // qqq : xxx : make sure it works

    // assert!( false );

  }

}
