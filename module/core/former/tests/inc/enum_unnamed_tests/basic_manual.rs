//! Purpose: Provides a manual implementation of constructors and `FormingEnd` for an enum
//! with unnamed (tuple) variants, including static methods and a standalone subformer starter,
//! to serve as a reference for verifying the `#[derive(Former)]` macro's behavior.
//!
//! Coverage:
//! - Rule 3d (Tuple + Default -> Subform): Manual implementation of static method `FunctionStep::run()`.
//! - Rule 2d (Tuple + `#[subform_scalar]` -> InnerFormer): Manual implementation of static method `FunctionStep::r#break()`.
//! - Rule 4a (#[standalone_constructors]): Manual implementation of the standalone subformer starter `break_variant()`.
//! - Rule 4b (Option 2 Logic): Manual implementation of `FormingEnd` for the variant end types.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `FunctionStep` with two single-field tuple variants: `Break(Break)` and `Run(Run)`.
//! - Manually implements static methods (`FunctionStep::r#break()`, `FunctionStep::run()`) and a standalone
//!   subformer starter (`break_variant()`) that mirror the expected generated code.
//! - Manually implements `FormingEnd` for the end types associated with the variant subformers.
//! - This file is included by `basic_only_test.rs` to provide the manual implementations that
//!   the shared tests compare against.

use super::*;
use former::StoragePreform;

// --- Inner Struct Definitions ---
// Re-enabled Former derive - testing if trailing comma issue is fixed
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition: bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command: String }

// --- Enum Definition ---
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionStep
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

  // Standalone constructors for #[standalone_constructors] attribute
  #[ inline( always ) ]
  pub fn break_variant()
  -> BreakFormer< BreakFormerDefinition< (), Self, FunctionStepBreakEnd > >
  {
    BreakFormer::begin( None, None, FunctionStepBreakEnd::default() )
  }

  #[ inline( always ) ]
  pub fn run_variant()
  -> RunFormer< RunFormerDefinition< (), Self, FunctionStepRunEnd > >
  {
    RunFormer::begin( None, None, FunctionStepRunEnd::default() )
  }
}

// Note: break_variant is now implemented as a method on the enum above

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
