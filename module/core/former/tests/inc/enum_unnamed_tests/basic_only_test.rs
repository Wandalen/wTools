#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// Purpose: Provides shared test assertions and logic for verifying the constructors generated
// by `#[ derive( Former ) ]` for enums with unnamed (tuple) variants that return subformers.
// This file is included by both `basic_derive.rs` and `basic_manual.rs`.
//
// Coverage:
// - Rule 3d (Tuple + Default -> Subform): Tests static method `FunctionStep::run()`.
// - Rule 2d (Tuple + `#[ subform_scalar ]` -> InnerFormer): Tests static method `FunctionStep::r#break()`.
// - Rule 4a (#[ standalone_constructors ]): Tests the standalone subformer starter `FunctionStep::break_variant()`.
// - Rule 4b (Option 2 Logic): Tests the use of subformer methods and `.form()`.
//
// Test Relevance/Acceptance Criteria:
// - Defines test functions (`build_break_variant_static`, `build_run_variant_static`, `standalone_break_variant`)
//   that invoke constructors provided by the including file (either derived or manual).
// - These constructors return subformers (`BreakFormer`, `RunFormer`).
// - The tests use the subformer methods (`.condition()`, `.command()`) to set fields and call `.form()`
//   to finalize the construction.
// - Asserts that the resulting `FunctionStep` enum instances are equal to the expected variants
//   (`FunctionStep::Break(...)`, `FunctionStep::Run(...)`).

#[ test ]
fn build_break_variant_static() // Test name kept for clarity, could be renamed
{
  let got = FunctionStep::r#break() // Use raw identifier here
  .condition( true )
  .form(); // This calls FunctionStepBreakEnd::call

  let expected = FunctionStep::Break( Break { condition : true } );
  assert_eq!( got, expected );
}

#[ test ]
fn build_run_variant_static() // Test name kept for clarity, could be renamed
{
  let got = FunctionStep::run()
  .command( "cargo build" )
  .form(); // This calls FunctionStepRunEnd::call

  let expected = FunctionStep::Run( Run { command : "cargo build".to_string() } );
  assert_eq!( got, expected );
}

#[ test ]
fn standalone_break_variant() // New test for standalone constructor
{
  // Expect a standalone constructor `break_variant` returning a subformer.
  let got = FunctionStep::break_variant()
    .condition( false ) // Use the setter provided by the subformer
    .form();

  let expected = FunctionStep::Break( Break { condition : false } );
  assert_eq!( got, expected );
}