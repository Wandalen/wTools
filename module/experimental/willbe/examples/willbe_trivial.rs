//! Demonstrates the willbe programmatic API by invoking the help command.
//!
//! Run with:
//! ```bash
//! cargo run --example willbe_trivial
//! ```

fn main()
{
  // Pass the program name as the first arg (skipped internally) then the command.
  let args = vec!
  [
    "willbe".to_string(), // argv[0] — program name, skipped by run()
    ".help".to_string(),  // list all available willbe commands
  ];

  willbe ::run( args ).expect( "help command must succeed" );
}
