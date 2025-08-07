//! Tests for runtime assertions.
#[ test ]
fn a_id_run()
{
  let result = std::panic::catch_unwind( || 
  {
    diagnostics_tools::a_id!( 1, 2 );
  } );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  let msg = if let Some( s ) = err.downcast_ref::< String >()
  {
    s.as_str()
  } else if let Some( s ) = err.downcast_ref::< &'static str >()
  {
    s
  } else
  {
    panic!( "Unknown panic payload type: {:?}", err );
  };
  let msg = String::from_utf8( strip_ansi_escapes::strip( &msg ).unwrap() ).unwrap();
  assert!( msg.contains( "assertion failed: `(left == right)`" ) );
  assert!( msg.contains( "Diff < left / right > :" ) );
  assert!( msg.contains( "<1" ) );
  assert!( msg.contains( ">2" ) );
}

#[ test ]
fn a_not_id_run()
{
  let result = std::panic::catch_unwind( || 
  {
    diagnostics_tools::a_not_id!( 1, 1 );
  } );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  let msg = if let Some( s ) = err.downcast_ref::< String >()
  {
    s.as_str()
  } else if let Some( s ) = err.downcast_ref::< &'static str >()
  {
    s
  } else
  {
    panic!( "Unknown panic payload type: {:?}", err );
  };
  let msg = String::from_utf8( strip_ansi_escapes::strip( &msg ).unwrap() ).unwrap();
  assert!( msg.contains( "assertion failed: `(left != right)`" ) );
  assert!( msg.contains( "Both sides:" ) );
  assert!( msg.contains( "1" ) );
}
