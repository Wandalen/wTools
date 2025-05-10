use super::*;
use former::Former;
use former::FormerEnd; // Import necessary traits
use former::ReturnContainer; // Import necessary types

// Define the inner structs that the enum variants will hold.
// These need to derive Former themselves if you want to build them easily,
// and they are used in this form in the tests.
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Prompt { pub content: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition: bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct InstructionsApplyToFiles { pub instruction: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command: String }

// The enum itself. We will manually implement Former for this.
#[derive(Debug, Clone, PartialEq)] // Remove #[derive(Former)] here
pub enum FunctionStep
{
  Prompt(Prompt),
  Break(Break),
  InstructionsApplyToFiles(InstructionsApplyToFiles),
  Run(Run),
}

// --- Manual Former Implementation for FunctionStep ---

// The main former struct for FunctionStep. It primarily provides starter methods.
pub struct FunctionStepFormer;

impl former::Former for FunctionStep
{
  type Former = FunctionStepFormer;
}

impl FunctionStepFormer
{
  /// Creates a new former for FunctionStep.
  pub fn new() -> Self
  {
    FunctionStepFormer
  }

  /// Starts building a `Prompt` variant.
  /// Returns a former for `Prompt` configured to return `FunctionStep`.
  pub fn prompt( self ) -> PromptFormer< ReturnContainer< FunctionStep > >
  {
    PromptFormer::new()
  }

  /// Starts building a `Break` variant.
  /// Returns a former for `Break` configured to return `FunctionStep`.
  pub fn r#break( self ) -> BreakFormer< ReturnContainer< FunctionStep > >
  {
    BreakFormer::new()
  }

  /// Starts building an `InstructionsApplyToFiles` variant.
  /// Returns a former for `InstructionsApplyToFiles` configured to return `FunctionStep`.
  pub fn instructions_apply_to_files( self ) -> InstructionsApplyToFilesFormer< ReturnContainer< FunctionStep > >
  {
    InstructionsApplyToFilesFormer::new()
  }

  /// Starts building a `Run` variant.
  /// Returns a former for `Run` configured to return `FunctionStep`.
  pub fn run( self ) -> RunFormer< ReturnContainer< FunctionStep > >
  {
    RunFormer::new()
  }

  // Note: There is no .form() method on FunctionStepFormer itself in this pattern.
  // The .form() is called on the sub-formers returned by the variant methods.
}

// --- Manual Implementations for ReturnContainer< FunctionStep > for each inner type ---
// These allow the .form() method on the inner type's former to return FunctionStep.

impl FormerEnd< Prompt > for ReturnContainer< FunctionStep >
{
  type Formed = FunctionStep;
  fn form( self, value : Prompt ) -> Self::Formed
  {
    FunctionStep::Prompt( value )
  }
}

impl FormerEnd< Break > for ReturnContainer< FunctionStep >
{
  type Formed = FunctionStep;
  fn form( self, value : Break ) -> Self::Formed
  {
    FunctionStep::Break( value )
  }
}

impl FormerEnd< InstructionsApplyToFiles > for ReturnContainer< FunctionStep >
{
  type Formed = FunctionStep;
  fn form( self, value : InstructionsApplyToFiles ) -> Self::Formed
  {
    FunctionStep::InstructionsApplyToFiles( value )
  }
}

impl FormerEnd< Run > for ReturnContainer< FunctionStep >
{
  type Formed = FunctionStep;
  fn form( self, value : Run ) -> Self::Formed
  {
    FunctionStep::Run( value )
  }
}

// Include the test logic.
include!("usecase1_only_test.rs");