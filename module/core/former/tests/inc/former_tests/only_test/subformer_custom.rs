
// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_constructor()
{

  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b",
  };
  a_id!( got, exp );

  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .perform();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b",
  };
  a_id!( got, exp );

  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .end();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b",
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
  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b",
  };
  a_id!( got, exp );

  // with HashMapSubformer
  let got = Command::< &str >::former()
  .name( "a" )
  .subject( "b" )
  .form();
  let exp = Command::< &str >
  {
    name : "a".to_string(),
    subject : "b",
  };
  a_id!( got, exp );

}

//

#[ test ]
fn aggregator_alternative_form()
{

  let exp = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .command_with_closure( "command1".to_string() )
    .subject( "b" )
    .end()
  .form()
  ;

  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .command_with_closure( "command1".to_string() )
    .subject( "b" )
    .end()
  .perform()
  ;
  a_id!( got, exp );

  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .command_with_closure( "command1".to_string() )
    .subject( "b" )
    .end()
  .end()
  ;
  a_id!( got, exp );

}

//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_with_closure()
{

  // with helper
  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .commands().add( ( "name1".to_string(), CommandFormer::< &str >::new_coercing( former::ReturnPreformed ).name( "name1" ).subject( "s" ).end() ) ).end()
  .command_with_closure( "command1".to_string() )
    .subject( "b" )
    .end()
  .command_with_closure( "command2".to_string() )
    .subject( "c" )
    .end()
  .form()
  ;

  let name1 = Command::< &str >
  {
    name : "name1".to_string(),
    subject : "s",
  };
  let command1 = Command::< &str >
  {
    name : "command1".to_string(),
    subject : "b",
  };
  let command2 = Command::< &str >
  {
    name : "command2".to_string(),
    subject : "c",
  };
  let exp = Aggregator
  {
    parameter1 : "p1".to_string(),
    commands : hmap!{ "name1" => name1, "command1" => command1, "command2" => command2 },
  };
  dbg!( &got );
  dbg!( &exp );
  a_id!( got, exp );

}

//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_with_type()
{

  // with helper
  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .commands().add( ( "name1".to_string(), CommandFormer::< &str >::new_coercing( former::ReturnPreformed ).name( "name1" ).subject( "s" ).end() ) ).end()
  .command_with_type( "command1".to_string() )
    .subject( "b" )
    .end()
  .command_with_type( "command2".to_string() )
    .subject( "c" )
    .end()
  .form()
  ;

  let name1 = Command::< &str >
  {
    name : "name1".to_string(),
    subject : "s",
  };
  let command1 = Command::< &str >
  {
    name : "command1".to_string(),
    subject : "b",
  };
  let command2 = Command::< &str >
  {
    name : "command2".to_string(),
    subject : "c",
  };
  let exp = Aggregator
  {
    parameter1 : "p1".to_string(),
    commands : hmap!{ "name1" => name1, "command1" => command1, "command2" => command2 },
  };
  dbg!( &got );
  dbg!( &exp );
  a_id!( got, exp );

}
