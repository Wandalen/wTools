#![ allow( missing_docs ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use ::process_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools :: *;

#[ cfg( feature = "enabled" ) ]
mod inc
{
  use super :: *;
  use the_module ::lifecycle ::signal;

  /// T04: `signal_name(9)` returns `"SIGKILL"`.
  #[ test ]
  fn signal_name_sigkill()
  {
    assert_eq!( signal ::signal_name( 9 ), "SIGKILL" );
  }

  /// T05: `signal_name(15)` returns `"SIGTERM"`.
  #[ test ]
  fn signal_name_sigterm()
  {
    assert_eq!( signal ::signal_name( 15 ), "SIGTERM" );
  }

  /// T06: `signal_name(2)` returns `"SIGINT"`.
  #[ test ]
  fn signal_name_sigint()
  {
    assert_eq!( signal ::signal_name( 2 ), "SIGINT" );
  }

  /// T07: `signal_name(999)` returns `"UNKNOWN"` for unmapped signals.
  #[ test ]
  fn signal_name_unknown()
  {
    assert_eq!( signal ::signal_name( 999 ), "UNKNOWN" );
  }

  /// T08: `signal_number("SIGKILL")` returns `Some(9)`.
  #[ test ]
  fn signal_number_sigkill()
  {
    assert_eq!( signal ::signal_number( "SIGKILL" ), Some( 9 ) );
  }

  /// T09: `signal_number` returns `None` for nonexistent signals.
  #[ test ]
  fn signal_number_nonexistent()
  {
    assert_eq!( signal ::signal_number( "NONEXISTENT" ), None );
  }

  /// T10: `all_signals()` contains at least SIGHUP(1), SIGINT(2), SIGTERM(15), SIGKILL(9).
  #[ test ]
  fn all_signals_contains_standard()
  {
    let signals = signal ::all_signals();
    assert!( signals.iter().any( | &( n, name ) | n == 1 && name == "SIGHUP" ) );
    assert!( signals.iter().any( | &( n, name ) | n == 2 && name == "SIGINT" ) );
    assert!( signals.iter().any( | &( n, name ) | n == 9 && name == "SIGKILL" ) );
    assert!( signals.iter().any( | &( n, name ) | n == 15 && name == "SIGTERM" ) );
  }

  /// Round-trip consistency: `signal_name(signal_number(name))` == name.
  #[ test ]
  fn signal_round_trip_consistency()
  {
    let name = "SIGKILL";
    let number = signal ::signal_number( name ).expect( "SIGKILL must be mapped" );
    assert_eq!( signal ::signal_name( number ), name );
  }
}
