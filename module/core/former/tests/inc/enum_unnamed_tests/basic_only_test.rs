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
