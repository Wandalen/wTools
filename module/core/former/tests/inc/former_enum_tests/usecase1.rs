// File: module/core/former/tests/inc/former_enum_tests/basic.rs
use super::*;

// Define the inner structs that the enum variants will hold.
// These need to derive Former themselves if you want to build them easily.
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Prompt { pub content: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition: bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct InstructionsApplyToFiles { pub instruction: String }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command: String }

// Derive Former on the enum.
// By default, this should generate subformer starter methods for each variant.
#[derive(Debug, Clone, PartialEq, former::Former)]
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
  // Construct the Prompt variant using the generated subformer starter
  let prompt_step = FunctionStep::prompt() // Expects subformer starter
    .content( "Explain the code." )
    .form(); // Calls the specialized PromptEnd
  let expected_prompt = FunctionStep::Prompt( Prompt { content: "Explain the code.".to_string() } );
  assert_eq!( prompt_step, expected_prompt );

  // Construct the Break variant using the generated subformer starter
  let break_step = FunctionStep::r#break() // Expects subformer starter (using raw identifier)
    .condition( true )
    .form(); // Calls the specialized BreakEnd
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( break_step, expected_break );

  // Construct the InstructionsApplyToFiles variant using the generated subformer starter
  let apply_step = FunctionStep::instructions_apply_to_files() // Expects subformer starter
    .instruction( "Apply formatting." )
    .form(); // Calls the specialized InstructionsApplyToFilesEnd
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( apply_step, expected_apply );

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
  // Construct the Prompt variant
  let prompt_step = FunctionStep::Prompt
  (
    Prompt::former()
    .content( "Explain the code." )
    .form()
  );
  let expected_prompt = FunctionStep::Prompt( Prompt { content: "Explain the code.".to_string() } );
  assert_eq!( prompt_step, expected_prompt );

  // Construct the Break variant
  let break_step = FunctionStep::Break
  (
    Break::former()
    .condition( true )
    .form()
  );
  let expected_break = FunctionStep::Break( Break { condition: true } );
  assert_eq!( break_step, expected_break );

  // Construct the InstructionsApplyToFiles variant
  let apply_step = FunctionStep::InstructionsApplyToFiles
  (
    InstructionsApplyToFiles::former()
    .instruction( "Apply formatting." )
    .form()
  );
  let expected_apply = FunctionStep::InstructionsApplyToFiles( InstructionsApplyToFiles { instruction: "Apply formatting.".to_string() } );
  assert_eq!( apply_step, expected_apply );

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