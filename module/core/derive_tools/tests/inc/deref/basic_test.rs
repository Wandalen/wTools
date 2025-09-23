//! # Test Matrix for `Deref`
//!
//! | ID   | Struct Type        | Fields      | Generics         | Attributes | Expected Behavior                                     | Test Type    |
//! |------|--------------------|-------------|------------------|------------|-------------------------------------------------------|--------------|
//! | T1.1 | Tuple Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
//! | T1.2 | Named Struct       | 1           | None             | -          | Implements `Deref` to the inner field.                | `tests/inc/deref/basic_test.rs` |
//! | T1.3 | Tuple Struct       | >1          | None             | -          | Fails to compile: `Deref` requires a single field.    | `trybuild`   |
//! | T1.4 | Named Struct       | >1          | None             | `#[ deref ]` | Implements `Deref` to the specified field.            | `tests/inc/deref/struct_named.rs` |
//! | T1.5 | Named Struct       | >1          | None             | -          | Fails to compile: `#[ deref ]` attribute is required.   | `trybuild`   |
//! | T1.6 | Enum               | Any         | Any              | -          | Fails to compile: `Deref` cannot be on an enum.       | `tests/inc/deref/compile_fail_enum.rs` |
//! | T1.7 | Unit Struct        | 0           | None             | -          | Fails to compile: `Deref` requires a field.           | `trybuild`   |
//! | T1.8 | Struct             | 1           | Lifetime         | -          | Implements `Deref` correctly with lifetimes.          | `tests/inc/deref/generics_lifetimes.rs` |
//! | T1.9 | Struct             | 1           | Type             | -          | Implements `Deref` correctly with type generics.      | `tests/inc/deref/generics_types.rs` |
//! | T1.10| Struct             | 1           | Const            | -          | Implements `Deref` correctly with const generics.     | `tests/inc/deref/generics_constants.rs` |
//! | T1.11| Struct             | 1           | Where clause     | -          | Implements `Deref` correctly with where clauses.      | `tests/inc/deref/bounds_where.rs` |
//!
// Original content of basic_test.rs will follow here.

use core ::ops ::Deref;
use derive_tools ::Deref;
// use macro_tools ::attr; // Removed

#[ derive( Deref ) ]

struct MyTuple(i32);

#[ test ]
fn basic_tuple_deref() 
{
  let x = MyTuple(10);
  assert_eq!(*x, 10);
}
