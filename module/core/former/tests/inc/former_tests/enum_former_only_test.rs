// File: module/core/former/tests/inc/former_tests/enum_former_only_test.rs

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