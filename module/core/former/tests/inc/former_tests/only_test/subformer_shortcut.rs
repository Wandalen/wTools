
#[ test ]
fn basic()
{

  // let x : < Vec< Descriptor > as former::ContainerAdd >::Element;

  let got = Parameters::former()
  .descriptors()
    .add( Descriptor::former().name( "a" ).form() )
    .add( Descriptor::former().name( "b" ).form() )
    .end()
  .form();

  let descriptors = vec!
  [
    Descriptor { name : "a".to_string(), is_mandatory : false },
    Descriptor { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parameters { descriptors };
  a_id!( got, exp );

}

#[ test ]
fn descriptor()
{

  let got = Parameters::former()
  .descriptor( "a" ).end()
  .descriptor( "b" ).end()
    // .add( Descriptor::former().name( "a" ).form() )
    // .add( Descriptor::former().name( "b" ).form() )
    // .end()
  .form();

  let descriptors = vec!
  [
    Descriptor { name : "a".to_string(), is_mandatory : false },
    Descriptor { name : "b".to_string(), is_mandatory : false },
  ];
  let exp = Parameters { descriptors };
  a_id!( got, exp );

}
