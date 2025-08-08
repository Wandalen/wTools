//! Test arrow key functionality with rustyline
//! 
//! This is a minimal test to verify arrow keys work for command history.
//! Run with: cargo run --example test_arrow_keys --features enhanced_repl

#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::DefaultEditor;
#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::error::ReadlineError;
#[ cfg( feature = "enhanced_repl" ) ]
use rustyline::history::History;

#[ cfg( feature = "enhanced_repl" ) ]
fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let mut rl = DefaultEditor::new()?;
  
  println!( "=== Arrow Key History Test ===" );
  println!( "Instructions:" );
  println!( "1. Type some commands (like 'hello', 'world', 'test')" );
  println!( "2. Press ↑ (up arrow) to navigate back through history" );
  println!( "3. Press ↓ (down arrow) to navigate forward" );
  println!( "4. Type 'history' to see current history" );
  println!( "5. Type 'quit' to exit" );
  println!();
  
  let mut command_count = 0;

  loop
  {
    let prompt = format!( "test[{}]> ", command_count );
    
    match rl.readline( &prompt )
    {
      Ok( input ) =>
      {
        let input = input.trim();
        
        match input
        {
          "" => continue,
          "quit" | "exit" =>
          {
            println!( "Goodbye!" );
            break;
          },
          "history" =>
          {
            let history = rl.history();
            if history.is_empty()
            {
              println!( "No commands in history" );
            }
            else
            {
              println!( "Command History ({} entries):", history.len() );
              for ( i, cmd ) in history.iter().enumerate()
              {
                println!( "  {}: {}", i + 1, cmd );
              }
            }
            continue;
          },
          _ =>
          {
            // Add to history and process
            rl.add_history_entry( input )?;
            command_count += 1;
            println!( "Processed: '{}' (try arrow keys now!)", input );
          }
        }
      },
      Err( ReadlineError::Interrupted ) =>
      {
        println!( "CTRL+C pressed" );
        break;
      },
      Err( ReadlineError::Eof ) =>
      {
        println!( "EOF (CTRL+D)" );
        break;
      },
      Err( err ) =>
      {
        println!( "Error: {:?}", err );
        break;
      }
    }
  }

  Ok( () )
}

#[ cfg( not( feature = "enhanced_repl" ) ) ]
fn main()
{
  println!( "This test requires the 'enhanced_repl' feature." );
  println!( "Run with: cargo run --example test_arrow_keys --features enhanced_repl" );
}