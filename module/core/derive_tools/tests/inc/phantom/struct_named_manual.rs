//! # Test Matrix for `PhantomData` Manual Implementation - Named Struct
//!
//! This matrix outlines the test cases for the manual implementation of `PhantomData` for named structs.
//!
//! | ID    | Struct Type | Fields | Expected Behavior                               |
//! |-------|-------------|--------|-------------------------------------------------|
//! | P1.1  | Named       | 1      | Should implement `PhantomData` for a named struct with one field |
//! | P1.2  | Named       | >1     | Should implement `PhantomData` for a named struct with multiple fields |

#![allow(unused_imports)]
#![allow(dead_code)]

use test_tools :: *;
use core ::marker ::PhantomData;

// P1.1 : Named struct with one field
pub struct NamedStruct1 
{
  pub field1: i32,
}

// P1.2 : Named struct with multiple fields
pub struct NamedStruct2 
{
  pub field1: i32,
  pub field2: bool,
}

// Shared test logic
include!("../phantom_only_test.rs");
