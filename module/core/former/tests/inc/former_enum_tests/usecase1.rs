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

// Attempt to derive Former on the enum.
// This is likely NOT the intended use case for the Former derive macro.
// It might compile but won't generate methods like `.prompt()`, `.break()`, etc.
#[derive(Debug, Clone, PartialEq, former::Former)]
enum FunctionStep
{
  Prompt(Prompt),
  Break(Break),
  InstructionsApplyToFiles(InstructionsApplyToFiles),
  Run(Run),
}

// Test that attempts to derive Former on an enum compile.
// Note: The functionality of the derived Former for an enum is undefined/unsupported
// for the pattern `enum_former().variant_name()`.
#[ test ]
fn enum_former_compiles()
{
  // This mainly checks if the derive macro runs without panicking on an enum.
  // The generated former won't have the methods like `.prompt()`, etc.
  // We create a dummy former just to ensure compilation.
  // Depending on the macro implementation, this might produce a former
  // that allows setting *internal* fields if the macro incorrectly assumes a struct layout,
  // or it might simply be empty or fail later.
  let _former = FunctionStep::former(); // Check if former() method exists
  println!("Deriving Former on an enum compiles, but the generated former is not designed for enum variant construction.");
}

// Demonstrate the standard/intended way to construct these enum variants
// when the inner types derive Former.
#[ test ]
fn enum_variant_construction()
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
