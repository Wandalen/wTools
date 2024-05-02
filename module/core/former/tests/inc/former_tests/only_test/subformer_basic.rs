// let ca = wca::ChildsParent::former()
// .command_with_closure( "echo" )
//   .name( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .perform()
// .command_with_closure( "exit" )
//   .name( "just exit" )
//   .routine( || exit() )
//   .perform()
// .perform()
// ;
// ca.execute( input ).unwrap();

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_with_closure()
{

  let got = Child::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .form();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  let got = Child::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .perform();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  let got = Child::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .end();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
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

  // with helper
  let got = Child::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .property( "property1", "simple property", 13isize )
  .property( "property2", "simple property 2", 13isize )
  .property( "property2", "simple property 3", 113isize )
  .form();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", "simple property", 13isize ),
      "property2" => Property::new( "property2", "simple property 3", 113isize ),
    },
    // properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // with HashMapSubformer
  let got = Child::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .properties()
    .add( ( "property1", Property::new( "property1", "simple property", 13isize ) ) )
    .add( ( "property2", Property::new( "property2", "simple property 2", 13isize ) ) )
    .add( ( "property2", Property::new( "property2", "simple property 3", 113isize ) ) )
    .end()
  .form();
  let exp = Child::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", "simple property", 13isize ),
      "property2" => Property::new( "property2", "simple property 3", 113isize ),
    },
    // properties : collection_tools::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}

//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn aggregator()
{

  // with helper
  let got = Parent::< &str >::former()
  .parameter1( "p1" )
  .commands().add( ( "name1".to_string(), ChildFormer::< &str >::new_coercing( former::ReturnPreformed ).name( "name1" ).subject( "s" ).end() ) ).end()
  .form()
  ;

  let name1 = Child::< &str >
  {
    name : "name1".to_string(),
    subject : "s".to_string(),
    properties : hmap!{},
  };
  let exp = Parent
  {
    parameter1 : "p1".to_string(),
    commands : hmap!{ "name1" => name1 },
  };
  dbg!( &got );
  dbg!( &exp );
  a_id!( got, exp );

}

//

#[ test ]
fn aggregator_alternative_form()
{

  let exp = Parent::< &str >::former()
  .parameter1( "p1" )
  .form()
  ;

  let got = Parent::< &str >::former()
  .parameter1( "p1" )
  .perform()
  ;
  a_id!( got, exp );

  let got = Parent::< &str >::former()
  .parameter1( "p1" )
  .end()
  ;
  a_id!( got, exp );

}
