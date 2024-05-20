
#[ test ]
fn child()
{

  let got = Parent::former()
  .child( "a" ).end()
  .child( "b" ).end()
  .form();

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}

#[ test ]
fn _child()
{

  let got = Parent::former()
  ._child().name( "a" ).end()
  ._child().name( "b" ).end()
  .form();

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
