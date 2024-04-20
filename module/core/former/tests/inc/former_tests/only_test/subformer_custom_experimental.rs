
//

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn command_with_helper()
{

  // with helper
  let got = Aggregator::< &str >::former()
  .parameter1( "p1" )
  .commands().add( ( "name1".to_string(), CommandFormer::< &str >::new_coercing( former::ReturnPreformed ).name( "name1" ).subject( "s" ).end() ) ).end()
  .command_with_helper( "command1".to_string() )
    .subject( "b" )
    .end()
  .command_with_helper( "command2".to_string() )
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
