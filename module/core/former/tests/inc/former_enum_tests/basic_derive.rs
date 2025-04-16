// File: module/core/former/tests/inc/former_tests/enum_former_derive.rs
#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use super::*;

// Define the inner structs
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition : bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command : String }

// Derive Former on the simplified enum - This should generate static methods
#[derive(Debug, Clone, PartialEq, former::Former)]
// #[debug]
enum FunctionStep
{
  Break( Break ),
  Run( Run ),
}

// Include the test logic
include!( "basic_only_test.rs" );
