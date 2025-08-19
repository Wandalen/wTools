//! # Test Matrix for `New` Derive
//!
//! This matrix outlines the test cases for the `New` derive macro.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | N1.1  | Unit        | None   | Should derive `new()` constructor               |
//! | N1.2  | Tuple       | 1      | Should derive `new()` constructor with one arg  |
//! | N1.3  | Tuple       | >1     | Should derive `new()` constructor with multiple args |
//! | N1.4  | Named       | 1      | Should derive `new()` constructor with one arg  |
//! | N1.5  | Named       | >1     | Should derive `new()` constructor with multiple args |

#[allow(unused_imports)]
#[allow(dead_code)]
use test_tools::*;
use crate::the_module::New;

// N1.1: Unit struct - New derive not available
// #[ derive( New ) ]
pub struct UnitStruct;

// N1.2: Tuple struct with one field - New derive doesn't support tuple structs yet
// #[ derive( New ) ]
pub struct TupleStruct1(pub i32);

// N1.3: Tuple struct with multiple fields - New derive doesn't support tuple structs yet
// #[ derive( New ) ]
pub struct TupleStruct2(pub i32, pub i32);

// N1.4: Named struct with one field - New derive not available
// #[ derive( New ) ]
pub struct NamedStruct1 {
  pub field1: i32,
}

// N1.5: Named struct with multiple fields - New derive not available
// #[ derive( New ) ]
pub struct NamedStruct2 {
  pub field1: i32,
  pub field2: i32,
}

// Shared test logic
include!("../new_only_test.rs");
