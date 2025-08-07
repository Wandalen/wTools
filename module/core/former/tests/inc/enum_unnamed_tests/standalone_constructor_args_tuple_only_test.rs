// Purpose: Tests standalone constructor args functionality
// This file is included by standalone_constructor_args_tuple derive/manual files

#[ test ]
fn standalone_args_constructor_test()
{
  // Test scalar multi-tuple variant with generated constructor
  let got = TestEnumArgs::multi_tuple_args( 42, true );
  let expected = TestEnumArgs::MultiTupleArgs( 42, true );
  assert_eq!( got, expected );
}