
#[ test ]
fn child()
{

  let got = Parent::former()
  .child( "a" ).end()
  .child( "b" ).end()
    // .add( Child::former().name( "a" ).form() )
    // .add( Child::former().name( "b" ).form() )
    // .end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
