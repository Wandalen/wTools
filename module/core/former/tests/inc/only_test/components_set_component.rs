

#[ test ]
fn component_set()
{

  let mut got : Person = Default::default();
  got.set( 13 );
  got.set( "John" );
  assert_eq!( got, Person { age : 13, name : "John".to_string() } );

}
