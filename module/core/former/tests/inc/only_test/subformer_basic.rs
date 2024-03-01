// let ca = wca::CommandsAggregator::former()
// .command( "echo" )
//   .name( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .perform()
// .command( "exit" )
//   .name( "just exit" )
//   .routine( || exit() )
//   .perform()
// .perform()
// ;
// ca.execute( input ).unwrap();

#[ test ]
fn basic()
{

  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : std::collections::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}

//

#[ test ]
fn properties()
{

  // with helper
  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .property( "property1", "simple property", 13isize )
  .property( "property2", "simple property 2", 13isize )
  .property( "property2", "simple property 3", 113isize )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", "simple property", 13isize ),
      "property2" => Property::new( "property2", "simple property 3", 113isize ),
    },
    // properties : std::collections::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

  // with HashMapSubformer
  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .properties()
    .insert( "property1", Property::new( "property1", "simple property", 13isize ) )
    .insert( "property2", Property::new( "property2", "simple property 2", 13isize ) )
    .insert( "property2", Property::new( "property2", "simple property 3", 113isize ) )
    .end()
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", "simple property", 13isize ),
      "property2" => Property::new( "property2", "simple property 3", 113isize ),
    },
    // properties : std::collections::HashMap::< &str, Property< &str > >::new(),
  };
  a_id!( got, exp );

}

//

#[ test ]
fn aggregator()
{

  // with helper
  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .command( "command1".to_string() )
    // .name( "a" )
    .subject( "b" )
    .property( "property1", "simple property", 13isize )
    .property( "property2", "simple property 3", 113isize )
    .end()
  .command( "command2".to_string() )
    .subject( "c" )
    .property( "property3", "x", 113isize )
    .end()
  .form()
  ;

  let command1 = Command::< &str >
  {
    name : "command1".to_string(),
    subject : "b".to_string(),
    properties : hmap!
    {
      "property1" => Property::new( "property1", "simple property", 13isize ),
      "property2" => Property::new( "property2", "simple property 3", 113isize ),
    },
  };
  let command2 = Command::< &str >
  {
    name : "command2".to_string(),
    subject : "c".to_string(),
    properties : hmap!
    {
      "property3" => Property::new( "property3", "x", 113isize ),
    },
  };
  let exp = Aggregator
  {
    parameter1 : "p1".to_string(),
    commands : hmap!{ "command1" => command1, "command2" => command2 },
  };
  dbg!( &got );
  dbg!( &exp );
  a_id!( got, exp );

}
