#![ allow( missing_docs ) ]

include!( "../../../../module/step/meta/src/module/terminal.rs" );

#[ allow( unused_imports ) ]
use ::process_tools as the_module;
#[ allow( unused_imports ) ]
use test_tools :: *;

#[ cfg( all( feature = "enabled", unix ) ) ]
mod inc
{
  use super :: *;
  use the_module ::lifecycle ::check;

  /// T01: Current process (`std::process::id()`) is detected as alive via `kill(pid, 0)`.
  #[ test ]
  fn is_process_alive_current_process()
  {
    let pid = i32 ::try_from( std ::process ::id() ).expect( "PID fits in i32" );
    let result = check ::is_process_alive( pid );
    assert!( result.is_ok(), "checking current process must not error" );
    assert!( result.unwrap(), "current process must be alive" );
  }

  /// T02: Negative PID returns `Err` (not a valid single-process identifier).
  #[ test ]
  fn is_process_alive_negative_pid()
  {
    let result = check ::is_process_alive( -1 );
    assert!( result.is_err(), "negative PID must return error" );
  }

  /// T03: Very large nonexistent PID returns `Ok(false)`.
  #[ test ]
  fn is_process_alive_nonexistent()
  {
    let result = check ::is_process_alive( 999_999_999 );
    assert!( result.is_ok(), "checking nonexistent PID must not error" );
    assert!( !result.unwrap(), "nonexistent PID must not be alive" );
  }

  /// T12: `wait_for_exit` returns `Ok(())` after a short-lived spawned child exits.
  #[ test ]
  fn wait_for_exit_spawned_child()
  {
    use std ::process ::Command;
    use core ::time ::Duration;

    let mut child = Command ::new( "sleep" )
      .arg( "0.1" )
      .spawn()
      .expect( "failed to spawn sleep" );

    let pid = i32 ::try_from( child.id() ).expect( "PID fits in i32" );

    // Reap the child in a background thread to prevent zombie from keeping PID alive.
    let handle = std ::thread ::spawn( move ||
    {
      let _ = child.wait();
    });

    let result = check ::wait_for_exit( pid, Duration ::from_secs( 5 ) );
    assert!( result.is_ok(), "wait_for_exit must succeed for short-lived child" );

    handle.join().expect( "reaper thread panicked" );
  }

