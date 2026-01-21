//! Example with custom validation

use multiline_input::collect_validated;

fn main()
{
  println!( "Multiline Input with Validation" );
  println!( "Message must not contain 'spam'\n" );

  let result = collect_validated( "Enter message:", |text|
  {
    if text.contains( "spam" )
    {
      Err( "Message contains prohibited content".to_string() )
    }
    else if text.trim().is_empty()
    {
      Err( "Message cannot be empty".to_string() )
    }
    else
    {
      Ok( () )
    }
  } );

  match result
  {
    Ok( Some( text ) ) =>
    {
      println!( "\nValid message:" );
      println!( "{}", text );
    }
    Ok( None ) =>
    {
      println!( "\nCancelled" );
    }
    Err( e ) =>
    {
      eprintln!( "Error: {}", e );
      std::process::exit( 1 );
    }
  }
}
