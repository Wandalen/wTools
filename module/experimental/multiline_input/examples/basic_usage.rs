//! Basic usage example for multiline_input crate

use multiline_input::collect;

fn main()
{
  println!( "Multiline Input Example" );
  println!( "Press ENTER to submit, CTRL+ENTER for newline, ESC to cancel\n" );

  match collect( "Enter your message:" )
  {
    Ok( Some( text ) ) =>
    {
      println!( "\nYou entered:" );
      println!( "{}", text );
      println!( "\nLength: {} characters", text.len() );
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
