#[ test ]
fn from_components()
{
  let src = SourceTuple( 42, "Hello".to_string(), 13.01 );

  // Convert from &SourceTuple
  let got: TargetTuple = ( &src ).into();
  let exp = TargetTuple( 42, "Hello".to_string() );
  assert_eq!( got, exp );

  // Convert using From ::from
  let got: TargetTuple = TargetTuple ::from( &src );
  let exp = TargetTuple( 42, "Hello".to_string() );
  assert_eq!( got, exp );

  // Ensure clone works if needed for the generic From< T > bound
  // let src_clone = src.clone(); // Would need #[ derive( Clone ) ] on SourceTuple
  // let got_clone: TargetTuple = src_clone.into();
  // assert_eq!( got_clone, exp );
}