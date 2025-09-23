#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: Tests the `#[ derive( Former ) ]` macro's generation of subformer starter methods for an enum
//! with multiple single-field tuple variants, where the inner types also derive `Former`. This file
//! verifies that the default behavior for single-field tuple variants is to generate a subformer,
//! allowing nested building.
//!
//! Coverage:
//! - Rule 3d (Tuple + Single-Field + Default): Verifies that for single-field tuple variants without specific attributes, the derived constructor is a subformer starter method.
//! - Rule 4b (Option 2 Logic): Demonstrates the usage of the subformer mechanism for multiple variants, allowing nested building of inner types.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `FunctionStep` with multiple single-field tuple variants (`Prompt`, `Break`, `InstructionsApplyToFiles`, `Run`).
//! - The inner types (`Prompt`, `Break`, etc.) also derive `Former`.
//! - Applies `#[ derive( Former ) ]` to the `FunctionStep` enum.
//! - Contains test functions that call the derived static methods (e.g., `FunctionStep::prompt()`, `FunctionStep::r#break()`).
//! - Uses the returned subformers to set fields of the inner types and calls `.form()` on the subformers to get the final `FunctionStep` enum instance.
//! - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the default behavior for single-field tuple variants is to generate subformer starters that correctly integrate with the inner types' formers.

use super::*;
use former::Former;

// Define the inner structs that the enum variants will hold.
// These need to derive Former themselves if you want to build them easily.
#[ derive( Debug, PartialEq ) ] // xxx: Former derive disabled - trailing comma issue
pub struct Prompt { pub content: String }

#[ derive( Debug, PartialEq ) ] // xxx: Former derive disabled - trailing comma issue
pub struct Break { pub condition: bool }

#[ derive( Debug, PartialEq ) ] // xxx: Former derive disabled - trailing comma issue
pub struct InstructionsApplyToFiles { pub instruction: String }

#[ derive( Debug, PartialEq ) ] // xxx: Former derive disabled - trailing comma issue
pub struct Run { pub command: String }

// Derive Former on the enum.
// By default, this should generate subformer starter methods for each variant.
// #[ debug ]
// FIX: Combined derive attributes
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Debug, Clone, PartialEq, Former ) ]
#[ derive( Debug, Clone, PartialEq ) ]
enum FunctionStep
{
  Prompt(Prompt),
  Break(Break),
  InstructionsApplyToFiles(InstructionsApplyToFiles),
  Run(Run),
}

// Renamed test to reflect its purpose: testing the subformer construction
#[ test ]
fn enum_variant_subformer_construction()
{
  // Test Matrix Row: T22.1 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Prompt variant using the generated subformer starter
  let prompt_step = FunctionStep::prompt() // Expects subformer starter
    .content( "Explain the code." )
    .form(); // Calls the specialized PromptEnd
  let expected_prompt = FunctionStep::Prompt( Prompt { content: "Explain the code.".to_string() } );
  assert_eq!( prompt_step, expected_prompt );

  // Test Matrix Row: T22.2 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Break variant using the generated subformer starter
  let break_step = FunctionStep::r#break() // Expects subformer starter (using raw identifier)
    .condition( true )
    .form(); // Callxqs the specialized BreakEnd
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( break_step, expected_break );

  // Test Matrix Row: T22.3 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the InstructionsApplyToFiles variant using the generated subformer starter
  let apply_step = FunctionStep::instructions_apply_to_files() // Expects subformer starter
    .instruction( "Apply formatting." )
    .form(); // Calls the specialized InstructionsApplyToFilesEnd
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( apply_step, expected_apply );

  // Test Matrix Row: T22.4 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Run variant using the generated subformer starter
  let run_step = FunctionStep::run() // Expects subformer starter
    .command( "cargo check" )
    .form(); // Calls the specialized RunEnd
  let expected_run = FunctionStep::Run( Run { command: "cargo check".to_string() } );
  assert_eq!( run_step, expected_run );
}

// Keep the original test demonstrating manual construction for comparison if desired,
// but it's not strictly necessary for testing the derive macro itself.
#[ test ]
fn enum_variant_manual_construction()
{
  // Test Matrix Row: T22.5 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Prompt variant
  let prompt_step = FunctionStep::Prompt
  (
    Prompt::former()
    .content( "Explain the code." )
    .form()
  );
  let expected_prompt = FunctionStep::Prompt( Prompt { content: "Explain the code.".to_string() } );
  assert_eq!( prompt_step, expected_prompt );

  // Test Matrix Row: T22.6 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Break variant
  let break_step = FunctionStep::Break
  (
    Break::former()
    .condition( true )
    .form()
  );
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( break_step, expected_break );

  // Test Matrix Row: T22.7 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the InstructionsApplyToFiles variant
  let apply_step = FunctionStep::InstructionsApplyToFiles
  (
    InstructionsApplyToFiles::former()
    .instruction( "Apply formatting." )
    .form()
  );
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( apply_step, expected_apply );

  // Test Matrix Row: T22.8 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Run variant
  let run_step = FunctionStep::Run
  (
    Run::former()
    .command( "cargo check" )
    .form()
  );
  let expected_run = FunctionStep::Run( Run { command: "cargo check".to_string() } );
  assert_eq!( run_step, expected_run );
}
// qqq : xxx : uncomment and make it working