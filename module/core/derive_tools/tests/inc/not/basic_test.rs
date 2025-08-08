//! # Test Matrix for `Not` Derive
//!
//! This matrix outlines the test cases for the `Not` derive macro.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | N1.1  | Unit        | None   | Should derive `Not` for unit structs            |
//! | N1.2  | Tuple       | 1      | Should derive `Not` for tuple structs with one field |
//! | N1.3  | Tuple       | >1     | Should not compile (Not requires one field)     |
//! | N1.4  | Named       | 1      | Should derive `Not` for named structs with one field |
//! | N1.5  | Named       | >1     | Should not compile (Not requires one field)     |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use the_module::Not;

// N1.1: Unit struct
#[ derive( Not ) ]
pub struct UnitStruct;

// N1.2: Tuple struct with one field
#[ derive( Not ) ]
pub struct TupleStruct1( pub bool );

// N1.3: Tuple struct with multiple fields - should not compile
// #[ derive( Not ) ]
// pub struct TupleStruct2( pub bool, pub bool );

// N1.4: Named struct with one field
#[ derive( Not ) ]
pub struct NamedStruct1
{
  pub field1 : bool,
}

// N1.5: Named struct with multiple fields - should not compile
// #[ derive( Not ) ]
// pub struct NamedStruct2
// {
//   pub field1 : bool,
//   pub field2 : bool,
// }

// Shared test logic
include!( "../not_only_test.rs" );
