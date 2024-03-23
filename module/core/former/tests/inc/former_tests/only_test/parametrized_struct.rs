#[ test ]
fn command_form()
{

  // form
  let got = Command::< &str >::former()
  .name( "a" )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // perform
  let got = Command::< &str >::former()
  .name( "a" )
  .perform();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // end
  let got = Command::< &str >::former()
  .name( "a" )
  .end();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}

//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_properties()
{

  // with HashMapSubformer
  let got = Command::< &str >::former()
  .name( "a" )
  .properties()
    .insert( "property1", Property::< &str >::new( "property1", 13isize ) )
    .insert( "property2", Property::new( "property2", 13isize ) )
    .insert( "property2", Property::new( "property2", 113isize ) )
    .end()
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", 13isize ),
      "property2" => Property::new( "property2", 113isize ),
    },
    // properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}
