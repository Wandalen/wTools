//! Example showing full configuration options

use multiline_input::Builder;

fn main()
{
  println!( "Multiline Input with Configuration" );
  println!( "Min 10 chars, max 500 chars, with line numbers and status\n" );

  let editor = Builder::new()
    .prompt( "Enter commit message:" )
    .min_length( 10 )
    .max_length( 500 )
    .show_line_numbers( true )
    .show_status( true )
    .show_char_count( true )
    .color( true )
    .placeholder( "Type your commit message here..." )
    .build();

  match editor.collect()
  {
    Ok( Some( text ) ) =>
    {
      println!( "\nCommit message:" );
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
