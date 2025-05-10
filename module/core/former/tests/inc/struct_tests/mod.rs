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

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
mod a_basic_manual;
#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
mod a_basic;
mod a_primitives_manual;
mod a_primitives;
mod tuple_struct;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
mod subform_collection_basic_scalar;
#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
mod subform_collection_basic_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_basic;

// = attribute

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod attribute_default_collection;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod attribute_default_primitive;
mod attribute_default_conflict;
mod attribute_storage_with_end;
mod attribute_storage_with_mutator;
mod attribute_perform;
mod attribute_setter;
mod attribute_alias;
mod attribute_feature;
mod attribute_multiple;

// = name collision

mod name_collision_former_hashmap_without_parameter;
mod name_collision_former_vector_without_parameter;
mod name_collisions;
mod keyword_field_derive;
mod keyword_subform_derive;

// = parametrization

mod parametrized_dyn_manual; // xxx2 : qqq2 : fix the issue

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod parametrized_struct_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod parametrized_struct_imm;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod parametrized_struct_where;
mod parametrized_field;
mod parametrized_field_where;

mod parametrized_slice_manual;
mod parametrized_slice;

// = etc

mod unsigned_primitive_types;
mod default_user_type;
mod user_type_no_default;
mod user_type_no_debug;
mod visibility;

// = collection former

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_common;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_btree_map;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_btree_set;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_binary_heap;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_hashmap;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_hashset;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_linked_list;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_vec;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod collection_former_vec_deque;

// = subform collection

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_playground;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_implicit;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_setter_off;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_named;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_collection_custom;

// = subform scalar

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_scalar_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_scalar;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_scalar_name;

// = subform entry

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_named;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_named_manual;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_setter_off;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_setter_on;

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_hashmap;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_entry_hashmap_custom;

// = subform all : scalar, subform_scalar, subform_entry, subform_collection

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_all;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_all_private;
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
mod subform_all_parametrized;

// = standalone constructor

mod standalone_constructor_manual;
mod standalone_constructor_derive;

// = compile-time

only_for_terminal_module!
{

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
