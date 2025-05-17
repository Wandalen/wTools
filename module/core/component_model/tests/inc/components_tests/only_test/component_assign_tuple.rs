#[ test ]
fn component_assign()
{
  let mut got : TupleStruct = Default::default();
  got.assign( 13 );
  got.assign( "John".to_string() );
  assert_eq!( got, TupleStruct( 13, "John".to_string() ) );

  // Test impute as well
  let mut got : TupleStruct = Default::default();
  got = got
  .impute( 13 )
  .impute( "John".to_string() )
  ;
  assert_eq!( got, TupleStruct( 13, "John".to_string() ) );
}