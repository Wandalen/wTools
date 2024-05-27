#[ test ]
fn command_form()
{

  // form
  let got = Child::< &str >::former()
  .name( "a" )
  .form();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // perform
  let got = Child::< &str >::former()
  .name( "a" )
  .perform();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // end
  let got = Child::< &str >::former()
  .name( "a" )
  .end();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}

//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_properties()
{

  // with HashMapFormer
  let got = Child::< &str >::former()
  .name( "a" )
  .properties()
    .add( ( "property1", Property::< &str >::new( "property1", 13isize ) ) )
    .add( ( "property2", Property::new( "property2", 13isize ) ) )
    .add( ( "property2", Property::new( "property2", 113isize ) ) )
    .end()
  .form();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    properties : collection_tools::hmap!
    {
      "property1" => Property::new( "property1", 13isize ),
      "property2" => Property::new( "property2", 113isize ),
    },
    // properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}
