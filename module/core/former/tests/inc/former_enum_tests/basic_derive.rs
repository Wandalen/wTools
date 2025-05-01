
use super::*;

// Define the inner structs
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition : bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command : String }

// Derive Former on the simplified enum - This should generate static methods
#[ derive( Debug, Clone, PartialEq, former::Former ) ]
#[ debug ]
#[ former( standalone_constructors ) ]
enum FunctionStep
{
  Break( Break ),
  Run( Run ),
}

// xxx : generated code for debugging

//


// xxx : generated code for debugging

// Include the test logic
include!( "basic_only_test.rs" );
// qqq : xxx : uncomment and make it working
