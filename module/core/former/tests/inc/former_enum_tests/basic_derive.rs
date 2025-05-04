// File: module/core/former/tests/inc/former_enum_tests/basic_derive.rs

use super::*;

// Define the inner structs
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition : bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command : String }

// Derive Former on the simplified enum - This should generate static methods
#[ derive( Debug, Clone, PartialEq, former::Former ) ]
// #[ debug ]
#[ former( standalone_constructors ) ]
enum FunctionStep
{
  Break( Break ),
  Run( Run ),
}

// Include the test logic
include!( "basic_only_test.rs" );