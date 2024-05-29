// use optimization_tools::*;
// use sudoku::*;
// use optimization::*;
// use test_tools::prelude::*;
// use deterministic_rand::Seed;

pub fn logger_init()
{
  use std::io::Write;

  // env_logger::init();

  let _ = env_logger::builder()
  // Ensure events are captured by `cargo test`
  .is_test( true )
  // Include all events in tests
  .filter_level( log::LevelFilter::max() )
  .format( | buf, record |
  {
    // let tab = record.key_values().get( "tab" );
    writeln!( buf, "{}", record.args() )
    // record.key_values().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec< _ >>().join(", ")
  })
  // Ignore errors initializing the logger if tests race to configure it
  .try_init()
  ;
}
