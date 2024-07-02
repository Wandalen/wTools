

#[ test ]
fn component_assign()
{

  let mut got : Person = Default::default();
  got.assign( 13 );
  got.assign( "John" );
  assert_eq!( got, Person { age : 13, name : "John".to_string() } );

  let mut got : Person = Default::default();
  got = got
  .impute( 13 )
  .impute( "John" )
  ;
  assert_eq!( got, Person { age : 13, name : "John".to_string() } );

}
