
use super::*;
use former::StoragePreform;

// --- Inner Struct Definitions ---
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition: bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command: String }

// --- Enum Definition ---
#[derive(Debug, Clone, PartialEq)]
enum FunctionStep
{
  Break(Break),
  Run(Run),
}

// --- Specialized End Structs ---
#[derive(Default, Debug)] pub struct FunctionStepBreakEnd;
#[derive(Default, Debug)] pub struct FunctionStepRunEnd;

// --- Static Variant Constructor Methods ---
impl FunctionStep
{
  #[ inline( always ) ]
  pub fn r#break() // Using raw identifier
  -> BreakFormer< BreakFormerDefinition< (), Self, FunctionStepBreakEnd > >
  {
    // Correct: Call associated function `begin` on the Former type
    BreakFormer::begin( None, None, FunctionStepBreakEnd::default() )
  }

  #[ inline( always ) ]
  pub fn run()
  -> RunFormer< RunFormerDefinition< (), Self, FunctionStepRunEnd > >
  {
    // Correct: Call associated function `begin` on the Former type
    RunFormer::begin( None, None, FunctionStepRunEnd::default() )
  }
}

  /// Manually implemented standalone subformer starter for the Break variant.
  #[ inline( always ) ]
  pub fn break_variant()
  -> BreakFormer< BreakFormerDefinition< (), Self, FunctionStepBreakEnd > >
  {
    BreakFormer::begin( None, None, FunctionStepBreakEnd::default() )
  }

// --- FormingEnd Implementations for End Structs ---

// End for Break variant
impl former::FormingEnd
<
  BreakFormerDefinitionTypes< (), FunctionStep > // Context is (), Formed is FunctionStep
>
for FunctionStepBreakEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : BreakFormerStorage, // Storage of the inner type (Break)
    _context : Option< () >,          // Context is () from ::begin
  ) -> FunctionStep                   // Returns the Enum type
  {
    let data = sub_storage.preform(); // Get the Break data
    FunctionStep::Break( data )       // Construct the enum variant
  }
}

// End for Run variant
impl former::FormingEnd
<
  RunFormerDefinitionTypes< (), FunctionStep > // Context is (), Formed is FunctionStep
>
for FunctionStepRunEnd
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    sub_storage : RunFormerStorage, // Storage of the inner type (Run)
    _context : Option< () >,        // Context is () from ::begin
  ) -> FunctionStep                 // Returns the Enum type
  {
    let data = sub_storage.preform(); // Get the Run data
    FunctionStep::Run( data )         // Construct the enum variant
  }
}

// Include the test logic
include!( "basic_only_test.rs" ); // Renamed from _static_only_test
