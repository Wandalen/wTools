// Purpose: Provides shared test assertions and logic for both the derived and manual implementations
// of subformer starter methods for an enum with multiple single-field tuple variants, where the
// inner types also derive `Former`. It tests that the constructors generated/implemented for this
// scenario behave as expected (returning subformers for nested building).
//
// Coverage:
// - Rule 3d (Tuple + Single-Field + Default): Tests that the constructor for single-field tuple variants without specific attributes is a subformer starter method.
// - Rule 4b (Option 2 Logic): Tests that the subformer mechanism works correctly for multiple variants, allowing nested building of inner types and returning the outer enum instance via `.form()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines the `FunctionStep` enum structure with multiple single-field tuple variants (`Prompt`, `Break`, `InstructionsApplyToFiles`, `Run`).
// - The inner types (`Prompt`, `Break`, etc.) are assumed to also derive `Former`.
// - Contains test functions (`enum_variant_subformer_construction`, `enum_variant_manual_construction`) that are included by the derive and manual test files.
// - The `enum_variant_subformer_construction` test calls the static methods (e.g., `FunctionStep::prompt()`, `FunctionStep::r#break()`) provided by the including file, uses the returned subformers to set fields, and calls `.form()`.
// - The `enum_variant_manual_construction` test demonstrates the equivalent manual construction using `InnerType::former()...form()`.
// - Both tests assert that the resulting enum instances match manually constructed expected values. This verifies that both derived and manual implementations correctly provide subformer starters and integrate with the inner types' formers for nested building.

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
  assert_eq!( FunctionStep::Prompt( prompt_step ), expected_prompt );

  // Test Matrix Row: T22.2 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Break variant using the generated subformer starter
  let break_step = FunctionStep::r#break() // Expects subformer starter (using raw identifier)
    .condition( true )
    .form(); // Callxqs the specialized BreakEnd
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( FunctionStep::Break( break_step ), expected_break );

  // Test Matrix Row: T22.3 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the InstructionsApplyToFiles variant using the generated subformer starter
  let apply_step = FunctionStep::instructions_apply_to_files() // Expects subformer starter
    .instruction( "Apply formatting." )
    .form(); // Calls the specialized InstructionsApplyToFilesEnd
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( FunctionStep::InstructionsApplyToFiles( apply_step ), expected_apply );

  // Test Matrix Row: T22.4 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Run variant using the generated subformer starter
  let run_step = FunctionStep::run() // Expects subformer starter
    .command( "cargo check" )
    .form(); // Calls the specialized RunEnd
  let expected_run = FunctionStep::Run( Run { command: "cargo check".to_string() } );
  assert_eq!( FunctionStep::Run( run_step ), expected_run );
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
  assert_eq!( FunctionStep::Prompt( prompt_step ), expected_prompt );

  // Test Matrix Row: T22.6 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Break variant
  let break_step = FunctionStep::Break
  (
    Break::former()
    .condition( true )
    .form()
  );
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( FunctionStep::Break( break_step ), expected_break );

  // Test Matrix Row: T22.7 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the InstructionsApplyToFiles variant
  let apply_step = FunctionStep::InstructionsApplyToFiles
  (
    InstructionsApplyToFiles::former()
    .instruction( "Apply formatting." )
    .form()
  );
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( FunctionStep::InstructionsApplyToFiles( apply_step ), expected_apply );

  // Test Matrix Row: T22.8 (Implicitly, as this tests the behavior expected by the matrix)
  // Construct the Run variant
  let run_step = FunctionStep::Run
  (
    Run::former()
    .command( "cargo check" )
    .form()
  );
  let expected_run = FunctionStep::Run( Run { command: "cargo check".to_string() } );
  assert_eq!( FunctionStep::Run( run_step ), expected_run );
}
// qqq : xxx : uncomment and make it working