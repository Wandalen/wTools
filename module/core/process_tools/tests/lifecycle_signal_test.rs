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

  // --- Corner cases ---

  /// Signal 0 (null signal used by `kill(pid, 0)`) is not in the table.
  #[ test ]
  fn signal_name_zero_returns_unknown()
  {
    assert_eq!( signal ::signal_name( 0 ), "UNKNOWN" );
  }

  /// Negative signal number returns "UNKNOWN".
  #[ test ]
  fn signal_name_negative_returns_unknown()
  {
    assert_eq!( signal ::signal_name( -1 ), "UNKNOWN" );
  }

  /// Signal 26 (one past the last entry) returns "UNKNOWN".
  #[ test ]
  fn signal_name_boundary_26_returns_unknown()
  {
    assert_eq!( signal ::signal_name( 26 ), "UNKNOWN" );
  }

  /// `i32::MAX` returns "UNKNOWN".
  #[ test ]
  fn signal_name_i32_max_returns_unknown()
  {
    assert_eq!( signal ::signal_name( i32 ::MAX ), "UNKNOWN" );
  }

  /// Empty string returns `None` for `signal_number`.
  #[ test ]
  fn signal_number_empty_string()
  {
    assert_eq!( signal ::signal_number( "" ), None );
  }

  /// Lowercase "sigkill" returns None (case-sensitive matching).
  #[ test ]
  fn signal_number_lowercase_returns_none()
  {
    assert_eq!( signal ::signal_number( "sigkill" ), None );
  }

  /// "KILL" without SIG prefix returns None.
  #[ test ]
  fn signal_number_without_sig_prefix()
  {
    assert_eq!( signal ::signal_number( "KILL" ), None );
  }

  /// "SIG" prefix alone returns None.
  #[ test ]
  fn signal_number_sig_prefix_only()
  {
    assert_eq!( signal ::signal_number( "SIG" ), None );
  }

  /// Table has exactly 25 entries.
  #[ test ]
  fn all_signals_exact_count()
  {
    assert_eq!( signal ::all_signals().len(), 25 );
  }

  /// No duplicate signal numbers in the table.
  #[ test ]
  fn all_signals_no_duplicate_numbers()
  {
    let signals = signal ::all_signals();
    let mut numbers = std ::collections ::HashSet ::new();
    for &( num, _ ) in signals
    {
      assert!( numbers.insert( num ), "duplicate signal number: {num}" );
    }
  }

  /// No duplicate signal names in the table.
  #[ test ]
  fn all_signals_no_duplicate_names()
  {
    let signals = signal ::all_signals();
    let mut names = std ::collections ::HashSet ::new();
    for &( _, name ) in signals
    {
      assert!( names.insert( name ), "duplicate signal name: {name}" );
    }
  }

  /// Round-trip consistency for ALL signals in the table.
  #[ test ]
  fn signal_round_trip_all()
  {
    for &( num, name ) in signal ::all_signals()
    {
      assert_eq!(
        signal ::signal_name( num ), name,
        "forward lookup failed for signal {num}"
      );
      assert_eq!(
        signal ::signal_number( name ), Some( num ),
        "reverse lookup failed for signal {name}"
      );
    }
  }
}
