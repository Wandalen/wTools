//! Purpose: Provides a hand-written implementation of the `Former` pattern's subformer starter methods
//! for an enum with multiple single-field tuple variants, where the inner types also derive `Former`.
//! This file demonstrates the manual implementation corresponding to the derived behavior, showing how
//! to manually create the starter methods and the `FormerEnd` implementations to allow nested building.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default): Manually implements the subformer starter methods for single-field tuple variants.
//! - Rule 4b (Option 2 Logic): Manually implements the `FormerEnd` trait for `ReturnContainer<FunctionStep>` for each inner type, allowing the inner formers to return the outer enum instance.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `FunctionStep` with multiple single-field tuple variants (`Prompt`, `Break`, `InstructionsApplyToFiles`, `Run`).
//! - The inner types (`Prompt`, `Break`, etc.) also derive `Former`.
//! - Provides a hand-written `FunctionStepFormer` struct and implements `former::Former` for `FunctionStep` to return it.
//! - Implements methods on `FunctionStepFormer` (e.g., `prompt()`, `r#break()`) that return formers for the inner types, configured with `ReturnContainer<FunctionStep>` as the end type.
//! - Implements `FormerEnd<InnerType>` for `ReturnContainer<FunctionStep>` for each inner type, defining how to construct the `FunctionStep` variant from the formed inner type.
//! - Includes shared test logic from `usecase1_only_test.rs`.
//! - The included tests call the manually implemented static methods (e.g., `FunctionStep::prompt()`), use the returned subformers to set fields of the inner types, and call `.form()` on the subformers.
//! - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the manual implementation correctly provides subformer starters and integrates with the inner types' formers.

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