  /// `is_pidfile_alive` returns `Ok(true)` when the file contains the current process PID.
  #[ test ]
  fn is_pidfile_alive_current_process()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "test.pid" );
    let pid = std ::process ::id();
    std ::fs ::write( &pidfile, pid.to_string() ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_ok(), "checking pidfile with current PID must not error" );
    assert!( result.unwrap(), "current process PID in file must be alive" );
  }

  /// `is_pidfile_alive` returns `Ok(false)` when the file contains a nonexistent PID.
  #[ test ]
  fn is_pidfile_alive_dead_process()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "test.pid" );
    std ::fs ::write( &pidfile, "999999999" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_ok(), "checking pidfile with dead PID must not error" );
    assert!( !result.unwrap(), "dead PID in file must not be alive" );
  }

  /// `is_pidfile_alive` returns `Err` when the file does not exist.
  #[ test ]
  fn is_pidfile_alive_missing_file()
  {
    let result = check ::is_pidfile_alive( std ::path ::Path ::new( "/nonexistent/test.pid" ) );
    assert!( result.is_err(), "missing pidfile must return error" );
  }

  // --- Corner cases: is_process_alive ---

  /// PID 0 (process group broadcast) is rejected with `InvalidInput`.
  #[ test ]
  fn is_process_alive_zero_pid()
  {
    let result = check ::is_process_alive( 0 );
    assert!( result.is_err(), "PID 0 must be rejected" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidInput );
  }

  /// PID 1 (init/systemd) is always alive, regardless of permissions.
  #[ test ]
  fn is_process_alive_pid_one()
  {
    let result = check ::is_process_alive( 1 );
    assert!( result.is_ok(), "PID 1 must not error" );
    // PID 1 is always running; kill(1, 0) returns 0 (root) or EPERM (non-root).
    // Both map to Ok(true).
    assert!( result.unwrap(), "PID 1 (init) must be alive" );
  }

  /// `i32::MAX` PID is extremely unlikely to exist.
  #[ test ]
  fn is_process_alive_i32_max()
  {
    let result = check ::is_process_alive( i32 ::MAX );
    assert!( result.is_ok(), "i32::MAX must not error" );
    assert!( !result.unwrap(), "i32::MAX PID should not exist" );
  }

  /// `i32::MIN` is rejected with `InvalidInput`.
  #[ test ]
  fn is_process_alive_i32_min()
  {
    let result = check ::is_process_alive( i32 ::MIN );
    assert!( result.is_err(), "i32::MIN must be rejected" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidInput );
  }

  // --- Corner cases: wait_for_exit ---

  /// Zero timeout on a live process returns `TimedOut` immediately.
  #[ test ]
  fn wait_for_exit_zero_timeout()
  {
    use core ::time ::Duration;
    let pid = i32 ::try_from( std ::process ::id() ).expect( "PID fits in i32" );
    let start = std ::time ::Instant ::now();
    let result = check ::wait_for_exit( pid, Duration ::ZERO );
    let elapsed = start.elapsed();
    assert!( result.is_err(), "zero timeout on live process must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::TimedOut );
    // Must return nearly instantly (no 50ms sleep).
    assert!(
      elapsed < Duration ::from_millis( 10 ),
      "zero timeout must not sleep, took {elapsed:?}"
    );
  }

  /// Negative PID propagates `Err` from `is_process_alive` through `wait_for_exit`.
  #[ test ]
  fn wait_for_exit_negative_pid_propagates_error()
  {
    use core ::time ::Duration;
    let result = check ::wait_for_exit( -1, Duration ::from_secs( 1 ) );
    assert!( result.is_err(), "negative PID must propagate error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidInput );
  }

  /// Zero PID propagates `Err` through `wait_for_exit`.
  #[ test ]
  fn wait_for_exit_zero_pid_propagates_error()
  {
    use core ::time ::Duration;
    let result = check ::wait_for_exit( 0, Duration ::from_secs( 1 ) );
    assert!( result.is_err() );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidInput );
  }

  /// `wait_for_exit` with dead PID returns `Ok` immediately.
  #[ test ]
  fn wait_for_exit_dead_pid_returns_ok()
  {
    use core ::time ::Duration;
    let result = check ::wait_for_exit( 999_999_999, Duration ::from_secs( 5 ) );
    assert!( result.is_ok(), "dead PID must return Ok(())" );
  }

  // --- Corner cases: is_pidfile_alive ---

  /// Empty pidfile content returns Err(InvalidData).
  #[ test ]
  fn is_pidfile_alive_empty_file()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "empty.pid" );
    std ::fs ::write( &pidfile, "" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "empty pidfile must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidData );
  }

  /// Non-numeric pidfile content returns Err(InvalidData).
  #[ test ]
  fn is_pidfile_alive_non_numeric()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "bad.pid" );
    std ::fs ::write( &pidfile, "abc" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "non-numeric pidfile must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidData );
  }

  /// Pidfile with "0" parses but `is_process_alive` rejects PID 0.
  #[ test ]
  fn is_pidfile_alive_zero_pid()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "zero.pid" );
    std ::fs ::write( &pidfile, "0" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "PID 0 from file must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidInput );
  }

  /// Pidfile with "-1" (negative) parses as `i32` but `is_process_alive` rejects it.
  #[ test ]
  fn is_pidfile_alive_negative_pid()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "neg.pid" );
    std ::fs ::write( &pidfile, "-1" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "negative PID from file must error" );
  }

  /// Pidfile with overflow value (exceeds i32) returns Err(InvalidData).
  #[ test ]
  fn is_pidfile_alive_overflow()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "big.pid" );
    std ::fs ::write( &pidfile, "99999999999999" ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "overflow PID must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidData );
  }

  /// Pidfile with whitespace-only content returns Err(InvalidData).
  #[ test ]
  fn is_pidfile_alive_whitespace_only()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "ws.pid" );
    std ::fs ::write( &pidfile, "   \n  " ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_err(), "whitespace-only pidfile must error" );
    assert_eq!( result.unwrap_err().kind(), std ::io ::ErrorKind ::InvalidData );
  }

  /// Pidfile with whitespace-padded valid PID parses correctly (trim handles it).
  #[ test ]
  fn is_pidfile_alive_whitespace_padded()
  {
    let dir = assert_fs ::TempDir ::new().unwrap();
    let pidfile = dir.path().join( "padded.pid" );
    let pid = std ::process ::id();
    std ::fs ::write( &pidfile, format!( "  {pid}  \n" ) ).unwrap();
    let result = check ::is_pidfile_alive( &pidfile );
    assert!( result.is_ok(), "whitespace-padded pidfile must parse OK" );
    assert!( result.unwrap(), "current PID (padded) must be alive" );
  }
}
