
#[ test ]
fn basic()
{

  // let x : < Vec< Child > as former::ContainerAdd >::Element;

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = vec!
  [
    Child { name : "a".to_string(), is_mandatory : false },
    Child { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

#[ test ]
fn descriptor()
{

  let got = Parent::former()
  .descriptor( "a" ).end()
  .descriptor( "b" ).end()
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
