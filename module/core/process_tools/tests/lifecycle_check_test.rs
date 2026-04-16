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
}
