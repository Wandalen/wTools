// let ca = wca::CommandsAggregator::former()
// .command( "echo" )
//   .hint( "prints all subjects and properties" )
//   .subject( "Subject", wca::Type::String, true )
//   .property( "property", "simple property", wca::Type::String, true )
//   .routine( f1 )
//   .perform()
// .command( "exit" )
//   .hint( "just exit" )
//   .routine( || exit() )
//   .perform()
// .perform()
// ;
// ca.execute( input ).unwrap();

#[ test ]
fn basic()
{

  let got = Command::< &str >::former()
  .hint( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    hint : "a".to_string(),
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
  .hint( "a" )
  .subject( "b" )
  .property( "property1", "simple property", 13isize )
  .property( "property2", "simple property 2", 13isize )
  .property( "property2", "simple property 3", 113isize )
  .form();
  let exp = Command::< &str >
  {
    hint : "a".to_string(),
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
  .hint( "a" )
  .subject( "b" )
  .properties()
    .insert( "property1", Property::new( "property1", "simple property", 13isize ) )
    .insert( "property2", Property::new( "property2", "simple property 2", 13isize ) )
    .insert( "property2", Property::new( "property2", "simple property 3", 113isize ) )
    .end()
  .form();
  let exp = Command::< &str >
  {
    hint : "a".to_string(),
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
