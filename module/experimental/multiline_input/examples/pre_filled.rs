//! Example with pre-filled text for editing

use multiline_input::Builder;

fn main()
{
  println!( "Edit Pre-filled Text" );
  println!( "Modify the TODO list below\n" );

  let initial_text = "- Task 1: Review code\n- Task 2: Write tests\n- Task 3: Update docs";

  let editor = Builder::new()
    .prompt( "Edit TODO list:" )
    .initial_text( initial_text )
    .show_line_numbers( true )
    .show_status( true )
    .build();

  match editor.collect()
  {
    Ok( Some( text ) ) =>
    {
      println!( "\nUpdated TODO list:" );
      println!( "{}", text );
    }
    Ok( None ) =>
    {
      println!( "\nNo changes made" );
    }
    Err( e ) =>
    {
      eprintln!( "Error: {}", e );
      std::process::exit( 1 );
    }
  }
}
